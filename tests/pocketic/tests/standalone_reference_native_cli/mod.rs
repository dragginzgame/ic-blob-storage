//! Actual tenant signatures preserve production completion refusals under restore.
use super::*;
use crate::{
    authenticated_cli::run,
    reference_cli::{change_native, native_requests},
};
use ic_agent::{Identity, identity::BasicIdentity};
use ic_testkit::pocket_ic::PocketIcBuilder;

fn live(f: &mut Fixture) -> String {
    f.harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string()
}
fn refused(args: &[String], code: &str) {
    assert_eq!(run(args, 3)["error"], code);
}

#[test]
fn signed_reference_inspection_keeps_unknown_unconfirmed_and_restored_refusals() {
    let mut f = Fixture::with_harness(Harness::with_builder(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
    ));
    f.tenant = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    f.enroll(f.operator).unwrap();
    let mut permission = f.manifest().permission;
    permission.upload.upload = u128::MAX;
    permission.upload.object = u128::MAX - 1;
    permission.upload.incarnation = u128::MAX - 2;
    permission.upload.first_reference = u128::MAX - 3;
    let input = ReferenceCommand {
        upload: permission.upload,
        reference: u128::MAX - 4,
        operation: u128::MAX - 5,
        action: ReferenceAction::Retain,
    };
    let dir = tempfile::tempdir().unwrap();
    let url = live(&mut f);
    let (mut receipt, mut status) = native_requests(&f.harness.pic, input, dir.path(), &url);
    let before = f.harness.pic.get_stable_memory(f.service);
    for args in [&receipt, &status] {
        refused(args, "reference_unknown");
    }
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    f.harness
        .pic
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (permission,),
        )
        .unwrap()
        .unwrap();
    let url = live(&mut f);
    for args in [&mut receipt, &mut status] {
        change_native(args, "--url", &url);
    }
    let before = f.harness.pic.get_stable_memory(f.service);
    for args in [&receipt, &status] {
        refused(args, "reference_unconfirmed");
    }
    let mut wrong = receipt.clone();
    change_native(&mut wrong, "--namespace", "1");
    refused(&wrong, "binding");
    wrong = receipt.clone();
    change_native(&mut wrong, "--actor", &f.operator.to_text());
    // Named operators/controllers cannot use a tenant's saved intent.
    refused(&wrong, "denied");
    let untrusted = dir.path().join("untrusted.der");
    std::fs::write(&untrusted, [1, 2, 3]).unwrap();
    wrong = receipt.clone();
    change_native(&mut wrong, "--root-key", untrusted.to_str().unwrap());
    refused(&wrong, "transport");
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let url = live(&mut f);
    for args in [&mut receipt, &mut status] {
        change_native(args, "--url", &url);
        refused(args, "reference_unconfirmed");
    }
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    assert!(f.configuration(f.operator).unwrap().fenced);
    assert_eq!(
        std::fs::read(dir.path().join("receipt.candid")).unwrap(),
        candid::encode_one(input).unwrap()
    );
}
