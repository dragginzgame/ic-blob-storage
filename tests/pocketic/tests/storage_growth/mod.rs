//! Real IC rollback after refused backing growth; no provider effect is involved.
use super::*;
use blob_test_protocol::storage::GrowthAdmission;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionMutation;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn refused_backing_growth_rolls_back_admission_and_neighbor_then_exact_retry_succeeds() {
    let f = Fixture::new().await;
    f.enroll(None, true).await.unwrap();
    let (permission, _) = f.permission(1, 1);
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let status = f.local_status().await;
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
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    assert!(
        crate::canister_rejection(&error)
            .reject_message
            .contains("fixture backing growth refused"),
        "expected actual backing refusal: {error:?}"
    );
    // Includes the populated independent neighbor and all manager/ledger metadata.
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    assert_eq!(f.local_status().await, status);
    assert_eq!(
        f.lookup(f.tenant, permission.request).await,
        Err(Failure::Unknown)
    );
    let result: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_control_as(
            f.service,
            f.tenant,
            "admit_with_growth",
            (GrowthAdmission {
                refuse: false,
                ..input
            },),
            REPLY_BYTES,
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
    result.unwrap();
    assert_eq!(f.status().await.operations, 1);
    assert_eq!(f.status().await.active, 1);
    assert_eq!(f.status().await.bytes, u128::from(permission.request.bytes));
    assert_eq!(f.admit(f.tenant, permission).await, Ok(false));
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation("fixture_post_upgrade", f.operator),
            Some(f.controller),
        )
        .await
        .unwrap();
    assert!(f.status().await.fenced);
    assert_eq!(f.status().await.operations, 1);
}
