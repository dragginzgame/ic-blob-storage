use super::*;
use crate::workflow::funding::outcome::inspect;
use crate::{
    model::billing::journal::record::response::{BalanceFieldRecord, FundingResponseRecord as R},
    ops::caffeine::funding::{TopUpReply, transport::CashierTopUpStatus},
    policy::billing::reconciliation::{FundingReconciliation, assess_funding_reconciliation},
};
use ic_blob_storage_contracts::dto::funding::outcome::FundingBalanceField;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeFailure;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeRequest;
use ic_blob_storage_contracts::dto::funding::outcome::FundingReportedBalance;
use ic_blob_storage_contracts::dto::funding::outcome::FundingResponse;
use ic_blob_storage_contracts::dto::operator::OperatorScope;

fn request(intent: FundingIntent) -> FundingOutcomeRequest {
    FundingOutcomeRequest {
        scope: OperatorScope {
            service: intent.service,
            cashier: intent.cashier,
            payment_account: intent.account,
            namespace: intent.namespace.get(),
        },
        operation: intent.operation.get(),
        offered: intent.offered.get(),
        target_balance: intent.target_balance.map(NonZeroU128::get),
    }
}
fn check_boundary(
    store: &StableFundingJournal<VectorMemory>,
    intent: FundingIntent,
    response: FundingResponse,
    fenced: bool,
) {
    let query = request(intent);
    let view = inspect(store, execution(), query).unwrap().unwrap();
    assert_eq!(view.request, query);
    assert_eq!(view.response, Some(response));
    assert_eq!(view.fenced, fenced);
    assert_eq!(
        view,
        candid::decode_one(&candid::encode_one(view).unwrap()).unwrap()
    );
}
#[test]
fn exact_outcome_boundary_rejects_changed_intent_and_preserves_absence_without_writes() {
    let memory = memories();
    let mut store = StableFundingJournal::install(copy(&memory), config(), allocation()).unwrap();
    let intent = input(u128::MAX, 900);
    let query = request(intent);
    assert_eq!(inspect(&store, execution(), query), Ok(None));
    store.prepare(execution(), intent).unwrap();
    let before = (
        memory.accounting.borrow().clone(),
        memory.intents.borrow().clone(),
    );
    let view = inspect(&store, execution(), query).unwrap().unwrap();
    assert_eq!(view.response, None);
    assert_eq!(
        view.phase,
        ic_blob_storage_contracts::dto::funding::FundingPhase::Prepared
    );
    assert_eq!(
        view.reconciliation,
        ic_blob_storage_contracts::dto::funding::outcome::FundingReconciliation::TransferUnknown(
            900
        )
    );
    for changed in [
        FundingOutcomeRequest {
            offered: 899,
            ..query
        },
        FundingOutcomeRequest {
            target_balance: Some(1),
            ..query
        },
    ] {
        assert_eq!(
            inspect(&store, execution(), changed),
            Err(FundingOutcomeFailure::Conflict)
        );
    }
    for invalid in [
        FundingOutcomeRequest {
            offered: 0,
            ..query
        },
        FundingOutcomeRequest {
            operation: 0,
            ..query
        },
        FundingOutcomeRequest {
            target_balance: Some(0),
            ..query
        },
        FundingOutcomeRequest {
            scope: OperatorScope {
                namespace: 0,
                ..query.scope
            },
            ..query
        },
    ] {
        assert_eq!(
            inspect(&store, execution(), invalid),
            Err(FundingOutcomeFailure::Invalid)
        );
    }
    check_scope(&store, query);
    assert!(
        (
            memory.accounting.borrow().clone(),
            memory.intents.borrow().clone()
        )
            .eq(&before),
        "inspection changed retained bytes"
    );
}
fn check_scope(store: &StableFundingJournal<VectorMemory>, query: FundingOutcomeRequest) {
    let foreign = Principal::anonymous();
    for scope in [
        OperatorScope {
            service: foreign,
            ..query.scope
        },
        OperatorScope {
            cashier: foreign,
            ..query.scope
        },
        OperatorScope {
            payment_account: foreign,
            ..query.scope
        },
        OperatorScope {
            namespace: u128::MAX,
            ..query.scope
        },
    ] {
        assert_eq!(
            inspect(store, execution(), FundingOutcomeRequest { scope, ..query }),
            Err(FundingOutcomeFailure::Binding)
        );
    }
    assert_eq!(
        inspect(
            store,
            UploadContext {
                actor: foreign,
                ..execution()
            },
            query
        ),
        Err(FundingOutcomeFailure::Denied)
    );
    assert_eq!(
        inspect(
            store,
            UploadContext {
                service: foreign,
                ..execution()
            },
            query
        ),
        Err(FundingOutcomeFailure::Binding)
    );
}
fn response_cases() -> Vec<(R, FundingResponse)> {
    let principal = Principal::self_authenticating(b"diagnostic principal");
    vec![
        (R::NotDispatched, FundingResponse::NotDispatched),
        (R::NotEnqueued, FundingResponse::NotEnqueued),
        (R::Rejected(u32::MAX), FundingResponse::Rejected(u32::MAX)),
        (
            R::Balance {
                total: u128::MAX,
                prepaid: 2,
                promotional: 0,
                ledger: u128::MAX,
            },
            FundingResponse::ReportedSuccess(FundingReportedBalance {
                total: u128::MAX,
                prepaid: 2,
                promotional: 0,
                ledger: u128::MAX,
            }),
        ),
        (
            R::NotAuthorized(principal),
            FundingResponse::NotAuthorized(principal),
        ),
        (
            R::AccountBalanceOverflow,
            FundingResponse::AccountBalanceOverflow,
        ),
        (R::InternalError, FundingResponse::InternalError),
        (R::TopUpWithoutCycles, FundingResponse::TopUpWithoutCycles),
        (R::ReplyTooLarge, FundingResponse::ReplyTooLarge),
        (R::InvalidReply, FundingResponse::InvalidReply),
        (
            R::InvalidBalance(BalanceFieldRecord::Total),
            FundingResponse::InvalidBalance(FundingBalanceField::Total),
        ),
        (
            R::InvalidBalance(BalanceFieldRecord::Prepaid),
            FundingResponse::InvalidBalance(FundingBalanceField::Prepaid),
        ),
        (
            R::InvalidBalance(BalanceFieldRecord::Promotional),
            FundingResponse::InvalidBalance(FundingBalanceField::Promotional),
        ),
        (
            R::InvalidBalance(BalanceFieldRecord::Ledger),
            FundingResponse::InvalidBalance(FundingBalanceField::Ledger),
        ),
    ]
}
#[test]
fn structured_outcomes_roundtrip_without_converting_reports_or_errors_to_credit() {
    for (response, expected) in response_cases() {
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
        check_boundary(&store, intent, expected, false);
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
        check_boundary(&restored, intent, expected, true);
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
