//! Maintained query shape over a controlled local source, not a deployed Cashier.
use super::*;
use blob_test_protocol::SourceMode;
use ic_blob_storage::model::gateway::GatewayListError;
use ic_blob_storage::model::gateway::membership::GatewayMembership;
use ic_blob_storage::model::gateway::registry::GatewayRegistry;
use ic_blob_storage::model::gateway::registry::GatewayScope;
use ic_blob_storage::model::gateway::registry::GatewaySyncError;
use ic_blob_storage::ops::caffeine::decoding::CaffeineReplyLimits;
use ic_blob_storage::ops::caffeine::gateway::GatewayReplyError;
use ic_blob_storage_pocketic_tests::fixture_calls;

use ic_blob_storage::ops::caffeine::query::CashierQuery;
use ic_blob_storage::ops::caffeine::query::CashierQueryRequest;
use ic_blob_storage_contracts::configuration::limits::GatewayListLimits;
use std::num::{NonZeroU128, NonZeroUsize};

fn n(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).unwrap()
}

fn limits() -> CaffeineReplyLimits {
    CaffeineReplyLimits {
        max_bytes: n(4096),
        decoding_quota: n(100_000),
        skipping_quota: n(1000),
        max_type_entries: n(32),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn gateway_query_feeds_an_exact_pending_sync_without_mutating_source_journals() {
    let f = Fixture::new().await;
    let request =
        CashierQueryRequest::new(f.gateway.into(), CashierQuery::StorageGateways).unwrap();
    let scope = GatewayScope::new(
        f.service.into(),
        NonZeroU128::new(1).unwrap(),
        f.gateway.into(),
    )
    .unwrap();
    let mut registry = GatewayRegistry::new(
        scope,
        GatewayMembership::new(GatewayListLimits {
            max_entries: n(4),
            max_unique: n(4),
        }),
    );
    let token = registry.begin_sync().unwrap();
    let before = f.journals().await;
    let bytes = f
        .harness
        .pic
        .query_call(
            request.cashier().into(),
            f.driver,
            request.method_name(),
            request.arguments().to_vec(),
        )
        .await
        .unwrap();
    request
        .apply_gateway_sync_reply(&mut registry, token, scope, &bytes, limits())
        .unwrap();
    assert_eq!(
        registry.gateways().principals(),
        &[ic_platform::Principal::from(f.gateway)]
    );
    assert_eq!(registry.sync_view().pending_sequence, None);
    let completed = registry.clone();
    assert_eq!(
        request.apply_gateway_sync_reply(&mut registry, token, scope, &bytes, limits()),
        Err(GatewayReplyError::Sync(GatewaySyncError::StaleSync).into())
    );
    assert_eq!(registry, completed);
    let token = registry.begin_sync().unwrap();
    registry.remove(f.gateway.into());
    let revoked = registry.clone();
    assert_eq!(
        request.apply_gateway_sync_reply(&mut registry, token, scope, &bytes, limits()),
        Err(GatewayReplyError::Sync(GatewaySyncError::StaleSync).into())
    );
    assert_eq!(registry, revoked);
    assert!(
        f.journals().await == before,
        "query and local registry work preserve fixture journals"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn gateway_query_refuses_other_callers_held_rejected_and_restored_sources() {
    let f = Fixture::new().await;
    let request =
        CashierQueryRequest::new(f.gateway.into(), CashierQuery::StorageGateways).unwrap();
    let query = |caller| {
        f.harness.pic.query_call(
            request.cashier().into(),
            caller,
            request.method_name(),
            request.arguments().to_vec(),
        )
    };
    let before = f.journals().await;
    for caller in [
        Principal::anonymous(),
        f.service,
        f.gateway,
        fake_principal(99),
    ] {
        assert!(query(caller).await.is_err());
    }
    assert!(
        f.journals().await == before,
        "denied callers preserve all journals"
    );
    for mode in [SourceMode::Hold, SourceMode::Reject] {
        let configured: bool = fixture_calls::update_fixture(
            &f.harness.pic,
            f.gateway,
            f.driver,
            "configure",
            (mode,),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
        assert!(configured);
        let before = f.journals().await;
        assert!(query(f.driver).await.is_err());
        assert!(
            f.journals().await == before,
            "query must not execute a scripted source action"
        );
    }
    let configured: bool = fixture_calls::update_fixture(
        &f.harness.pic,
        f.gateway,
        f.driver,
        "configure",
        (SourceMode::Valid,),
    )
    .await
    .map(|(reply,)| reply)
    .unwrap();
    assert!(configured);
    assert!(query(f.driver).await.is_ok());
    f.restore_source().await;
    let before = f.journals().await;
    assert!(query(f.driver).await.is_err());
    assert!(
        f.journals().await == before,
        "restored source remains fenced"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn gateway_query_passive_fault_replies_preserve_source_and_pending_sync() {
    let f = Fixture::new().await;
    let request =
        CashierQueryRequest::new(f.gateway.into(), CashierQuery::StorageGateways).unwrap();
    let scope = GatewayScope::new(
        f.service.into(),
        NonZeroU128::new(1).unwrap(),
        f.gateway.into(),
    )
    .unwrap();
    let mut registry = GatewayRegistry::new(
        scope,
        GatewayMembership::new(GatewayListLimits {
            max_entries: n(4),
            max_unique: n(4),
        }),
    );
    let token = registry.begin_sync().unwrap();
    let pending = registry.clone();
    for (mode, expected) in [
        (SourceMode::Malformed, GatewayReplyError::InvalidReply),
        (SourceMode::Oversized, GatewayReplyError::ReplyTooLarge),
        (
            SourceMode::Empty,
            GatewayReplyError::Sync(GatewaySyncError::InvalidList(GatewayListError::Empty)),
        ),
    ] {
        let configured: bool = fixture_calls::update_fixture(
            &f.harness.pic,
            f.gateway,
            f.driver,
            "configure",
            (mode,),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
        assert!(configured);
        let before = f.journals().await;
        let bytes = f
            .harness
            .pic
            .query_call(
                request.cashier().into(),
                f.driver,
                request.method_name(),
                request.arguments().to_vec(),
            )
            .await
            .expect("passive fault mode returns bytes for client validation");
        assert_eq!(
            request.apply_gateway_sync_reply(&mut registry, token, scope, &bytes, limits()),
            Err(expected.into()),
            "mode {mode:?}"
        );
        assert_eq!(registry, pending);
        assert!(
            f.journals().await == before,
            "passive replies preserve journals"
        );
    }
}
