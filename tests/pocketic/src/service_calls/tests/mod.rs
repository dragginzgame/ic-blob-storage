use super::*;
use ic_testkit::pocket_ic::{ClientError, ErrorCode, RejectCode, RejectResponse};

#[test]
fn typed_canister_refusal_retains_its_exact_diagnostics() {
    let refusal = RejectResponse {
        reject_code: RejectCode::CanisterError,
        reject_message: "original canister trap".into(),
        error_code: ErrorCode::CanisterCalledTrap,
        certified: false,
    };
    let failure = ClientError::Canister(refusal.clone());
    assert_eq!(canister_rejection(&failure), &refusal);
}

#[test]
#[should_panic(expected = "expected canister rejection, received client failure")]
fn protocol_failure_cannot_satisfy_a_canister_refusal_assertion() {
    let failure = ClientError::Protocol("response was not admitted".into());
    let _ = canister_rejection(&failure);
}
