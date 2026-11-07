//! Native local upload setup with real tenant/uploader signing and fenced history.
use super::*;
use crate::authenticated_cli::live;
use crate::{
    account_native_cli::signer,
    upload_setup_cli::{Client, uploader},
};
use ic_testkit::pocket_ic::PocketIcBuilder;
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_signed_upload_setup_recovers_lost_and_pending_replies_then_cancels_and_fences()
{
    let mut f = Fixture::with_profile(
        Harness::with_builder(
            PocketIcBuilder::new(crate::support::REQUEST_TIMEOUT)
                .with_nns_subnet()
                .with_application_subnet(),
        )
        .await,
        fake_principal(5),
        fake_principal(2),
        uploader(),
        Envelope::Regular,
        "signed-setup-fixture",
    )
    .await;
    f.tenant = signer();
    f.enroll(f.operator).await.unwrap();
    let input = f.manifest();
    let trusted = f.harness.pic.root_key().await.unwrap().unwrap();
    let url = live(&mut f.harness.pic).await;
    let body = vec![42; usize::try_from(input.permission.upload.bytes).unwrap()];
    let host = f.configuration(f.operator).await.unwrap();
    let client = Client::new(
        input.clone(),
        &body,
        &url,
        &trusted,
        "standalone",
        ServiceInstallationInput {
            configuration: host.configuration,
            project: host.project,
        },
    );
    client.before_restore();
    let original = f.admission(input.permission).await;
    assert_eq!(
        original.state,
        ic_blob_storage_contracts::dto::upload::UploadState::Cancelled
    );
    assert!(original.revoked);
    let scope = ic_blob_storage_contracts::dto::operator::OperatorScope {
        service: f.service.into(),
        namespace: f.config.namespace,
        cashier: f.config.billing.cashier,
        payment_account: f.config.payment_account,
    };
    crate::upload_setup_cli::cancelled_totals(&f.harness.pic, scope, f.operator).await;
    f.harness.pic.stop_progress().await.unwrap();
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    f.harness.pic.auto_progress().await.unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    client.restored();
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    assert_eq!(f.admission(input.permission).await, original);
    crate::upload_setup_cli::cancelled_totals(&f.harness.pic, scope, f.operator).await;
    f.harness.pic.stop_live().await.unwrap();
}
