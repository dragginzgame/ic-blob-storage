use super::*;
use crate::{
    model::billing::journal::record::response::{BalanceFieldRecord, FundingResponseRecord as R},
    ops::caffeine::funding::{TopUpReply, transport::CashierTopUpStatus},
    policy::billing::reconciliation::{FundingReconciliation, assess_funding_reconciliation},
};

#[test]
fn structured_outcomes_roundtrip_without_converting_reports_or_errors_to_credit() {
    for response in [
        R::NotDispatched,
        R::NotEnqueued,
        R::Rejected(u32::MAX),
        R::Balance {
            total: u128::MAX,
            prepaid: 2,
            promotional: 0,
            ledger: u128::MAX,
        },
        R::NotAuthorized(Principal::self_authenticating(b"diagnostic principal")),
        R::AccountBalanceOverflow,
        R::InternalError,
        R::TopUpWithoutCycles,
        R::ReplyTooLarge,
        R::InvalidReply,
        R::InvalidBalance(BalanceFieldRecord::Total),
        R::InvalidBalance(BalanceFieldRecord::Prepaid),
        R::InvalidBalance(BalanceFieldRecord::Promotional),
        R::InvalidBalance(BalanceFieldRecord::Ledger),
    ] {
        let m = memories();
        let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
        let intent = input(1, 900);
        store.prepare(execution(), intent).unwrap();
        store.mark_attempted(execution(), intent).unwrap();
        let unsent = matches!(response, R::NotDispatched | R::NotEnqueued);
        let transport = if unsent {
            FundingTransportOutcome::NotEnqueued
        } else {
            FundingTransportOutcome::Callback { refunded: 500 }
        };
        assert_eq!(
            store.complete_transport(execution(), intent, source(), transport, Some(response)),
            Ok(true)
        );
        let before = store.outcome(execution(), intent).unwrap().unwrap();
        assert!(before.response.is_some());
        assert_eq!(
            assess_funding_reconciliation(before.transfer),
            if unsent {
                FundingReconciliation::NoTransfer
            } else {
                FundingReconciliation::CreditRequired {
                    accepted_cycles: n(400),
                }
            }
        );
        if let R::Balance {
            total,
            prepaid,
            promotional,
            ledger,
        } = response
        {
            assert_eq!(
                before.response,
                Some(CashierTopUpStatus::Replied(Ok(
                    TopUpReply::ReportedSuccess {
                        balance: crate::model::billing::balance::BalanceAmounts::new(
                            total,
                            prepaid,
                            promotional,
                            ledger
                        )
                    }
                )))
            );
        }
        let totals = store.allocation(execution()).unwrap();
        assert_eq!(
            store.complete_transport(execution(), intent, source(), transport, Some(response)),
            Ok(false)
        );
        assert_eq!(
            store.record_transport(execution(), intent, source(), transport),
            Ok(false)
        );
        assert_eq!(store.allocation(execution()), Ok(totals));
        let row = store.required(intent).unwrap();
        assert_eq!(FundingIntentRecord::from_bytes(row.to_bytes()), row);
        drop(store);
        let mut restored = StableFundingJournal::open(copy(&m), config(), allocation()).unwrap();
        assert_eq!(restored.outcome(execution(), intent), Ok(Some(before)));
        assert_eq!(
            restored.complete_transport(execution(), intent, source(), transport, Some(response)),
            Err(FundingJournalError::Fenced)
        );
    }
}

#[test]
fn absent_pending_and_transport_only_outcomes_remain_distinct_and_authorized() {
    let mut store = StableFundingJournal::install(memories(), config(), allocation()).unwrap();
    let intent = input(1, 900);
    let stranger = UploadContext {
        actor: Principal::anonymous(),
        ..execution()
    };
    assert_eq!(
        store.outcome(stranger, intent),
        Err(FundingJournalError::NotOperator)
    );
    assert_eq!(store.outcome(execution(), intent), Ok(None));
    store.prepare(execution(), intent).unwrap();
    let before = store.outcome(execution(), intent).unwrap().unwrap();
    assert_eq!(before.response, None);
    assert_eq!(
        assess_funding_reconciliation(before.transfer),
        FundingReconciliation::TransferUnknown {
            offered_cycles: n(900)
        }
    );
    let transport = FundingTransportOutcome::Callback { refunded: 500 };
    assert_eq!(
        store.complete_transport(
            execution(),
            intent,
            source(),
            transport,
            Some(R::InternalError)
        ),
        Err(FundingIntentError::NotAttempted.into())
    );
    store.mark_attempted(execution(), intent).unwrap();
    store
        .record_transport(execution(), intent, source(), transport)
        .unwrap();
    let before = store.outcome(execution(), intent).unwrap().unwrap();
    assert_eq!(before.response, None);
    assert_eq!(
        assess_funding_reconciliation(before.transfer),
        FundingReconciliation::CreditRequired {
            accepted_cycles: n(400)
        }
    );
    let totals = store.allocation(execution()).unwrap();
    assert_eq!(
        store.complete_transport(
            execution(),
            intent,
            source(),
            transport,
            Some(R::InternalError)
        ),
        Ok(true)
    );
    assert_eq!(store.allocation(execution()), Ok(totals));
    assert_eq!(
        store.complete_transport(
            execution(),
            intent,
            source(),
            transport,
            Some(R::InvalidReply)
        ),
        Err(FundingIntentError::OutcomeConflict.into())
    );
    assert_eq!(
        store.complete_transport(
            execution(),
            intent,
            source(),
            transport,
            Some(R::NotEnqueued)
        ),
        Err(FundingIntentError::OutcomeConflict.into())
    );
    assert_eq!(
        store.outcome(stranger, intent),
        Err(FundingJournalError::NotOperator)
    );
    assert_eq!(
        store.outcome(execution(), input(1, 899)),
        Err(FundingIntentError::Conflict.into())
    );
    assert_eq!(store.allocation(execution()), Ok(totals));
}
