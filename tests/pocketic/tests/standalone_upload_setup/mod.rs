//! Native local upload setup with real tenant/uploader signing and fenced history.
use super::*;
use crate::{
    account_native_cli::signer,
    upload_setup_cli::{Client, uploader},
};
use ic_testkit::pocket_ic::PocketIcBuilder;
#[test]
fn standalone_signed_upload_setup_recovers_lost_and_pending_replies_then_cancels_and_fences() {
    let mut f = Fixture::with_harness(Harness::with_builder(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
    ));
    f.tenant = signer();
    f.uploader = uploader();
    f.enroll(f.operator).unwrap();
    let input = f.manifest();
    let trusted = f.harness.pic.root_key().unwrap();
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string();
    let body = vec![42; usize::try_from(input.permission.upload.bytes).unwrap()];
    let client = Client::new(input.clone(), &body, &url, &trusted, "standalone");
    client.before_restore();
    let original = f.admission(input.permission);
    assert_eq!(
        original.state,
        ic_blob_storage::dto::upload::UploadState::Cancelled
    );
    assert!(original.revoked);
    let scope = ic_blob_storage::dto::operator::OperatorScope {
        service: f.service,
        namespace: f.config.namespace,
        cashier: f.config.billing.cashier,
        payment_account: f.config.payment_account,
    };
    crate::upload_setup_cli::cancelled_totals(&f.harness.pic, scope, f.operator);
    f.harness.pic.stop_progress();
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    f.harness.pic.auto_progress();
    let before = f.harness.pic.get_stable_memory(f.service);
    client.restored();
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    assert_eq!(f.admission(input.permission), original);
    crate::upload_setup_cli::cancelled_totals(&f.harness.pic, scope, f.operator);
    f.harness.pic.stop_live();
}
