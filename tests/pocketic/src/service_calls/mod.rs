//! Explicit calls using each method's declared codec; no server setup, authority, retry or lifecycle policy.
use ic_blob_storage_contracts::dto::tenant::{
    TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest,
};

/// Typed local calls to the current Blob-owned interface.
pub trait ServiceCallExt {
    /// Call the typed tenant query codec once under the caller's reply ceiling.
    /// # Errors
    /// Preserves encoding, rejection and bounded decoding failures.
    fn query_tenant_as(
        &self,
        canister: impl Into<ic_platform::principal::ExternalPrincipal>,
        caller: impl Into<ic_platform::principal::ExternalPrincipal>,
        argument: TenantScope,
        max_reply_bytes: usize,
    ) -> Result<Result<TenantEnrollmentResponse, TenantFailure>, ServiceCallError>;
    /// Call the typed tenant update codec once under the caller's reply ceiling.
    /// # Errors
    /// Preserves encoding, rejection and bounded decoding failures.
    fn update_tenant_as(
        &self,
        canister: impl Into<ic_platform::principal::ExternalPrincipal>,
        caller: impl Into<ic_platform::principal::ExternalPrincipal>,
        argument: TenantUpdateRequest,
        max_reply_bytes: usize,
    ) -> Result<Result<TenantEnrollmentResponse, TenantFailure>, ServiceCallError>;

    /// Execute one raw read-only call and decode its exact bounded reply.
    /// # Errors
    /// Preserves request encoding, platform rejection and reply decoding failures.
    fn query_blob_as<R: serde::de::DeserializeOwned, A: serde::Serialize>(
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
    fn update_blob_as<R: serde::de::DeserializeOwned, A: serde::Serialize>(
        &self,
        canister: impl Into<ic_platform::principal::ExternalPrincipal>,
        caller: impl Into<ic_platform::principal::ExternalPrincipal>,
        method: &str,
        argument: A,
        max_reply_bytes: usize,
    ) -> Result<R, ServiceCallError>;
}
/// Codec failures and typed platform failures remain distinct.
#[derive(Debug)]
pub enum ServiceCallError {
    /// Request encoding refused before dispatch.
    Encoding(ic_blob_storage_contracts::wire::Error),
    /// Typed client failure, retaining canister refusals and uncertain request context.
    Platform(ic_platform::pocket_ic::ClientError),
    /// Reply admission or decoding refused after dispatch.
    Decoding(ic_blob_storage_contracts::wire::Error),
}
impl ServiceCallExt for ic_platform::pocket_ic::PocketIc {
    fn query_tenant_as(
        &self,
        canister: impl Into<ic_platform::principal::ExternalPrincipal>,
        caller: impl Into<ic_platform::principal::ExternalPrincipal>,
        argument: TenantScope,
        max_reply_bytes: usize,
    ) -> Result<Result<TenantEnrollmentResponse, TenantFailure>, ServiceCallError> {
        use ic_blob_storage_contracts::{protocol, wire};
        let args = wire::tenant::inspection_request(argument)
            .map_err(|_| ServiceCallError::Encoding(wire::Error::Malformed))?;
        let reply = self
            .query_call(
                canister.into(),
                caller.into(),
                protocol::TENANT_INSPECTION_METHOD,
                args,
            )
            .map_err(ServiceCallError::Platform)?;
        if reply.len() > max_reply_bytes {
            return Err(ServiceCallError::Decoding(wire::Error::Limit));
        }
        wire::tenant::decode_inspection_response(&reply)
            .map_err(|_| ServiceCallError::Decoding(wire::Error::Malformed))
    }
    fn update_tenant_as(
        &self,
        canister: impl Into<ic_platform::principal::ExternalPrincipal>,
        caller: impl Into<ic_platform::principal::ExternalPrincipal>,
        argument: TenantUpdateRequest,
        max_reply_bytes: usize,
    ) -> Result<Result<TenantEnrollmentResponse, TenantFailure>, ServiceCallError> {
        use ic_blob_storage_contracts::{protocol, wire};
        let args = wire::tenant::update_request(argument)
            .map_err(|_| ServiceCallError::Encoding(wire::Error::Malformed))?;
        let reply = self
            .update_call(
                canister.into(),
                caller.into(),
                protocol::TENANT_UPDATE_METHOD,
                args,
            )
            .map_err(ServiceCallError::Platform)?;
        if reply.len() > max_reply_bytes {
            return Err(ServiceCallError::Decoding(wire::Error::Limit));
        }
        wire::tenant::decode_update_response(&reply)
            .map_err(|_| ServiceCallError::Decoding(wire::Error::Malformed))
    }

    fn query_blob_as<R: serde::de::DeserializeOwned, A: serde::Serialize>(
        &self,
        canister: impl Into<ic_platform::principal::ExternalPrincipal>,
        caller: impl Into<ic_platform::principal::ExternalPrincipal>,
        method: &str,
        argument: A,
        max_reply_bytes: usize,
    ) -> Result<R, ServiceCallError> {
        let args = ic_blob_storage_contracts::wire::encode_request(method, argument)
            .map_err(ServiceCallError::Encoding)?;
        let reply = self
            .query_call(canister.into(), caller.into(), method, args)
            .map_err(ServiceCallError::Platform)?;
        ic_blob_storage_contracts::wire::decode_response(method, &reply, max_reply_bytes)
            .map_err(ServiceCallError::Decoding)
    }
    fn update_blob_as<R: serde::de::DeserializeOwned, A: serde::Serialize>(
        &self,
        canister: impl Into<ic_platform::principal::ExternalPrincipal>,
        caller: impl Into<ic_platform::principal::ExternalPrincipal>,
        method: &str,
        argument: A,
        max_reply_bytes: usize,
    ) -> Result<R, ServiceCallError> {
        let args = ic_blob_storage_contracts::wire::encode_request(method, argument)
            .map_err(ServiceCallError::Encoding)?;
        let reply = self
            .update_call(canister.into(), caller.into(), method, args)
            .map_err(ServiceCallError::Platform)?;
        ic_blob_storage_contracts::wire::decode_response(method, &reply, max_reply_bytes)
            .map_err(ServiceCallError::Decoding)
    }
}

impl std::fmt::Display for ServiceCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encoding(e) => write!(f, "request: {e}"),
            Self::Platform(e) => write!(f, "platform: {e}"),
            Self::Decoding(e) => write!(f, "reply: {e}"),
        }
    }
}
impl std::error::Error for ServiceCallError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Platform(e) => Some(e),
            _ => None,
        }
    }
}

/// Assert a canister refusal, never misclassifying a failed/uncertain client exchange.
/// # Panics
/// Refuses transport, protocol and other client failures, retaining their diagnostic context.
#[must_use]
pub fn canister_rejection(
    failure: &ic_platform::pocket_ic::ClientError,
) -> &ic_platform::pocket_ic::RejectResponse {
    match failure {
        ic_platform::pocket_ic::ClientError::Canister(rejection) => rejection,
        other => panic!("expected canister rejection, received client failure: {other:?}"),
    }
}

#[cfg(test)]
mod tests;
