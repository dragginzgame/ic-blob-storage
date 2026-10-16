//! Sequential passive queries, with bounded decoding and exact response binding.

use super::{
    input::{Inventory, PreparedBlob},
    selection::Selection,
};
use crate::operator::{
    Failure,
    ops::{Report, decode},
    transport::query_target,
};
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityFailure;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityRequest;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityResponse;
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityFailure;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityResponse;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryFailure;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryRequest;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryResponse;
use ic_blob_storage_contracts::dto::upload::history::UploadContentState;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryEntry;
use ic_testkit_candid::DecodeLimits;
use serde_json::{Value, json};

fn decode_capacity(scope: TenantScope, bytes: &[u8]) -> Result<UploadCapacityResponse, Failure> {
    use ic_blob_storage_contracts::upload::capacity::reply::{
        UploadCapacityReplyError as E, decode,
    };
    decode(scope, bytes, 4096.try_into().expect("positive reply bound")).map_err(
        |error| match error {
            E::Limit => Failure::ReplyTooLarge,
            E::Invalid => Failure::InvalidReply,
            E::Binding => Failure::Binding,
            E::Remote(error) => match error {
                UploadCapacityFailure::Binding => Failure::Binding,
                UploadCapacityFailure::Denied | UploadCapacityFailure::NotEnrolled => {
                    Failure::Denied
                }
                UploadCapacityFailure::Invalid | UploadCapacityFailure::Internal => {
                    Failure::InvalidReply
                }
            },
        },
    )
}
pub(super) fn query(selection: &Selection, inventory: &Inventory) -> Result<Report, Failure> {
    inspect(selection, inventory, |method, args| {
        query_target(&selection.target, method, args)
    })
}

fn inspect(
    selection: &Selection,
    inventory: &Inventory,
    mut query: impl FnMut(&str, Vec<u8>) -> Result<Vec<u8>, Failure>,
) -> Result<Report, Failure> {
    let capacity = decode_capacity(
        selection.scope,
        &query(
            "blob_upload_capacity",
            candid::encode_one(selection.scope).expect("capacity scope"),
        )?,
    )?;
    let mut blockers = Vec::new();
    if capacity.fenced {
        blockers.push("service_fenced");
    }
    if !capacity.enrollment.active {
        blockers.push("tenant_suspended");
    }
    let mut unseen_objects = 0u64;
    let mut unseen_bytes = 0u128;
    let mut unseen_chunks = 0u64;
    let mut contents = Vec::new();
    for blob in &inventory.blobs {
        let lookup = UploadDiscoveryRequest {
            scope: selection.scope,
            root: *blob.root.as_bytes(),
        };
        let observed = discover(lookup, &mut query)?;
        if observed.fenced {
            blockers.push("service_fenced");
        }
        let mut entry = match observed.content {
            None => {
                unseen_objects += 1;
                unseen_bytes += u128::from(blob.bytes);
                unseen_chunks += blob.chunks;
                let mut reasons = Vec::new();
                if blob.bytes > capacity.max_object_bytes {
                    reasons.push("object_bytes");
                }
                if blob.headers as u64 > capacity.max_headers
                    || blob.header_bytes as u64 > capacity.max_header_bytes
                {
                    reasons.push("metadata");
                }
                if !reasons.is_empty() {
                    blockers.push("new_object_limits");
                }
                json!({"status":"not_visible", "blockers":reasons, "admission":"not_proven",
                    "new_object_reference_capacity":"not_assessed"})
            }
            Some(content) => {
                let (entry, blocker) = existing(selection, blob, content, &mut query)?;
                blockers.extend(blocker);
                entry
            }
        };
        entry["discovery_fenced"] = json!(observed.fenced);
        contents.push(
            json!({"root":blob.root.to_string(),"assets":blob.assets,"bytes":blob.bytes.to_string(),
            "fresh_reference_demand":blob.assets.len(),"observation":entry}),
        );
    }
    blockers.extend(
        ic_blob_storage_contracts::upload::capacity::demand_blockers(
            &capacity,
            unseen_objects,
            unseen_bytes,
            unseen_chunks,
        ),
    );
    blockers.sort_unstable();
    blockers.dedup();
    Ok(Report {
        blocked: !blockers.is_empty(),
        value: json!({
            "scope":"pocketic_fixture", "caller_identity":"simulated", "mode":"query",
            "consistency":"sequential_observations", "admission":"not_proven",
            "body_verification":"not_performed", "reference_demand":"one_fresh_reference_per_asset",
            "target":{"service":selection.scope.service.to_text(),"tenant":selection.scope.tenant.to_text(),
                "namespace":selection.scope.namespace.to_string(),"server":selection.target.server.to_string(),"instance":selection.target.instance},
            "capacity":{"remaining_objects":capacity.remaining_objects,"remaining_active_uploads":capacity.remaining_active_uploads,
                "remaining_manifest_chunks":capacity.remaining_manifest_chunks,"remaining_bytes":capacity.remaining_bytes.to_string(),
                "remaining_logical_bytes":capacity.remaining_logical_bytes.to_string(),"remaining_physical_bytes":capacity.remaining_physical_bytes.to_string(),"remaining_liability_bytes":capacity.remaining_liability_bytes.to_string(),
                "tenant_active":capacity.enrollment.active,"tenant_generation":capacity.enrollment.generation.to_string(),"fenced":capacity.fenced},
            "not_visible_demand":{"objects":unseen_objects,"bytes":unseen_bytes.to_string(),"manifest_chunks":unseen_chunks},
            "blockers":blockers,"contents":contents,
        }),
    })
}

fn existing(
    selection: &Selection,
    blob: &PreparedBlob,
    content: UploadHistoryEntry,
    query: &mut impl FnMut(&str, Vec<u8>) -> Result<Vec<u8>, Failure>,
) -> Result<(Value, Option<&'static str>), Failure> {
    if content.request.bytes != blob.bytes {
        return Err(Failure::Binding);
    }
    let mut entry = json!({"state":format!("{:?}",content.state),
        "original_operation":content.request.upload.to_string(),
        "original_object":content.request.object.to_string(),
        "original_incarnation":content.request.incarnation.to_string(),
        "original_first_reference":content.request.first_reference.to_string()});
    let blocker = match content.state {
        UploadContentState::Live => {
            let lookup = ReferenceCapacityRequest {
                scope: selection.scope,
                root: content.request.root,
            };
            let response: ReferenceCapacityResponse =
                decode::<Result<_, ReferenceCapacityFailure>>(
                    &query(
                        "blob_reference_capacity",
                        candid::encode_one(lookup).expect("content scope"),
                    )?,
                    DecodeLimits {
                        reply_bytes: 4096,
                        decoding_quota: 100_000,
                        skipping_quota: 1000,
                        type_entries: 64,
                        header_bytes: 4096,
                    },
                )?
                .map_err(|error| match error {
                    ReferenceCapacityFailure::Binding => Failure::Binding,
                    ReferenceCapacityFailure::Denied => Failure::Denied,
                    ReferenceCapacityFailure::Invalid | ReferenceCapacityFailure::Internal => {
                        Failure::InvalidReply
                    }
                })?;
            if response.request != lookup {
                return Err(Failure::Binding);
            }
            let available = response.headroom.ok_or(Failure::Conflict)?;
            if available.fresh_retains
                > available
                    .reference_slots
                    .min(available.unreserved_receipts / 2)
            {
                return Err(Failure::InvalidReply);
            }
            entry["status"] = json!("live_requires_retain");
            entry["fresh_retains"] = json!(available.fresh_retains);
            entry["fenced"] = json!(response.fenced);
            if response.fenced {
                Some("service_fenced")
            } else {
                (available.fresh_retains < blob.assets.len() as u64).then_some("reference_capacity")
            }
        }
        UploadContentState::Reserved | UploadContentState::ExposurePossible => {
            entry["status"] = json!("recover_existing_operation");
            Some("recover_existing_operation")
        }
        UploadContentState::Cancelled
        | UploadContentState::DeletionPending
        | UploadContentState::ProviderDeleted
        | UploadContentState::Settled => {
            entry["status"] = json!("retired_root");
            Some("retired_root")
        }
    };
    Ok((entry, blocker))
}

fn discover(
    request: UploadDiscoveryRequest,
    query: &mut impl FnMut(&str, Vec<u8>) -> Result<Vec<u8>, Failure>,
) -> Result<UploadDiscoveryResponse, Failure> {
    use ic_blob_storage_contracts::upload::discovery::reply::{
        UploadDiscoveryReplyError as E, decode,
    };
    let bytes = query(
        "blob_lookup_content",
        candid::encode_one(request).expect("content scope"),
    )?;
    let response = decode(
        request,
        &bytes,
        4096.try_into().expect("positive reply bound"),
    )
    .map_err(|error| match error {
        E::Limit => Failure::ReplyTooLarge,
        E::Invalid => Failure::InvalidReply,
        E::Binding | E::Content => Failure::Binding,
        E::Remote(error) => match error {
            UploadDiscoveryFailure::Binding => Failure::Binding,
            UploadDiscoveryFailure::Denied => Failure::Denied,
            UploadDiscoveryFailure::Invalid | UploadDiscoveryFailure::Internal => {
                Failure::InvalidReply
            }
        },
    })?;
    Ok(response)
}

#[cfg(test)]
mod tests;
