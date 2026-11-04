//! Actual standalone account read updates signed by the native operator client.
use super::{configure, fixture_with_operator};
use crate::{
    account_native_cli::{NativeAccountCli, signer},
    support::Harness,
};
use ic_testkit::pocket_ic::PocketIcBuilder;

#[test]
fn standalone_native_account_observations_keep_scope_reports_and_restore_fences() {
    let mut f = fixture_with_operator(
        Harness::with_builder(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        ),
        signer(),
    );
    let scope = f.operator_scope();
    let root = f.harness.pic.root_key().unwrap().unwrap();
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .unwrap()
        .to_string();
    let cli = NativeAccountCli::new(scope, &url, root, "standalone");
    let before = f.harness.pic.get_stable_memory(f.service).unwrap();
    cli.reports(|kind, bytes| configure(&f, kind, bytes));
    cli.refusals(|kind, bytes| configure(&f, kind, bytes));
    assert_eq!(f.harness.pic.get_stable_memory(f.service).unwrap(), before);
    f.harness.pic.stop_progress().unwrap();
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).unwrap();
    let source = f
        .harness
        .pic
        .get_stable_memory(scope.cashier.into())
        .unwrap();
    f.harness.pic.auto_progress().unwrap();
    cli.fenced();
    assert_eq!(f.harness.pic.get_stable_memory(f.service).unwrap(), before);
    assert_eq!(
        f.harness
            .pic
            .get_stable_memory(scope.cashier.into())
            .unwrap(),
        source
    );
    f.harness.pic.stop_live().unwrap();
}
