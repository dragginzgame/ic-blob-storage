//! Standalone passive funding scope and restored empty history.
use super::*;
use ic_blob_storage::dto::funding::outcome::{
    FundingOutcomeFailure, FundingOutcomeRequest, FundingOutcomeResponse,
};
use ic_blob_storage::dto::{
    funding::{
        FundingHistoryCursor, FundingHistoryFailure, FundingHistoryPage, FundingHistoryRequest,
    },
    operator::OperatorScope,
};
use ic_blob_storage::ops::service::funding::history::boundary::FUNDING_HISTORY_METHOD;
use ic_blob_storage::ops::service::funding::outcome::boundary::FUNDING_OUTCOME_METHOD;

#[test]
fn standalone_exact_funding_outcome_checks_authority_even_for_absent_history() {
    let f = Fixture::new();
    let scope = OperatorScope {
        service: f.service,
        cashier: f.config.billing.cashier,
        payment_account: f.config.payment_account,
        namespace: f.config.namespace,
    };
    let request = FundingOutcomeRequest {
        scope,
        operation: u128::MAX,
        offered: u128::MAX,
        target_balance: Some(u128::MAX),
    };
    let read = |actor,
                request: FundingOutcomeRequest|
     -> Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure> {
        f.harness
            .pic
            .query_candid_as(f.service, actor, FUNDING_OUTCOME_METHOD, (request,))
            .unwrap()
    };
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(read(f.operator, request), Ok(None));
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(read(actor, request), Err(FundingOutcomeFailure::Denied));
    }
    for changed in [
        OperatorScope {
            service: f.tenant,
            ..scope
        },
        OperatorScope {
            cashier: f.tenant,
            ..scope
        },
        OperatorScope {
            payment_account: f.tenant,
            ..scope
        },
        OperatorScope {
            namespace: 1,
            ..scope
        },
    ] {
        assert_eq!(
            read(
                f.operator,
                FundingOutcomeRequest {
                    scope: changed,
                    ..request
                }
            ),
            Err(FundingOutcomeFailure::Binding)
        );
    }
    assert_eq!(
        read(
            f.operator,
            FundingOutcomeRequest {
                offered: 0,
                ..request
            }
        ),
        Err(FundingOutcomeFailure::Invalid)
    );
    let replicated: Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.operator, FUNDING_OUTCOME_METHOD, (request,))
        .unwrap();
    assert_eq!(replicated, Ok(None));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(read(f.operator, request), Ok(None));
    assert_eq!(
        read(f.controller, request),
        Err(FundingOutcomeFailure::Denied)
    );
    assert!(f.configuration(f.operator).unwrap().fenced);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &restored);
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One scoped passive inspection journey through standalone restoration"
)]
fn standalone_funding_history_is_scoped_passive_and_stays_fenced_after_restore() {
    let f = Fixture::new();
    let scope = OperatorScope {
        service: f.service,
        namespace: f.config.namespace,
        cashier: f.config.billing.cashier,
        payment_account: f.config.payment_account,
    };
    let request = FundingHistoryRequest {
        scope,
        cursor: None,
    };
    let read = |actor,
                input: FundingHistoryRequest|
     -> Result<FundingHistoryPage, FundingHistoryFailure> {
        f.harness
            .pic
            .query_candid_as(f.service, actor, FUNDING_HISTORY_METHOD, (input,))
            .unwrap()
    };
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.tenant, f.uploader, f.controller, Principal::anonymous()] {
        assert_eq!(read(actor, request), Err(FundingHistoryFailure::Denied));
    }
    for changed in [
        OperatorScope {
            service: f.tenant,
            ..scope
        },
        OperatorScope {
            namespace: 1,
            ..scope
        },
        OperatorScope {
            cashier: f.tenant,
            ..scope
        },
        OperatorScope {
            payment_account: f.tenant,
            ..scope
        },
    ] {
        assert_eq!(
            read(
                f.operator,
                FundingHistoryRequest {
                    scope: changed,
                    ..request
                }
            ),
            Err(FundingHistoryFailure::Binding)
        );
        assert_eq!(
            read(
                f.operator,
                FundingHistoryRequest {
                    cursor: Some(FundingHistoryCursor {
                        scope: changed,
                        before_operation: u128::MAX
                    }),
                    ..request
                }
            ),
            Err(FundingHistoryFailure::CursorScope)
        );
    }
    assert_eq!(
        read(
            f.operator,
            FundingHistoryRequest {
                cursor: Some(FundingHistoryCursor {
                    scope,
                    before_operation: 0
                }),
                ..request
            }
        ),
        Err(FundingHistoryFailure::Invalid)
    );
    let mut expected = FundingHistoryPage {
        request,
        entries: vec![],
        next: None,
        fenced: false,
    };
    assert_eq!(read(f.operator, request), Ok(expected.clone()));
    let replicated: Result<FundingHistoryPage, FundingHistoryFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.operator, FUNDING_HISTORY_METHOD, (request,))
        .unwrap();
    assert_eq!(replicated, Ok(expected.clone()));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    expected.fenced = true;
    let restored = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(read(f.operator, request), Ok(expected));
    assert_eq!(
        read(f.controller, request),
        Err(FundingHistoryFailure::Denied)
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &restored);
}
