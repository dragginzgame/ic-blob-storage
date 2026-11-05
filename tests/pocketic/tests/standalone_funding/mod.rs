//! Standalone passive funding scope and restored empty history.
use super::*;
use ic_blob_storage_contracts::dto::funding::FundingHistoryCursor;
use ic_blob_storage_contracts::dto::funding::FundingHistoryFailure;
use ic_blob_storage_contracts::dto::funding::FundingHistoryPage;
use ic_blob_storage_contracts::dto::funding::FundingHistoryRequest;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeFailure;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeRequest;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeResponse;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_blob_storage_contracts::protocol::FUNDING_HISTORY_METHOD;
use ic_blob_storage_contracts::protocol::FUNDING_OUTCOME_METHOD;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One funding-outcome journey compares authority, exact identity and absent history"
)]
async fn standalone_exact_funding_outcome_checks_authority_even_for_absent_history() {
    let f = Fixture::new().await;
    let scope = OperatorScope {
        service: f.service.into(),
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
    let read = async |actor,
                      request: FundingOutcomeRequest|
           -> Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure> {
        f.harness
            .pic
            .query_blob_as(
                f.service,
                actor,
                FUNDING_OUTCOME_METHOD,
                request,
                REPLY_BYTES,
            )
            .await
            .unwrap()
    };
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(read(f.operator, request).await, Ok(None));
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(
            read(actor, request).await,
            Err(FundingOutcomeFailure::Denied)
        );
    }
    for changed in [
        OperatorScope {
            service: f.tenant.into(),
            ..scope
        },
        OperatorScope {
            cashier: f.tenant.into(),
            ..scope
        },
        OperatorScope {
            payment_account: f.tenant.into(),
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
            )
            .await,
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
        )
        .await,
        Err(FundingOutcomeFailure::Invalid)
    );
    let replicated: Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure> = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            f.operator,
            FUNDING_OUTCOME_METHOD,
            request,
            REPLY_BYTES,
        )
        .await
        .unwrap();
    assert_eq!(replicated, Ok(None));
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(read(f.operator, request).await, Ok(None));
    assert_eq!(
        read(f.controller, request).await,
        Err(FundingOutcomeFailure::Denied)
    );
    assert!(f.configuration(f.operator).await.unwrap().fenced);
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &restored,
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One scoped passive inspection journey through standalone restoration"
)]
async fn standalone_funding_history_is_scoped_passive_and_stays_fenced_after_restore() {
    let f = Fixture::new().await;
    let scope = OperatorScope {
        service: f.service.into(),
        namespace: f.config.namespace,
        cashier: f.config.billing.cashier,
        payment_account: f.config.payment_account,
    };
    let request = FundingHistoryRequest {
        scope,
        cursor: None,
    };
    let read = async |actor,
                      input: FundingHistoryRequest|
           -> Result<FundingHistoryPage, FundingHistoryFailure> {
        f.harness
            .pic
            .query_blob_as(f.service, actor, FUNDING_HISTORY_METHOD, input, REPLY_BYTES)
            .await
            .unwrap()
    };
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    for actor in [f.tenant, f.uploader, f.controller, Principal::anonymous()] {
        assert_eq!(
            read(actor, request).await,
            Err(FundingHistoryFailure::Denied)
        );
    }
    for changed in [
        OperatorScope {
            service: f.tenant.into(),
            ..scope
        },
        OperatorScope {
            namespace: 1,
            ..scope
        },
        OperatorScope {
            cashier: f.tenant.into(),
            ..scope
        },
        OperatorScope {
            payment_account: f.tenant.into(),
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
            )
            .await,
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
            )
            .await,
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
        )
        .await,
        Err(FundingHistoryFailure::Invalid)
    );
    let mut expected = FundingHistoryPage {
        request,
        entries: vec![],
        next: None,
        fenced: false,
    };
    assert_eq!(read(f.operator, request).await, Ok(expected.clone()));
    let replicated: Result<FundingHistoryPage, FundingHistoryFailure> = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            f.operator,
            FUNDING_HISTORY_METHOD,
            request,
            REPLY_BYTES,
        )
        .await
        .unwrap();
    assert_eq!(replicated, Ok(expected.clone()));
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    expected.fenced = true;
    let restored = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(read(f.operator, request).await, Ok(expected));
    assert_eq!(
        read(f.controller, request).await,
        Err(FundingHistoryFailure::Denied)
    );
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &restored,
    );
}
