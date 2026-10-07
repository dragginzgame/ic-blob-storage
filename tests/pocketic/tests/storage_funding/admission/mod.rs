use super::*;
use blob_test_protocol::storage::funding::admission::{Attempt, Blocker as B, Preparation, View};

impl Fixture {
    fn inspect_attempt(&self, actor: Principal, intent: Intent) -> Result<View, Failure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "funding_attempt", (intent,))
            .unwrap()
    }
    fn guarded_attempt(&self, actor: Principal, intent: Intent) -> Result<Attempt, Failure> {
        self.harness
            .pic
            .update_candid_as(self.service, actor, "mark_funding_attempt", (intent,))
            .unwrap()
    }
    fn inspect_preparation(&self, actor: Principal, intent: Intent) -> Result<View, Failure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "funding_preparation", (intent,))
            .unwrap()
    }
    fn guarded_prepare(&self, actor: Principal, intent: Intent) -> Result<Preparation, Failure> {
        self.harness
            .pic
            .update_candid_as(self.service, actor, "prepare_funding", (intent,))
            .unwrap()
    }
}
#[test]
fn guarded_first_attempt_preserves_unknowns_and_exact_reservation_through_restore() {
    let f = Fixture::new();
    for id in 1..4 {
        let intent = f.funding_intent(id, 900);
        f.funding(f.operator, intent, Action::Prepare).unwrap();
        f.funding(f.operator, intent, Action::Attempt).unwrap();
        f.funding(f.operator, intent, Action::Callback(900))
            .unwrap();
    }
    let intent = f.funding_intent(4, 900);
    let missing = f.inspect_attempt(f.operator, intent).unwrap();
    assert_unknowns(&missing.blockers);
    assert!(missing.blockers.contains(&B::IntentMissing));
    f.funding(f.operator, intent, Action::Prepare).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let preview = f.inspect_attempt(f.operator, intent).unwrap();
    assert_eq!(preview.intent, intent);
    assert_unknowns(&preview.blockers);
    // The existing final reservation consumes no new slot or attachment allowance.
    for unexpected in [
        B::IntentMissing,
        B::JournalFull,
        B::ReservationMismatch,
        B::JournalUncredited,
        B::AllocationReserve(0),
    ] {
        assert!(!preview.blockers.contains(&unexpected));
    }
    assert_eq!(
        f.guarded_attempt(f.operator, intent),
        Ok(Attempt::Blocked(preview.blockers))
    );
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(f.inspect_attempt(actor, intent), Err(Failure::Denied));
        assert_eq!(f.guarded_attempt(actor, intent), Err(Failure::Denied));
    }
    for changed in [
        Intent {
            service: f.other,
            ..intent
        },
        Intent {
            cashier: f.other,
            ..intent
        },
        Intent {
            account: f.other,
            ..intent
        },
        Intent {
            namespace: 2,
            ..intent
        },
    ] {
        assert_eq!(
            f.guarded_attempt(f.operator, changed),
            Err(Failure::Binding)
        );
    }
    for changed in [
        Intent {
            offered: 899,
            ..intent
        },
        Intent {
            target_balance: Some(1),
            ..intent
        },
    ] {
        assert_eq!(
            f.guarded_attempt(f.operator, changed),
            Err(Failure::Conflict)
        );
    }
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    let restored = f.inspect_attempt(f.operator, intent).unwrap();
    assert_unknowns(&restored.blockers);
    assert!(restored.blockers.contains(&B::JournalFenced));
    assert_eq!(
        f.guarded_attempt(f.operator, intent),
        Ok(Attempt::Blocked(restored.blockers))
    );
    assert_eq!(
        f.funding_lookup(f.operator, intent),
        Ok(Some(Phase::Prepared))
    );
}
#[test]
fn guarded_attempt_does_not_retry_uncertain_or_terminal_intents_or_hide_older_acceptance() {
    let f = Fixture::new();
    let prior = f.funding_intent(1, 100);
    f.funding(f.operator, prior, Action::Prepare).unwrap();
    f.funding(f.operator, prior, Action::Attempt).unwrap();
    let uncertain = f.inspect_attempt(f.operator, prior).unwrap();
    assert_unknowns(&uncertain.blockers);
    assert!(uncertain.blockers.contains(&B::AlreadyAttempted));
    assert_eq!(
        f.guarded_attempt(f.operator, prior),
        Ok(Attempt::Blocked(uncertain.blockers))
    );
    f.funding(f.operator, prior, Action::Callback(0)).unwrap();
    let terminal = f.inspect_attempt(f.operator, prior).unwrap();
    assert!(terminal.blockers.contains(&B::AlreadyAttempted));
    assert!(terminal.blockers.contains(&B::JournalUncredited));
    assert_eq!(
        f.guarded_attempt(f.operator, prior),
        Ok(Attempt::Blocked(terminal.blockers))
    );
    let current = f.funding_intent(2, 800);
    f.funding(f.operator, current, Action::Prepare).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let held = f.inspect_attempt(f.operator, current).unwrap();
    assert_unknowns(&held.blockers);
    assert!(held.blockers.contains(&B::JournalUncredited));
    assert!(!held.blockers.contains(&B::AlreadyAttempted));
    assert_eq!(
        f.guarded_attempt(f.operator, current),
        Ok(Attempt::Blocked(held.blockers))
    );
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
}
fn assert_unknowns(blockers: &[B]) {
    for blocker in [
        B::ProviderUnqualified,
        B::RecoveryUnknown,
        B::AccountActivityUnknown,
        B::SpendabilityUnknown,
    ] {
        assert!(blockers.contains(&blocker));
    }
}
#[test]
fn guarded_funding_keeps_unknown_host_evidence_and_rejected_calls_leave_no_reservation() {
    let f = Fixture::new();
    let intent = f.funding_intent(9, 900);
    let before = f.harness.pic.get_stable_memory(f.service);
    let preview = f.inspect_preparation(f.operator, intent).unwrap();
    assert_eq!(preview.intent, intent);
    assert_unknowns(&preview.blockers);
    assert!(!preview.blockers.contains(&B::JournalUncredited));
    assert_eq!(
        f.guarded_prepare(f.operator, intent),
        Ok(Preparation::Blocked(preview.blockers))
    );
    assert_eq!(f.funding_lookup(f.operator, intent), Ok(None));
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(f.inspect_preparation(actor, intent), Err(Failure::Denied));
        assert_eq!(f.guarded_prepare(actor, intent), Err(Failure::Denied));
    }
    for changed in [
        Intent {
            service: f.other,
            ..intent
        },
        Intent {
            cashier: f.other,
            ..intent
        },
        Intent {
            account: f.other,
            ..intent
        },
        Intent {
            namespace: 2,
            ..intent
        },
    ] {
        assert_eq!(
            f.inspect_preparation(f.operator, changed),
            Err(Failure::Binding)
        );
        assert_eq!(
            f.guarded_prepare(f.operator, changed),
            Err(Failure::Binding)
        );
    }
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    // A previously returned preview has no role in the update's next assessment.
    f.funding(f.operator, intent, Action::Prepare).unwrap();
    let Preparation::Blocked(blocked) = f.guarded_prepare(f.operator, intent).unwrap() else {
        panic!("retained identity cannot prepare again")
    };
    assert_unknowns(&blocked);
    assert!(blocked.contains(&B::JournalUncredited));
    assert!(blocked.contains(&B::IdentityRetained));
    assert_eq!(
        f.inspect_preparation(
            f.operator,
            Intent {
                offered: 899,
                ..intent
            }
        ),
        Err(Failure::Conflict)
    );
}
#[test]
fn funding_preparation_reports_history_limits_and_restoration_without_clearing_unknowns() {
    let f = Fixture::new();
    for id in [2, 4, 6, 8] {
        let intent = f.funding_intent(id, 900);
        f.funding(f.operator, intent, Action::Prepare).unwrap();
        f.funding(f.operator, intent, Action::Attempt).unwrap();
        f.funding(f.operator, intent, Action::Callback(900))
            .unwrap();
    }
    let stale = f
        .inspect_preparation(f.operator, f.funding_intent(7, 1))
        .unwrap();
    assert!(stale.blockers.contains(&B::IdentityStale));
    let new = f.funding_intent(9, 901);
    let before = f.inspect_preparation(f.operator, new).unwrap();
    assert_unknowns(&before.blockers);
    assert!(before.blockers.contains(&B::JournalFull));
    assert!(before.blockers.contains(&B::AllocationReserve(900)));
    assert!(!before.blockers.contains(&B::JournalUncredited));
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    let restored = f.inspect_preparation(f.operator, new).unwrap();
    assert_unknowns(&restored.blockers);
    assert!(restored.blockers.contains(&B::JournalFenced));
    assert!(restored.blockers.contains(&B::JournalFull));
    assert_eq!(
        f.guarded_prepare(f.operator, new),
        Ok(Preparation::Blocked(restored.blockers))
    );
    assert_eq!(f.funding_lookup(f.operator, new), Ok(None));
}
