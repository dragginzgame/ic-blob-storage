//! Passive signed funding queries beside an occupied standalone upload and stopped Cashier.
use super::{super::*, sync};
use crate::{account_native_cli::signer, funding_assessment_cli::AssessmentCli};
use ic_testkit::pocket_ic::PocketIcBuilder;

#[test]
fn standalone_signed_funding_assessment_preserves_occupied_and_restored_state() {
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
    f.harness
        .pic
        .stop_canister(scope.cashier, Some(f.controller))
        .unwrap();
    let root = f.harness.pic.root_key().unwrap();
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string();
    let before = f.harness.pic.get_stable_memory(f.service);
    let source = f.harness.pic.get_stable_memory(scope.cashier);
    let cli = AssessmentCli::new(scope, &url, &root, "standalone");
    cli.inspect(false);
    cli.refusals();
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert_eq!(f.harness.pic.get_stable_memory(scope.cashier), source);
    f.harness.pic.stop_progress();
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service);
    f.harness.pic.auto_progress();
    cli.inspect(true);
    cli.refusals();
    assert_eq!(f.harness.pic.get_stable_memory(f.service), restored);
    assert_eq!(f.harness.pic.get_stable_memory(scope.cashier), source);
    f.harness.pic.stop_live();
}
