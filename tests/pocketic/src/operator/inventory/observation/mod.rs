//! Sequential passive queries, with bounded decoding and exact response binding.

use super::{
    input::{Inventory, PreparedBlob},
    selection::Selection,
};
use crate::operator::{
    Failure,
    ops::{Report, query_target},
};
use blob_test_protocol::admission::{
    ContentLookup, ContentObservation, ContentState, Failure as RemoteFailure,
    planning::AdmissionCapacity, release::ReferenceCapacity,
};
use candid::{CandidType, DecoderConfig, Deserialize, decode_one_with_config};
use serde_json::{Value, json};

fn decode<T: CandidType + for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, Failure> {
    if bytes.len() > 4096 {
        return Err(Failure::ReplyTooLarge);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(100_000)
        .set_skipping_quota(1000)
        .set_max_type_len(64)
        .set_max_header_len(4096)
        .set_full_error_message(false);
    let result: Result<T, RemoteFailure> =
        decode_one_with_config(bytes, &config).map_err(|_| Failure::InvalidReply)?;
    result.map_err(|error| match error {
        RemoteFailure::WrongService | RemoteFailure::WrongNamespace => Failure::Binding,
        RemoteFailure::NotProject | RemoteFailure::NotEnrolled | RemoteFailure::NotObserver => {
            Failure::Denied
        }
        _ => Failure::InvalidReply,
    })
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
    let capacity: AdmissionCapacity = decode(&query(
        "admission_capacity",
        candid::encode_one(selection.scope).expect("capacity scope"),
    )?)?;
    if capacity.scope != selection.scope {
        return Err(Failure::Binding);
    }
    if capacity.enrollment.generation == 0
        || capacity.max_object_bytes == 0
        || capacity.max_headers == 0
        || capacity.max_header_bytes == 0
    {
        return Err(Failure::InvalidReply);
    }
    let mut blockers = Vec::new();
    if !capacity.enrollment.active {
        blockers.push("tenant_suspended");
    }
    let mut unseen_objects = 0u64;
    let mut unseen_bytes = 0u128;
    let mut unseen_chunks = 0u64;
    let mut contents = Vec::new();
    for blob in &inventory.blobs {
        let lookup = ContentLookup {
            service: selection.scope.service,
            tenant: selection.scope.tenant,
            namespace: selection.scope.namespace,
            root: *blob.root.as_bytes(),
        };
        let observed: Option<ContentObservation> = decode(&query(
            "lookup_content",
            candid::encode_one(lookup).expect("content scope"),
        )?)?;
        let entry = match observed {
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
        contents.push(
            json!({"root":blob.root.to_string(),"assets":blob.assets,"bytes":blob.bytes.to_string(),
            "fresh_reference_demand":blob.assets.len(),"observation":entry}),
        );
    }
    if unseen_objects > capacity.remaining_objects {
        blockers.push("object_history_capacity");
    }
    if unseen_bytes > capacity.remaining_bytes {
        blockers.push("byte_capacity");
    }
    if unseen_chunks > capacity.remaining_manifest_chunks {
        blockers.push("manifest_capacity");
    }
    // Capacity is a concurrency ceiling, not a limit on a sequential release's size.
    if unseen_objects != 0 && capacity.remaining_active_uploads == 0 {
        blockers.push("no_active_upload_slot");
    }
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
                "tenant_active":capacity.enrollment.active,"tenant_generation":capacity.enrollment.generation.to_string()},
            "not_visible_demand":{"objects":unseen_objects,"bytes":unseen_bytes.to_string(),"manifest_chunks":unseen_chunks},
            "blockers":blockers,"contents":contents,
        }),
    })
}

fn existing(
    selection: &Selection,
    blob: &PreparedBlob,
    content: ContentObservation,
    query: &mut impl FnMut(&str, Vec<u8>) -> Result<Vec<u8>, Failure>,
) -> Result<(Value, Option<&'static str>), Failure> {
    check_binding(selection, blob, content)?;
    let mut entry = json!({"state":format!("{:?}",content.state), "original_operation":content.request.id.to_string()});
    let blocker = match content.state {
        ContentState::Live => {
            let lookup = ContentLookup {
                service: content.request.service,
                tenant: content.request.tenant,
                namespace: content.request.namespace,
                root: content.request.root,
            };
            let available: Option<ReferenceCapacity> = decode(&query(
                "reference_capacity",
                candid::encode_one(lookup).expect("content scope"),
            )?)?;
            let available = available.ok_or(Failure::Conflict)?;
            if available.fresh_retains
                > available
                    .reference_slots
                    .min(available.unreserved_receipts / 2)
            {
                return Err(Failure::InvalidReply);
            }
            entry["status"] = json!("live_requires_retain");
            entry["fresh_retains"] = json!(available.fresh_retains);
            (available.fresh_retains < blob.assets.len() as u64).then_some("reference_capacity")
        }
        ContentState::Reserved | ContentState::ExposurePossible => {
            entry["status"] = json!("recover_existing_operation");
            Some("recover_existing_operation")
        }
        ContentState::Cancelled
        | ContentState::DeletionPending
        | ContentState::ProviderDeleted
        | ContentState::Settled => {
            entry["status"] = json!("retired_root");
            Some("retired_root")
        }
    };
    Ok((entry, blocker))
}

fn check_binding(
    selection: &Selection,
    blob: &PreparedBlob,
    content: ContentObservation,
) -> Result<(), Failure> {
    let request = content.request;
    if request.service != selection.scope.service
        || request.tenant != selection.scope.tenant
        || request.namespace != selection.scope.namespace
        || request.root != *blob.root.as_bytes()
        || request.bytes != blob.bytes
        || request.id == 0
    {
        return Err(Failure::Binding);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
