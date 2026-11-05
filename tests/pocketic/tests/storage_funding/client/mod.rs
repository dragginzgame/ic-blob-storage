//! Real replicated observers, bounded replies and passive inspection after restore.
use super::*;
use blob_test_protocol::consumer::{Failure as ConsumerFailure, funding::FundingHistoryInspection};
use ic_blob_storage_contracts::dto::funding::FundingHistoryPage;
use ic_blob_storage_contracts::dto::funding::FundingHistoryRequest;
use ic_blob_storage_contracts::dto::funding::FundingPhase;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeRequest;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeResponse;
use ic_blob_storage_contracts::dto::funding::outcome::FundingReconciliation;
use ic_blob_storage_pocketic_tests::control_calls::ControlCallExt;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
fn consumer_wasm() -> Vec<u8> {
    std::fs::read(fixture_path("BLOB_CONSUMER_PROBE_WASM")).unwrap()
}
async fn fixture(authorized: bool) -> (Fixture, Principal) {
    let harness = Harness::new().await;
    let driver = fake_principal(2);
    let client = harness
        .pic
        .create_canister_with_settings(
            Some(driver),
            Some(CanisterSettings {
                controllers: Some(vec![driver]),
                ..CanisterSettings::default()
            }),
        )
        .await
        .unwrap();
    let f = Fixture::with_operator(
        harness,
        if authorized {
            client
        } else {
            fake_principal(1)
        },
    )
    .await;
    f.harness
        .pic
        .install_canister(
            client,
            consumer_wasm(),
            blob_test_protocol::fixture_wire::encode_args("fixture_init", (driver, f.service))
                .unwrap(),
            Some(driver),
        )
        .await
        .unwrap();
    (f, client)
}
async fn history(
    f: &Fixture,
    client: Principal,
    request: FundingHistoryRequest,
    max_reply_bytes: u32,
) -> Result<FundingHistoryPage, ConsumerFailure> {
    f.harness
        .pic
        .update_control_as(
            client,
            f.controller,
            "fixture_funding_history",
            FundingHistoryInspection {
                request,
                max_reply_bytes,
            },
            REPLY_BYTES,
        )
        .await
        .unwrap()
}
async fn outcome(
    f: &Fixture,
    client: Principal,
    request: FundingOutcomeRequest,
) -> Result<Option<FundingOutcomeResponse>, ConsumerFailure> {
    f.harness
        .pic
        .update_control_as(
            client,
            f.controller,
            "fixture_funding_outcome",
            request,
            REPLY_BYTES,
        )
        .await
        .unwrap()
}
fn request(f: &Fixture, operation: u128) -> FundingOutcomeRequest {
    FundingOutcomeRequest {
        scope: f.funding_scope(),
        operation,
        offered: 100,
        target_balance: Some(u128::MAX),
    }
}
async fn seed(f: &Fixture) {
    for operation in [1, u128::MAX - 1, u128::MAX] {
        let intent = Intent {
            target_balance: Some(u128::MAX),
            ..f.funding_intent(operation, 100)
        };
        f.funding(f.operator, intent, Action::Prepare)
            .await
            .unwrap();
        f.funding(f.operator, intent, Action::Attempt)
            .await
            .unwrap();
        match operation {
            1 => {
                f.funding(f.operator, intent, Action::NotEnqueued)
                    .await
                    .unwrap();
            }
            u128::MAX => {}
            _ => {
                f.funding(f.operator, intent, Action::Callback(80))
                    .await
                    .unwrap();
            }
        }
    }
}
async fn observe(f: &Fixture, client: Principal, fenced: bool) {
    let input = FundingHistoryRequest {
        scope: f.funding_scope(),
        cursor: None,
    };
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let client_before = f.harness.pic.get_stable_memory(client).await.unwrap();
    let first = history(f, client, input, 16_384).await.unwrap();
    assert_eq!(first.fenced, fenced);
    assert_eq!(
        first
            .entries
            .iter()
            .map(|e| e.operation)
            .collect::<Vec<_>>(),
        vec![u128::MAX, u128::MAX - 1]
    );
    assert_eq!(first.entries[0].phase, FundingPhase::Uncertain);
    assert_eq!(
        history(f, client, input, 1).await,
        Err(ConsumerFailure::Transport)
    );
    let last = history(
        f,
        client,
        FundingHistoryRequest {
            cursor: first.next,
            ..input
        },
        16_384,
    )
    .await
    .unwrap();
    assert_eq!(
        last.entries.iter().map(|e| e.operation).collect::<Vec<_>>(),
        vec![1]
    );
    assert_eq!(last.next, None);
    for (operation, reconciliation) in [
        (1, FundingReconciliation::NoTransfer),
        (u128::MAX - 1, FundingReconciliation::CreditRequired(20)),
        (u128::MAX, FundingReconciliation::TransferUnknown(100)),
    ] {
        let view = outcome(f, client, request(f, operation))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(view.request, request(f, operation));
        assert_eq!(view.reconciliation, reconciliation);
        assert_eq!(view.response, None);
        assert_eq!(view.fenced, fenced);
    }
    assert_eq!(outcome(f, client, request(f, 2)).await, Ok(None));
    assert_eq!(
        outcome(
            f,
            client,
            FundingOutcomeRequest {
                offered: 99,
                ..request(f, 1)
            }
        )
        .await,
        Err(ConsumerFailure::Transport)
    );
    assert!(
        f.harness
            .pic
            .get_stable_memory(f.service)
            .await
            .unwrap()
            .eq(&before),
        "inspection changed service bytes"
    );
    assert!(
        f.harness
            .pic
            .get_stable_memory(client)
            .await
            .unwrap()
            .eq(&client_before),
        "inspection changed client bytes"
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn funding_client_reads_pages_and_exact_outcomes_through_both_restorations() {
    let (f, client) = fixture(true).await;
    seed(&f).await;
    observe(&f, client, false).await;
    f.harness
        .pic
        .upgrade_canister(
            client,
            consumer_wasm(),
            blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
            Some(f.controller),
        )
        .await
        .unwrap();
    observe(&f, client, false).await;
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation("fixture_post_upgrade", f.operator),
            Some(f.controller),
        )
        .await
        .unwrap();
    observe(&f, client, true).await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn funding_client_requires_host_authentication_service_operator_and_replicated_execution() {
    for authorized in [false, true] {
        let (f, client) = fixture(authorized).await;
        let request = FundingHistoryRequest {
            scope: f.funding_scope(),
            cursor: None,
        };
        let input = FundingHistoryInspection {
            request,
            max_reply_bytes: 16_384,
        };
        let denied: Result<FundingHistoryPage, ConsumerFailure> = f
            .harness
            .pic
            .update_control_as(
                client,
                f.other,
                "fixture_funding_history",
                input,
                REPLY_BYTES,
            )
            .await
            .unwrap();
        assert_eq!(denied, Err(ConsumerFailure::Denied));
        let query: Result<FundingHistoryPage, ConsumerFailure> = f
            .harness
            .pic
            .query_control_as(
                client,
                f.controller,
                "fixture_query_funding",
                input,
                REPLY_BYTES,
            )
            .await
            .unwrap();
        assert_eq!(query, Err(ConsumerFailure::Transport));
        if authorized {
            assert_eq!(
                history(&f, client, request, 16_384).await.unwrap().entries,
                []
            );
        } else {
            assert_eq!(
                history(&f, client, request, 16_384).await,
                Err(ConsumerFailure::Transport)
            );
            assert_eq!(
                outcome(&f, client, self::request(&f, 1)).await,
                Err(ConsumerFailure::Transport)
            );
        }
        let changed = FundingHistoryRequest {
            scope: ic_blob_storage_contracts::dto::operator::OperatorScope {
                namespace: 2,
                ..request.scope
            },
            ..request
        };
        assert_eq!(
            history(&f, client, changed, 16_384).await,
            Err(ConsumerFailure::Transport)
        );
    }
}
