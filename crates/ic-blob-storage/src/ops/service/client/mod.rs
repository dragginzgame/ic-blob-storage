//! Single-call mechanics shared by the role-specific replicated clients.
use candid::{CandidType, Principal};
use ic_cdk::call::{Call, CallFailed, Response};
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
pub(super) async fn call<A: CandidType>(
    service: Principal,
    method: &str,
    timeout: NonZeroU32,
    argument: A,
) -> Result<Response, CallFailed> {
    Call::bounded_wait(service, method)
        .change_timeout(timeout.get())
        .with_arg(argument)
        .await
}

/// Preserve raw rejection codes and distinguish calls that could not be enqueued.
pub(super) fn failure<E>(error: CallFailed, not_enqueued: E, rejected: fn(u32) -> E) -> E {
    match error {
        CallFailed::CallRejected(error) => rejected(error.raw_reject_code()),
        CallFailed::InsufficientLiquidCycleBalance(_) | CallFailed::CallPerformFailed(_) => {
            not_enqueued
        }
    }
}
