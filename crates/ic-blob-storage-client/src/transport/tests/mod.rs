use super::*;
use crate::tenants::TenantClientError;
use ic_platform::cdk::call::{CallPerformFailed, CallRejected, InsufficientLiquidCycleBalance};

#[test]
fn remote_rejections_preserve_raw_codes_including_unknown_outcomes() {
    // A bounded-wait unknown outcome must never become proof of non-submission.
    // Future platform codes must also survive without enum conversion or fallback.
    for code in [1, 2, 3, 4, 5, 6, 9, u32::MAX] {
        let error = CallFailed::CallRejected(CallRejected::with_rejection(
            code,
            "remote outcome".to_owned(),
        ));
        assert_eq!(
            failure(
                ClientCallError::Transport(error),
                TenantClientError::NotEnqueued,
                TenantClientError::Rejected,
            ),
            TenantClientError::Rejected(code),
        );
    }
}

#[test]
fn only_local_encoding_and_submission_failures_project_to_not_enqueued() {
    for error in [
        ClientCallError::Encoding,
        ClientCallError::Transport(CallFailed::InsufficientLiquidCycleBalance(
            InsufficientLiquidCycleBalance {
                available: 1,
                required: 2,
            },
        )),
        ClientCallError::Transport(CallFailed::CallPerformFailed(CallPerformFailed)),
    ] {
        assert_eq!(
            failure(
                error,
                TenantClientError::NotEnqueued,
                TenantClientError::Rejected,
            ),
            TenantClientError::NotEnqueued,
        );
    }
}
