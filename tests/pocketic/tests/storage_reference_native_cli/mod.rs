//! Local provider facts substitute completion/settlement; tenant queries are signed.
use super::*;
use crate::{
    authenticated_cli::run,
    reference_cli::{change_native, native_requests},
};
use blob_test_protocol::storage::ProviderFact;
use ic_agent::{Identity, identity::BasicIdentity};
use ic_blob_storage_contracts::dto::reference::ReferenceAction;
use ic_blob_storage_contracts::dto::reference::ReferenceCommand;
use ic_blob_storage_contracts::dto::reference::ReferenceFailure;
use ic_blob_storage_contracts::dto::reference::ReferenceMutationResponse;
use ic_testkit::pocket_ic::PocketIcBuilder;
use serde_json::{Value, json};

fn live(f: &mut Fixture, receipt: &mut [String], status: &mut [String]) {
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string();
    for args in [receipt, status] {
        change_native(args, "--url", &url);
    }
}
fn apply(f: &Fixture, command: ReferenceCommand) {
    f.harness
        .pic
        .update_candid_as::<Result<ReferenceMutationResponse, ReferenceFailure>, _>(
            f.service,
            f.tenant,
            "blob_apply_reference",
            (command,),
        )
        .unwrap()
        .unwrap();
}
fn unchanged(f: &Fixture, before: &[u8]) {
    assert!(
        f.harness
            .pic
            .get_stable_memory(f.service)
            .iter()
            .eq(before.iter())
    );
}
fn phase(f: &Fixture, receipt: &[String], status: &[String], live: bool, fenced: bool) -> Value {
    let before = f.harness.pic.get_stable_memory(f.service);
    let history = run(receipt, 0);
    assert_eq!(
        history["result"],
        json!({"state":"success","change":"changed"})
    );
    assert_eq!(history["reference_liveness"], "not_observed");
    assert_eq!(history["fence"], "not_observed");
    assert_eq!(history["retry_authorized"], false);
    let current = run(status, 0);
    assert_eq!(current["live"], live);
    assert_eq!(current["fenced"], fenced);
    assert_eq!(current["publication_authorized"], false);
    unchanged(f, &before);
    history
}

fn failed_receipt(
    f: &mut Fixture,
    input: ReferenceCommand,
    directory: &std::path::Path,
    receipt: &mut [String],
    status: &mut [String],
) -> Vec<String> {
    let before = f.harness.pic.get_stable_memory(f.service);
    let path = directory.join("receipt.candid");
    std::fs::write(
        &path,
        candid::encode_one(ReferenceCommand {
            action: ReferenceAction::Release,
            ..input
        })
        .unwrap(),
    )
    .unwrap();
    assert_eq!(run(receipt, 3)["error"], "reference_conflict");
    std::fs::write(&path, candid::encode_one(input).unwrap()).unwrap();
    unchanged(f, &before);
    f.harness.pic.stop_live();
    // Keep the original object's reserved final-release receipt within its three-slot budget.
    let (permission, manifest) = f.permission(u128::MAX - 3, 8);
    f.admit(f.tenant, permission).unwrap();
    f.prepare(&manifest).unwrap();
    f.expose(permission.request).unwrap();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    let failed = ReferenceCommand {
        upload: admission_input(permission).upload,
        operation: 3,
        action: ReferenceAction::Release,
        ..input
    };
    apply(f, failed);
    apply(
        f,
        ReferenceCommand {
            reference: failed.upload.first_reference,
            operation: 4,
            ..failed
        },
    );
    for fact in [ProviderFact::Deleted, ProviderFact::Settled] {
        f.fact(permission.request, fact).unwrap();
    }
    let path = directory.join("failed.candid");
    std::fs::write(&path, candid::encode_one(failed).unwrap()).unwrap();
    live(f, receipt, status);
    let mut failure = receipt.to_vec();
    change_native(&mut failure, "--request", path.to_str().unwrap());
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        run(&failure, 0)["result"],
        json!({"state":"failure","failure":"unknown_reference"})
    );
    unchanged(f, &before);
    f.harness.pic.stop_live();
    failure
}

#[test]
fn signed_history_stays_successful_after_release_while_current_liveness_and_fence_change() {
    let mut f = Fixture::with_operator(
        Harness::with_builder(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        ),
        Fake::principal(2),
    );
    f.tenant = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    f.enroll(None, true).unwrap();
    let (permission, manifest) = f.permission(u128::MAX - 2, 7);
    f.admit(f.tenant, permission).unwrap();
    f.prepare(&manifest).unwrap();
    f.expose(permission.request).unwrap();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    let input = ReferenceCommand {
        upload: admission_input(permission).upload,
        reference: u128::MAX - 1,
        operation: u128::MAX,
        action: ReferenceAction::Retain,
    };
    let dir = tempfile::tempdir().unwrap();
    let (mut receipt, mut status) =
        native_requests(&f.harness.pic, input, dir.path(), "http://127.0.0.1:1");
    live(&mut f, &mut receipt, &mut status);
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(run(&receipt, 0)["outcome"], "absent");
    assert_eq!(run(&status, 0)["live"], false);
    unchanged(&f, &before);
    f.harness.pic.stop_live();
    // Discard the mutation acknowledgment; the saved exact intent remains the recovery input.
    apply(&f, input);
    live(&mut f, &mut receipt, &mut status);
    let accepted = phase(&f, &receipt, &status, true, false);
    assert_eq!(accepted["operation"], u128::MAX.to_string());
    f.harness.pic.stop_live();
    apply(
        &f,
        ReferenceCommand {
            operation: 2,
            action: ReferenceAction::Release,
            ..input
        },
    );
    live(&mut f, &mut receipt, &mut status);
    assert_eq!(
        phase(&f, &receipt, &status, false, false)["result"],
        accepted["result"]
    );
    let mut failure = failed_receipt(&mut f, input, dir.path(), &mut receipt, &mut status);
    apply(
        &f,
        ReferenceCommand {
            reference: input.upload.first_reference,
            operation: 4,
            action: ReferenceAction::Release,
            ..input
        },
    );
    for fact in [ProviderFact::Deleted, ProviderFact::Settled] {
        f.fact(permission.request, fact).unwrap();
    }
    live(&mut f, &mut receipt, &mut status);
    phase(&f, &receipt, &status, false, false);
    f.harness.pic.stop_live();
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    live(&mut f, &mut receipt, &mut status);
    phase(&f, &receipt, &status, false, true);
    change_native(
        &mut failure,
        "--url",
        &receipt[receipt.iter().position(|s| s == "--url").unwrap() + 1],
    );
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        run(&failure, 0)["result"],
        json!({"state":"failure","failure":"unknown_reference"})
    );
    unchanged(&f, &before);
    assert_eq!(
        std::fs::read(dir.path().join("receipt.candid")).unwrap(),
        candid::encode_one(input).unwrap()
    );
    f.harness.pic.stop_live();
    for bytes in [b"DIDL".to_vec(), vec![0; 4097]] {
        let failure = f
            .harness
            .pic
            .query_call(f.service, f.tenant, "blob_reference_receipt", bytes)
            .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
    }
}
