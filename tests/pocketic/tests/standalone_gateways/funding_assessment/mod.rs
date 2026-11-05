//! Passive signed funding queries beside an occupied standalone upload and stopped Cashier.
use super::{super::*, sync};
use crate::{account_native_cli::signer, funding_assessment_cli::AssessmentCli};
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
use ic_testkit::pocket_ic::PocketIcBuilder;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_signed_funding_assessment_preserves_occupied_and_restored_state() {
    let mut f = sync::fixture_with_operator(
        Harness::with_builder(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        )
        .await,
        signer(),
    )
    .await;
    f.enroll(f.operator).await.unwrap();
    let request = f.manifest();
    f.harness
        .pic
        .update_blob_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            request.permission,
            REPLY_BYTES,
        )
        .await
        .unwrap()
        .unwrap();
    f.prepare(f.uploader, &request).await.unwrap();
    let scope = f.operator_scope();
    f.harness
        .pic
        .stop_canister(scope.cashier.into(), Some(f.controller))
        .await
        .unwrap();
    let root = f.harness.pic.root_key().await.unwrap().unwrap();
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .await
        .unwrap()
        .to_string();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let source = f
        .harness
        .pic
        .get_stable_memory(scope.cashier.into())
        .await
        .unwrap();
    let cli = AssessmentCli::new(scope, &url, &root, "standalone");
    cli.inspect(false);
    cli.refusals();
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    assert_eq!(
        f.harness
            .pic
            .get_stable_memory(scope.cashier.into())
            .await
            .unwrap(),
        source
    );
    f.harness.pic.stop_progress().await.unwrap();
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    f.harness.pic.auto_progress().await.unwrap();
    cli.inspect(true);
    cli.refusals();
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        restored
    );
    assert_eq!(
        f.harness
            .pic
            .get_stable_memory(scope.cashier.into())
            .await
            .unwrap(),
        source
    );
    f.harness.pic.stop_live().await.unwrap();
}
