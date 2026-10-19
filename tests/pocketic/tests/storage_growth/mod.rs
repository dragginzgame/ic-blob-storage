//! Real IC rollback after refused backing growth; no provider effect is involved.
use super::*;
use crate::support::REPLY_LIMITS;
use blob_test_protocol::storage::GrowthAdmission;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionMutation;

#[test]
fn refused_backing_growth_rolls_back_admission_and_neighbor_then_exact_retry_succeeds() {
    let f = Fixture::new();
    f.enroll(None, true).unwrap();
    let (permission, _) = f.permission(1, 1);
    let before = f.harness.pic.get_stable_memory(f.service);
    let status = f.local_status();
    let input = GrowthAdmission {
        permission: admission_input(permission),
        refuse: true,
    };
    let error = f
        .harness
        .pic
        .update_call(
            f.service,
            f.tenant,
            "admit_with_growth",
            ic_blob_storage_pocketic_tests::control_calls::encode_control_one(input).unwrap(),
        )
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert!(
        error
            .reject_message
            .contains("fixture backing growth refused"),
        "expected actual backing refusal: {error:?}"
    );
    // Includes the populated independent neighbor and all manager/ledger metadata.
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert_eq!(f.local_status(), status);
    assert_eq!(
        f.lookup(f.tenant, permission.request),
        Err(Failure::Unknown)
    );
    let result: Result<UploadAdmissionMutation, UploadAdmissionFailure> =
        ic_blob_storage_pocketic_tests::control_calls::update_control_args(
            &f.harness.pic,
            f.service,
            f.tenant,
            "admit_with_growth",
            (GrowthAdmission {
                refuse: false,
                ..input
            },),
            (REPLY_LIMITS).reply_bytes,
        )
        .map(|(reply,)| reply)
        .unwrap();
    result.unwrap();
    assert_eq!(f.status().operations, 1);
    assert_eq!(f.status().active, 1);
    assert_eq!(f.status().bytes, u128::from(permission.request.bytes));
    assert_eq!(f.admit(f.tenant, permission), Ok(false));
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    assert!(f.status().fenced);
    assert_eq!(f.status().operations, 1);
}
