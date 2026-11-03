//! Passive funding admission against actual transfer journals, not a payment client.
use super::*;
use blob_test_protocol::funding::preview::{
    FundingPreviewBlocker, FundingPreviewFailure, FundingPreviewRequest, FundingPreviewView,
};
use ic_blob_storage_pocketic_tests::fixture_calls;

fn request(f: &Fixture, id: u64, amount: u128) -> FundingPreviewRequest {
    FundingPreviewRequest {
        service: f.sender,
        peer: f.receiver,
        id,
        requested_cycles: amount,
        revision: f.funding_status().budget.revision,
    }
}
fn preview(f: &Fixture, input: FundingPreviewRequest) -> FundingPreviewView {
    let before = f.journals();
    let (result,): (Result<FundingPreviewView, FundingPreviewFailure>,) =
        fixture_calls::query_fixture(
            &f.harness.pic,
            f.sender,
            f.driver,
            "preview_funding",
            (input,),
        )
        .unwrap();
    assert_eq!(
        f.journals(),
        before,
        "preview cannot consume identities or receipts"
    );
    let result = result.unwrap();
    assert_eq!(result.request, input);
    assert_eq!(result.available_cycles, None);
    result
}
fn blocked(f: &Fixture, id: u64, amount: u128) -> Vec<FundingPreviewBlocker> {
    let view = preview(f, request(f, id, amount));
    view.blockers
}
fn transfer(
    f: &Fixture,
    id: u64,
    accept: u128,
    trap: bool,
) -> Result<Result<FundingObservation, FundingFailure>, ic_platform_pocket_ic::ClientError> {
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
        .map(|bytes| {
            blob_test_protocol::fixture_wire::decode_one(
                "fund",
                &bytes,
                blob_test_protocol::fixture_wire::MAX_BYTES,
            )
            .unwrap()
        })
}

#[test]
fn empty_journal_and_gross_cycles_cannot_supply_missing_funding_evidence() {
    let f = Fixture::new();
    for _ in 0..2 {
        let blockers = blocked(&f, u64::MAX, u128::MAX);
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
        .unwrap();
    let blockers = blocked(&f, u64::MAX, 1);
    assert!(blockers.contains(&FundingPreviewBlocker::SpendabilityUnknown));
    assert!(
        !blockers
            .iter()
            .any(|b| matches!(b, FundingPreviewBlocker::ReserveWouldBeViolated { .. }))
    );
}

#[test]
fn budget_revision_changes_on_admission_and_completion_even_for_full_refunds() {
    let f = Fixture::new();
    let before = request(&f, 3, 1);
    let initial = preview(&f, before).budget;
    transfer(&f, 1, 0, false).unwrap().unwrap();
    let after = preview(&f, before);
    assert!(
        after
            .blockers
            .contains(&FundingPreviewBlocker::BudgetRevisionStale)
    );
    assert_eq!(after.budget.revision, 2);
    assert_eq!(after.budget.available, initial.available);
    assert_eq!(after.budget.refunded, 1_000_000);
    let current = preview(&f, request(&f, 3, 1));
    assert!(
        !current
            .blockers
            .contains(&FundingPreviewBlocker::BudgetRevisionStale)
    );
    assert!(current.available_cycles.is_none());
}

#[test]
fn credit_uncertainty_and_identity_reuse_survive_later_refunds_and_restore() {
    let f = Fixture::new();
    transfer(&f, 1, 400_000, false).unwrap().unwrap();
    transfer(&f, 2, 0, false).unwrap().unwrap();
    let blockers = blocked(&f, 3, 1);
    assert!(blockers.contains(&FundingPreviewBlocker::FundingUncertain));
    let reused = blocked(&f, 1, u128::MAX);
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
        .unwrap();
    let restored = blocked(&f, 1, 1);
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

#[test]
fn trapped_callback_keeps_the_original_identity_and_full_transfer_uncertain() {
    let f = Fixture::new();
    let failure = transfer(&f, 7, 500_000, true).unwrap_err();
    assert_eq!(
        crate::canister_rejection(&failure).reject_code,
        ic_platform_pocket_ic::RejectCode::CanisterError
    );
    for id in [7, 8] {
        let blockers = blocked(&f, id, 1);
        assert!(blockers.contains(&FundingPreviewBlocker::FundingUncertain));
        assert_eq!(
            blockers.contains(&FundingPreviewBlocker::AlreadyAdmitted),
            id == 7
        );
    }
}

#[test]
fn exhausted_no_transfer_history_still_blocks_new_identities() {
    let f = Fixture::new();
    for id in 0..16 {
        transfer(&f, id, 0, false).unwrap().unwrap();
    }
    let blockers = blocked(&f, 17, 1);
    assert!(blockers.contains(&FundingPreviewBlocker::JournalFull));
    assert!(!blockers.contains(&FundingPreviewBlocker::FundingUncertain));
    assert!(blockers.contains(&FundingPreviewBlocker::RecoveryUnknown));
}

#[test]
fn wrong_binding_denied_callers_and_invalid_amounts_never_change_journals() {
    let f = Fixture::new();
    let before = f.journals();
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
            .map(|(reply,)| reply)
            .unwrap();
        assert_eq!(result, Err(expected));
    }
    assert!(
        f.journals() == before,
        "rejected previews must remain passive"
    );
}

#[test]
fn update_only_preview_method_cannot_trigger_a_transfer_fallback() {
    let f = Fixture::new();
    let pic = &f.harness.pic;
    let canister = pic.create_canister().unwrap();
    pic.install_canister(
        canister,
        operator_method_mode::update_wasm("preview_funding", &[]),
        vec![],
        None,
    )
    .unwrap();
    let result: Result<(Result<FundingPreviewView, FundingPreviewFailure>,), _> =
        fixture_calls::query_fixture(
            pic,
            canister,
            f.driver,
            "preview_funding",
            (request(&f, 1, 1),),
        );
    assert!(result.is_err());
    assert_eq!(pic.get_stable_memory(canister).unwrap(), Vec::<u8>::new());
}

#[test]
fn preview_reports_the_same_amount_bound_enforced_before_update_admission() {
    let f = Fixture::new();
    let before = f.journals();
    let blockers = blocked(&f, 1, u128::MAX);
    let maximum = blockers
        .iter()
        .find_map(|b| match b {
            FundingPreviewBlocker::AmountLimitExceeded { maximum_cycles } => Some(*maximum_cycles),
            _ => None,
        })
        .unwrap();
    assert!(
        !blocked(&f, 1, maximum)
            .iter()
            .any(|b| matches!(b, FundingPreviewBlocker::AmountLimitExceeded { .. }))
    );
    assert!(
        blocked(&f, 1, maximum + 1)
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
    .map(|(reply,)| reply)
    .unwrap();
    assert_eq!(result, Err(FundingFailure::Limit));
    assert!(
        f.journals() == before,
        "over-limit previews and updates cannot retain partial intents"
    );
}
