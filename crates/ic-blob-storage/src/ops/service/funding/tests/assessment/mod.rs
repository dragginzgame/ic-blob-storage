use super::*;
use crate::{
    dto::{
        funding::assessment::{
            FundingPreparationBlocker as B, FundingPreparationFailure as E,
            FundingPreparationRequest,
        },
        operator::OperatorScope,
    },
    workflow::funding::assessment::inspect,
};
fn request(operation: u128, offered: u128) -> FundingPreparationRequest {
    let i = input(operation, offered);
    FundingPreparationRequest {
        scope: OperatorScope {
            service: i.service,
            namespace: i.namespace.get(),
            cashier: i.cashier,
            payment_account: i.account,
        },
        operation,
        offered,
        target_balance: None,
    }
}
fn external(blockers: &[B]) {
    for b in [
        B::ProviderUnqualified,
        B::RecoveryUnknown,
        B::FundingUnknown,
        B::SpendabilityUnknown,
    ] {
        assert!(blockers.contains(&b));
    }
}
#[test]
fn passive_assessment_preserves_full_width_identity_and_unknowns_without_reserving() {
    let m = memories();
    let store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
    let request = FundingPreparationRequest {
        target_balance: Some(u128::MAX),
        ..request(u128::MAX, u128::MAX)
    };
    let before = (m.accounting.borrow().clone(), m.intents.borrow().clone());
    let reply = inspect(&store, execution(), request).unwrap();
    assert_eq!(reply.request, request);
    external(&reply.blockers);
    assert!(reply.blockers.contains(&B::AllocationReserve {
        transferable_cycles: 900
    }));
    assert_eq!(reply.journal.retained_intents, 0);
    assert_eq!(reply.journal.last_operation, None);
    assert_eq!(reply.journal.attachment_allowance, 900);
    assert_eq!(
        (m.accounting.borrow().clone(), m.intents.borrow().clone()),
        before
    );
}
#[test]
fn original_obligations_retained_identity_and_lifetime_capacity_remain_blockers() {
    let mut store = StableFundingJournal::install(memories(), config(), allocation()).unwrap();
    for id in 1..=4 {
        let original = input(id, 1);
        store.prepare(execution(), original).unwrap();
        store.mark_attempted(execution(), original).unwrap();
        store
            .record_transport(
                execution(),
                original,
                source(),
                FundingTransportOutcome::Callback { refunded: 0 },
            )
            .unwrap();
    }
    let totals = store.allocation(execution()).unwrap();
    for (id, identity) in [
        (4, B::IdentityRetained),
        (3, B::IdentityRetained),
        (5, B::JournalFull),
    ] {
        let reply = inspect(&store, execution(), request(id, 1)).unwrap();
        external(&reply.blockers);
        for b in [identity, B::JournalFull, B::JournalUncredited] {
            assert!(reply.blockers.contains(&b));
        }
        assert_eq!(reply.journal.transport_accepted, 4);
        assert_eq!(reply.journal.retained_intents, 4);
    }
    assert_eq!(
        inspect(&store, execution(), request(4, 2)),
        Err(E::Conflict)
    );
    assert_eq!(
        inspect(
            &store,
            execution(),
            FundingPreparationRequest {
                target_balance: Some(1),
                ..request(4, 1)
            }
        ),
        Err(E::Conflict)
    );
    assert_eq!(store.allocation(execution()), Ok(totals));
}
#[test]
fn authority_validation_and_restore_keep_exact_pending_reservations_inspectable() {
    let m = memories();
    let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
    store.prepare(execution(), input(8, 900)).unwrap();
    store.mark_attempted(execution(), input(8, 900)).unwrap();
    for invalid in [
        FundingPreparationRequest {
            operation: 0,
            ..request(9, 1)
        },
        FundingPreparationRequest {
            offered: 0,
            ..request(9, 1)
        },
        FundingPreparationRequest {
            target_balance: Some(0),
            ..request(9, 1)
        },
    ] {
        assert_eq!(inspect(&store, execution(), invalid), Err(E::Invalid));
    }
    assert_eq!(
        inspect(
            &store,
            UploadContext {
                actor: Principal::anonymous(),
                ..execution()
            },
            request(9, 1)
        ),
        Err(E::Denied)
    );
    assert_eq!(
        inspect(
            &store,
            execution(),
            FundingPreparationRequest {
                scope: OperatorScope {
                    namespace: 2,
                    ..request(9, 1).scope
                },
                ..request(9, 1)
            }
        ),
        Err(E::Binding)
    );
    let reply = inspect(&store, execution(), request(7, 1)).unwrap();
    assert!(reply.blockers.contains(&B::IdentityStale));
    drop(store);
    let restored = StableFundingJournal::open(copy(&m), config(), allocation()).unwrap();
    let before = (m.accounting.borrow().clone(), m.intents.borrow().clone());
    let reply = inspect(&restored, execution(), request(8, 900)).unwrap();
    external(&reply.blockers);
    for b in [
        B::JournalFenced,
        B::IdentityRetained,
        B::JournalUncredited,
        B::AllocationReserve {
            transferable_cycles: 0,
        },
    ] {
        assert!(reply.blockers.contains(&b));
    }
    assert_eq!(reply.journal.reserved_or_uncertain, 900);
    assert_eq!(
        (m.accounting.borrow().clone(), m.intents.borrow().clone()),
        before
    );
}
