use super::*;
use crate::{
    dto::{
        funding::{FundingHistoryEntry, FundingHistoryRequest, FundingPhase},
        operator::OperatorScope,
    },
    workflow::funding::history::inspect,
};

#[test]
fn shared_history_preserves_full_width_attachment_target_and_refund_without_writes() {
    let memory = memories();
    let allocation =
        FundingAllocation::new(u128::MAX, n(100), NonZeroUsize::new(4).unwrap()).unwrap();
    let mut store = StableFundingJournal::install(copy(&memory), config(), allocation).unwrap();
    let intent = FundingIntent {
        target_balance: Some(n(u128::MAX)),
        ..input(u128::MAX, u128::MAX - 100)
    };
    let scope = OperatorScope {
        service: intent.service,
        cashier: intent.cashier,
        payment_account: intent.account,
        namespace: intent.namespace.get(),
    };
    let query = FundingHistoryRequest {
        scope,
        cursor: None,
    };
    store.prepare(execution(), intent).unwrap();
    store.mark_attempted(execution(), intent).unwrap();
    let refunded = intent.offered.get() - 1;
    store
        .record_transport(
            execution(),
            intent,
            source(),
            FundingTransportOutcome::Callback { refunded },
        )
        .unwrap();
    let before = (
        memory.accounting.borrow().clone(),
        memory.intents.borrow().clone(),
    );
    let mut page = inspect(&store, execution(), query, NonZeroUsize::MIN).unwrap();
    assert_eq!(page.request, query);
    assert_eq!(
        page.entries,
        vec![FundingHistoryEntry {
            scope,
            operation: u128::MAX,
            offered: intent.offered.get(),
            target_balance: Some(u128::MAX),
            phase: FundingPhase::Callback { refunded }
        }]
    );
    assert_eq!(page.next, None);
    assert!(!page.fenced);
    let roundtrip = candid::decode_one(&candid::encode_one(&page).unwrap()).unwrap();
    assert_eq!(page, roundtrip);
    let restored = StableFundingJournal::open(copy(&memory), config(), allocation).unwrap();
    page.fenced = true;
    assert_eq!(
        inspect(&restored, execution(), query, NonZeroUsize::MIN),
        Ok(page)
    );
    assert!(
        (
            memory.accounting.borrow().clone(),
            memory.intents.borrow().clone()
        )
            .eq(&before),
        "history changed stable bytes"
    );
}
