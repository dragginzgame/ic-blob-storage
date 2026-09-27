//! End-to-end shared workflow, using actual local IC calls and synthetic host facts.
use super::*;
use blob_test_protocol::storage::funding::{
    admission::Blocker as B,
    transport::{DispatchInput, DispatchResult, EvidenceScenario},
};
fn request(intent: Intent) -> DispatchInput {
    DispatchInput {
        intent,
        operating_reserve: 1_000_000_000,
        other_liabilities: 0,
        evidence: EvidenceScenario::Complete,
        attempt_fault: None,
        callback_fault: None,
    }
}
impl Fixture {
    fn guarded_dispatch(
        &self,
        actor: Principal,
        input: DispatchInput,
    ) -> Result<DispatchResult, Failure> {
        self.harness
            .pic
            .update_candid_as(
                self.service,
                actor,
                "fixture_guarded_funding_dispatch",
                (input,),
            )
            .unwrap()
    }
}
fn settled(value: Result<DispatchResult, Failure>) -> Observation {
    let DispatchResult::Settled(observation) = value.unwrap() else {
        panic!("expected durable settlement")
    };
    observation
}
fn refused(value: Result<DispatchResult, Failure>) -> (Vec<B>, bool) {
    let DispatchResult::Blocked {
        blockers,
        holds_unknown,
    } = value.unwrap()
    else {
        panic!("expected admission refusal")
    };
    (blockers, holds_unknown)
}
#[test]
fn guarded_dispatch_requires_complete_holds_and_authority_before_marking_or_sending() {
    let f = Fixture::with_cashier();
    let intent = f.funding_intent(1, 900);
    f.configure_cashier(intent, 400, FundingReplyMode::Malformed);
    assert_eq!(
        f.guarded_dispatch(f.operator, request(intent)),
        Err(Failure::Unknown)
    );
    f.funding(f.operator, intent, Action::Prepare).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(
            f.guarded_dispatch(actor, request(intent)),
            Err(Failure::Denied)
        );
    }
    assert_eq!(
        f.guarded_dispatch(
            f.operator,
            request(Intent {
                offered: 899,
                ..intent
            })
        ),
        Err(Failure::Conflict)
    );
    assert_eq!(
        f.guarded_dispatch(
            f.operator,
            request(Intent {
                cashier: f.other,
                ..intent
            })
        ),
        Err(Failure::Binding)
    );
    let (unknown, holds_unknown) = refused(f.guarded_dispatch(
        f.operator,
        DispatchInput {
            evidence: EvidenceScenario::Unknown,
            ..request(intent)
        },
    ));
    assert!(holds_unknown);
    for blocker in [
        B::ProviderUnqualified,
        B::RecoveryUnknown,
        B::AccountActivityUnknown,
        B::SpendabilityUnknown,
    ] {
        assert!(unknown.contains(&blocker));
    }
    let (otherwise_ready, holds_unknown) = refused(f.guarded_dispatch(
        f.operator,
        DispatchInput {
            evidence: EvidenceScenario::MissingHolds,
            ..request(intent)
        },
    ));
    assert!(otherwise_ready.is_empty());
    assert!(holds_unknown);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert!(f.incoming().is_empty());
    let result = settled(f.guarded_dispatch(f.operator, request(intent)));
    assert_eq!(
        (result.accepted, result.refunded, result.outcome),
        (400, Some(500), FundingOutcome::InvalidReply)
    );
    let retained = f.retained_outcome(f.operator, intent).unwrap().unwrap();
    assert_eq!(retained.response, Some(FundingOutcome::InvalidReply));
    assert_eq!(
        retained.reconciliation,
        FundingReconciliationView::CreditRequired(400)
    );
    assert!(
        refused(f.guarded_dispatch(f.operator, request(intent)))
            .0
            .contains(&B::AlreadyAttempted)
    );
    let later = f.funding_intent(2, 500);
    f.funding(f.operator, later, Action::Prepare).unwrap();
    assert!(
        refused(f.guarded_dispatch(f.operator, request(later)))
            .0
            .contains(&B::JournalUncredited)
    );
    assert_eq!(f.incoming().len(), 1);
    f.upgrade_storage();
    assert!(
        refused(f.guarded_dispatch(f.operator, request(later)))
            .0
            .contains(&B::JournalFenced)
    );
    assert_eq!(f.retained_outcome(f.operator, intent), Ok(Some(retained)));
}
#[test]
fn guarded_dispatch_settles_unsent_liquidity_refusal_and_allows_a_distinct_intent() {
    let f = Fixture::with_cashier();
    let intent = f.funding_intent(1, 900);
    f.configure_cashier(intent, 900, FundingReplyMode::Success);
    f.funding(f.operator, intent, Action::Prepare).unwrap();
    let result = settled(f.guarded_dispatch(
        f.operator,
        DispatchInput {
            other_liabilities: u128::MAX,
            ..request(intent)
        },
    ));
    assert_eq!(
        (result.accepted, result.refunded, result.outcome),
        (0, None, FundingOutcome::LiquidityBlocked)
    );
    assert!(result.call_cost > 0);
    assert!(f.incoming().is_empty());
    assert_eq!(
        f.funding_lookup(f.operator, intent),
        Ok(Some(Phase::NotEnqueued))
    );
    assert_eq!(f.funding_allocation().not_enqueued, 900);
    assert!(
        refused(f.guarded_dispatch(f.operator, request(intent)))
            .0
            .contains(&B::AlreadyAttempted)
    );
    let second = Intent {
        target_balance: Some(u128::MAX),
        ..f.funding_intent(2, 900)
    };
    f.configure_cashier(second, 0, FundingReplyMode::Success);
    f.funding(f.operator, second, Action::Prepare).unwrap();
    let returned = settled(f.guarded_dispatch(f.operator, request(second)));
    assert_eq!((returned.accepted, returned.refunded), (0, Some(900)));
    assert_eq!(f.funding_allocation().available, 1000);
    let third = f.funding_intent(3, 900);
    f.configure_cashier(third, 900, FundingReplyMode::Success);
    f.funding(f.operator, third, Action::Prepare).unwrap();
    assert_eq!(
        settled(f.guarded_dispatch(f.operator, request(third))).accepted,
        900
    );
    assert_eq!(
        f.incoming().iter().map(|r| r.id).collect::<Vec<_>>(),
        vec![2, 3]
    );
    assert_eq!(f.funding_allocation().accepted, 900);
}
#[test]
fn guarded_dispatch_marker_trap_sends_nothing_and_preserves_prepared_state() {
    let f = Fixture::with_cashier();
    let intent = f.funding_intent(1, 900);
    f.configure_cashier(intent, 400, FundingReplyMode::Success);
    f.funding(f.operator, intent, Action::Prepare).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let error = f
        .harness
        .pic
        .update_call(
            f.service,
            f.operator,
            "fixture_guarded_funding_dispatch",
            candid::encode_one(DispatchInput {
                attempt_fault: Some(WriteFault::FundingIntents),
                ..request(intent)
            })
            .unwrap(),
        )
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert!(f.incoming().is_empty());
    assert_eq!(
        f.funding_lookup(f.operator, intent),
        Ok(Some(Phase::Prepared))
    );
    assert_eq!(
        settled(f.guarded_dispatch(f.operator, request(intent))).accepted,
        400
    );
}
#[test]
fn guarded_dispatch_callback_traps_retain_uncertainty_after_remote_acceptance() {
    for fault in [WriteFault::FundingIntents, WriteFault::FundingAccounting] {
        let f = Fixture::with_cashier();
        let intent = f.funding_intent(1, 900);
        f.configure_cashier(intent, 400, FundingReplyMode::Success);
        f.funding(f.operator, intent, Action::Prepare).unwrap();
        let before = f.funding_allocation();
        let error = f
            .harness
            .pic
            .update_call(
                f.service,
                f.operator,
                "fixture_guarded_funding_dispatch",
                candid::encode_one(DispatchInput {
                    callback_fault: Some(fault),
                    ..request(intent)
                })
                .unwrap(),
            )
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
        assert_eq!(
            f.incoming(),
            vec![FundingReceiptRecord {
                id: 1,
                available: 900,
                accepted: 400
            }]
        );
        assert_eq!(f.funding_allocation(), before);
        let uncertain = f.retained_outcome(f.operator, intent).unwrap().unwrap();
        assert_eq!(uncertain.phase, Phase::Uncertain);
        assert_eq!(uncertain.response, None);
        assert_eq!(
            uncertain.reconciliation,
            FundingReconciliationView::TransferUnknown(900)
        );
        assert!(
            refused(f.guarded_dispatch(f.operator, request(intent)))
                .0
                .contains(&B::AlreadyAttempted)
        );
        f.upgrade_storage();
        assert!(
            refused(f.guarded_dispatch(f.operator, request(intent)))
                .0
                .contains(&B::JournalFenced)
        );
        assert_eq!(f.retained_outcome(f.operator, intent), Ok(Some(uncertain)));
        assert_eq!(f.incoming().len(), 1);
    }
}

#[test]
fn delayed_guarded_dispatch_releases_the_borrow_but_retains_its_exact_reservation() {
    let f = Fixture::with_cashier();
    let intent = Intent {
        target_balance: Some(u128::MAX),
        ..f.funding_intent(1, 900)
    };
    f.configure_cashier(intent, 400, FundingReplyMode::DelayedSuccess);
    f.funding(f.operator, intent, Action::Prepare).unwrap();
    let reserved = f.funding_allocation();
    let call = f
        .harness
        .pic
        .submit_call(
            f.service,
            f.operator,
            "fixture_guarded_funding_dispatch",
            candid::encode_one(request(intent)).unwrap(),
        )
        .unwrap();
    let mut pending = None;
    for _ in 0..20 {
        f.harness.pic.tick();
        let view = f.retained_outcome(f.operator, intent).unwrap().unwrap();
        if view.phase == Phase::Uncertain {
            pending = Some(view);
            break;
        }
    }
    let pending = pending.expect("observe the committed attempt before its delayed callback");
    assert_eq!(pending.intent, intent);
    assert_eq!(pending.response, None);
    assert_eq!(
        pending.reconciliation,
        FundingReconciliationView::TransferUnknown(900)
    );
    assert_eq!(f.funding_allocation(), reserved);
    assert!(
        refused(f.guarded_dispatch(f.operator, request(intent)))
            .0
            .contains(&B::AlreadyAttempted)
    );
    let reply: Result<DispatchResult, Failure> =
        candid::decode_one(&f.harness.pic.await_call(call).unwrap()).unwrap();
    assert_eq!(settled(reply).accepted, 400);
    let retained = f.retained_outcome(f.operator, intent).unwrap().unwrap();
    assert_eq!(retained.intent, intent);
    assert_eq!(retained.phase, Phase::Callback(500));
    assert_eq!(retained.response, Some(FundingOutcome::ReportedSuccess));
    assert_eq!(
        f.incoming(),
        vec![FundingReceiptRecord {
            id: 1,
            available: 900,
            accepted: 400
        }]
    );
}
