//! Signed gateway decisions through standalone, with occupied unrelated owners.
use super::{super::*, sync};
use crate::{account_native_cli::signer, gateway_native_cli::GatewayCli};
use ic_testkit::pocket_ic::PocketIcBuilder;

#[test]
fn standalone_signed_gateway_decisions_preserve_uncertainty_and_occupied_restore_fences() {
    let mut f = sync::fixture_with_operator(
        Harness::with_builder(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        ),
        signer(),
    );
    f.enroll(f.operator).unwrap();
    let request = f.manifest();
    f.harness
        .pic
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (request.permission,),
        )
        .unwrap()
        .unwrap();
    f.prepare(f.uploader, &request).unwrap();
    let scope = f.operator_scope();
    let root = f.harness.pic.root_key().unwrap();
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string();
    let cli = GatewayCli::new(scope, Fake::principal(9), url, &root, "standalone");
    let before = f.harness.pic.get_stable_memory(f.service);
    let source = f.harness.pic.get_stable_memory(scope.cashier);
    cli.refusals();
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert_eq!(f.harness.pic.get_stable_memory(scope.cashier), source);
    let pending = cli.decisions(|mode| sync::mode(&f, mode));
    f.harness.pic.stop_progress();
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let source = f.harness.pic.get_stable_memory(scope.cashier);
    f.harness.pic.auto_progress();
    cli.fenced(&pending);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert_eq!(f.harness.pic.get_stable_memory(scope.cashier), source);
    f.harness.pic.stop_live();
}
