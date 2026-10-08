use super::*;
mod admission;
mod assessment;
mod attempt;
mod credit;
mod history;
mod history_boundary;
mod outcome;
mod renewal;
mod summary;
use crate::model::billing::journal::FundingIntentState;
use crate::ops::service::tenant::tests::config;
use candid::Principal;
use ic_blob_storage_contracts::funding::transfer::FundingTransfer;
use ic_blob_storage_contracts::funding::transfer::FundingTransferError;
use ic_memory::ic_stable_structures::{Storable, VectorMemory};
use std::num::{NonZeroU128, NonZeroUsize};
fn n(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).unwrap()
}
fn execution() -> UploadContext {
    UploadContext {
        service: config().bindings().service,
        actor: config().bindings().operator,
    }
}
fn allocation() -> FundingAllocation {
    FundingAllocation::new(1000, n(100), NonZeroUsize::new(4).unwrap()).unwrap()
}
fn memories() -> FundingMemories<VectorMemory> {
    FundingMemories {
        accounting: VectorMemory::default(),
        intents: VectorMemory::default(),
    }
}
fn copy(m: &FundingMemories<VectorMemory>) -> FundingMemories<VectorMemory> {
    FundingMemories {
        accounting: m.accounting.clone(),
        intents: m.intents.clone(),
    }
}
fn input(operation: u128, offered: u128) -> FundingIntent {
    FundingIntent {
        service: execution().service,
        cashier: config().billing().cashier(),
        account: config().bindings().payment_account,
        namespace: config().bindings().namespace,
        operation: n(operation),
        offered: n(offered),
        target_balance: None,
    }
}
#[test]
fn exact_intents_hold_full_reservations_and_cannot_repeat_an_uncertain_attempt() {
    let mut store = StableFundingJournal::install(memories(), config(), allocation()).unwrap();
    let original = input(u128::MAX, 900);
    assert_eq!(
        store.prepare(execution(), original),
        Ok(FundingIntentAdmission::Created)
    );
    let before = store.allocation(execution()).unwrap();
    assert_eq!(before.available(), 100);
    assert_eq!(before.reserved_or_uncertain(), 900);
    assert_eq!(
        store.prepare(execution(), original),
        Ok(FundingIntentAdmission::Existing(
            FundingIntentState::Prepared
        ))
    );
    assert_eq!(
        store.prepare(execution(), input(u128::MAX, 899)),
        Err(FundingIntentError::Conflict.into())
    );
    assert_eq!(
        store.record_transport(
            execution(),
            original,
            source(),
            FundingTransportOutcome::NotEnqueued
        ),
        Err(FundingIntentError::NotAttempted.into())
    );
    store.mark_attempted(execution(), original).unwrap();
    assert_eq!(
        store.mark_attempted(execution(), original),
        Err(FundingIntentError::AlreadyAttempted.into())
    );
    assert_eq!(
        store.prepare(execution(), original),
        Ok(FundingIntentAdmission::Existing(
            FundingIntentState::Uncertain
        ))
    );
    assert_eq!(store.allocation(execution()), Ok(before));
}
#[test]
fn incremental_totals_match_shared_reconstruction_and_terminal_replay_never_refunds_twice() {
    let mut store = StableFundingJournal::install(memories(), config(), allocation()).unwrap();
    let mut transfers = Vec::new();
    for (id, amount, outcome) in [
        (1, 900, FundingTransportOutcome::Callback { refunded: 500 }),
        (2, 500, FundingTransportOutcome::NotEnqueued),
        (3, 500, FundingTransportOutcome::Callback { refunded: 500 }),
    ] {
        let request = input(id, amount);
        store.prepare(execution(), request).unwrap();
        assert_eq!(
            store.prepare(execution(), input(id + 1, 1)),
            Err(FundingAllocationError::PendingNotLast.into())
        );
        store.mark_attempted(execution(), request).unwrap();
        store
            .record_transport(execution(), request, source(), outcome)
            .unwrap();
        let before = store.allocation(execution()).unwrap();
        assert_eq!(
            store.record_transport(execution(), request, source(), outcome),
            Ok(false)
        );
        assert_eq!(store.allocation(execution()), Ok(before));
        transfers.push(match outcome {
            FundingTransportOutcome::NotEnqueued => FundingTransfer::not_enqueued(n(amount)),
            FundingTransportOutcome::Callback { refunded } => {
                FundingTransfer::unbounded_callback(n(amount), refunded).unwrap()
            }
        });
        assert_eq!(
            store.allocation(execution()),
            allocation()
                .reconstruct(&transfers)
                .map_err(FundingJournalError::Allocation)
        );
    }
    assert_eq!(
        store.record_transport(
            execution(),
            input(1, 900),
            source(),
            FundingTransportOutcome::NotEnqueued
        ),
        Err(FundingIntentError::OutcomeConflict.into())
    );
    store.prepare(execution(), input(4, 500)).unwrap();
    assert_eq!(store.allocation(execution()).unwrap().accepted(), 400);
    assert_eq!(
        store
            .allocation(execution())
            .unwrap()
            .reserved_or_uncertain(),
        500
    );
}
#[test]
fn invalid_refunds_capacity_and_lifetime_return_overflow_leave_intent_and_totals_unchanged() {
    let maximum = FundingAllocation::new(u128::MAX, n(1), NonZeroUsize::new(2).unwrap()).unwrap();
    let mut store = StableFundingJournal::install(memories(), config(), maximum).unwrap();
    for id in [1, 2] {
        let request = input(id, u128::MAX - 1);
        store.prepare(execution(), request).unwrap();
        store.mark_attempted(execution(), request).unwrap();
        let before = store.allocation(execution()).unwrap();
        assert_eq!(
            store.record_transport(
                execution(),
                request,
                source(),
                FundingTransportOutcome::Callback {
                    refunded: u128::MAX
                }
            ),
            Err(FundingIntentError::Transfer(FundingTransferError::RefundExceedsOffer).into())
        );
        assert_eq!(store.allocation(execution()), Ok(before));
        let result = store.record_transport(
            execution(),
            request,
            source(),
            FundingTransportOutcome::Callback {
                refunded: u128::MAX - 1,
            },
        );
        if id == 1 {
            assert_eq!(result, Ok(true));
        } else {
            assert_eq!(result, Err(FundingAllocationError::TotalsOverflow.into()));
            assert_eq!(
                store.lookup(execution(), request).unwrap().unwrap().state,
                FundingIntentState::Uncertain
            );
            assert_eq!(store.allocation(execution()), Ok(before));
        }
    }
    assert_eq!(
        store.prepare(execution(), input(3, 1)),
        Err(FundingAllocationError::Capacity.into())
    );
}
#[test]
fn every_phase_reopens_for_inspection_with_all_mutations_fenced() {
    for phase in 0..4 {
        let m = memories();
        let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
        let request = input(9, 900);
        store.prepare(execution(), request).unwrap();
        if phase > 0 {
            store.mark_attempted(execution(), request).unwrap();
        }
        if phase == 2 {
            store
                .record_transport(
                    execution(),
                    request,
                    source(),
                    FundingTransportOutcome::NotEnqueued,
                )
                .unwrap();
        }
        if phase == 3 {
            store
                .record_transport(
                    execution(),
                    request,
                    source(),
                    FundingTransportOutcome::Callback { refunded: 200 },
                )
                .unwrap();
        }
        let before = store.lookup(execution(), request).unwrap();
        let totals = store.allocation(execution()).unwrap();
        drop(store);
        let mut restored = StableFundingJournal::open(copy(&m), config(), allocation()).unwrap();
        assert!(restored.is_fenced());
        assert_eq!(restored.lookup(execution(), request), Ok(before));
        assert_eq!(restored.allocation(execution()), Ok(totals));
        assert_eq!(
            restored.prepare(execution(), request),
            Err(FundingJournalError::Fenced)
        );
        assert_eq!(
            restored.mark_attempted(execution(), request),
            Err(FundingJournalError::Fenced)
        );
        assert_eq!(
            restored.record_transport(
                execution(),
                request,
                source(),
                FundingTransportOutcome::NotEnqueued
            ),
            Err(FundingJournalError::Fenced)
        );
    }
}
#[test]
fn scope_authority_stale_identity_and_corrupt_history_cannot_reset_obligations() {
    let m = memories();
    let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
    let request = input(9, 400);
    let stranger = Principal::anonymous();
    assert_eq!(
        store.prepare(
            UploadContext {
                actor: stranger,
                ..execution()
            },
            request
        ),
        Err(FundingJournalError::NotOperator)
    );
    for changed in [
        FundingIntent {
            service: stranger,
            ..request
        },
        FundingIntent {
            cashier: stranger,
            ..request
        },
        FundingIntent {
            account: stranger,
            ..request
        },
        FundingIntent {
            namespace: n(2),
            ..request
        },
    ] {
        assert_eq!(
            store.prepare(execution(), changed),
            Err(FundingJournalError::Binding)
        );
        assert_eq!(
            store.lookup(execution(), changed),
            Err(FundingJournalError::Binding)
        );
    }
    store.prepare(execution(), request).unwrap();
    assert_eq!(
        store.prepare(execution(), input(8, 1)),
        Err(FundingJournalError::StaleIdentity)
    );
    store.intents.remove(&9);
    drop(store);
    let before = m.accounting.borrow().clone();
    assert!(matches!(
        StableFundingJournal::open(copy(&m), config(), allocation()),
        Err(FundingJournalError::InvalidRecord)
    ));
    assert_eq!(*m.accounting.borrow(), before);
    assert!(matches!(
        StableFundingJournal::install(copy(&m), config(), allocation()),
        Err(FundingJournalError::AlreadyAllocated)
    ));
}
#[test]
fn full_width_records_roundtrip_and_changed_allocation_rejects_restoration() {
    let m = memories();
    let maximum = FundingAllocation::new(u128::MAX, n(1), NonZeroUsize::MIN).unwrap();
    let mut store = StableFundingJournal::install(copy(&m), config(), maximum).unwrap();
    let request = input(u128::MAX, u128::MAX - 1);
    store.prepare(execution(), request).unwrap();
    let record = store.required(request).unwrap();
    assert_eq!(FundingIntentRecord::from_bytes(record.to_bytes()), record);
    let metadata = store.totals().unwrap();
    assert_eq!(
        FundingJournalRecord::from_bytes(metadata.to_bytes()),
        metadata
    );
    drop(store);
    assert!(matches!(
        StableFundingJournal::open(copy(&m), config(), allocation()),
        Err(FundingJournalError::Binding)
    ));
    assert!(StableFundingJournal::open(copy(&m), config(), maximum).is_ok());
}

#[test]
fn orphaned_changed_and_omitted_history_rejects_without_repair() {
    for damage in 0..4 {
        let m = memories();
        let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
        let request = input(1, 400);
        store.prepare(execution(), request).unwrap();
        match damage {
            0 => {
                store
                    .intents
                    .insert(2, FundingIntentRecord::new(input(2, 1)));
            }
            1 => {
                store
                    .intents
                    .insert(1, FundingIntentRecord::new(input(2, 400)));
            }
            2 => {
                store
                    .intents
                    .insert(1, FundingIntentRecord::new(input(1, 399)));
            }
            _ => {
                store.accounting.remove(&[0; 32]);
            }
        }
        drop(store);
        let before = m.intents.borrow().clone();
        assert!(matches!(
            StableFundingJournal::open(copy(&m), config(), allocation()),
            Err(FundingJournalError::InvalidRecord)
        ));
        assert_eq!(*m.intents.borrow(), before);
    }
}

#[test]
fn wide_principals_and_amounts_fit_bounded_v1_codecs() {
    let base = config();
    let bindings = ic_blob_storage_contracts::configuration::service::ServiceBindings {
        service: Principal::self_authenticating(b"service"),
        operator: Principal::self_authenticating(b"operator"),
        payment_account: Principal::self_authenticating(b"payer"),
        namespace: NonZeroU128::MAX,
    };
    let billing = ic_blob_storage_contracts::configuration::billing::BillingConfiguration::new(
        Principal::self_authenticating(b"cashier"),
        base.billing().funding_limits(),
        8,
        4,
    )
    .unwrap();
    let wide = ServiceConfiguration::new(bindings, base.limits(), billing).unwrap();
    let allocation =
        FundingAllocation::new(u128::MAX, n(1), NonZeroUsize::new(2).unwrap()).unwrap();
    let intent = FundingIntent {
        service: bindings.service,
        cashier: billing.cashier(),
        account: bindings.payment_account,
        namespace: bindings.namespace,
        operation: NonZeroU128::MAX,
        offered: n(u128::MAX - 1),
        target_balance: Some(NonZeroU128::MAX),
    };
    let row = FundingIntentRecord::new(intent)
        .attempted()
        .unwrap()
        .complete(FundingTransportOutcome::Callback {
            refunded: u128::MAX - 1,
        })
        .unwrap()
        .with_response(
            crate::model::billing::journal::record::response::FundingResponseRecord::Balance {
                total: u128::MAX,
                prepaid: u128::MAX,
                promotional: u128::MAX,
                ledger: u128::MAX,
            },
        )
        .unwrap();
    assert_eq!(FundingIntentRecord::from_bytes(row.to_bytes()), row);
    let record = FundingJournalRecord::new(&wide, allocation)
        .reserve(intent)
        .unwrap()
        .resolve(row.transfer().unwrap())
        .unwrap();
    assert_eq!(FundingJournalRecord::from_bytes(record.to_bytes()), record);
}

fn source() -> crate::model::billing::journal::FundingTransportContext {
    crate::model::billing::journal::FundingTransportContext {
        service: execution().service,
        cashier: config().billing().cashier(),
    }
}

#[test]
fn target_balance_is_exact_identity_and_wrong_callback_context_cannot_release_reservation() {
    let m = memories();
    let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
    let original = FundingIntent {
        target_balance: Some(n(100)),
        ..input(1, 900)
    };
    store.prepare(execution(), original).unwrap();
    for changed in [
        FundingIntent {
            target_balance: None,
            ..original
        },
        FundingIntent {
            target_balance: Some(n(101)),
            ..original
        },
    ] {
        assert_eq!(
            store.prepare(execution(), changed),
            Err(FundingIntentError::Conflict.into())
        );
        assert_eq!(
            store.request(execution(), changed),
            Err(FundingIntentError::Conflict.into())
        );
        assert_eq!(
            store.mark_attempted(execution(), changed),
            Err(FundingIntentError::Conflict.into())
        );
    }
    let prepared = store.request(execution(), original).unwrap();
    let issued = store.mark_attempted(execution(), original).unwrap();
    assert_eq!(issued, prepared);
    assert_eq!(issued.account(), original.account);
    assert_eq!(issued.target_balance(), original.target_balance);
    let before = store.allocation(execution()).unwrap();
    let foreign = Principal::self_authenticating(b"foreign");
    for context in [
        crate::model::billing::journal::FundingTransportContext {
            service: foreign,
            ..source()
        },
        crate::model::billing::journal::FundingTransportContext {
            cashier: foreign,
            ..source()
        },
    ] {
        assert_eq!(
            store.record_transport(
                execution(),
                original,
                context,
                FundingTransportOutcome::Callback { refunded: 900 }
            ),
            Err(FundingJournalError::TransportBinding)
        );
    }
    assert_eq!(store.allocation(execution()), Ok(before));
    assert_eq!(
        store.lookup(execution(), original).unwrap().unwrap().state,
        FundingIntentState::Uncertain
    );
    drop(store);
    let restored = StableFundingJournal::open(copy(&m), config(), allocation()).unwrap();
    assert_eq!(restored.request(execution(), original), Ok(issued));
    assert_eq!(restored.allocation(execution()), Ok(before));
}
