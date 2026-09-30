//! Signed one-page inventory preserves filtered progress and inspection-only restoration.
use super::*;
use crate::authenticated_cli::{PEM, run};
use ic_agent::{Identity, identity::BasicIdentity};
use ic_testkit::pocket_ic::PocketIcBuilder;
use serde_json::{Value, json};
use std::path::Path;

fn live(f: &mut Fixture, args: &mut [String]) {
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None);
    change(args, "--url", url.as_str());
}
fn change(args: &mut [String], flag: &str, value: &str) {
    let index = args.iter().position(|s| s == flag).unwrap();
    args[index + 1] = value.into();
}
fn arguments(f: &Fixture, directory: &Path) -> Vec<String> {
    let key = directory.join("operator.pem");
    let root = directory.join("root.der");
    std::fs::write(&key, PEM).unwrap();
    std::fs::write(&root, f.harness.pic.root_key().unwrap()).unwrap();
    [
        "upload-history",
        "--network",
        "local",
        "--url",
        "http://127.0.0.1:1",
        "--identity",
        key.to_str().unwrap(),
        "--root-key",
        root.to_str().unwrap(),
        "--operator",
        &f.operator.to_text(),
        "--service",
        &f.service.to_text(),
        "--namespace",
        &f.config.namespace.to_string(),
        "--filter",
        "active",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
fn populate(f: &Fixture) -> UploadAdmissionRequest {
    f.enroll(f.operator).unwrap();
    let mut permission = UploadAdmissionRequest {
        upload: ReferenceUpload {
            service: f.service,
            tenant: f.tenant,
            namespace: f.config.namespace,
            upload: 1,
            object: 2,
            incarnation: 3,
            first_reference: 4,
            root: [0; 32],
            bytes: 1,
        },
        uploader: f.uploader,
        expires_at_ns: u64::MAX,
    };
    for id in 1u8..=65 {
        permission.upload.upload = if id == 65 { u128::MAX } else { u128::from(id) };
        permission.upload.object = if id == 65 {
            u128::MAX - 1
        } else {
            u128::from(id) + 100
        };
        permission.upload.incarnation = u128::MAX - 2;
        permission.upload.first_reference = u128::MAX - 3;
        permission.upload.root = [id; 32];
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
        if id != 65 {
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
        }
    }
    permission
}
fn cursor(args: &[String], path: &Path, next: &Value) -> Vec<String> {
    std::fs::write(path, next.to_string()).unwrap();
    let mut args = args.to_vec();
    args.extend(["--cursor".into(), path.to_str().unwrap().into()]);
    args
}
fn all_pages(args: &[String], path: &Path) {
    let mut all = args.to_vec();
    change(&mut all, "--filter", "all");
    let first = run(&all, 0);
    assert_eq!(first["entries"][0]["state"], "cancelled");
    assert_eq!(
        first["entries"].as_array().unwrap().last().unwrap()["upload"]["upload"],
        "32"
    );
    let second = run(&cursor(&all, path, &first["next"]), 0);
    assert_eq!(second["entries"][0]["upload"]["upload"], "33");
    assert_eq!(second["next"]["after_request"], "64");
    let last = run(&cursor(&all, path, &second["next"]), 0);
    assert_eq!(
        last["entries"][0]["upload"]["upload"],
        u128::MAX.to_string()
    );
    assert_eq!(last["next"], Value::Null);
}
fn rejected_cursor(args: &[String], path: &Path, next: &Value) {
    let mut saved = next.clone();
    saved["namespace"] = "1".into();
    let mut args = cursor(args, path, &saved);
    // A dead URL and absent key prove scope rejection precedes network/identity access.
    change(&mut args, "--url", "http://127.0.0.1:1");
    change(&mut args, "--identity", "absent.pem");
    assert_eq!(run(&args, 3)["error"], "cursor_scope");
}

#[test]
fn standalone_history_cli_follows_empty_progress_and_retains_inventory_after_upgrade() {
    let mut f = Fixture::with_harness(Harness::with_builder(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
    ));
    f.operator = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    f.config.operator = f.operator;
    f.config.resources.max_objects = 65;
    f.config.resources.max_tenant_objects = 65;
    f.config.resources.max_object_bytes = 1;
    f.config.resources.max_chunks = 65;
    f.config.resources.max_tenant_chunks = 65;
    f.harness
        .pic
        .reinstall_canister(
            f.service,
            wasm(),
            installation(&f.config),
            Some(f.controller),
        )
        .unwrap();
    let permission = populate(&f);
    let dir = tempfile::tempdir().unwrap();
    let saved = dir.path().join("cursor.json");
    let mut args = arguments(&f, dir.path());
    let before = f.harness.pic.get_stable_memory(f.service);
    live(&mut f, &mut args);
    let empty = run(&args, 0);
    assert_eq!(empty["entries"], json!([]));
    assert_eq!(empty["scanned"], "64");
    assert_eq!(empty["next"]["after_request"], "64");
    assert_eq!(empty["fenced"], false);
    let resumed = run(&cursor(&args, &saved, &empty["next"]), 0);
    assert_eq!(
        resumed["entries"][0]["upload"]["object"],
        permission.upload.object.to_string()
    );
    assert_eq!(resumed["entries"][0]["state"], "reserved");
    assert_eq!(resumed["next"], Value::Null);
    assert_eq!(resumed["retry_authorized"], false);
    all_pages(&args, &saved);
    rejected_cursor(&args, &saved, &empty["next"]);
    let mut wrong = args.clone();
    change(&mut wrong, "--namespace", "1");
    assert_eq!(run(&wrong, 3)["error"], "binding");
    change(&mut wrong, "--namespace", &f.config.namespace.to_string());
    change(&mut wrong, "--operator", &f.controller.to_text());
    assert_eq!(run(&wrong, 3)["error"], "identity_binding");
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    live(&mut f, &mut args);
    let restored = run(&cursor(&args, &saved, &empty["next"]), 0);
    assert_eq!(restored["entries"], resumed["entries"]);
    assert_eq!(restored["fenced"], true);
    assert_eq!(restored["retry_authorized"], false);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    assert!(f.configuration(f.operator).unwrap().fenced);
}
