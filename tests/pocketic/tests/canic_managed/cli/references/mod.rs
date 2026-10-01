//! Actual signed tenant operations beside trusted completion; provider bytes/exposure are local.
mod download;
use super::{Fixture, OUTSIDER_PEM, live, local_subnet_key, manifest, verifier};
use crate::{
    authenticated_cli::{PEM, run},
    submission_proxy::{Dispatch, Proxy, Reply},
};
use candid::Principal;
use ic_agent::{Identity, identity::BasicIdentity};
use ic_blob_storage::dto::{
    download::{DownloadFailure, DownloadRequest, DownloadResponse},
    operator::{LocalServiceStatus, LocalStatusFailure, OperatorScope},
    reference::{
        ReferenceAction, ReferenceCommand,
        capacity::{ReferenceCapacityFailure, ReferenceCapacityRequest, ReferenceCapacityResponse},
        status::ReferenceStatusRequest,
    },
    tenant::{TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest},
    upload::{
        admission::{UploadAdmissionFailure, UploadAdmissionMutation, UploadAdmissionRequest},
        manifest::{UploadManifestFailure, UploadManifestMutation},
    },
};
use ic_testkit::pic::CandidCallExt;
use serde_json::{Value, json};
use std::{
    net::TcpListener,
    path::{Path, PathBuf},
    time::Duration,
};

fn save(directory: &Path, name: &str, value: &Value) {
    std::fs::write(
        directory.join(name),
        serde_json::to_vec_pretty(value).unwrap(),
    )
    .unwrap();
}
fn change(args: &mut [String], flag: &str, value: &str) {
    let index = args.iter().position(|s| s == flag).unwrap();
    args[index + 1] = value.into();
}
fn update_tenant(
    f: &Fixture,
    scope: TenantScope,
    expected: Option<ic_blob_storage::dto::tenant::TenantEnrollment>,
    active: bool,
) -> TenantEnrollmentResponse {
    f.pic()
        .update_candid_as::<Result<_, TenantFailure>, _>(
            f.app(),
            Principal::from_slice(&[2, 1]),
            "blob_update_tenant",
            (TenantUpdateRequest {
                scope,
                expected,
                active,
            },),
        )
        .unwrap()
        .unwrap()
}
fn prepare(f: &Fixture, scope: TenantScope) -> UploadAdmissionRequest {
    let input = manifest(f, scope.tenant, Principal::from_slice(&[6, 1]));
    f.pic()
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.app(),
            scope.tenant,
            "blob_admit_upload",
            (input.permission,),
        )
        .unwrap()
        .unwrap();
    f.pic()
        .update_candid_as::<Result<UploadManifestMutation, UploadManifestFailure>, _>(
            f.app(),
            input.permission.uploader,
            "blob_prepare_upload",
            (&input,),
        )
        .unwrap()
        .unwrap();
    f.pic()
        .update_candid_as::<Result<(), blob_canic_probe::ProbeExposureFailure>, _>(
            f.app(),
            Principal::from_slice(&[2, 1]),
            "probe_expose_upload",
            (input.permission,),
        )
        .unwrap()
        .unwrap();
    input.permission
}
fn complete(f: &Fixture, base: &[String], report: &Path, permission: UploadAdmissionRequest) {
    let observed = report.join("observed");
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let mut args = verifier::command(base, "observe-upload", &observed);
    args.extend([
        "--permission".into(),
        report.join("permission.candid").to_str().unwrap().into(),
        "--gateway".into(),
        format!("http://{}/", listener.local_addr().unwrap()),
        "--max-bytes".into(),
        "10".into(),
    ]);
    verifier::observe(f, &args, &observed, &listener, permission);
    assert_eq!(
        run(&verifier::command(base, "submit-attestation", &observed), 0)["outcome"],
        "accepted"
    );
}

struct Tenant<'a> {
    f: &'a Fixture,
    base: Vec<String>,
    report: &'a Path,
    retain: ReferenceCommand,
}
impl Tenant<'_> {
    fn arguments(&self, name: &str, command: ReferenceCommand, label: &str) -> Vec<String> {
        let file = self.report.join(format!("{label}.candid"));
        std::fs::write(&file, candid::encode_one(command).unwrap()).unwrap();
        let mut args = self.base.clone();
        args[0] = name.into();
        args.extend(["--request".into(), file.to_str().unwrap().into()]);
        args
    }
    fn receipt(&self, command: ReferenceCommand, label: &str) -> Value {
        let value = run(&self.arguments("reference-receipt", command, label), 0);
        save(self.report, &format!("{label}.json"), &value);
        assert_eq!(value["reference_liveness"], "not_observed");
        assert_eq!(value["fence"], "not_observed");
        assert_eq!(value["retry_authorized"], false);
        value
    }
    fn status(&self, reference: u128, live: bool, fenced: bool, label: &str) {
        let file = self.report.join(format!("{label}.candid"));
        std::fs::write(
            &file,
            candid::encode_one(ReferenceStatusRequest {
                upload: self.retain.upload,
                reference,
            })
            .unwrap(),
        )
        .unwrap();
        let mut args = self.base.clone();
        args[0] = "reference-status".into();
        args.extend(["--request".into(), file.to_str().unwrap().into()]);
        let value = run(&args, 0);
        assert_eq!(value["live"], live);
        assert_eq!(value["fenced"], fenced);
        assert_eq!(value["publication_authorized"], false);
        save(self.report, &format!("{label}.json"), &value);
    }
    fn submit(&self, command: ReferenceCommand, label: &str, exit: i32) -> Value {
        let mut args = self.arguments("submit-reference", command, label);
        args.extend([
            "--run-dir".into(),
            self.report.join(label).to_str().unwrap().into(),
        ]);
        let value = run(&args, exit);
        save(self.report, &format!("{label}-cli.json"), &value);
        value
    }
    fn download(&self, reference: u128) -> Result<DownloadResponse, DownloadFailure> {
        let u = self.retain.upload;
        self.f
            .pic()
            .update_candid_as(
                self.f.app(),
                u.tenant,
                "blob_download_descriptor",
                (DownloadRequest {
                    service: u.service,
                    namespace: u.namespace,
                    tenant: u.tenant,
                    object: u.object,
                    incarnation: u.incarnation,
                    root: u.root,
                    reference,
                },),
            )
            .unwrap()
    }
    fn capacity(&self) -> ReferenceCapacityResponse {
        self.f
            .pic()
            .query_candid_as::<Result<_, ReferenceCapacityFailure>, _>(
                self.f.app(),
                self.retain.upload.tenant,
                "blob_reference_capacity",
                (ReferenceCapacityRequest {
                    scope: TenantScope {
                        service: self.f.app(),
                        namespace: u128::MAX,
                        tenant: self.retain.upload.tenant,
                    },
                    root: self.retain.upload.root,
                },),
            )
            .unwrap()
            .unwrap()
    }
    fn accounting(&self, fenced: bool) {
        let input = blob_canic_probe::configuration::input();
        let status: LocalServiceStatus = self
            .f
            .pic()
            .query_candid_as::<Result<_, LocalStatusFailure>, _>(
                self.f.app(),
                input.operator,
                "blob_local_status",
                (OperatorScope {
                    service: self.f.app(),
                    namespace: input.namespace,
                    cashier: input.billing.cashier,
                    payment_account: input.payment_account,
                },),
            )
            .unwrap()
            .unwrap();
        assert_eq!(
            [
                status.uploads.reserved_bytes,
                status.uploads.logical_bytes,
                status.uploads.physical_bytes,
                status.uploads.liability_bytes
            ],
            [0, 0, 10, 10]
        );
        assert_eq!(
            [
                status.uploads.fenced,
                status.funding.fenced,
                status.gateways.fenced,
                status.reads.fenced
            ],
            [fenced; 4]
        );
    }
}

fn refusals(t: &Tenant<'_>, keys: &Path) {
    let before = t.f.pic().get_stable_memory(t.f.app());
    let dir = t.report.join("refused");
    let mut args = t.arguments("submit-reference", t.retain, "authority-request");
    args.extend(["--run-dir".into(), dir.to_str().unwrap().into()]);
    change(&mut args, "--namespace", "1");
    assert_eq!(run(&args, 3)["error"], "binding");
    change(&mut args, "--namespace", &u128::MAX.to_string());
    change(
        &mut args,
        "--identity",
        keys.join("verifier.pem").to_str().unwrap(),
    );
    assert_eq!(run(&args, 3)["error"], "identity_binding");
    change(
        &mut args,
        "--actor",
        &BasicIdentity::from_raw_key(&[42; 32])
            .sender()
            .unwrap()
            .to_text(),
    );
    assert_eq!(run(&args, 3)["error"], "denied");
    assert!(!dir.exists());
    // An interrupted claim, even with no files, cannot authorize a new submission.
    change(&mut args, "--actor", &t.retain.upload.tenant.to_text());
    change(
        &mut args,
        "--identity",
        keys.join("identity.pem").to_str().unwrap(),
    );
    std::fs::create_dir(&dir).unwrap();
    assert_eq!(run(&args, 3)["error"], "submission_already_claimed");
    assert!(std::fs::read_dir(&dir).unwrap().next().is_none());
    assert_eq!(t.f.pic().get_stable_memory(t.f.app()), before);
}

fn retain_and_recover(t: &Tenant<'_>, url: &str, reply: Reply) {
    assert_eq!(t.receipt(t.retain, "absent")["outcome"], "absent");
    t.status(t.retain.reference, false, false, "before-retain");
    let directory = t.report.join("retain");
    let proxy = Proxy::start(
        url.into(),
        directory.clone(),
        Dispatch {
            service: t.f.app(),
            actor: t.retain.upload.tenant,
            method: "blob_apply_reference",
            argument_file: "request.candid",
        },
        reply,
    );
    let mut args = t.arguments("submit-reference", t.retain, "retain-request");
    args.extend(["--run-dir".into(), directory.to_str().unwrap().into()]);
    change(&mut args, "--url", &proxy.url);
    let value = run(&args, if reply == Reply::Drop { 3 } else { 0 });
    save(t.report, "retain-cli.json", &value);
    let expected = match reply {
        Reply::Pass => "recorded",
        Reply::Drop => "uncertain",
        Reply::Pending => "pending",
    };
    let outcome: Value =
        serde_json::from_slice(&std::fs::read(directory.join("outcome.json")).unwrap()).unwrap();
    assert_eq!(outcome["outcome"], expected);
    assert_eq!(outcome["retry_authorized"], false);
    let intent = std::fs::read(directory.join("intent.json")).unwrap();
    assert_eq!(run(&args, 3)["error"], "submission_already_claimed");
    assert_eq!(
        std::fs::read(directory.join("intent.json")).unwrap(),
        intent
    );
    let request_bytes = std::fs::read(directory.join("request.candid")).unwrap();
    assert_eq!(request_bytes, candid::encode_one(t.retain).unwrap());
    // Recover with the durable dispatch copy, even when the incoming file is gone.
    std::fs::remove_file(t.report.join("retain-request.candid")).unwrap();
    args[0] = "reference-receipt".into();
    let index = args.iter().position(|s| s == "--run-dir").unwrap();
    args.drain(index..=index + 1);
    change(
        &mut args,
        "--request",
        directory.join("request.candid").to_str().unwrap(),
    );
    let recovered = run(&args, 0);
    assert_eq!(recovered["outcome"], "found");
    assert_eq!(
        recovered["result"],
        json!({"state":"success","change":"changed"})
    );
    assert_eq!(recovered["reference_liveness"], "not_observed");
    save(t.report, "recovered-retain.json", &recovered);
    assert_eq!(proxy.calls(), 1);
    drop(proxy);
    t.status(t.retain.reference, true, false, "after-retain");
    assert_eq!(t.download(t.retain.reference).unwrap().bytes, 10);
}

fn cleanup(t: &Tenant<'_>, enrollment: TenantEnrollmentResponse, failed: ReferenceCommand) {
    assert_eq!(t.capacity().headroom.unwrap().fresh_retains, 0);
    let first = t.retain.upload.first_reference;
    let first_release = ReferenceCommand {
        reference: first,
        operation: u128::MAX - 7,
        action: ReferenceAction::Release,
        ..t.retain
    };
    assert_eq!(
        t.submit(first_release, "release-first", 0)["result"]["state"],
        "success"
    );
    t.status(first, false, false, "first-released");
    assert_eq!(t.download(first), Err(DownloadFailure::Unavailable));
    assert_eq!(t.download(t.retain.reference).unwrap().bytes, 10);
    update_tenant(t.f, enrollment.scope, enrollment.enrollment, false);
    let fresh = ReferenceCommand {
        reference: 1,
        operation: 1,
        ..t.retain
    };
    assert_eq!(
        t.submit(fresh, "inactive-retain", 3)["error"],
        "reference_inactive"
    );
    assert_eq!(
        t.download(t.retain.reference),
        Err(DownloadFailure::Inactive)
    );
    let last = ReferenceCommand {
        operation: u128::MAX - 8,
        action: ReferenceAction::Release,
        ..t.retain
    };
    assert_eq!(
        t.submit(last, "release-last", 0)["result"]["state"],
        "success"
    );
    t.status(t.retain.reference, false, false, "last-released");
    // A deliberate exact service replay is a distinct caller-authorized dispatch.
    // Recovery above never sends it; immutable history must not resurrect a reference.
    assert_eq!(t.submit(t.retain, "explicit-replay", 0)["replayed"], true);
    t.status(t.retain.reference, false, false, "replay-not-live");
    let accepted = t.receipt(t.retain, "historical-success");
    let original_failure = t.receipt(failed, "historical-failure");
    assert_eq!(accepted["result"]["state"], "success");
    assert_eq!(original_failure["result"]["failure"], "unknown_reference");
    t.accounting(false);
    t.f.pic().stop_progress();
    t.f.upgrade_same_release(Duration::from_secs(5));
    let before = t.f.pic().get_stable_memory(t.f.app());
    t.f.pic().auto_progress();
    assert_eq!(t.receipt(t.retain, "restored-success"), accepted);
    assert_eq!(t.receipt(failed, "restored-failure"), original_failure);
    t.status(first, false, true, "restored-first");
    t.status(t.retain.reference, false, true, "restored-last");
    assert_eq!(
        t.submit(t.retain, "fenced-replay", 3)["error"],
        "reference_fenced"
    );
    assert_eq!(t.download(t.retain.reference), Err(DownloadFailure::Fenced));
    t.accounting(true);
    assert_eq!(t.f.pic().get_stable_memory(t.f.app()), before);
}

fn journey(reply: Reply, label: &str) {
    let temporary = tempfile::tempdir().unwrap();
    let report = std::env::var_os("BLOB_MANAGED_REFERENCE_REPORT").map_or_else(
        || temporary.path().join(label),
        |root| PathBuf::from(root).join(label),
    );
    std::fs::create_dir(&report).unwrap();
    let tenant = BasicIdentity::from_raw_key(&[43; 32]).sender().unwrap();
    let verifier_identity = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    let mut input = blob_canic_probe::configuration::input();
    input.completion_verifier = verifier_identity;
    input.resources.max_receipts_per_object = 4;
    let f = Fixture::with_input(&input);
    let scope = TenantScope {
        service: f.app(),
        namespace: input.namespace,
        tenant,
    };
    save(
        &report,
        "fixture-plan.json",
        &json!({"evidence":"local_substitute","reply":label,
        "service":f.app().to_text(),"namespace":input.namespace.to_string(),"project":input.project,
        "operator":input.operator.to_text(),"tenant":tenant.to_text(),"verifier":verifier_identity.to_text(),
        "uploader":Principal::from_slice(&[6,1]).to_text(),"max_signed_updates":10,"max_cli_invocations":60,
        "max_source_gets":1,"max_content_bytes":"10","deployed_provider_requests":0,"attached_provider_cycles":"0"}),
    );
    let enrollment = update_tenant(&f, scope, None, true);
    let permission = prepare(&f, scope);
    std::fs::write(
        report.join("permission.candid"),
        candid::encode_one(permission).unwrap(),
    )
    .unwrap();
    let root = local_subnet_key(&f);
    std::fs::write(temporary.path().join("root.der"), &root).unwrap();
    std::fs::write(report.join("fixture-root.der"), root).unwrap();
    std::fs::write(temporary.path().join("identity.pem"), PEM).unwrap();
    std::fs::write(temporary.path().join("verifier.pem"), PEM).unwrap();
    let (mut replica, url) = live(&f);
    let base = verifier::arguments(&f, verifier_identity, &url, temporary.path());
    complete(&f, &base, &report, permission);
    std::fs::write(temporary.path().join("identity.pem"), OUTSIDER_PEM).unwrap();
    let t = Tenant {
        f: &f,
        base: verifier::arguments(&f, tenant, &url, temporary.path()),
        report: &report,
        retain: ReferenceCommand {
            upload: permission.upload,
            reference: u128::MAX - 4,
            operation: u128::MAX - 5,
            action: ReferenceAction::Retain,
        },
    };
    refusals(&t, temporary.path());
    let failed = ReferenceCommand {
        reference: u128::MAX - 9,
        operation: u128::MAX - 6,
        action: ReferenceAction::Release,
        ..t.retain
    };
    let value = t.submit(failed, "unknown-release", 0);
    assert_eq!(value["outcome"], "recorded");
    assert_eq!(
        value["result"],
        json!({"state":"failure","failure":"unknown_reference"})
    );
    retain_and_recover(&t, &url, reply);
    cleanup(&t, enrollment, failed);
    save(
        &report,
        "fixture-summary.json",
        &json!({"evidence":"local_substitute","reply":label,
        "source_gets":1,"signed_updates":8,"reference_update_retries":0,
        "explicit_historical_replays":2,"deployed_provider_requests":0,"attached_provider_cycles":"0",
        "logical_bytes":"0","physical_bytes":"10","liability_bytes":"10","restored_fenced":true}),
    );
    replica.stop_live();
}

#[test]
fn managed_native_tenant_reference_acknowledgment_cleanup_and_fenced_history() {
    journey(Reply::Pass, "acknowledged");
}
#[test]
fn managed_native_tenant_reference_lost_reply_recovers_without_resend() {
    journey(Reply::Drop, "dropped");
}
#[test]
fn managed_native_tenant_reference_pending_reply_recovers_without_resend() {
    journey(Reply::Pending, "pending");
}
