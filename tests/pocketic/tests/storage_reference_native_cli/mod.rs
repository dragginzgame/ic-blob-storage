//! Local provider facts substitute completion/settlement; tenant queries are signed.
use super::*;
use crate::{
    authenticated_cli::run,
    reference_cli::{change_native, native_requests},
};
use blob_test_protocol::storage::ProviderFact;
use ic_blob_storage_contracts::dto::reference::ReferenceAction;
use ic_blob_storage_contracts::dto::reference::ReferenceCommand;
use ic_blob_storage_contracts::dto::reference::ReferenceFailure;
use ic_blob_storage_contracts::dto::reference::ReferenceMutationResponse;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
use ic_host_fs::testing::FixtureDirectory;
use ic_platform::agent::{BasicIdentity, Identity};
use ic_testkit::pocket_ic::PocketIcBuilder;
use serde_json::{Value, json};

async fn live(f: &mut Fixture, receipt: &mut [String], status: &mut [String]) {
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .await
        .unwrap()
        .to_string();
    for args in [receipt, status] {
        change_native(args, "--url", &url);
    }
}
async fn apply(f: &Fixture, command: ReferenceCommand) {
    f.harness
        .pic
        .update_blob_as::<Result<ReferenceMutationResponse, ReferenceFailure>, _>(
            f.service,
            f.tenant,
            "blob_apply_reference",
            command,
            REPLY_BYTES,
        )
        .await
        .unwrap()
        .unwrap();
}
async fn unchanged(f: &Fixture, before: &[u8]) {
    assert!(
        f.harness
            .pic
            .get_stable_memory(f.service)
            .await
            .unwrap()
            .iter()
            .eq(before.iter())
    );
}
async fn phase(
    f: &Fixture,
    receipt: &[String],
    status: &[String],
    live: bool,
    fenced: bool,
) -> Value {
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
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
    unchanged(f, &before).await;
    history
}

async fn failed_receipt(
    f: &mut Fixture,
    input: ReferenceCommand,
    directory: &std::path::Path,
    receipt: &mut [String],
    status: &mut [String],
) -> Vec<String> {
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let path = directory.join("receipt.wire");
    std::fs::write(
        &path,
        ic_blob_storage_contracts::document::encode(ReferenceCommand {
            action: ReferenceAction::Release,
            ..input
        })
        .unwrap(),
    )
    .unwrap();
    assert_eq!(run(receipt, 3)["error"], "reference_conflict");
    std::fs::write(
        &path,
        ic_blob_storage_contracts::document::encode(input).unwrap(),
    )
    .unwrap();
    unchanged(f, &before).await;
    f.harness.pic.stop_live().await.unwrap();
    // Keep the original object's reserved final-release receipt within its three-slot budget.
    let (permission, manifest) = f.permission(u128::MAX - 3, 8);
    f.admit(f.tenant, permission).await.unwrap();
    f.prepare(&manifest).await.unwrap();
    f.expose(permission.request).await.unwrap();
    let failed = ReferenceCommand {
        upload: admission_input(permission).upload,
        operation: 3,
        action: ReferenceAction::Release,
        ..input
    };
    apply(f, failed).await;
    apply(
        f,
        ReferenceCommand {
            reference: failed.upload.first_reference,
            operation: 4,
            ..failed
        },
    )
    .await;
    for fact in [ProviderFact::Deleted, ProviderFact::Settled] {
        f.fact(permission.request, fact).await.unwrap();
    }
    let path = directory.join("failed.wire");
    std::fs::write(
        &path,
        ic_blob_storage_contracts::document::encode(failed).unwrap(),
    )
    .unwrap();
    live(f, receipt, status).await;
    let mut failure = receipt.to_vec();
    change_native(&mut failure, "--request", path.to_str().unwrap());
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        run(&failure, 0)["result"],
        json!({"state":"failure","failure":"unknown_reference"})
    );
    unchanged(f, &before).await;
    f.harness.pic.stop_live().await.unwrap();
    failure
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered installed scenario retains lifecycle and byte-preservation assertions while making fallible client operations explicit"
)]
async fn signed_history_stays_successful_after_release_while_current_liveness_and_fence_change() {
    let mut f = Fixture::with_operator(
        Harness::with_builder(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        )
        .await,
        fake_principal(2),
    )
    .await;
    f.tenant = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    f.enroll(None, true).await.unwrap();
    let (permission, manifest) = f.permission(u128::MAX - 2, 7);
    f.admit(f.tenant, permission).await.unwrap();
    f.prepare(&manifest).await.unwrap();
    f.expose(permission.request).await.unwrap();
    let input = ReferenceCommand {
        upload: admission_input(permission).upload,
        reference: u128::MAX - 1,
        operation: u128::MAX,
        action: ReferenceAction::Retain,
    };
    let dir = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let (mut receipt, mut status) =
        native_requests(&f.harness.pic, input, dir.path(), "http://127.0.0.1:1").await;
    live(&mut f, &mut receipt, &mut status).await;
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(run(&receipt, 0)["outcome"], "absent");
    assert_eq!(run(&status, 0)["live"], false);
    unchanged(&f, &before).await;
    f.harness.pic.stop_live().await.unwrap();
    // Discard the mutation acknowledgment; the saved exact intent remains the recovery input.
    apply(&f, input).await;
    live(&mut f, &mut receipt, &mut status).await;
    let accepted = phase(&f, &receipt, &status, true, false).await;
    assert_eq!(accepted["operation"], u128::MAX.to_string());
    f.harness.pic.stop_live().await.unwrap();
    apply(
        &f,
        ReferenceCommand {
            operation: 2,
            action: ReferenceAction::Release,
            ..input
        },
    )
    .await;
    live(&mut f, &mut receipt, &mut status).await;
    assert_eq!(
        phase(&f, &receipt, &status, false, false).await["result"],
        accepted["result"]
    );
    let mut failure = failed_receipt(&mut f, input, dir.path(), &mut receipt, &mut status).await;
    apply(
        &f,
        ReferenceCommand {
            reference: input.upload.first_reference,
            operation: 4,
            action: ReferenceAction::Release,
            ..input
        },
    )
    .await;
    for fact in [ProviderFact::Deleted, ProviderFact::Settled] {
        f.fact(permission.request, fact).await.unwrap();
    }
    live(&mut f, &mut receipt, &mut status).await;
    phase(&f, &receipt, &status, false, false).await;
    f.harness.pic.stop_live().await.unwrap();
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation("fixture_post_upgrade", f.operator),
            Some(f.controller),
        )
        .await
        .unwrap();
    live(&mut f, &mut receipt, &mut status).await;
    phase(&f, &receipt, &status, false, true).await;
    change_native(
        &mut failure,
        "--url",
        &receipt[receipt.iter().position(|s| s == "--url").unwrap() + 1],
    );
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        run(&failure, 0)["result"],
        json!({"state":"failure","failure":"unknown_reference"})
    );
    unchanged(&f, &before).await;
    assert_eq!(
        std::fs::read(dir.path().join("receipt.wire")).unwrap(),
        ic_blob_storage_contracts::document::encode(input).unwrap()
    );
    f.harness.pic.stop_live().await.unwrap();
    for bytes in [b"DIDL".to_vec(), vec![0; 4097]] {
        let failure = f
            .harness
            .pic
            .query_call(f.service, f.tenant, "blob_reference_receipt", bytes)
            .await
            .unwrap_err();
        assert_eq!(
            crate::canister_rejection(&failure).reject_code,
            RejectCode::CanisterError
        );
    }
}
