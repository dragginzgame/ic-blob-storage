//! Signed native assessment uses production host facts; no provider or update dispatch.
use super::*;
use crate::authenticated_cli::{PEM, run};
use ic_agent::{Identity, identity::BasicIdentity};
use ic_testkit::pocket_ic::PocketIcBuilder;

fn live(f: &mut Fixture, args: &mut [String]) {
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None);
    let index = args.iter().position(|s| s == "--url").unwrap();
    args[index + 1] = url.to_string();
}

fn args(f: &Fixture, dir: &std::path::Path, permission: UploadAdmissionRequest) -> Vec<String> {
    let key = dir.join("identity.pem");
    let root = dir.join("root.der");
    let saved = dir.join("permission.candid");
    std::fs::write(&key, PEM).unwrap();
    std::fs::write(&root, f.harness.pic.root_key().unwrap()).unwrap();
    std::fs::write(&saved, candid::encode_one(permission).unwrap()).unwrap();
    [
        "certificate-assessment",
        "--network",
        "local",
        "--url",
        "http://127.0.0.1:1",
        "--identity",
        key.to_str().unwrap(),
        "--root-key",
        root.to_str().unwrap(),
        "--actor",
        &f.uploader.to_text(),
        "--service",
        &f.service.to_text(),
        "--namespace",
        &f.config.namespace.to_string(),
        "--permission",
        saved.to_str().unwrap(),
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn rejects_changed_permission_and_trust(
    args: &[String],
    dir: &std::path::Path,
    permission: UploadAdmissionRequest,
) {
    let saved = dir.join("permission.candid");
    let mut changed = permission;
    changed.expires_at_ns -= 1;
    std::fs::write(&saved, candid::encode_one(changed).unwrap()).unwrap();
    assert_eq!(run(args, 3)["error"], "binding");
    std::fs::write(&saved, candid::encode_one(permission).unwrap()).unwrap();
    let root = dir.join("root.der");
    let trusted = std::fs::read(&root).unwrap();
    let mut wrong = trusted.clone();
    *wrong.last_mut().unwrap() ^= 1;
    std::fs::write(&root, wrong).unwrap();
    assert_eq!(run(args, 3)["error"], "transport");
    std::fs::write(&root, trusted).unwrap();
}

#[test]
fn standalone_certificate_cli_preserves_blockers_exact_intent_and_lifecycle_refusals() {
    let mut f = Fixture::with_harness(Harness::with_builder(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
    ));
    f.uploader = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    f.enroll(f.operator).unwrap();
    let mut manifest = f.manifest();
    manifest.permission.upload.upload = u128::MAX;
    manifest.permission.upload.object = u128::MAX - 1;
    manifest.permission.upload.incarnation = u128::MAX - 2;
    manifest.permission.upload.first_reference = u128::MAX - 3;
    let permission = manifest.permission;
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
    let dir = tempfile::tempdir().unwrap();
    let mut args = args(&f, dir.path(), permission);
    live(&mut f, &mut args);
    assert_eq!(run(&args, 3)["error"], "unprepared");
    f.harness.pic.stop_live();
    f.prepare(f.uploader, &manifest).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    live(&mut f, &mut args);
    let value = run(&args, 0);
    assert_eq!(value["observation"], "certificate_assessment");
    assert_eq!(
        value["upload"]["object"],
        permission.upload.object.to_string()
    );
    assert_eq!(value["expires_at_ns"], permission.expires_at_ns.to_string());
    assert_eq!(value["blockers"], serde_json::json!(["trusted_uploader"]));
    assert_eq!(value["issuance_authorized"], false);
    assert_eq!(value["retry_authorized"], false);
    rejects_changed_permission_and_trust(&args, dir.path(), permission);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    f.harness
        .pic
        .update_candid_as::<Result<UploadRevocationResponse, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_revoke_upload",
            (permission,),
        )
        .unwrap()
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    live(&mut f, &mut args);
    assert_eq!(run(&args, 3)["error"], "assessment_revoked");
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    let revoked = f.admission(permission);
    assert!(revoked.revoked);
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    live(&mut f, &mut args);
    assert_eq!(run(&args, 3)["error"], "assessment_permission_fenced");
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    assert!(f.configuration(f.operator).unwrap().fenced);
    assert_eq!(f.admission(permission), revoked);
}
