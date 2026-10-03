//! Strict reply correlation and conservative planning over sequential observations.
use super::{Failure, Run};
use crate::native::upload_inputs::PreparedInput;
use candid::{CandidType, DecoderConfig, Deserialize, Principal, decode_one_with_config};
use ic_blob_storage::dto::{
    tenant::TenantScope,
    upload::{
        capacity::{UploadCapacityFailure, UploadCapacityResponse},
        discovery::{UploadDiscoveryFailure, UploadDiscoveryRequest, UploadDiscoveryResponse},
        history::{UploadContentState, UploadHistoryEntry},
    },
};
use ic_blob_storage::ops::service::uploads::{
    capacity::UPLOAD_CAPACITY_METHOD, discovery::UPLOAD_DISCOVERY_METHOD,
};
use serde_json::{Value, json};
use std::{collections::BTreeSet, future::Future};

pub(in crate::native) fn scope(
    inputs: &[PreparedInput],
    service: Principal,
    namespace: u128,
    actor: Principal,
) -> Result<TenantScope, Failure> {
    let first = inputs.first().ok_or(Failure::Arguments)?.permission.upload;
    if first.service != service || first.namespace != namespace {
        return Err(Failure::Binding);
    }
    if first.tenant != actor {
        return Err(Failure::Denied);
    }
    Ok(TenantScope {
        service,
        namespace,
        tenant: actor,
    })
}
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
    decode_one_with_config(bytes, &config).map_err(|_| Failure::InvalidReply)
}
fn capacity(scope: TenantScope, bytes: &[u8]) -> Result<UploadCapacityResponse, Failure> {
    let r: Result<UploadCapacityResponse, UploadCapacityFailure> = decode(bytes)?;
    let r = r.map_err(|e| match e {
        UploadCapacityFailure::Binding => Failure::Binding,
        UploadCapacityFailure::Denied => Failure::Denied,
        UploadCapacityFailure::NotEnrolled => Failure::NotEnrolled,
        UploadCapacityFailure::Invalid => Failure::ServiceInvalid,
        UploadCapacityFailure::Internal => Failure::ServiceInternal,
    })?;
    if r.scope != scope {
        return Err(Failure::Binding);
    }
    if r.enrollment.generation == 0
        || r.max_object_bytes == 0
        || r.max_headers == 0
        || r.max_header_bytes == 0
    {
        return Err(Failure::InvalidReply);
    }
    Ok(r)
}
fn discovery(
    request: UploadDiscoveryRequest,
    bytes: &[u8],
) -> Result<UploadDiscoveryResponse, Failure> {
    let r: Result<UploadDiscoveryResponse, UploadDiscoveryFailure> = decode(bytes)?;
    let r = r.map_err(|e| match e {
        UploadDiscoveryFailure::Binding => Failure::Binding,
        UploadDiscoveryFailure::Denied => Failure::Denied,
        UploadDiscoveryFailure::Invalid => Failure::ServiceInvalid,
        UploadDiscoveryFailure::Internal => Failure::ServiceInternal,
    })?;
    if r.request != request {
        return Err(Failure::Binding);
    }
    if let Some(c) = r.content {
        let u = c.request;
        if u.service != request.scope.service
            || u.namespace != request.scope.namespace
            || u.tenant != request.scope.tenant
            || u.root != request.root
        {
            return Err(Failure::Binding);
        }
        if u.upload == 0
            || u.object == 0
            || u.incarnation == 0
            || u.first_reference == 0
            || u.bytes == 0
        {
            return Err(Failure::InvalidReply);
        }
    }
    Ok(r)
}
async fn observe<Q, F>(
    run: &Run,
    index: usize,
    method: &'static str,
    args: Vec<u8>,
    query: &mut Q,
) -> Result<Vec<u8>, Failure>
where
    Q: FnMut(&'static str, Vec<u8>) -> F,
    F: Future<Output = Result<Vec<u8>, Failure>>,
{
    run.bytes(&format!("query-{index:04}-args.candid"), &args)?;
    run.json(&format!("query-{index:04}-intent.json"),&json!({"method":method,"argument_sha256":crate::native::upload_inputs::digest(&args),"updates":0,"automatic_retries":0}))?;
    let bytes = query(method, args).await?;
    run.bytes(&format!("query-{index:04}-reply.candid"), &bytes)?;
    Ok(bytes)
}
pub(in crate::native) async fn inspect<Q, F>(
    scope: TenantScope,
    inputs: &[PreparedInput],
    run: &Run,
    mut query: Q,
) -> Result<Value, Failure>
where
    Q: FnMut(&'static str, Vec<u8>) -> F,
    F: Future<Output = Result<Vec<u8>, Failure>>,
{
    let c = capacity(
        scope,
        &observe(
            run,
            0,
            UPLOAD_CAPACITY_METHOD,
            candid::encode_one(scope).map_err(|_| Failure::Arguments)?,
            &mut query,
        )
        .await?,
    )?;
    let mut blockers = BTreeSet::new();
    if c.fenced {
        blockers.insert("service_fenced");
    }
    if !c.enrollment.active {
        blockers.insert("tenant_suspended");
    }
    let mut demand_bytes = 0u128;
    let mut demand_chunks = 0u64;
    let mut demand_objects = 0u64;
    let mut roots = BTreeSet::new();
    let mut entries = Vec::new();
    for (index, input) in inputs.iter().enumerate() {
        let p = input.permission;
        let request = UploadDiscoveryRequest {
            scope,
            root: p.upload.root,
        };
        let r = discovery(
            request,
            &observe(
                run,
                index + 1,
                UPLOAD_DISCOVERY_METHOD,
                candid::encode_one(request).map_err(|_| Failure::Arguments)?,
                &mut query,
            )
            .await?,
        )?;
        if r.fenced {
            blockers.insert("service_fenced");
        }
        if !roots.insert(p.upload.root) {
            blockers.insert("duplicate_planned_root");
        }
        let (status, existing) = if let Some(content) = r.content {
            let (status, entry) = existing(content, p.upload.bytes, &mut blockers)?;
            (status, Some(entry))
        } else {
            demand_objects += 1;
            demand_bytes += u128::from(p.upload.bytes);
            demand_chunks += input.chunks() as u64;
            if p.upload.bytes > c.max_object_bytes {
                blockers.insert("object_bytes");
            }
            let (headers, bytes) = input.metadata_demand();
            if headers > c.max_headers || bytes > c.max_header_bytes {
                blockers.insert("metadata");
            }
            ("not_visible", None)
        };
        entries.push(json!({"directory":format!("file-{index:04}"),"proposed_permission":crate::native::upload_setup::permission_json(p),"status":status,"original_upload":existing,"discovery_fenced":r.fenced}));
    }
    if demand_objects > c.remaining_objects {
        blockers.insert("object_history_capacity");
    }
    if demand_chunks > c.remaining_manifest_chunks {
        blockers.insert("manifest_capacity");
    }
    if demand_bytes > c.remaining_bytes {
        blockers.insert("byte_capacity");
    }
    if demand_objects > 0 && c.remaining_active_uploads == 0 {
        blockers.insert("no_active_upload_slot");
    }
    Ok(
        json!({"schema":1,"observation":"publish_check","complete":true,
        "authentication":"query_signatures","consistency":"sequential_observations",
        "scope":{"service":scope.service.to_text(),"namespace":scope.namespace.to_string(),"tenant":scope.tenant.to_text()},
        "capacity":{"remaining_objects":c.remaining_objects,"remaining_active_uploads":c.remaining_active_uploads,"remaining_manifest_chunks":c.remaining_manifest_chunks,"remaining_bytes":c.remaining_bytes.to_string(),"max_object_bytes":c.max_object_bytes.to_string(),"tenant_active":c.enrollment.active,"tenant_generation":c.enrollment.generation.to_string(),"fenced":c.fenced},
        "not_visible_demand":{"objects":demand_objects,"manifest_chunks":demand_chunks,"bytes":demand_bytes.to_string()},
        "blockers":blockers,"blocked":!blockers.is_empty(),"files":entries,
        "queries":inputs.len()+1,"snapshot_bodies_verified":true,"identities_allocated":false,
        "capacity_reserved":false,"admission_proven":false,"installed_project_or_uploader_verified":false,
        "provider_requests":0,"service_updates":0,"retry_authorized":false,"publication_authorized":false}),
    )
}

fn existing(
    content: UploadHistoryEntry,
    bytes: u64,
    blockers: &mut BTreeSet<&'static str>,
) -> Result<(&'static str, Value), Failure> {
    if content.request.bytes != bytes {
        return Err(Failure::Binding);
    }
    let status = match content.state {
        UploadContentState::Reserved | UploadContentState::ExposurePossible => {
            "recover_existing_operation"
        }
        UploadContentState::Live => "live_requires_retain",
        UploadContentState::Cancelled
        | UploadContentState::DeletionPending
        | UploadContentState::ProviderDeleted
        | UploadContentState::Settled => "retired_root",
    };
    blockers.insert(status);
    Ok((
        status,
        crate::native::references::upload_json(content.request),
    ))
}
