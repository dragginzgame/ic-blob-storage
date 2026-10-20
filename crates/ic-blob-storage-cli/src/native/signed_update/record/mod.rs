//! One common dispatch schema for writing and inspecting historical signed claims.
use super::DEADLINE_SECONDS;
use crate::native::Failure;
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_platform::Principal;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

/// Facts derived by the owner that signs and dispatches the retained packet.
#[derive(Deserialize, Serialize)]
pub(super) struct SignedDispatchRecord {
    pub(super) request_id: String,
    pub(super) ingress_expiry_ns: String,
    pub(super) signed_request_sha256: String,
    pub(super) root_key_sha256: String,
    pub(super) max_update_requests: u8,
    pub(super) request_deadline_seconds: u8,
    pub(super) retry_authorized: bool,
}

/// Preserve the flat intent schema alongside operation-owned authority evidence.
#[derive(Deserialize, Serialize)]
pub(super) struct DispatchIntentRecord<R> {
    #[serde(flatten)]
    pub(super) signed: SignedDispatchRecord,
    #[serde(flatten)]
    pub(super) operation: R,
}

/// Inspect historical correspondence only; never return a dispatchable envelope.
/// Operation-owned identity and policy evidence stays with the caller.
/// Expiry is matched to the retained envelope, never interpreted as retry authority.
pub(in crate::native) fn inspect<R: DeserializeOwned>(
    intent: &[u8],
    signed: Vec<u8>,
    actor: Principal,
    service: Principal,
    method: &str,
    argument: &[u8],
    root: &[u8],
) -> Result<R, Failure> {
    let intent: DispatchIntentRecord<R> = serde_json::from_slice(intent).map_err(|error| {
        if error.is_data() {
            Failure::Binding
        } else {
            Failure::Arguments
        }
    })?;
    let metadata = intent.signed;
    if metadata.signed_request_sha256 != ContentDigest::compute(&signed).to_string()
        || metadata.root_key_sha256 != ContentDigest::compute(root).to_string()
        || metadata.max_update_requests != 1
        || metadata.request_deadline_seconds != DEADLINE_SECONDS
        || metadata.retry_authorized
    {
        return Err(Failure::Binding);
    }
    let expiry = metadata
        .ingress_expiry_ns
        .parse()
        .map_err(|_| Failure::Binding)?;
    ic_platform::agent::signed_update_inspect(
        actor.into(),
        service.into(),
        method,
        argument,
        expiry,
        signed,
    )
    .map_err(|_| Failure::Binding)?;
    Ok(intent.operation)
}
