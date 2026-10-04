//! Signed historical recovery; exposure/content/provider outcomes are fixture substitutes.
use super::*;
use crate::authenticated_cli::{PEM, run};
use ic_agent::{Identity, identity::BasicIdentity};
use ic_blob_storage::dto::{
    reference::{ReferenceAction, ReferenceCommand, ReferenceFailure, ReferenceMutationResponse},
    upload::completion::{
        UploadAttestationFailure, UploadAttestationMutation, UploadAttestationRequest,
    },
};
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
fn arguments(
    f: &Fixture,
    url: &str,
    key: &std::path::Path,
    root: &std::path::Path,
    saved: &std::path::Path,
) -> Vec<String> {
    [
        "upload-attestation",
        "--network",
        "local",
        "--url",
        url,
        "--identity",
        key.to_str().unwrap(),
        "--root-key",
        root.to_str().unwrap(),
        "--actor",
        &f.tenant.to_text(),
        "--service",
        &f.service.to_text(),
        "--namespace",
        "1",
        "--verifier",
        &f.operator.to_text(),
        "--statement",
        saved.to_str().unwrap(),
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
fn rejects_conflicts(
    f: &Fixture,
    args: &mut [String],
    saved: &std::path::Path,
    root: &std::path::Path,
    statement: &UploadAttestationRequest,
    accepted: &serde_json::Value,
) {
    for time in [false, true] {
        let mut conflict = *statement;
        if time {
            conflict.observed_at_ns -= 1;
        } else {
            conflict.content_digest[0] ^= 1;
        }
        std::fs::write(saved, candid::encode_one(conflict).unwrap()).unwrap();
        let observed = run(args, 0);
        assert_eq!(observed["outcome"], "conflict");
        assert_eq!(observed["receipt"], accepted["receipt"]);
        assert_eq!(observed["retry_authorized"], false);
    }
    let mut changed = *statement;
    changed.permission.expires_at_ns -= 1;
    std::fs::write(saved, candid::encode_one(changed).unwrap()).unwrap();
    assert_eq!(run(args, 3)["error"], "attestation_refused");
    std::fs::write(saved, candid::encode_one(statement).unwrap()).unwrap();
    change(args, "--verifier", &f.other.to_text());
    assert_eq!(run(args, 3)["error"], "binding");
    change(args, "--verifier", &f.operator.to_text());
    let trusted = std::fs::read(root).unwrap();
    let mut wrong = trusted.clone();
    *wrong.last_mut().unwrap() ^= 1;
    std::fs::write(root, wrong).unwrap();
    assert_eq!(run(args, 3)["error"], "transport");
    std::fs::write(root, trusted).unwrap();
}
fn release(f: &Fixture, upload: ic_blob_storage::dto::reference::ReferenceUpload) {
    f.harness
        .pic
        .update_candid_as::<Result<ReferenceMutationResponse, ReferenceFailure>, _>(
            f.service,
            f.tenant,
            "blob_apply_reference",
            (ReferenceCommand {
                upload,
                operation: 1,
                reference: upload.first_reference,
                action: ReferenceAction::Release,
            },),
        )
        .unwrap()
        .unwrap();
}
#[test]
fn signed_attestation_recovery_preserves_lost_reply_conflicts_and_settled_restored_history() {
    let mut f = Fixture::with_operator(
        Harness::with_builder(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        ),
        Fake::principal(1),
    );
    f.tenant = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    f.enroll(None, true).unwrap();
    let (permission, manifest) = f.permission(u128::MAX, 5);
    f.admit(f.tenant, permission).unwrap();
    f.prepare(&manifest).unwrap();
    f.expose(permission.request).unwrap();
    let statement = UploadAttestationRequest {
        permission: admission_input(permission),
        content_digest: [17; 32],
        observed_at_ns: f.harness.pic.get_time().as_nanos_since_unix_epoch(),
    };
    let dir = tempfile::tempdir().unwrap();
    let key = dir.path().join("identity.pem");
    let root = dir.path().join("root.der");
    let saved = dir.path().join("statement.candid");
    let original = candid::encode_one(statement).unwrap();
    std::fs::write(&key, PEM).unwrap();
    std::fs::write(&root, f.harness.pic.root_key().unwrap()).unwrap();
    std::fs::write(&saved, &original).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let url = live(&mut f);
    let mut args = arguments(&f, &url, &key, &root, &saved);
    let absent = run(&args, 0);
    assert_eq!(absent["outcome"], "absent");
    assert_eq!(absent["retry_authorized"], false);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.harness.pic.stop_live();
    // Deliberately discard the mutation acknowledgment, as a worker with a lost reply must.
    f.harness
        .pic
        .update_candid_as::<Result<UploadAttestationMutation, UploadAttestationFailure>, _>(
            f.service,
            f.operator,
            "blob_attest_upload",
            (statement,),
        )
        .unwrap()
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    change(&mut args, "--url", &live(&mut f));
    let accepted = run(&args, 0);
    assert_eq!(accepted["outcome"], "matched");
    assert_eq!(accepted["upload"]["upload"], u128::MAX.to_string());
    assert_eq!(accepted["retry_authorized"], false);
    rejects_conflicts(&f, &mut args, &saved, &root, &statement, &accepted);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.harness.pic.stop_live();
    release(&f, statement.permission.upload);
    f.fact(
        permission.request,
        blob_test_protocol::storage::ProviderFact::Deleted,
    )
    .unwrap();
    f.fact(
        permission.request,
        blob_test_protocol::storage::ProviderFact::Settled,
    )
    .unwrap();
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    change(&mut args, "--url", &live(&mut f));
    let restored = run(&args, 0);
    assert_eq!(restored["outcome"], "matched");
    assert_eq!(restored["receipt"], accepted["receipt"]);
    assert_eq!(restored["fenced"], true);
    assert_eq!(restored["current_availability"], "not_observed");
    assert_eq!(restored["reference_liveness"], "not_observed");
    assert_eq!(restored["billing_cessation"], "not_established");
    assert_eq!(restored["retry_authorized"], false);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert_eq!(std::fs::read(saved).unwrap(), original);
    f.harness.pic.stop_live();
}
