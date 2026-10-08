//! Authenticated historical manifest + native byte verification, with no provider effects.
use super::*;
use crate::authenticated_cli::{PEM, run};
use ic_agent::{Identity, identity::BasicIdentity};
use ic_testkit::pocket_ic::PocketIcBuilder;

fn live(f: &mut Fixture) -> String {
    f.harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string()
}
fn change(args: &mut [String], flag: &str, value: &str) {
    let index = args.iter().position(|s| s == flag).unwrap();
    args[index + 1] = value.into();
}
fn rejects_changed_bytes(args: &[String], body: &std::path::Path, original: &[u8]) {
    let mut corrupt = original.to_vec();
    corrupt[1024 * 1024] ^= 1;
    for bytes in [&corrupt[..], &original[..original.len() - 1]] {
        std::fs::write(body, bytes).unwrap();
        assert_eq!(run(args, 3)["error"], "content_mismatch");
    }
    std::fs::write(body, original).unwrap();
}
fn rejects_wrong_root(args: &[String], root: &std::path::Path) {
    let trusted = std::fs::read(root).unwrap();
    let mut wrong = trusted.clone();
    *wrong.last_mut().unwrap() ^= 1;
    std::fs::write(root, wrong).unwrap();
    assert_eq!(run(args, 3)["error"], "transport");
    std::fs::write(root, trusted).unwrap();
}
#[test]
fn standalone_verify_upload_uses_signed_original_manifest_and_preserves_restore_fences() {
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
    let key = dir.path().join("identity.pem");
    let root = dir.path().join("root.der");
    let saved = dir.path().join("permission.candid");
    let body = dir.path().join("body");
    let original = vec![42; 10 * 1024 * 1024];
    std::fs::write(&key, PEM).unwrap();
    std::fs::write(&root, f.harness.pic.root_key().unwrap()).unwrap();
    std::fs::write(&saved, candid::encode_one(permission).unwrap()).unwrap();
    std::fs::write(&body, &original).unwrap();
    let url = live(&mut f);
    let mut args: Vec<String> = [
        "verify-upload",
        "--network",
        "local",
        "--url",
        &url,
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
        "--body",
        body.to_str().unwrap(),
        "--max-bytes",
        "10485760",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    assert_eq!(run(&args, 3)["error"], "unprepared");
    f.harness.pic.stop_live();
    f.prepare(f.uploader, &manifest).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    change(&mut args, "--url", &live(&mut f));
    let report = run(&args, 0);
    assert_eq!(report["observation"], "local_upload_bytes");
    assert_eq!(
        report["upload"]["object"],
        permission.upload.object.to_string()
    );
    assert_eq!(report["upload"]["bytes"], "10485760");
    assert_eq!(report["provider_completion"], "not_established");
    assert_eq!(report["retry_authorized"], false);
    rejects_changed_bytes(&args, &body, &original);
    let mut changed = permission;
    changed.expires_at_ns -= 1;
    std::fs::write(&saved, candid::encode_one(changed).unwrap()).unwrap();
    assert_eq!(run(&args, 3)["error"], "manifest_refused");
    std::fs::write(&saved, candid::encode_one(permission).unwrap()).unwrap();
    rejects_wrong_root(&args, &root);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    change(&mut args, "--url", &live(&mut f));
    let restored = run(&args, 0);
    assert_eq!(restored["content_digest"], report["content_digest"]);
    assert_eq!(restored["retry_authorized"], false);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    assert!(f.configuration(f.operator).unwrap().fenced);
    assert_eq!(
        f.admission(permission).state,
        ic_blob_storage_contracts::dto::upload::UploadState::Reserved
    );
}
