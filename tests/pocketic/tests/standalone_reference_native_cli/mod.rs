//! Signed submission/inspection preserves actual standalone phase and restore refusals.
use super::*;
use crate::{
    authenticated_cli::run,
    reference_cli::{change_native, native_requests},
    submission_proxy::{Dispatch, Proxy, Reply},
};
use ic_agent::{Identity, identity::BasicIdentity};
use ic_testkit::pocket_ic::PocketIcBuilder;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

fn live(f: &mut Fixture) -> String {
    f.harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string()
}
fn refused(args: &[String], code: &str) {
    assert_eq!(run(args, 3)["error"], code);
}

fn save(directory: &Path, name: &str, value: &Value) {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(directory.join(name))
        .unwrap();
    file.write_all(&serde_json::to_vec_pretty(value).unwrap())
        .unwrap();
}
fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
fn submission(receipt: &[String], directory: &Path) -> Vec<String> {
    let mut args = receipt.to_vec();
    args[0] = "submit-reference".into();
    args.extend(["--run-dir".into(), directory.to_str().unwrap().into()]);
    args
}
fn response(directory: &Path, failure: ReferenceFailure) {
    let value: Result<ReferenceMutationResponse, ReferenceFailure> =
        candid::decode_one(&std::fs::read(directory.join("response.candid")).unwrap()).unwrap();
    assert_eq!(value, Err(failure));
    let outcome = read_json(&directory.join("outcome.json"));
    assert_eq!(outcome["outcome"], "refused");
    assert_eq!(outcome["result"], Value::Null);
    assert_eq!(outcome["retry_authorized"], false);
    assert_eq!(outcome["provider_deletion"], "not_established");
    assert_eq!(outcome["billing_cessation"], "not_established");
}
fn unknown_submission(f: &Fixture, receipt: &[String], report: &Path, url: &str, reply: Reply) {
    let directory = report.join("unknown");
    let proxy = Proxy::start(
        url.into(),
        directory.clone(),
        Dispatch {
            service: f.service,
            actor: f.tenant,
            method: "blob_apply_reference",
            argument_file: "request.candid",
        },
        reply,
    );
    let before = f.harness.pic.get_stable_memory(f.service);
    let mut args = submission(receipt, &directory);
    change_native(&mut args, "--url", &proxy.url);
    let value = run(&args, if reply == Reply::Pending { 0 } else { 3 });
    save(report, "unknown-submission.json", &value);
    match reply {
        Reply::Pass => {
            assert_eq!(value["error"], "reference_unknown");
            response(&directory, ReferenceFailure::Unknown);
        }
        Reply::Drop => assert_eq!(value["error"], "transport"),
        Reply::Pending => assert_eq!(value["outcome"], "pending"),
    }
    let outcome = std::fs::read(directory.join("outcome.json")).unwrap();
    let recorded: Value = serde_json::from_slice(&outcome).unwrap();
    assert_eq!(
        recorded["outcome"],
        match reply {
            Reply::Pass => "refused",
            Reply::Drop => "uncertain",
            Reply::Pending => "pending",
        }
    );
    assert_eq!(recorded["result"], Value::Null);
    assert_eq!(recorded["retry_authorized"], false);
    let intent = std::fs::read(directory.join("intent.json")).unwrap();
    // Recovery uses the dispatch copy and never repeats the update. A refused
    // lookup cannot establish the original result or turn uncertainty into absence.
    let mut recover = receipt.to_vec();
    change_native(&mut recover, "--url", &proxy.url);
    change_native(
        &mut recover,
        "--request",
        directory.join("request.candid").to_str().unwrap(),
    );
    let lookup = run(&recover, 3);
    assert_eq!(lookup["error"], "reference_unknown");
    save(report, "unknown-receipt.json", &lookup);
    assert_eq!(
        std::fs::read(directory.join("outcome.json")).unwrap(),
        outcome
    );
    assert_eq!(run(&args, 3)["error"], "submission_already_claimed");
    assert_eq!(
        std::fs::read(directory.join("intent.json")).unwrap(),
        intent
    );
    assert_eq!(proxy.calls(), 1);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}
fn phase_submission(
    f: &Fixture,
    receipt: &[String],
    report: &Path,
    label: &str,
    failure: ReferenceFailure,
    code: &str,
) {
    let directory = report.join(label);
    let before = f.harness.pic.get_stable_memory(f.service);
    let args = submission(receipt, &directory);
    let value = run(&args, 3);
    assert_eq!(value["error"], code);
    save(report, &format!("{label}-submission.json"), &value);
    response(&directory, failure);
    refused(&args, "submission_already_claimed");
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}
fn authority(f: &Fixture, receipt: &[String], directory: &Path) {
    let before = f.harness.pic.get_stable_memory(f.service);
    let mut wrong = receipt.to_vec();
    change_native(&mut wrong, "--namespace", "1");
    refused(&wrong, "binding");
    wrong.clone_from_slice(receipt);
    change_native(&mut wrong, "--actor", &f.operator.to_text());
    // Named operators/controllers cannot use a tenant's saved intent.
    refused(&wrong, "denied");
    let untrusted = directory.join("untrusted.der");
    std::fs::write(&untrusted, [1, 2, 3]).unwrap();
    wrong.clone_from_slice(receipt);
    change_native(&mut wrong, "--root-key", untrusted.to_str().unwrap());
    refused(&wrong, "transport");
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}
fn restored(
    f: &mut Fixture,
    receipt: &mut [String],
    status: &mut [String],
    directory: &Path,
    report: &Path,
    mut current: ReferenceCommand,
) -> ReferenceCommand {
    f.harness.pic.stop_live();
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let url = live(f);
    for args in [&mut *receipt, status] {
        change_native(args, "--url", &url);
        refused(args, "reference_unconfirmed");
    }
    current.operation -= 1;
    std::fs::write(
        directory.join("receipt.candid"),
        candid::encode_one(current).unwrap(),
    )
    .unwrap();
    phase_submission(
        f,
        receipt,
        report,
        "fenced-release",
        ReferenceFailure::Fenced,
        "reference_fenced",
    );
    let accounting = f.local_status(f.operator, f.operator_scope()).unwrap();
    assert_eq!(
        [
            accounting.uploads.fenced,
            accounting.funding.fenced,
            accounting.gateways.fenced,
            accounting.reads.fenced
        ],
        [true; 4]
    );
    assert_eq!(
        [
            accounting.uploads.reserved_bytes,
            accounting.uploads.logical_bytes,
            accounting.uploads.physical_bytes,
            accounting.uploads.liability_bytes
        ],
        [u128::from(current.upload.bytes); 4]
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    assert!(f.configuration(f.operator).unwrap().fenced);
    current
}

fn report_directory(f: &Fixture, temporary: &Path, label: &str, bytes: u64) -> PathBuf {
    let report = std::env::var_os("BLOB_STANDALONE_REFERENCE_REPORT").map_or_else(
        || temporary.join(label),
        |root| PathBuf::from(root).join(label),
    );
    std::fs::create_dir(&report).unwrap();
    save(
        &report,
        "fixture-plan.json",
        &json!({"evidence":"local_standalone_refusal","reply":label,
        "service":f.service.to_text(),"tenant":f.tenant.to_text(),"operator":f.operator.to_text(),
        "controller":f.controller.to_text(),"namespace":f.config.namespace.to_string(),
        "declared_bytes":bytes.to_string(),"provider_content":"not_observed",
        "max_signed_updates":8,"max_cli_invocations":40,"provider_gets":0,
        "deployed_provider_requests":0,"attached_provider_cycles":"0"}),
    );
    std::fs::write(
        report.join("fixture-root.der"),
        f.harness.pic.root_key().unwrap(),
    )
    .unwrap();
    report
}
fn prepared(
    f: &mut Fixture,
    manifest: &UploadManifestRequest,
    receipt: &mut [String],
    status: &mut [String],
) {
    f.harness.pic.stop_live();
    f.harness
        .pic
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (manifest.permission,),
        )
        .unwrap()
        .unwrap();
    f.prepare(f.uploader, manifest).unwrap();
    let url = live(f);
    for args in [receipt, status] {
        change_native(args, "--url", &url);
        refused(args, "reference_unconfirmed");
    }
}
fn journey(reply: Reply, label: &str) {
    let mut f = Fixture::with_harness(Harness::with_builder(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
    ));
    f.tenant = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    f.enroll(f.operator).unwrap();
    let mut manifest = f.manifest();
    manifest.permission.upload.upload = u128::MAX;
    manifest.permission.upload.object = u128::MAX - 1;
    manifest.permission.upload.incarnation = u128::MAX - 2;
    manifest.permission.upload.first_reference = u128::MAX - 3;
    let permission = manifest.permission;
    let input = ReferenceCommand {
        upload: permission.upload,
        reference: u128::MAX - 4,
        operation: u128::MAX - 5,
        action: ReferenceAction::Retain,
    };
    let dir = tempfile::tempdir().unwrap();
    let report = report_directory(&f, dir.path(), label, permission.upload.bytes);
    let url = live(&mut f);
    let (mut receipt, mut status) = native_requests(&f.harness.pic, input, dir.path(), &url);
    let before = f.harness.pic.get_stable_memory(f.service);
    for args in [&receipt, &status] {
        refused(args, "reference_unknown");
    }
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    unknown_submission(&f, &receipt, &report, &url, reply);
    prepared(&mut f, &manifest, &mut receipt, &mut status);
    let before = f.harness.pic.get_stable_memory(f.service);
    let mut current = input;
    current.operation -= 1;
    std::fs::write(
        dir.path().join("receipt.candid"),
        candid::encode_one(current).unwrap(),
    )
    .unwrap();
    phase_submission(
        &f,
        &receipt,
        &report,
        "unconfirmed-retain",
        ReferenceFailure::Unconfirmed,
        "reference_unconfirmed",
    );
    current.operation -= 1;
    current.action = ReferenceAction::Release;
    std::fs::write(
        dir.path().join("receipt.candid"),
        candid::encode_one(current).unwrap(),
    )
    .unwrap();
    phase_submission(
        &f,
        &receipt,
        &report,
        "unconfirmed-release",
        ReferenceFailure::Unconfirmed,
        "reference_unconfirmed",
    );
    authority(&f, &receipt, dir.path());
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    current = restored(
        &mut f,
        &mut receipt,
        &mut status,
        dir.path(),
        &report,
        current,
    );
    assert_eq!(
        std::fs::read(dir.path().join("receipt.candid")).unwrap(),
        candid::encode_one(current).unwrap()
    );
    save(
        &report,
        "fixture-summary.json",
        &json!({"evidence":"local_standalone_refusal","reply":label,
        "signed_update_intents":4,"uncertain_lookup_resolves_original_outcome":false,
        "provider_gets":0,"deployed_provider_requests":0,"attached_provider_cycles":"0",
        "restored_fenced":true,"reservation_bytes":permission.upload.bytes.to_string(),
        "provider_content":"not_observed","retry_authorized":false}),
    );
}

#[test]
fn signed_reference_inspection_keeps_unknown_unconfirmed_and_restored_refusals() {
    journey(Reply::Pass, "acknowledged");
}
#[test]
fn standalone_native_reference_submission_keeps_lost_reply_uncertain_when_inspection_refuses() {
    journey(Reply::Drop, "dropped");
}
#[test]
fn standalone_native_reference_submission_keeps_pending_reply_unresolved_when_inspection_refuses() {
    journey(Reply::Pending, "pending");
}
