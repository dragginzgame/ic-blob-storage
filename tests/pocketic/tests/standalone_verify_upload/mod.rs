//! Authenticated historical manifest + native byte verification, with no provider effects.
use super::*;
use crate::authenticated_cli::{change, identity_files, live, run};
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
use ic_host_fs::testing::FixtureDirectory;
use ic_platform::agent::{BasicIdentity, Identity};
use ic_testkit::pocket_ic::PocketIcBuilder;

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
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "Keep original-manifest verification and restore fencing in one installed scenario"
)]
async fn standalone_verify_upload_uses_signed_original_manifest_and_preserves_restore_fences() {
    let mut f = Fixture::with_harness(
        Harness::with_builder(
            PocketIcBuilder::new(crate::support::REQUEST_TIMEOUT)
                .with_nns_subnet()
                .with_application_subnet(),
        )
        .await,
    )
    .await;
    f.uploader = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    f.enroll(f.operator).await.unwrap();
    let mut manifest = f.manifest();
    manifest.permission.upload.upload = u128::MAX;
    manifest.permission.upload.object = u128::MAX - 1;
    manifest.permission.upload.incarnation = u128::MAX - 2;
    manifest.permission.upload.first_reference = u128::MAX - 3;
    let permission = manifest.permission;
    f.harness
        .pic
        .update_blob_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            permission,
            REPLY_BYTES,
        )
        .await
        .unwrap()
        .unwrap();
    let dir = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let (key, root) = identity_files(&f.harness.pic, dir.path(), "identity.pem").await;
    let saved = dir.path().join("permission.wire");
    let body = dir.path().join("body");
    let original = vec![42; 10 * 1024 * 1024];
    std::fs::write(
        &saved,
        ic_blob_storage_contracts::document::encode(permission).unwrap(),
    )
    .unwrap();
    std::fs::write(&body, &original).unwrap();
    let url = live(&mut f.harness.pic).await;
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
    f.harness.pic.stop_live().await.unwrap();
    f.prepare(f.uploader, &manifest).await.unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    change(&mut args, "--url", &live(&mut f.harness.pic).await);
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
    std::fs::write(
        &saved,
        ic_blob_storage_contracts::document::encode(changed).unwrap(),
    )
    .unwrap();
    assert_eq!(run(&args, 3)["error"], "manifest_refused");
    std::fs::write(
        &saved,
        ic_blob_storage_contracts::document::encode(permission).unwrap(),
    )
    .unwrap();
    rejects_wrong_root(&args, &root);
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    f.harness.pic.stop_live().await.unwrap();
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    change(&mut args, "--url", &live(&mut f.harness.pic).await);
    let restored = run(&args, 0);
    assert_eq!(restored["content_digest"], report["content_digest"]);
    assert_eq!(restored["retry_authorized"], false);
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    f.harness.pic.stop_live().await.unwrap();
    assert!(f.configuration(f.operator).await.unwrap().fenced);
    assert_eq!(
        f.admission(permission).await.state,
        ic_blob_storage_contracts::dto::upload::UploadState::Reserved
    );
}
