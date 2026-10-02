use super::*;
pub(super) fn before_restore(c: &Client) {
    let args = c.args("admit-upload", Some("wrong-role"), true);
    assert_eq!(c.call(&args, 3)["error"], "binding");
    assert!(!c.report.join("wrong-role").exists());
    let mut args = c.args("admit-upload", Some("wrong-key"), false);
    change(
        &mut args,
        "--identity",
        c.temporary.path().join("uploader.pem").to_str().unwrap(),
    );
    assert_eq!(c.call(&args, 3)["error"], "identity_binding");
    assert!(!c.report.join("wrong-key").exists());
    assert_eq!(
        c.call(&c.args("prepare-upload", Some("unknown"), true), 3)["error"],
        "upload_unknown"
    );
    assert_eq!(
        c.dispatch("admit-upload", "admit-lost", Reply::Drop)["error"],
        "transport"
    );
    let mut invalid = c.permission.clone();
    invalid.declaration.chunks[0][0] ^= 1;
    save(
        &c.report.join("invalid-manifest.candid"),
        &candid::encode_one(invalid).unwrap(),
    );
    let mut invalid_args = c.args("prepare-upload", Some("invalid-prepare"), true);
    change(
        &mut invalid_args,
        "--request",
        c.report.join("invalid-manifest.candid").to_str().unwrap(),
    );
    assert_eq!(c.call(&invalid_args, 3)["error"], "invalid_reply");
    assert!(!c.report.join("invalid-prepare").exists());
    assert_eq!(
        c.dispatch("prepare-upload", "prepare-pending", Reply::Pending)["outcome"],
        "pending"
    );
    super::inputs::verify(c);
    super::inputs::unconfirmed_reference(c);
    let mut args = c.args("certificate-assessment", None, true);
    let n = args.iter().position(|a| a == "--request").unwrap();
    args[n] = "--permission".into();
    assert_eq!(c.call(&args, 0)["blockers"], json!([]));
    let mut changed = c.permission.permission;
    changed.expires_at_ns -= 1;
    save(
        &c.report.join("changed-permission.candid"),
        &candid::encode_one(changed).unwrap(),
    );
    let mut args = c.args("upload-permission", None, false);
    change(
        &mut args,
        "--request",
        c.report.join("changed-permission.candid").to_str().unwrap(),
    );
    assert_eq!(c.call(&args, 3)["error"], "upload_conflict");
    let revoked = c.dispatch("revoke-upload", "revoke", Reply::Pass);
    assert_eq!(revoked["observation"]["admission"]["state"], "cancelled");
    assert_eq!(revoked["observation"]["changed"], true);
    assert_eq!(
        c.call(&c.args("prepare-upload", Some("revoked-prepare"), true), 3)["error"],
        "upload_revoked"
    );
    let manifest = c.call(&c.args("upload-manifest", None, false), 0);
    assert!(manifest["observation"]["manifest"].is_object());
}
pub(super) fn restored(c: &Client) {
    let p = c.call(&c.args("upload-permission", None, false), 0);
    assert_eq!(p["observation"]["state"], "cancelled");
    assert_eq!(p["observation"]["revoked"], true);
    let m = c.call(&c.args("upload-manifest", None, true), 0);
    assert!(m["observation"]["manifest"].is_object());
    for (command, uploader, label) in [
        ("revoke-upload", false, "fenced-revoke"),
        ("prepare-upload", true, "fenced-prepare"),
    ] {
        assert_eq!(
            c.call(&c.args(command, Some(label), uploader), 3)["error"],
            "upload_fenced"
        );
    }
    save(&c.report.join("fixture-summary.json"),&serde_json::to_vec_pretty(&json!({"cli_invocations":c.count.get(),"provider_requests":0,"fixture_exposure_or_completion":false,"attached_provider_cycles":"0"})).unwrap());
}
