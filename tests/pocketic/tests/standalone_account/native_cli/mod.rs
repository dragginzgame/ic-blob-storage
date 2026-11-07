//! Actual standalone account read updates signed by the native operator client.
use super::{configure, fixture_with_operator};
use crate::authenticated_cli::live;
use crate::{
    account_native_cli::{NativeAccountCli, signer},
    support::Harness,
};
use ic_testkit::pocket_ic::PocketIcBuilder;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_native_account_observations_keep_scope_reports_and_restore_fences() {
    let mut f = fixture_with_operator(
        Harness::with_builder(
            PocketIcBuilder::new(crate::support::REQUEST_TIMEOUT)
                .with_nns_subnet()
                .with_application_subnet(),
        )
        .await,
        signer(),
    )
    .await;
    let scope = f.operator_scope();
    let root = f.harness.pic.root_key().await.unwrap().unwrap();
    let url = live(&mut f.harness.pic).await;
    let cli = NativeAccountCli::new(scope, &url, root, "standalone");
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    cli.reports(|kind, bytes| configure(&f, kind, bytes)).await;
    cli.refusals(|kind, bytes| configure(&f, kind, bytes)).await;
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    f.harness.pic.stop_progress().await.unwrap();
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let source = f
        .harness
        .pic
        .get_stable_memory(scope.cashier.into())
        .await
        .unwrap();
    f.harness.pic.auto_progress().await.unwrap();
    cli.fenced();
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
    f.harness.pic.stop_live().await.unwrap();
}
