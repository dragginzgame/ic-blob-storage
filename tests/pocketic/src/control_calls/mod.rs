//! Explicit Wire document calls to private consumer controls; never a Blob service fallback.
/// Typed local calls to the private consumer probe control interface.
pub trait ControlCallExt {
    /// Execute one raw read-only call and decode its exact bounded reply.
    /// # Errors
    /// Preserves request encoding, platform rejection and reply decoding failures.
    fn query_control_as<R: serde::Serialize + serde::de::DeserializeOwned, A: serde::Serialize>(
        &self,
        canister: impl Into<ic_platform::principal::ExternalPrincipal>,
        caller: impl Into<ic_platform::principal::ExternalPrincipal>,
        method: &str,
        argument: A,
        max_reply_bytes: usize,
    ) -> Result<R, ControlCallError>;
    /// Execute one replicated call without retry and decode its bounded reply.
    /// # Errors
    /// Preserves failures without treating an unusable reply as absence of effects.
    fn update_control_as<R: serde::Serialize + serde::de::DeserializeOwned, A: serde::Serialize>(
        &self,
        canister: impl Into<ic_platform::principal::ExternalPrincipal>,
        caller: impl Into<ic_platform::principal::ExternalPrincipal>,
        method: &str,
        argument: A,
        max_reply_bytes: usize,
    ) -> Result<R, ControlCallError>;
}
/// Codec failures and platform rejection remain distinct.
#[derive(Debug)]
pub enum ControlCallError {
    /// Request encoding refused before dispatch.
    Encoding(ic_blob_storage_contracts::document::Error),
    /// Platform rejection from this call.
    Rejected(ic_platform::pocket_ic::RejectResponse),
    /// Reply admission or decoding refused after dispatch.
    Decoding(ic_blob_storage_contracts::document::Error),
}
impl ControlCallExt for ic_platform::pocket_ic::PocketIc {
    fn query_control_as<R: serde::Serialize + serde::de::DeserializeOwned, A: serde::Serialize>(
        &self,
        canister: impl Into<ic_platform::principal::ExternalPrincipal>,
        caller: impl Into<ic_platform::principal::ExternalPrincipal>,
        method: &str,
        argument: A,
        max_reply_bytes: usize,
    ) -> Result<R, ControlCallError> {
        let args = ic_blob_storage_contracts::document::encode(argument)
            .map_err(ControlCallError::Encoding)?;
        let reply = self
            .query_call(canister.into(), caller.into(), method, args)
            .map_err(ControlCallError::Rejected)?;
        ic_blob_storage_contracts::document::decode(&reply, max_reply_bytes)
            .map_err(ControlCallError::Decoding)
    }
    fn update_control_as<R: serde::Serialize + serde::de::DeserializeOwned, A: serde::Serialize>(
        &self,
        canister: impl Into<ic_platform::principal::ExternalPrincipal>,
        caller: impl Into<ic_platform::principal::ExternalPrincipal>,
        method: &str,
        argument: A,
        max_reply_bytes: usize,
    ) -> Result<R, ControlCallError> {
        let args = ic_blob_storage_contracts::document::encode(argument)
            .map_err(ControlCallError::Encoding)?;
        let reply = self
            .update_call(canister.into(), caller.into(), method, args)
            .map_err(ControlCallError::Rejected)?;
        ic_blob_storage_contracts::document::decode(&reply, max_reply_bytes)
            .map_err(ControlCallError::Decoding)
    }
}

impl std::fmt::Display for ControlCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encoding(e) => write!(f, "request: {e}"),
            Self::Rejected(e) => write!(f, "platform rejection: {e:?}"),
            Self::Decoding(e) => write!(f, "reply: {e}"),
        }
    }
}
impl std::error::Error for ControlCallError {}

/// Encode a single private storage-control argument in its tuple envelope.
/// # Errors
/// Refuses values outside the document codec budgets.
pub fn encode_control_one<T: serde::Serialize>(
    value: T,
) -> Result<Vec<u8>, ic_blob_storage_contracts::document::Error> {
    ic_blob_storage_contracts::document::encode((value,))
}
/// Decode a single private storage-control result from its tuple envelope.
/// # Errors
/// Refuses malformed, trailing, foreign or over-budget replies.
pub fn decode_control_one<T: serde::Serialize + serde::de::DeserializeOwned>(
    bytes: &[u8],
    max: usize,
) -> Result<T, ic_blob_storage_contracts::document::Error> {
    ic_blob_storage_contracts::document::decode::<(T,)>(bytes, max).map(|(value,)| value)
}
