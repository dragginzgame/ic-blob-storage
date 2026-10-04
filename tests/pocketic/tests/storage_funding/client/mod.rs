//! Real replicated observers, bounded replies and passive inspection after restore.
use super::*;
use blob_test_protocol::consumer::{Failure as ConsumerFailure, funding::FundingHistoryInspection};
use ic_blob_storage::dto::funding::{
    FundingHistoryPage, FundingHistoryRequest, FundingPhase,
    outcome::{FundingOutcomeRequest, FundingOutcomeResponse, FundingReconciliation},
};
fn consumer_wasm() -> Vec<u8> {
    std::fs::read(fixture_path("BLOB_CONSUMER_PROBE_WASM")).unwrap()
}
fn fixture(authorized: bool) -> (Fixture, Principal) {
    let harness = Harness::new();
    let driver = Fake::principal(2);
    let client = harness.pic.create_canister_with_settings(
        Some(driver),
        Some(CanisterSettings {
            controllers: Some(vec![driver]),
            ..CanisterSettings::default()
        }),
    );
    let f = Fixture::with_operator(
        harness,
        if authorized {
            client
        } else {
            Fake::principal(1)
        },
    );
    f.harness.pic.install_canister(
        client,
        consumer_wasm(),
        candid::encode_args((driver, f.service)).unwrap(),
        Some(driver),
    );
    (f, client)
}
fn history(
    f: &Fixture,
    client: Principal,
    request: FundingHistoryRequest,
    max_reply_bytes: u32,
) -> Result<FundingHistoryPage, ConsumerFailure> {
    f.harness
        .pic
        .update_candid_as(
            client,
            f.controller,
            "fixture_funding_history",
            (FundingHistoryInspection {
                request,
                max_reply_bytes,
            },),
        )
        .unwrap()
}
fn outcome(
    f: &Fixture,
    client: Principal,
    request: FundingOutcomeRequest,
) -> Result<Option<FundingOutcomeResponse>, ConsumerFailure> {
    f.harness
        .pic
        .update_candid_as(client, f.controller, "fixture_funding_outcome", (request,))
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
fn seed(f: &Fixture) {
    for operation in [1, u128::MAX - 1, u128::MAX] {
        let intent = Intent {
            target_balance: Some(u128::MAX),
            ..f.funding_intent(operation, 100)
        };
        f.funding(f.operator, intent, Action::Prepare).unwrap();
        f.funding(f.operator, intent, Action::Attempt).unwrap();
        match operation {
            1 => {
                f.funding(f.operator, intent, Action::NotEnqueued).unwrap();
            }
            u128::MAX => {}
            _ => {
                f.funding(f.operator, intent, Action::Callback(80)).unwrap();
            }
        }
    }
}
fn observe(f: &Fixture, client: Principal, fenced: bool) {
    let input = FundingHistoryRequest {
        scope: f.funding_scope(),
        cursor: None,
    };
    let before = f.harness.pic.get_stable_memory(f.service);
    let client_before = f.harness.pic.get_stable_memory(client);
    let first = history(f, client, input, 16_384).unwrap();
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
        history(f, client, input, 1),
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
        let view = outcome(f, client, request(f, operation)).unwrap().unwrap();
        assert_eq!(view.request, request(f, operation));
        assert_eq!(view.reconciliation, reconciliation);
        assert_eq!(view.response, None);
        assert_eq!(view.fenced, fenced);
    }
    assert_eq!(outcome(f, client, request(f, 2)), Ok(None));
    assert_eq!(
        outcome(
            f,
            client,
            FundingOutcomeRequest {
                offered: 99,
                ..request(f, 1)
            }
        ),
        Err(ConsumerFailure::Transport)
    );
    assert!(
        f.harness.pic.get_stable_memory(f.service).eq(&before),
        "inspection changed service bytes"
    );
    assert!(
        f.harness.pic.get_stable_memory(client).eq(&client_before),
        "inspection changed client bytes"
    );
}
#[test]
fn funding_client_reads_pages_and_exact_outcomes_through_both_restorations() {
    let (f, client) = fixture(true);
    seed(&f);
    observe(&f, client, false);
    f.harness
        .pic
        .upgrade_canister(
            client,
            consumer_wasm(),
            candid::encode_args(()).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    observe(&f, client, false);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    observe(&f, client, true);
}
#[test]
fn funding_client_requires_host_authentication_service_operator_and_replicated_execution() {
    for authorized in [false, true] {
        let (f, client) = fixture(authorized);
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
            .update_candid_as(client, f.other, "fixture_funding_history", (input,))
            .unwrap();
        assert_eq!(denied, Err(ConsumerFailure::Denied));
        let query: Result<FundingHistoryPage, ConsumerFailure> = f
            .harness
            .pic
            .query_candid_as(client, f.controller, "fixture_query_funding", (input,))
            .unwrap();
        assert_eq!(query, Err(ConsumerFailure::Transport));
        if authorized {
            assert_eq!(history(&f, client, request, 16_384).unwrap().entries, []);
        } else {
            assert_eq!(
                history(&f, client, request, 16_384),
                Err(ConsumerFailure::Transport)
            );
            assert_eq!(
                outcome(&f, client, self::request(&f, 1)),
                Err(ConsumerFailure::Transport)
            );
        }
        let changed = FundingHistoryRequest {
            scope: ic_blob_storage::dto::operator::OperatorScope {
                namespace: 2,
                ..request.scope
            },
            ..request
        };
        assert_eq!(
            history(&f, client, changed, 16_384),
            Err(ConsumerFailure::Transport)
        );
    }
}
