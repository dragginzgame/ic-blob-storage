//! Explicit calls using each method's declared codec; no server setup, authority, retry or lifecycle policy.
/// Typed local calls to the current Blob-owned interface.
pub trait ServiceCallExt {
    /// Execute one raw read-only call and decode its exact bounded reply.
    /// # Errors
    /// Preserves request encoding, platform rejection and reply decoding failures.
    fn query_blob_as<R: serde::de::DeserializeOwned, A: serde::Serialize>(
        &self,
        canister: ic_platform::Principal,
        caller: ic_platform::Principal,
        method: &str,
        argument: A,
        max_reply_bytes: usize,
    ) -> Result<R, ServiceCallError>;
    /// Execute one replicated call without retry and decode its bounded reply.
    /// # Errors
    /// Preserves failures without treating an unusable reply as absence of effects.
    fn update_blob_as<R: serde::de::DeserializeOwned, A: serde::Serialize>(
        &self,
        canister: ic_platform::Principal,
        caller: ic_platform::Principal,
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
    Rejected(ic_testkit::pocket_ic::RejectResponse),
    /// Reply admission or decoding refused after dispatch.
    Decoding(ic_blob_storage_contracts::wire::Error),
}
impl ServiceCallExt for ic_testkit::pocket_ic::PocketIc {
    fn query_blob_as<R: serde::de::DeserializeOwned, A: serde::Serialize>(
        &self,
        canister: ic_platform::Principal,
        caller: ic_platform::Principal,
        method: &str,
        argument: A,
        max_reply_bytes: usize,
    ) -> Result<R, ServiceCallError> {
        let args = encode_argument(method, argument).map_err(ServiceCallError::Encoding)?;
        let reply = self
            .query_call(canister, caller, method, args)
            .map_err(ServiceCallError::Rejected)?;
        decode_reply(method, &reply, max_reply_bytes).map_err(ServiceCallError::Decoding)
    }
    fn update_blob_as<R: serde::de::DeserializeOwned, A: serde::Serialize>(
        &self,
        canister: ic_platform::Principal,
        caller: ic_platform::Principal,
        method: &str,
        argument: A,
        max_reply_bytes: usize,
    ) -> Result<R, ServiceCallError> {
        let args = encode_argument(method, argument).map_err(ServiceCallError::Encoding)?;
        let reply = self
            .update_call(canister, caller, method, args)
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

// Test-only native DTO projection for this generic fixture interface. CBOR here
// is an in-process Serde bridge, never a tenant endpoint fallback or wire packet.
fn encode_argument<A: serde::Serialize>(
    method: &str,
    argument: A,
) -> Result<Vec<u8>, ic_blob_storage_contracts::wire::Error> {
    use ic_blob_storage_contracts::{protocol, wire};
    let bytes = wire::encode(argument)?;
    match method {
        protocol::TENANT_INSPECTION_METHOD => {
            wire::tenant::inspection_request(wire::decode(&bytes, 4096)?)
                .map_err(|_| wire::Error::Invalid)
        }
        protocol::TENANT_UPDATE_METHOD => wire::tenant::update_request(wire::decode(&bytes, 4096)?)
            .map_err(|_| wire::Error::Invalid),
        _ => Ok(bytes),
    }
}
fn decode_reply<R: serde::de::DeserializeOwned>(
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
        _ => return wire::decode(bytes, max),
    }
    .map_err(|_| wire::Error::Invalid)?;
    wire::decode(&wire::encode(value)?, wire::MAX_MESSAGE_BYTES)
}
