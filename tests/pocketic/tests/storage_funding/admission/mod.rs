use super::*;
use blob_test_protocol::storage::funding::admission::{Attempt, Blocker as B, Preparation, View};
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;

impl Fixture {
    async fn inspect_attempt(&self, actor: Principal, intent: Intent) -> Result<View, Failure> {
        self.harness
            .pic
            .query_control_as(
                self.service,
                actor,
                "funding_attempt",
                (intent,),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
    async fn guarded_attempt(&self, actor: Principal, intent: Intent) -> Result<Attempt, Failure> {
        self.harness
            .pic
            .update_control_as(
                self.service,
                actor,
                "mark_funding_attempt",
                (intent,),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
    async fn inspect_preparation(&self, actor: Principal, intent: Intent) -> Result<View, Failure> {
        self.harness
            .pic
            .query_control_as(
                self.service,
                actor,
                "funding_preparation",
                (intent,),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
    async fn guarded_prepare(
        &self,
        actor: Principal,
        intent: Intent,
    ) -> Result<Preparation, Failure> {
        self.harness
            .pic
            .update_control_as(
                self.service,
                actor,
                "prepare_funding",
                (intent,),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "Keep the original first funding attempt and reservation across unknown outcomes and restoration"
)]
async fn guarded_first_attempt_preserves_unknowns_and_exact_reservation_through_restore() {
    let f = Fixture::new().await;
    for id in 1..4 {
        let intent = f.funding_intent(id, 900);
        f.funding(f.operator, intent, Action::Prepare)
            .await
            .unwrap();
        f.funding(f.operator, intent, Action::Attempt)
            .await
            .unwrap();
        f.funding(f.operator, intent, Action::Callback(900))
            .await
            .unwrap();
    }
    let intent = f.funding_intent(4, 900);
    let missing = f.inspect_attempt(f.operator, intent).await.unwrap();
    assert_unknowns(&missing.blockers);
    assert!(missing.blockers.contains(&B::IntentMissing));
    f.funding(f.operator, intent, Action::Prepare)
        .await
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let preview = f.inspect_attempt(f.operator, intent).await.unwrap();
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
        f.guarded_attempt(f.operator, intent).await,
        Ok(Attempt::Blocked(preview.blockers))
    );
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(f.inspect_attempt(actor, intent).await, Err(Failure::Denied));
        assert_eq!(f.guarded_attempt(actor, intent).await, Err(Failure::Denied));
    }
    for changed in [
        Intent {
            service: f.other.into(),
            ..intent
        },
        Intent {
            cashier: f.other.into(),
            ..intent
        },
        Intent {
            account: f.other.into(),
            ..intent
        },
        Intent {
            namespace: 2,
            ..intent
        },
    ] {
        assert_eq!(
            f.guarded_attempt(f.operator, changed).await,
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
            f.guarded_attempt(f.operator, changed).await,
            Err(Failure::Conflict)
        );
    }
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
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
    let restored = f.inspect_attempt(f.operator, intent).await.unwrap();
    assert_unknowns(&restored.blockers);
    assert!(restored.blockers.contains(&B::JournalFenced));
    assert_eq!(
        f.guarded_attempt(f.operator, intent).await,
        Ok(Attempt::Blocked(restored.blockers))
    );
    assert_eq!(
        f.funding_lookup(f.operator, intent).await,
        Ok(Some(Phase::Prepared))
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn guarded_attempt_does_not_retry_uncertain_or_terminal_intents_or_hide_older_acceptance() {
    let f = Fixture::new().await;
    let prior = f.funding_intent(1, 100);
    f.funding(f.operator, prior, Action::Prepare).await.unwrap();
    f.funding(f.operator, prior, Action::Attempt).await.unwrap();
    let uncertain = f.inspect_attempt(f.operator, prior).await.unwrap();
    assert_unknowns(&uncertain.blockers);
    assert!(uncertain.blockers.contains(&B::AlreadyAttempted));
    assert_eq!(
        f.guarded_attempt(f.operator, prior).await,
        Ok(Attempt::Blocked(uncertain.blockers))
    );
    f.funding(f.operator, prior, Action::Callback(0))
        .await
        .unwrap();
    let terminal = f.inspect_attempt(f.operator, prior).await.unwrap();
    assert!(terminal.blockers.contains(&B::AlreadyAttempted));
    assert!(terminal.blockers.contains(&B::JournalUncredited));
    assert_eq!(
        f.guarded_attempt(f.operator, prior).await,
        Ok(Attempt::Blocked(terminal.blockers))
    );
    let current = f.funding_intent(2, 800);
    f.funding(f.operator, current, Action::Prepare)
        .await
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let held = f.inspect_attempt(f.operator, current).await.unwrap();
    assert_unknowns(&held.blockers);
    assert!(held.blockers.contains(&B::JournalUncredited));
    assert!(!held.blockers.contains(&B::AlreadyAttempted));
    assert_eq!(
        f.guarded_attempt(f.operator, current).await,
        Ok(Attempt::Blocked(held.blockers))
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
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
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn guarded_funding_keeps_unknown_host_evidence_and_rejected_calls_leave_no_reservation() {
    let f = Fixture::new().await;
    let intent = f.funding_intent(9, 900);
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let preview = f.inspect_preparation(f.operator, intent).await.unwrap();
    assert_eq!(preview.intent, intent);
    assert_unknowns(&preview.blockers);
    assert!(!preview.blockers.contains(&B::JournalUncredited));
    assert_eq!(
        f.guarded_prepare(f.operator, intent).await,
        Ok(Preparation::Blocked(preview.blockers))
    );
    assert_eq!(f.funding_lookup(f.operator, intent).await, Ok(None));
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(
            f.inspect_preparation(actor, intent).await,
            Err(Failure::Denied)
        );
        assert_eq!(f.guarded_prepare(actor, intent).await, Err(Failure::Denied));
    }
    for changed in [
        Intent {
            service: f.other.into(),
            ..intent
        },
        Intent {
            cashier: f.other.into(),
            ..intent
        },
        Intent {
            account: f.other.into(),
            ..intent
        },
        Intent {
            namespace: 2,
            ..intent
        },
    ] {
        assert_eq!(
            f.inspect_preparation(f.operator, changed).await,
            Err(Failure::Binding)
        );
        assert_eq!(
            f.guarded_prepare(f.operator, changed).await,
            Err(Failure::Binding)
        );
    }
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    // A previously returned preview has no role in the update's next assessment.
    f.funding(f.operator, intent, Action::Prepare)
        .await
        .unwrap();
    let Preparation::Blocked(blocked) = f.guarded_prepare(f.operator, intent).await.unwrap() else {
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
        )
        .await,
        Err(Failure::Conflict)
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn funding_preparation_reports_history_limits_and_restoration_without_clearing_unknowns() {
    let f = Fixture::new().await;
    for id in [2, 4, 6, 8] {
        let intent = f.funding_intent(id, 900);
        f.funding(f.operator, intent, Action::Prepare)
            .await
            .unwrap();
        f.funding(f.operator, intent, Action::Attempt)
            .await
            .unwrap();
        f.funding(f.operator, intent, Action::Callback(900))
            .await
            .unwrap();
    }
    let stale = f
        .inspect_preparation(f.operator, f.funding_intent(7, 1))
        .await
        .unwrap();
    assert!(stale.blockers.contains(&B::IdentityStale));
    let new = f.funding_intent(9, 901);
    let before = f.inspect_preparation(f.operator, new).await.unwrap();
    assert_unknowns(&before.blockers);
    assert!(before.blockers.contains(&B::JournalFull));
    assert!(before.blockers.contains(&B::AllocationReserve(900)));
    assert!(!before.blockers.contains(&B::JournalUncredited));
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
    let restored = f.inspect_preparation(f.operator, new).await.unwrap();
    assert_unknowns(&restored.blockers);
    assert!(restored.blockers.contains(&B::JournalFenced));
    assert!(restored.blockers.contains(&B::JournalFull));
    assert_eq!(
        f.guarded_prepare(f.operator, new).await,
        Ok(Preparation::Blocked(restored.blockers))
    );
    assert_eq!(f.funding_lookup(f.operator, new).await, Ok(None));
}
