use super::*;
use crate::{
    policy::billing::{
        RecoveryState,
        admission::{
            FundingActivity,
            attempt::{FundingAttemptBlocker as B, FundingAttemptEvidence},
            evidence::FundingEvidenceBlocker,
        },
    },
    workflow::funding::attempt::{FundingAttemptResult, inspect_attempt, mark_first_attempt},
};

fn evidence(intent: FundingIntent) -> FundingAttemptEvidence {
    FundingAttemptEvidence {
        intent,
        provider_qualified: true,
        available_for_offer: Some(1000),
        recovery: Some(RecoveryState::Reconciled),
        other_activity: Some(FundingActivity::Clear),
    }
}
fn blocked(result: Result<FundingAttemptResult, FundingJournalError>) -> Vec<B> {
    let FundingAttemptResult::Blocked(assessment) = result.unwrap() else {
        panic!("expected refusal")
    };
    assessment.blockers().to_vec()
}
#[test]
fn full_history_can_consume_its_exact_last_reservation_once_without_recharging() {
    let m = memories();
    let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
    for id in 1..4 {
        let intent = input(id, 900);
        store.prepare(execution(), intent).unwrap();
        store.mark_attempted(execution(), intent).unwrap();
        store
            .record_transport(
                execution(),
                intent,
                source(),
                FundingTransportOutcome::NotEnqueued,
            )
            .unwrap();
    }
    let intent = FundingIntent {
        target_balance: Some(n(u128::MAX)),
        ..input(4, 900)
    };
    store.prepare(execution(), intent).unwrap();
    let before = store.allocation(execution()).unwrap();
    assert_eq!(before.transferable(), 0);
    assert!(
        inspect_attempt(&store, execution(), intent, evidence(intent))
            .unwrap()
            .can_mark_attempt()
    );
    let FundingAttemptResult::Marked(request) =
        mark_first_attempt(&mut store, execution(), intent, evidence(intent)).unwrap()
    else {
        panic!("exact prepared reservation fits")
    };
    assert_eq!(request, store.request(execution(), intent).unwrap());
    assert_eq!(store.allocation(execution()), Ok(before));
    assert_eq!(
        store.lookup(execution(), intent).unwrap().unwrap().state,
        FundingIntentState::Uncertain
    );
    // A prior clear preview cannot permit a second attempt after this state change.
    assert!(
        blocked(mark_first_attempt(
            &mut store,
            execution(),
            intent,
            evidence(intent)
        ))
        .contains(&B::AlreadyAttempted)
    );
    drop(store);
    let mut restored = StableFundingJournal::open(copy(&m), config(), allocation()).unwrap();
    let refusal = blocked(mark_first_attempt(
        &mut restored,
        execution(),
        intent,
        evidence(intent),
    ));
    assert!(refusal.contains(&B::JournalFenced));
    assert!(refusal.contains(&B::AlreadyAttempted));
}
#[test]
fn attempt_exclusion_binds_every_field_and_refusals_preserve_the_reservation() {
    let m = memories();
    let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
    let intent = input(1, 900);
    assert!(
        blocked(mark_first_attempt(
            &mut store,
            execution(),
            intent,
            evidence(intent)
        ))
        .contains(&B::IntentMissing)
    );
    store.prepare(execution(), intent).unwrap();
    let before = (m.intents.borrow().clone(), m.accounting.borrow().clone());
    let p = Principal::from_slice(&[99]);
    for changed in [
        FundingIntent {
            service: p,
            ..intent
        },
        FundingIntent {
            cashier: p,
            ..intent
        },
        FundingIntent {
            account: p,
            ..intent
        },
        FundingIntent {
            namespace: n(2),
            ..intent
        },
        FundingIntent {
            operation: n(2),
            ..intent
        },
        FundingIntent {
            offered: n(899),
            ..intent
        },
        FundingIntent {
            target_balance: Some(n(1)),
            ..intent
        },
    ] {
        assert_eq!(
            mark_first_attempt(&mut store, execution(), intent, evidence(changed)),
            Err(FundingJournalError::Binding)
        );
    }
    assert_eq!(
        mark_first_attempt(
            &mut store,
            UploadContext {
                actor: p,
                ..execution()
            },
            intent,
            evidence(intent)
        ),
        Err(FundingJournalError::NotOperator)
    );
    let unknown = FundingAttemptEvidence {
        provider_qualified: false,
        available_for_offer: None,
        recovery: None,
        other_activity: None,
        ..evidence(intent)
    };
    let refusal = blocked(mark_first_attempt(&mut store, execution(), intent, unknown));
    for expected in [
        B::ProviderUnqualified,
        B::Evidence(FundingEvidenceBlocker::SpendabilityUnknown),
        B::Evidence(FundingEvidenceBlocker::RecoveryUnknown),
        B::Evidence(FundingEvidenceBlocker::FundingUnknown),
    ] {
        assert!(refusal.contains(&expected));
    }
    assert_eq!(
        (m.intents.borrow().clone(), m.accounting.borrow().clone()),
        before
    );
    drop(store);
    let mut restored = StableFundingJournal::open(copy(&m), config(), allocation()).unwrap();
    assert!(
        blocked(mark_first_attempt(
            &mut restored,
            execution(),
            intent,
            evidence(intent)
        ))
        .contains(&B::JournalFenced)
    );
    assert_eq!(
        restored.lookup(execution(), intent).unwrap().unwrap().state,
        FundingIntentState::Prepared
    );
}
#[test]
fn exact_pending_exclusion_cannot_hide_older_accepted_cycles() {
    let mut store = StableFundingJournal::install(memories(), config(), allocation()).unwrap();
    let prior = input(1, 100);
    store.prepare(execution(), prior).unwrap();
    store.mark_attempted(execution(), prior).unwrap();
    store
        .record_transport(
            execution(),
            prior,
            source(),
            FundingTransportOutcome::Callback { refunded: 0 },
        )
        .unwrap();
    // Low-level bookkeeping permits allocation use; workflow must still reject
    // older acceptance even when a trusted host claims all OTHER activity clear.
    let intent = input(2, 800);
    store.prepare(execution(), intent).unwrap();
    let before = store.allocation(execution()).unwrap();
    assert_eq!(
        blocked(mark_first_attempt(
            &mut store,
            execution(),
            intent,
            evidence(intent)
        )),
        vec![B::JournalUncredited]
    );
    assert_eq!(
        store.lookup(execution(), intent).unwrap().unwrap().state,
        FundingIntentState::Prepared
    );
    assert_eq!(store.allocation(execution()), Ok(before));
}
