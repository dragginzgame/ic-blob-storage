//! Single-call mechanics shared by the role-specific replicated clients.
use ic_platform::Principal;
use ic_platform::cdk::call::{Call, CallFailed, Response, WaitPolicy};
use std::num::NonZeroU32;

/// Admit ordinary identities and the platform's bounded-wait timeout range.
/// Request scope and actor authority remain with each client.
pub(super) fn configured(principals: &[Principal], timeout: NonZeroU32) -> bool {
    timeout.get() <= 300
        && principals
            .iter()
            .all(|p| *p != Principal::anonymous() && *p != Principal::management_canister())
}

/// Dispatch once after caller-owned binding, execution and request validation.
/// Attaches no cycles and performs no fallback or retry.
pub(super) async fn call<A: serde::Serialize>(
    service: Principal,
    method: &str,
    timeout: NonZeroU32,
    argument: A,
) -> Result<Response, ClientCallError> {
    let bytes = ic_blob_storage_contracts::wire::encode_request(method, argument)
        .map_err(|_| ClientCallError::Encoding)?;
    call_raw(service, method, timeout, &bytes).await
}
/// Send the caller's already encoded method-bound request once, without retry.
pub(super) async fn call_raw(
    service: Principal,
    method: &str,
    timeout: NonZeroU32,
    bytes: &[u8],
) -> Result<Response, ClientCallError> {
    Call::bounded_wait(service, method)
        .with_wait_policy(WaitPolicy::Bounded {
            timeout_seconds: timeout.get(),
        })
        .with_raw_args(bytes)
        .await
        .map_err(ClientCallError::Transport)
}

/// Encoding failure is local; transport failure retains its platform classification.
pub(super) enum ClientCallError {
    Encoding,
    Transport(CallFailed),
}

/// Preserve raw rejection codes and distinguish calls that could not be enqueued.
pub(super) fn failure<E>(error: ClientCallError, not_enqueued: E, rejected: fn(u32) -> E) -> E {
    match error {
        ClientCallError::Transport(CallFailed::CallRejected(error)) => {
            rejected(error.raw_reject_code())
        }
        ClientCallError::Encoding
        | ClientCallError::Transport(
            CallFailed::InsufficientLiquidCycleBalance(_) | CallFailed::CallPerformFailed(_),
        ) => not_enqueued,
    }
}

#[cfg(test)]
mod tests;
