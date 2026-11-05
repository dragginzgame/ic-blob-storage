//! Passive funding admission against actual transfer journals, not a payment client.
use super::*;
use blob_test_protocol::funding::preview::{
    FundingPreviewBlocker, FundingPreviewFailure, FundingPreviewRequest, FundingPreviewView,
};
use ic_blob_storage_pocketic_tests::fixture_calls;

async fn request(f: &Fixture, id: u64, amount: u128) -> FundingPreviewRequest {
    FundingPreviewRequest {
        service: f.sender,
        peer: f.receiver,
        id,
        requested_cycles: amount,
        revision: f.funding_status().await.budget.revision,
    }
}
async fn preview(f: &Fixture, input: FundingPreviewRequest) -> FundingPreviewView {
    let before = f.journals().await;
    let (result,): (Result<FundingPreviewView, FundingPreviewFailure>,) =
        fixture_calls::query_fixture(
            &f.harness.pic,
            f.sender,
            f.driver,
            "preview_funding",
            (input,),
        )
        .await
        .unwrap();
    assert_eq!(
        f.journals().await,
        before,
        "preview cannot consume identities or receipts"
    );
    let result = result.unwrap();
    assert_eq!(result.request, input);
    assert_eq!(result.available_cycles, None);
    result
}
async fn blocked(f: &Fixture, id: u64, amount: u128) -> Vec<FundingPreviewBlocker> {
    let view = preview(f, request(f, id, amount).await).await;
    view.blockers
}
async fn transfer(
    f: &Fixture,
    id: u64,
    accept: u128,
    trap: bool,
) -> Result<Result<FundingObservation, FundingFailure>, ic_testkit::pocket_ic::ClientError> {
    f.harness
        .pic
        .update_call(
            f.sender,
            f.driver,
            "fund",
            blob_test_protocol::fixture_wire::encode_one(
                "fund",
                FundingRequest {
                    id,
                    offered: 1_000_000,
                    accept,
                    reply: FundingReplyMode::Success,
                    trap_callback: trap,
                },
            )
            .unwrap(),
        )
        .await
        .map(|bytes| {
            blob_test_protocol::fixture_wire::decode_one(
                "fund",
                &bytes,
                blob_test_protocol::fixture_wire::MAX_BYTES,
            )
            .unwrap()
        })
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn empty_journal_and_gross_cycles_cannot_supply_missing_funding_evidence() {
    let f = Fixture::new().await;
    for _ in 0..2 {
        let blockers = blocked(&f, u64::MAX, u128::MAX).await;
        for name in [
            FundingPreviewBlocker::ProviderUnqualified,
            FundingPreviewBlocker::RecoveryUnknown,
            FundingPreviewBlocker::NotConfigured,
            FundingPreviewBlocker::SpendabilityUnknown,
        ] {
            assert!(blockers.contains(&name));
        }
        assert!(!blockers.contains(&FundingPreviewBlocker::FundingUncertain));
        assert!(!blockers.contains(&FundingPreviewBlocker::AlreadyAdmitted));
    }
    f.harness
        .pic
        .add_cycles(f.sender, 5_000_000_000_000)
        .await
        .unwrap();
    let blockers = blocked(&f, u64::MAX, 1).await;
    assert!(blockers.contains(&FundingPreviewBlocker::SpendabilityUnknown));
    assert!(
        !blockers
            .iter()
            .any(|b| matches!(b, FundingPreviewBlocker::ReserveWouldBeViolated { .. }))
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn budget_revision_changes_on_admission_and_completion_even_for_full_refunds() {
    let f = Fixture::new().await;
    let before = request(&f, 3, 1).await;
    let initial = preview(&f, before).await.budget;
    transfer(&f, 1, 0, false).await.unwrap().unwrap();
    let after = preview(&f, before).await;
    assert!(
        after
            .blockers
            .contains(&FundingPreviewBlocker::BudgetRevisionStale)
    );
    assert_eq!(after.budget.revision, 2);
    assert_eq!(after.budget.available, initial.available);
    assert_eq!(after.budget.refunded, 1_000_000);
    let current = preview(&f, request(&f, 3, 1).await).await;
    assert!(
        !current
            .blockers
            .contains(&FundingPreviewBlocker::BudgetRevisionStale)
    );
    assert!(current.available_cycles.is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn credit_uncertainty_and_identity_reuse_survive_later_refunds_and_restore() {
    let f = Fixture::new().await;
    transfer(&f, 1, 400_000, false).await.unwrap().unwrap();
    transfer(&f, 2, 0, false).await.unwrap().unwrap();
    let blockers = blocked(&f, 3, 1).await;
    assert!(blockers.contains(&FundingPreviewBlocker::FundingUncertain));
    let reused = blocked(&f, 1, u128::MAX).await;
    assert!(reused.contains(&FundingPreviewBlocker::AlreadyAdmitted));
    f.harness
        .pic
        .upgrade_canister(
            f.sender,
            std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).unwrap(),
            blob_test_protocol::fixture_wire::encode_one(
                "fixture_post_upgrade",
                FundingUpgradeArgs {
                    trap_after_restore: false,
                },
            )
            .unwrap(),
            None,
        )
        .await
        .unwrap();
    let restored = blocked(&f, 1, 1).await;
    for name in [
        FundingPreviewBlocker::AlreadyAdmitted,
        FundingPreviewBlocker::FundingUncertain,
        FundingPreviewBlocker::RecoveryFenced,
        FundingPreviewBlocker::SpendabilityUnknown,
    ] {
        assert!(restored.contains(&name));
    }
    assert!(!restored.contains(&FundingPreviewBlocker::RecoveryUnknown));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn trapped_callback_keeps_the_original_identity_and_full_transfer_uncertain() {
    let f = Fixture::new().await;
    let failure = transfer(&f, 7, 500_000, true).await.unwrap_err();
    assert_eq!(
        crate::canister_rejection(&failure).reject_code,
        ic_testkit::pocket_ic::RejectCode::CanisterError
    );
    for id in [7, 8] {
        let blockers = blocked(&f, id, 1).await;
        assert!(blockers.contains(&FundingPreviewBlocker::FundingUncertain));
        assert_eq!(
            blockers.contains(&FundingPreviewBlocker::AlreadyAdmitted),
            id == 7
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn exhausted_no_transfer_history_still_blocks_new_identities() {
    let f = Fixture::new().await;
    for id in 0..16 {
        transfer(&f, id, 0, false).await.unwrap().unwrap();
    }
    let blockers = blocked(&f, 17, 1).await;
    assert!(blockers.contains(&FundingPreviewBlocker::JournalFull));
    assert!(!blockers.contains(&FundingPreviewBlocker::FundingUncertain));
    assert!(blockers.contains(&FundingPreviewBlocker::RecoveryUnknown));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn wrong_binding_denied_callers_and_invalid_amounts_never_change_journals() {
    let f = Fixture::new().await;
    let before = f.journals().await;
    let request = FundingPreviewRequest {
        revision: 0,
        service: f.sender,
        peer: f.receiver,
        id: 1,
        requested_cycles: 1,
    };
    for (caller, input, expected) in [
        (
            Principal::anonymous(),
            request,
            FundingPreviewFailure::Denied,
        ),
        (
            f.driver,
            FundingPreviewRequest {
                service: f.receiver,
                ..request
            },
            FundingPreviewFailure::Binding,
        ),
        (
            f.driver,
            FundingPreviewRequest {
                peer: f.authority,
                ..request
            },
            FundingPreviewFailure::Binding,
        ),
        (
            f.driver,
            FundingPreviewRequest {
                requested_cycles: 0,
                ..request
            },
            FundingPreviewFailure::InvalidAmount,
        ),
    ] {
        let result: Result<FundingPreviewView, FundingPreviewFailure> =
            fixture_calls::query_fixture(
                &f.harness.pic,
                f.sender,
                caller,
                "preview_funding",
                (input,),
            )
            .await
            .map(|(reply,)| reply)
            .unwrap();
        assert_eq!(result, Err(expected));
    }
    assert!(
        f.journals().await == before,
        "rejected previews must remain passive"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn update_only_preview_method_cannot_trigger_a_transfer_fallback() {
    let f = Fixture::new().await;
    let pic = &f.harness.pic;
    let canister = pic.create_canister().await.unwrap();
    pic.install_canister(
        canister,
        operator_method_mode::update_wasm("preview_funding", &[]),
        vec![],
        None,
    )
    .await
    .unwrap();
    let result: Result<(Result<FundingPreviewView, FundingPreviewFailure>,), _> =
        fixture_calls::query_fixture(
            pic,
            canister,
            f.driver,
            "preview_funding",
            (request(&f, 1, 1).await,),
        )
        .await;
    assert!(result.is_err());
    assert_eq!(
        pic.get_stable_memory(canister).await.unwrap(),
        Vec::<u8>::new()
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn preview_reports_the_same_amount_bound_enforced_before_update_admission() {
    let f = Fixture::new().await;
    let before = f.journals().await;
    let blockers = blocked(&f, 1, u128::MAX).await;
    let maximum = blockers
        .iter()
        .find_map(|b| match b {
            FundingPreviewBlocker::AmountLimitExceeded { maximum_cycles } => Some(*maximum_cycles),
            _ => None,
        })
        .unwrap();
    assert!(
        !blocked(&f, 1, maximum)
            .await
            .iter()
            .any(|b| matches!(b, FundingPreviewBlocker::AmountLimitExceeded { .. }))
    );
    assert!(
        blocked(&f, 1, maximum + 1)
            .await
            .iter()
            .any(|b| matches!(b, FundingPreviewBlocker::AmountLimitExceeded { .. }))
    );
    let result: Result<FundingObservation, FundingFailure> = fixture_calls::update_fixture(
        &f.harness.pic,
        f.sender,
        f.driver,
        "fund",
        (FundingRequest {
            id: 1,
            offered: maximum + 1,
            accept: 0,
            reply: FundingReplyMode::Success,
            trap_callback: true,
        },),
    )
    .await
    .map(|(reply,)| reply)
    .unwrap();
    assert_eq!(result, Err(FundingFailure::Limit));
    assert!(
        f.journals().await == before,
        "over-limit previews and updates cannot retain partial intents"
    );
}
