//! Explicit calls using each method's declared codec; no server setup, authority, retry or lifecycle policy.
/// Typed local calls to the current Blob-owned interface.
pub trait ServiceCallExt {
    /// Execute one raw read-only call and decode its exact bounded reply.
    /// # Errors
    /// Preserves request encoding, platform rejection and reply decoding failures.
    fn query_blob_as<R: serde::Serialize + serde::de::DeserializeOwned, A: serde::Serialize>(
        &self,
        canister: impl Into<ic_platform::principal::ExternalPrincipal>,
        caller: impl Into<ic_platform::principal::ExternalPrincipal>,
        method: &str,
        argument: A,
        max_reply_bytes: usize,
    ) -> Result<R, ServiceCallError>;
    /// Execute one replicated call without retry and decode its bounded reply.
    /// # Errors
    /// Preserves failures without treating an unusable reply as absence of effects.
    fn update_blob_as<R: serde::Serialize + serde::de::DeserializeOwned, A: serde::Serialize>(
        &self,
        canister: impl Into<ic_platform::principal::ExternalPrincipal>,
        caller: impl Into<ic_platform::principal::ExternalPrincipal>,
        method: &str,
        argument: A,
        max_reply_bytes: usize,
    ) -> Result<R, ServiceCallError>;
}
/// Codec failures and platform rejection remain distinct.
#[derive(Debug)]
pub enum ServiceCallError {
    /// Request encoding refused before dispatch.
    Encoding(ic_blob_storage_contracts::wire::Error),
    /// Platform rejection from this call.
    Rejected(ic_platform::pocket_ic::RejectResponse),
    /// Reply admission or decoding refused after dispatch.
    Decoding(ic_blob_storage_contracts::wire::Error),
}
impl ServiceCallExt for ic_platform::pocket_ic::PocketIc {
    fn query_blob_as<R: serde::Serialize + serde::de::DeserializeOwned, A: serde::Serialize>(
        &self,
        canister: impl Into<ic_platform::principal::ExternalPrincipal>,
        caller: impl Into<ic_platform::principal::ExternalPrincipal>,
        method: &str,
        argument: A,
        max_reply_bytes: usize,
    ) -> Result<R, ServiceCallError> {
        let args = encode_argument(method, argument).map_err(ServiceCallError::Encoding)?;
        let reply = self
            .query_call(canister.into(), caller.into(), method, args)
            .map_err(ServiceCallError::Rejected)?;
        decode_reply(method, &reply, max_reply_bytes).map_err(ServiceCallError::Decoding)
    }
    fn update_blob_as<R: serde::Serialize + serde::de::DeserializeOwned, A: serde::Serialize>(
        &self,
        canister: impl Into<ic_platform::principal::ExternalPrincipal>,
        caller: impl Into<ic_platform::principal::ExternalPrincipal>,
        method: &str,
        argument: A,
        max_reply_bytes: usize,
    ) -> Result<R, ServiceCallError> {
        let args = encode_argument(method, argument).map_err(ServiceCallError::Encoding)?;
        let reply = self
            .update_call(canister.into(), caller.into(), method, args)
            .map_err(ServiceCallError::Rejected)?;
        decode_reply(method, &reply, max_reply_bytes).map_err(ServiceCallError::Decoding)
    }
}

impl std::fmt::Display for ServiceCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encoding(e) => write!(f, "request: {e}"),
            Self::Rejected(e) => write!(f, "platform rejection: {e:?}"),
            Self::Decoding(e) => write!(f, "reply: {e}"),
        }
    }
}
impl std::error::Error for ServiceCallError {}

// Test-only native DTO projection for this generic fixture interface. The Wire document here
// is an in-process Serde bridge, never a tenant endpoint fallback or wire packet.
fn encode_argument<A: serde::Serialize>(
    method: &str,
    argument: A,
) -> Result<Vec<u8>, ic_blob_storage_contracts::wire::Error> {
    use ic_blob_storage_contracts::{protocol, wire};
    if !matches!(
        method,
        protocol::TENANT_INSPECTION_METHOD | protocol::TENANT_UPDATE_METHOD
    ) {
        return wire::encode_request(method, argument);
    }
    let bytes = ic_blob_storage_contracts::document::encode(argument)
        .map_err(|_| wire::Error::Malformed)?;
    match method {
        protocol::TENANT_INSPECTION_METHOD => wire::tenant::inspection_request(
            ic_blob_storage_contracts::document::decode(&bytes, 4096)
                .map_err(|_| wire::Error::Malformed)?,
        )
        .map_err(|_| wire::Error::Malformed),
        protocol::TENANT_UPDATE_METHOD => wire::tenant::update_request(
            ic_blob_storage_contracts::document::decode(&bytes, 4096)
                .map_err(|_| wire::Error::Malformed)?,
        )
        .map_err(|_| wire::Error::Malformed),
        _ => Ok(bytes),
    }
}
fn decode_reply<R: serde::Serialize + serde::de::DeserializeOwned>(
    method: &str,
    bytes: &[u8],
    max: usize,
) -> Result<R, ic_blob_storage_contracts::wire::Error> {
    use ic_blob_storage_contracts::{protocol, wire};
    if bytes.len() > max {
        return Err(wire::Error::Limit);
    }
    let value = match method {
        protocol::TENANT_INSPECTION_METHOD => wire::tenant::decode_inspection_response(bytes),
        protocol::TENANT_UPDATE_METHOD => wire::tenant::decode_update_response(bytes),
        _ => return wire::decode_response(method, bytes, max),
    }
    .map_err(|_| wire::Error::Malformed)?;
    ic_blob_storage_contracts::document::decode(
        &ic_blob_storage_contracts::document::encode(value).map_err(|_| wire::Error::Malformed)?,
        wire::MAX_MESSAGE_BYTES,
    )
    .map_err(|_| wire::Error::Malformed)
}
