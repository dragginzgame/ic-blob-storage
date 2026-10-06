//! Synthetic independent receipts exercise the journal, not provider qualification.
use super::*;
use crate::{
    model::billing::journal::credit::{FundingCreditConfirmation, FundingCreditError},
    policy::billing::{
        RecoveryState,
        admission::{
            FundingActivity, evidence::FundingAdmissionEvidence, journal::FundingHostEvidence,
        },
    },
    workflow::funding::{
        FundingPreparationResult,
        credit::{FundingCreditResult, confirm},
        prepare_new,
    },
};

pub(super) fn receipt(
    intent: FundingIntent,
    accepted: u128,
    digest: u8,
) -> FundingCreditConfirmation {
    FundingCreditConfirmation {
        intent,
        accepted_cycles: n(accepted),
        receipt_digest: [digest; 32],
    }
}
pub(super) fn paid(
    store: &mut StableFundingJournal<VectorMemory>,
    intent: FundingIntent,
    refunded: u128,
) {
    store.prepare(execution(), intent).unwrap();
    store.mark_attempted(execution(), intent).unwrap();
    store
        .record_transport(
            execution(),
            intent,
            source(),
            FundingTransportOutcome::Callback { refunded },
        )
        .unwrap();
}
fn host(intent: FundingIntent) -> FundingHostEvidence {
    FundingHostEvidence {
        scope: crate::model::billing::journal::FundingJournalScope {
            service: intent.service,
            cashier: intent.cashier,
            account: intent.account,
            namespace: intent.namespace,
        },
        provider_qualified: true,
        admission: FundingAdmissionEvidence {
            available_cycles: Some(1000),
            recovery: Some(RecoveryState::Reconciled),
            activity: Some(FundingActivity::Clear),
        },
    }
}
#[test]
fn exact_credit_enables_next_guarded_topup_without_refunding_spent_allocation() {
    let mut store = StableFundingJournal::install(memories(), config(), allocation()).unwrap();
    let first = input(1, 200);
    paid(&mut store, first, 50);
    let second = input(2, 200);
    assert!(matches!(
        prepare_new(&mut store, execution(), second, host(second)).unwrap(),
        FundingPreparationResult::Blocked(_)
    ));
    let before = store.allocation(execution()).unwrap();
    assert_eq!(
        confirm(&mut store, execution(), first, |_| None),
        Ok(FundingCreditResult::NotEstablished)
    );
    assert_eq!(store.allocation(execution()).unwrap(), before);
    assert_eq!(
        confirm(&mut store, execution(), first, |_| Some(receipt(
            first, 150, 1
        ))),
        Ok(FundingCreditResult::Confirmed)
    );
    assert_eq!(
        confirm(&mut store, execution(), first, |_| Some(receipt(
            first, 150, 1
        ))),
        Ok(FundingCreditResult::AlreadyConfirmed)
    );
    let credited = store.allocation(execution()).unwrap();
    assert_eq!(
        (
            credited.available(),
            credited.accepted(),
            credited.refunded()
        ),
        (850, 150, 50)
    );
    assert_eq!(
        (credited.credit_confirmed(), credited.uncredited()),
        (150, 0)
    );
    assert!(matches!(
        prepare_new(&mut store, execution(), second, host(second)).unwrap(),
        FundingPreparationResult::Prepared(_)
    ));
    let evidence = crate::policy::billing::admission::attempt::FundingAttemptEvidence {
        intent: second,
        provider_qualified: true,
        available_for_offer: Some(1000),
        recovery: Some(RecoveryState::Reconciled),
        other_activity: Some(FundingActivity::Clear),
    };
    assert!(matches!(
        crate::workflow::funding::attempt::mark_first_attempt(
            &mut store,
            execution(),
            second,
            evidence
        )
        .unwrap(),
        crate::workflow::funding::attempt::FundingAttemptResult::Marked(_)
    ));
    store
        .record_transport(
            execution(),
            second,
            source(),
            FundingTransportOutcome::Callback { refunded: 0 },
        )
        .unwrap();
    assert_eq!(store.allocation(execution()).unwrap().uncredited(), 200);
    store
        .record_credit(execution(), receipt(second, 200, 2))
        .unwrap();
    let totals = store.allocation(execution()).unwrap();
    assert_eq!(
        (
            totals.available(),
            totals.accepted(),
            totals.credit_confirmed(),
            totals.uncredited()
        ),
        (650, 350, 350, 0)
    );
    assert_eq!(
        store.lookup(execution(), first).unwrap().unwrap().state,
        FundingIntentState::Callback { refunded: 50 }
    );
}

#[test]
fn receipt_amount_conflict_and_reuse_refuse_without_mutating_any_record() {
    let m = memories();
    let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
    let first = input(1, 100);
    paid(&mut store, first, 0);
    for (proof, error) in [
        (receipt(first, 99, 1), FundingCreditError::AmountMismatch),
        (receipt(first, 100, 0), FundingCreditError::EmptyReceipt),
    ] {
        let before = (m.accounting.borrow().clone(), m.intents.borrow().clone());
        assert_eq!(store.record_credit(execution(), proof), Err(error.into()));
        assert_eq!(
            (m.accounting.borrow().clone(), m.intents.borrow().clone()),
            before
        );
    }
    store
        .record_credit(execution(), receipt(first, 100, 1))
        .unwrap();
    let second = input(2, 100);
    paid(&mut store, second, 0);
    for (proof, error) in [
        (receipt(first, 100, 2), FundingCreditError::Conflict),
        (receipt(second, 100, 1), FundingCreditError::ReceiptReused),
    ] {
        let before = (m.accounting.borrow().clone(), m.intents.borrow().clone());
        assert_eq!(store.record_credit(execution(), proof), Err(error.into()));
        assert_eq!(
            (m.accounting.borrow().clone(), m.intents.borrow().clone()),
            before
        );
    }
    assert_eq!(store.allocation(execution()).unwrap().uncredited(), 100);
}

#[test]
fn authority_exact_identity_and_restoration_precede_host_credit_acquisition() {
    let m = memories();
    let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
    let original = input(1, 100);
    paid(&mut store, original, 0);
    let before = (m.accounting.borrow().clone(), m.intents.borrow().clone());
    assert_eq!(
        confirm(
            &mut store,
            UploadContext {
                actor: Principal::anonymous(),
                ..execution()
            },
            original,
            |_| panic!("must authenticate before acquisition")
        ),
        Err(FundingJournalError::NotOperator)
    );
    assert_eq!(
        confirm(&mut store, execution(), input(1, 99), |_| panic!(
            "must bind before acquisition"
        )),
        Err(FundingIntentError::Conflict.into())
    );
    assert_eq!(
        confirm(&mut store, execution(), original, |_| Some(receipt(
            input(2, 100),
            100,
            1
        ))),
        Err(FundingJournalError::Binding)
    );
    assert_eq!(
        (m.accounting.borrow().clone(), m.intents.borrow().clone()),
        before
    );
    store
        .record_credit(execution(), receipt(original, 100, 1))
        .unwrap();
    let saved = store.outcome(execution(), original).unwrap();
    let totals = store.allocation(execution()).unwrap();
    drop(store);
    let mut restored = StableFundingJournal::open(copy(&m), config(), allocation()).unwrap();
    assert_eq!(restored.outcome(execution(), original).unwrap(), saved);
    assert_eq!(restored.allocation(execution()).unwrap(), totals);
    assert_eq!(
        confirm(&mut restored, execution(), original, |_| panic!(
            "fence before acquisition"
        )),
        Err(FundingJournalError::Fenced)
    );
}

#[test]
fn uncertain_unsent_and_fully_refunded_intents_cannot_claim_credit() {
    for outcome in [
        None,
        Some(FundingTransportOutcome::NotEnqueued),
        Some(FundingTransportOutcome::Callback { refunded: 100 }),
    ] {
        let mut store = StableFundingJournal::install(memories(), config(), allocation()).unwrap();
        let intent = input(1, 100);
        store.prepare(execution(), intent).unwrap();
        store.mark_attempted(execution(), intent).unwrap();
        if let Some(outcome) = outcome {
            store
                .record_transport(execution(), intent, source(), outcome)
                .unwrap();
        }
        assert_eq!(
            confirm(&mut store, execution(), intent, |_| panic!(
                "known acceptance required"
            )),
            Err(FundingCreditError::AcceptanceRequired.into())
        );
    }
}

#[test]
fn restoration_refuses_inconsistent_credit_totals_and_duplicate_receipts_without_repair() {
    for duplicate in [false, true] {
        let m = memories();
        let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
        let first = input(1, 100);
        paid(&mut store, first, 0);
        let uncredited = store.totals().unwrap();
        store
            .record_credit(execution(), receipt(first, 100, 1))
            .unwrap();
        if duplicate {
            let second = input(2, 100);
            paid(&mut store, second, 0);
            let forged = store
                .required(second)
                .unwrap()
                .confirm_credit(receipt(second, 100, 1))
                .unwrap();
            store.intents.insert(second.operation.get(), forged);
            let totals = store.totals().unwrap().confirm_credit(n(100)).unwrap();
            store.write_totals(&totals);
        } else {
            store.write_totals(&uncredited);
        }
        drop(store);
        let before = (m.accounting.borrow().clone(), m.intents.borrow().clone());
        assert!(matches!(
            StableFundingJournal::open(copy(&m), config(), allocation()),
            Err(FundingJournalError::InvalidRecord)
        ));
        assert_eq!(
            (m.accounting.borrow().clone(), m.intents.borrow().clone()),
            before
        );
    }
}

#[test]
fn restoration_refuses_missing_misdirected_and_orphan_receipt_index_rows_without_repair() {
    for corruption in 0..3 {
        let m = memories();
        let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
        let first = input(1, 100);
        paid(&mut store, first, 0);
        store
            .record_credit(execution(), receipt(first, 100, 1))
            .unwrap();
        match corruption {
            0 => {
                store.accounting.remove(&[1; 32]);
            }
            1 => {
                store
                    .accounting
                    .insert([1; 32], FundingAccountingRecord::Receipt { operation: 2 });
            }
            _ => {
                store
                    .accounting
                    .insert([2; 32], FundingAccountingRecord::Receipt { operation: 1 });
            }
        }
        drop(store);
        let before = (m.accounting.borrow().clone(), m.intents.borrow().clone());
        assert!(matches!(
            StableFundingJournal::open(copy(&m), config(), allocation()),
            Err(FundingJournalError::InvalidRecord)
        ));
        assert_eq!(
            (m.accounting.borrow().clone(), m.intents.borrow().clone()),
            before
        );
    }
}
