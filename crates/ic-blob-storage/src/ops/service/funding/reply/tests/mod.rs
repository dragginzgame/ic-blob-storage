use super::*;
use crate::dto::funding::{
    FundingHistoryCursor, FundingHistoryEntry,
    outcome::{
        FundingOutcomeResponse, FundingReconciliation, FundingReportedBalance, FundingResponse,
    },
};
fn scope() -> OperatorScope {
    OperatorScope {
        service: Principal::from_slice(&[1, 1]),
        namespace: u128::MAX,
        cashier: Principal::from_slice(&[2, 1]),
        payment_account: Principal::from_slice(&[3, 1]),
    }
}
fn limits() -> FundingHistoryReplyLimits {
    FundingHistoryReplyLimits {
        bytes: 16_384.try_into().unwrap(),
        entries: 32.try_into().unwrap(),
    }
}
fn request() -> FundingOutcomeRequest {
    FundingOutcomeRequest {
        scope: scope(),
        operation: u128::MAX,
        offered: u128::MAX,
        target_balance: Some(u128::MAX),
    }
}
fn page() -> FundingHistoryPage {
    let input = request();
    FundingHistoryPage {
        request: FundingHistoryRequest {
            scope: scope(),
            cursor: None,
        },
        entries: vec![
            FundingHistoryEntry {
                scope: scope(),
                operation: input.operation,
                offered: input.offered,
                target_balance: input.target_balance,
                phase: FundingPhase::Callback {
                    refunded: u128::MAX - 1,
                },
            },
            FundingHistoryEntry {
                scope: scope(),
                operation: 4,
                offered: 1,
                target_balance: None,
                phase: FundingPhase::NotEnqueued,
            },
        ],
        next: Some(FundingHistoryCursor {
            scope: scope(),
            before_operation: 4,
        }),
        fenced: true,
    }
}
fn bytes<T: CandidType>(value: T) -> Vec<u8> {
    candid::encode_one(value).unwrap()
}
fn history_bytes(page: FundingHistoryPage) -> Vec<u8> {
    bytes(Ok::<_, FundingHistoryFailure>(page))
}
#[test]
fn history_checks_bounds_order_and_continuation_without_narrowing_amounts() {
    let page = page();
    let input = page.request;
    for (changed, error) in [
        (
            {
                let mut p = page.clone();
                p.entries.swap(0, 1);
                p
            },
            FundingReplyError::Invalid,
        ),
        (
            {
                let mut p = page.clone();
                p.entries[1] = p.entries[0];
                p
            },
            FundingReplyError::Invalid,
        ),
        (
            {
                let mut p = page.clone();
                p.entries[0].scope.namespace = 1;
                p
            },
            FundingReplyError::Binding,
        ),
        (
            {
                let mut p = page.clone();
                p.request.scope.payment_account = p.request.scope.cashier;
                p
            },
            FundingReplyError::Binding,
        ),
        (
            {
                let mut p = page.clone();
                p.entries[0].target_balance = Some(0);
                p
            },
            FundingReplyError::Invalid,
        ),
        (
            {
                let mut p = page.clone();
                p.entries[0].operation = 0;
                p
            },
            FundingReplyError::Invalid,
        ),
        (
            {
                let mut p = page.clone();
                p.entries[0].offered = 0;
                p
            },
            FundingReplyError::Invalid,
        ),
        (
            {
                let mut p = page.clone();
                p.entries[1].phase = FundingPhase::Callback { refunded: 2 };
                p
            },
            FundingReplyError::Invalid,
        ),
        (
            {
                let mut p = page.clone();
                p.next.as_mut().unwrap().before_operation = 3;
                p
            },
            FundingReplyError::Invalid,
        ),
        (
            {
                let mut p = page.clone();
                p.next.as_mut().unwrap().scope.namespace = 1;
                p
            },
            FundingReplyError::Binding,
        ),
        (
            {
                let mut p = page.clone();
                p.entries.clear();
                p
            },
            FundingReplyError::Invalid,
        ),
    ] {
        assert_eq!(
            history(input, &history_bytes(changed), limits()),
            Err(error)
        );
    }
}
#[test]
fn history_bounds_and_terminal_cursor() {
    let page = page();
    let input = page.request;
    let encoded = history_bytes(page.clone());
    assert_eq!(history(input, &encoded, limits()), Ok(page.clone()));
    for limit in [
        FundingHistoryReplyLimits {
            bytes: 1.try_into().unwrap(),
            ..limits()
        },
        FundingHistoryReplyLimits {
            entries: 1.try_into().unwrap(),
            ..limits()
        },
    ] {
        assert_eq!(
            history(input, &encoded, limit),
            Err(FundingReplyError::Limit)
        );
    }
    let mut terminal = page;
    terminal.request.cursor = terminal.next;
    terminal.next = None;
    terminal.entries.clear();
    assert_eq!(
        history(terminal.request, &history_bytes(terminal.clone()), limits()),
        Ok(terminal.clone())
    );
    terminal.entries.push(FundingHistoryEntry {
        operation: 4,
        ..page_entry()
    });
    assert_eq!(
        history(terminal.request, &history_bytes(terminal.clone()), limits()),
        Err(FundingReplyError::Invalid)
    );
}
fn page_entry() -> FundingHistoryEntry {
    page().entries[0]
}
#[test]
fn standalone_sized_page_fits_decoder_budget_with_full_width_amounts() {
    let mut page = page();
    page.entries = (0..32)
        .map(|offset| FundingHistoryEntry {
            operation: u128::MAX - offset,
            ..page_entry()
        })
        .collect();
    page.next.as_mut().unwrap().before_operation = u128::MAX - 31;
    assert_eq!(
        history(page.request, &history_bytes(page.clone()), limits()),
        Ok(page)
    );
}
#[test]
fn decoder_keeps_refusal_absence_and_invalid_inputs_distinct() {
    let input = page().request;
    assert_eq!(
        history(input, b"bad", limits()),
        Err(FundingReplyError::Invalid)
    );
    assert_eq!(
        history(
            input,
            &bytes(Err::<FundingHistoryPage, _>(FundingHistoryFailure::Denied)),
            limits()
        ),
        Err(FundingReplyError::History(FundingHistoryFailure::Denied))
    );
    let request = request();
    assert_eq!(
        outcome::inspect(
            request,
            &bytes(Ok::<Option<FundingOutcomeResponse>, FundingOutcomeFailure>(
                None
            )),
            limits().bytes
        ),
        Ok(None)
    );
    assert_eq!(
        outcome::inspect(
            request,
            &bytes(Err::<Option<FundingOutcomeResponse>, _>(
                FundingOutcomeFailure::Conflict
            )),
            limits().bytes
        ),
        Err(FundingReplyError::Outcome(FundingOutcomeFailure::Conflict))
    );
    for changed in [
        FundingOutcomeRequest {
            operation: 0,
            ..request
        },
        FundingOutcomeRequest {
            offered: 0,
            ..request
        },
        FundingOutcomeRequest {
            target_balance: Some(0),
            ..request
        },
    ] {
        assert_eq!(check_outcome(changed), Err(FundingReplyError::Invalid));
    }
    for (cursor, error) in [
        (
            FundingHistoryCursor {
                scope: scope(),
                before_operation: 0,
            },
            FundingReplyError::Invalid,
        ),
        (
            FundingHistoryCursor {
                scope: OperatorScope {
                    namespace: 1,
                    ..scope()
                },
                before_operation: 1,
            },
            FundingReplyError::Binding,
        ),
    ] {
        assert_eq!(
            check_history(FundingHistoryRequest {
                cursor: Some(cursor),
                ..input
            }),
            Err(error)
        );
    }
}
fn decode_outcome(
    view: &FundingOutcomeResponse,
) -> Result<Option<FundingOutcomeResponse>, FundingReplyError> {
    outcome::inspect(
        request(),
        &bytes(Ok::<_, FundingOutcomeFailure>(Some(view))),
        limits().bytes,
    )
}
#[test]
fn outcome_preserves_uncertainty_and_rejects_inconsistent_transport_claims() {
    let mut view = FundingOutcomeResponse {
        request: request(),
        phase: FundingPhase::Prepared,
        response: None,
        reconciliation: FundingReconciliation::TransferUnknown(u128::MAX),
        fenced: true,
    };
    for phase in [FundingPhase::Prepared, FundingPhase::Uncertain] {
        view.phase = phase;
        assert_eq!(decode_outcome(&view), Ok(Some(view)));
        assert_eq!(
            decode_outcome(&FundingOutcomeResponse {
                reconciliation: FundingReconciliation::NoTransfer,
                ..view
            }),
            Err(FundingReplyError::Invalid)
        );
        assert_eq!(
            decode_outcome(&FundingOutcomeResponse {
                response: Some(FundingResponse::InvalidReply),
                ..view
            }),
            Err(FundingReplyError::Invalid)
        );
    }
    check_callback(view);
}
fn check_callback(mut view: FundingOutcomeResponse) {
    view.phase = FundingPhase::Callback {
        refunded: u128::MAX - 1,
    };
    view.response = Some(FundingResponse::ReportedSuccess(FundingReportedBalance {
        total: u128::MAX,
        prepaid: u128::MAX,
        promotional: 0,
        ledger: 0,
    }));
    view.reconciliation = FundingReconciliation::CreditRequired(1);
    assert_eq!(decode_outcome(&view), Ok(Some(view)));
    for changed in [
        FundingOutcomeResponse {
            request: FundingOutcomeRequest {
                target_balance: None,
                ..request()
            },
            ..view
        },
        FundingOutcomeResponse {
            request: FundingOutcomeRequest {
                operation: 1,
                ..request()
            },
            ..view
        },
        FundingOutcomeResponse {
            request: FundingOutcomeRequest {
                offered: 1,
                ..request()
            },
            ..view
        },
    ] {
        assert_eq!(decode_outcome(&changed), Err(FundingReplyError::Binding));
    }
    for changed in [
        FundingOutcomeResponse {
            reconciliation: FundingReconciliation::NoTransfer,
            ..view
        },
        FundingOutcomeResponse {
            reconciliation: FundingReconciliation::CreditRequired(2),
            ..view
        },
        FundingOutcomeResponse {
            reconciliation: FundingReconciliation::TransferUnknown(u128::MAX),
            ..view
        },
        FundingOutcomeResponse {
            response: Some(FundingResponse::NotEnqueued),
            ..view
        },
    ] {
        assert_eq!(decode_outcome(&changed), Err(FundingReplyError::Invalid));
    }
    view.phase = FundingPhase::Callback {
        refunded: u128::MAX,
    };
    view.reconciliation = FundingReconciliation::NoTransfer;
    assert_eq!(decode_outcome(&view), Ok(Some(view)));
    view.phase = FundingPhase::NotEnqueued;
    for response in [
        None,
        Some(FundingResponse::NotDispatched),
        Some(FundingResponse::NotEnqueued),
    ] {
        view.response = response;
        assert_eq!(decode_outcome(&view), Ok(Some(view)));
    }
    assert_eq!(
        outcome::inspect(
            request(),
            &bytes(Ok::<_, FundingOutcomeFailure>(Some(view))),
            NonZeroUsize::MIN
        ),
        Err(FundingReplyError::Limit)
    );
}
