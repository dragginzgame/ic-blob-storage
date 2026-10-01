//! Common native signing/trust tooling against the public managed fixture.
mod references;
mod upload_setup;
mod verifier;
use super::{Fixture, endpoints::manifest};
use crate::{
    account_native_cli::OUTSIDER_PEM,
    authenticated_cli::{PEM, arguments, run},
};
use ic_agent::{Identity, identity::BasicIdentity};
use ic_blob_storage::dto::{
    operator::OperatorScope,
    tenant::{TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest},
    upload::{
        admission::{UploadAdmissionFailure, UploadAdmissionMutation, UploadAdmissionRequest},
        manifest::{UploadManifestFailure, UploadManifestMutation, UploadManifestRequest},
    },
};
use ic_testkit::{
    pic::CandidCallExt,
    pocket_ic::{PocketIc, common::rest::RawSubnetId},
};
use serde_json::{Value, json};
use std::{path::Path, time::Duration};

fn change(args: &mut [String], flag: &str, value: &str) {
    let index = args.iter().position(|s| s == flag).unwrap();
    args[index + 1] = value.into();
}

pub(super) fn local_subnet_key(f: &Fixture) -> Vec<u8> {
    // The public Canic helper owns an application-only instance. With no NNS,
    // its certificates have no delegation. Pin that subnet's key through the
    // independently controlled test API, never through the CLI's replica URL.
    assert!(f.pic().topology().get_nns().is_none());
    let subnet = f.pic().get_subnet(f.app()).unwrap();
    let url = f
        .pic()
        .get_server_url()
        .join(&format!("instances/{}/read/pub_key", f.pic().instance_id()))
        .unwrap();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let response = reqwest::Client::builder()
                .no_proxy()
                .redirect(reqwest::redirect::Policy::none())
                .retry(reqwest::retry::never())
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap()
                .post(url)
                .header("Content-Type", "application/json")
                .body(serde_json::to_vec(&RawSubnetId::from(subnet)).unwrap())
                .send()
                .await
                .unwrap()
                .error_for_status()
                .unwrap();
            serde_json::from_slice(&response.bytes().await.unwrap()).unwrap()
        })
}

pub(super) fn live(f: &Fixture) -> (PocketIc, String) {
    // The public helper keeps ownership. This supported non-owning handle only
    // controls its HTTP gateway; dropping it never deletes the managed instance.
    let mut pic = PocketIc::new_from_existing_instance(
        f.pic().get_server_url(),
        f.pic().instance_id(),
        Some(30_000),
    );
    let url = pic.make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None);
    (pic, url.to_string())
}

fn status_and_history(args: &[String], fenced: bool) -> Value {
    let status = run(args, 0);
    for owner in ["uploads", "funding", "gateways", "reads"] {
        assert_eq!(status[owner]["fenced"], fenced);
    }
    let mut history = args.to_vec();
    history[0] = "funding-history".into();
    let page = run(&history, 0);
    assert_eq!(page["observation"], "funding_history");
    assert_eq!(page["fenced"], fenced);
    assert_eq!(page["entries"], json!([]));
    assert_eq!(page["next"], Value::Null);
    status
}

fn upload_history(args: &[String]) -> Vec<String> {
    let mut result = args.to_vec();
    result[0] = "upload-history".into();
    for flag in ["--cashier", "--payer"] {
        let index = result.iter().position(|s| s == flag).unwrap();
        result.drain(index..=index + 1);
    }
    result.extend(["--filter".into(), "all".into()]);
    result
}

fn assessment(args: &[String], permission: &Path) -> Vec<String> {
    let mut result = upload_history(args);
    result[0] = "certificate-assessment".into();
    let index = result.iter().position(|s| s == "--operator").unwrap();
    result[index] = "--actor".into();
    let index = result.iter().position(|s| s == "--filter").unwrap();
    result.drain(index..=index + 1);
    result.extend(["--permission".into(), permission.to_str().unwrap().into()]);
    result
}

fn enroll_and_admit(
    f: &Fixture,
    scope: OperatorScope,
    signer: candid::Principal,
) -> UploadManifestRequest {
    let tenant = TenantScope {
        service: scope.service,
        namespace: scope.namespace,
        tenant: scope.payment_account,
    };
    f.pic()
        .update_candid_as::<Result<TenantEnrollmentResponse, TenantFailure>, _>(
            f.app(),
            signer,
            "blob_update_tenant",
            (TenantUpdateRequest {
                scope: tenant,
                expected: None,
                active: true,
            },),
        )
        .unwrap()
        .unwrap();
    // This signer is explicitly bound as both operator and uploader.
    let declaration = manifest(f, tenant.tenant, signer);
    f.pic()
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.app(),
            tenant.tenant,
            "blob_admit_upload",
            (declaration.permission,),
        )
        .unwrap()
        .unwrap();
    declaration
}

fn identity_scope_and_trust_refusals(args: &[String], key: &Path, root: &Path, trusted: &[u8]) {
    let outsider = BasicIdentity::from_raw_key(&[43; 32]).sender().unwrap();
    let mut wrong = args.to_vec();
    change(&mut wrong, "--operator", &outsider.to_text());
    assert_eq!(run(&wrong, 3)["error"], "identity_binding");
    // A different valid signer reaches service authorization and is refused.
    std::fs::write(key, OUTSIDER_PEM).unwrap();
    assert_eq!(run(&wrong, 3)["error"], "denied");
    std::fs::write(key, PEM).unwrap();
    for (flag, value) in [
        ("--namespace", "1".into()),
        ("--payer", outsider.to_text()),
        ("--cashier", outsider.to_text()),
    ] {
        let mut wrong = args.to_vec();
        change(&mut wrong, flag, &value);
        assert_eq!(run(&wrong, 3)["error"], "binding");
    }
    let mut untrusted = trusted.to_vec();
    *untrusted.last_mut().unwrap() ^= 1;
    std::fs::write(root, untrusted).unwrap();
    assert_eq!(run(args, 3)["error"], "transport");
    std::fs::write(root, trusted).unwrap();
}

fn exact_permission_refusal(args: &[String], saved: &Path, original: UploadAdmissionRequest) {
    let mut changed = original;
    changed.expires_at_ns -= 1;
    std::fs::write(saved, candid::encode_one(changed).unwrap()).unwrap();
    assert_eq!(run(args, 3)["error"], "binding");
    std::fs::write(saved, candid::encode_one(original).unwrap()).unwrap();
}

fn prepared_assessment(args: &[String], permission: UploadAdmissionRequest) {
    let value = run(args, 0);
    assert_eq!(value["observation"], "certificate_assessment");
    assert_eq!(
        value["upload"]["object"],
        permission.upload.object.to_string()
    );
    assert_eq!(value["expires_at_ns"], u64::MAX.to_string());
    assert_eq!(
        value["blockers"],
        json!([
            "precharge_limits",
            "provider_namespace",
            "replay_charging",
            "recovery"
        ])
    );
    assert_eq!(value["issuance_authorized"], false);
    assert_eq!(value["retry_authorized"], false);
}

fn verification_arguments(certificate: &[String], body: &Path) -> Vec<String> {
    let mut args = certificate.to_vec();
    args[0] = "verify-upload".into();
    args.extend([
        "--body".into(),
        body.to_str().unwrap().into(),
        "--max-bytes".into(),
        "10".into(),
    ]);
    args
}

fn verify_bytes_and_refuse_changes(args: &[String], body: &Path) -> Value {
    let report = run(args, 0);
    assert_eq!(report["observation"], "local_upload_bytes");
    assert_eq!(report["manifest_authentication"], "query_signatures");
    assert_eq!(report["upload"]["bytes"], "10");
    assert_eq!(
        report["content_digest"],
        ic_blob_storage::model::identity::ContentDigest::compute(&[42; 10]).to_string()
    );
    assert_eq!(report["provider_completion"], "not_established");
    assert_eq!(report["provider_availability"], "not_observed");
    assert_eq!(report["retry_authorized"], false);
    for changed in [&[43; 10][..], &[42; 9][..], &[42; 11][..]] {
        std::fs::write(body, changed).unwrap();
        assert_eq!(run(args, 3)["error"], "content_mismatch");
    }
    std::fs::write(body, [42; 10]).unwrap();
    report
}

#[test]
fn managed_signed_cli_preserves_identity_trust_exact_permission_and_fenced_inventory() {
    let signer = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    let mut input = blob_canic_probe::configuration::input();
    input.operator = signer;
    let f = Fixture::with_input(&input);
    let scope = OperatorScope {
        service: f.app(),
        namespace: input.namespace,
        cashier: input.billing.cashier,
        payment_account: input.payment_account,
    };
    let declaration = enroll_and_admit(&f, scope, signer);
    let directory = tempfile::tempdir().unwrap();
    let key = directory.path().join("identity.pem");
    let root = directory.path().join("root.der");
    let saved = directory.path().join("permission.candid");
    let body = directory.path().join("body");
    std::fs::write(&key, PEM).unwrap();
    let trusted = local_subnet_key(&f);
    std::fs::write(&root, &trusted).unwrap();
    std::fs::write(&saved, candid::encode_one(declaration.permission).unwrap()).unwrap();
    std::fs::write(&body, [42; 10]).unwrap();
    let (mut replica, url) = live(&f);
    let args = arguments("status", scope, signer, &url, &key, &root);
    let certificate = assessment(&args, &saved);
    let verification = verification_arguments(&certificate, &body);
    let before = f.pic().get_stable_memory(f.app());
    let status = status_and_history(&args, false);
    assert_eq!(status["operator"], signer.to_text());
    assert_eq!(status["scope"]["namespace"], u128::MAX.to_string());
    assert_eq!(
        status["funding"]["available_allocation"],
        u128::MAX.to_string()
    );
    assert_eq!(run(&certificate, 3)["error"], "unprepared");
    assert_eq!(run(&verification, 3)["error"], "unprepared");
    identity_scope_and_trust_refusals(&args, &key, &root, &trusted);
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
    f.pic().stop_progress();
    f.pic()
        .update_candid_as::<Result<UploadManifestMutation, UploadManifestFailure>, _>(
            f.app(),
            signer,
            "blob_prepare_upload",
            (&declaration,),
        )
        .unwrap()
        .unwrap();
    let before = f.pic().get_stable_memory(f.app());
    f.pic().auto_progress();
    prepared_assessment(&certificate, declaration.permission);
    let report = verify_bytes_and_refuse_changes(&verification, &body);
    let history_args = upload_history(&args);
    let history = run(&history_args, 0);
    assert_eq!(
        history["entries"][0]["upload"]["object"],
        declaration.permission.upload.object.to_string()
    );
    assert_eq!(history["entries"][0]["state"], "reserved");
    assert_eq!(history["fenced"], false);
    assert_eq!(history["retry_authorized"], false);
    exact_permission_refusal(&certificate, &saved, declaration.permission);
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
    f.pic().stop_progress();
    f.upgrade_same_release(Duration::from_secs(5));
    let before = f.pic().get_stable_memory(f.app());
    f.pic().auto_progress();
    status_and_history(&args, true);
    assert_eq!(
        run(&certificate, 3)["error"],
        "assessment_permission_fenced"
    );
    let restored = run(&history_args, 0);
    assert_eq!(restored["entries"], history["entries"]);
    assert_eq!(restored["fenced"], true);
    assert_eq!(restored["retry_authorized"], false);
    let restored_bytes = run(&verification, 0);
    assert_eq!(restored_bytes["content_digest"], report["content_digest"]);
    assert_eq!(restored_bytes["provider_completion"], "not_established");
    assert_eq!(restored_bytes["retry_authorized"], false);
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
    replica.stop_live();
}
