//! Real inter-canister callbacks with a deliberately reentrant local list source.
//! This proves fixture composition, not the deployed Cashier or service adapters.

#![cfg(not(target_family = "wasm"))]

use ic_blob_storage_pocketic_tests::fixture_calls;
use ic_blob_storage_pocketic_tests::installation;
mod support;
mod sync_request;

use blob_test_protocol::{SourceMode, SourceObservation, SyncFailure};
use ic_platform::principal::ExternalPrincipal as Principal;

use ic_testkit::fake_principal;
use ic_testkit::pocket_ic::PocketIc;
use support::Harness;

struct Fixture {
    harness: Harness,
    service: Principal,
    source: Principal,
    tenant: Principal,
    gateway: Principal,
    driver: Principal,
}

impl Fixture {
    async fn new() -> Self {
        let harness = Harness::new().await;
        let pic = &harness.pic;
        let (tenant, gateway, driver) = (fake_principal(1), fake_principal(3), fake_principal(4));
        let create = || installation::controlled_canister(pic, driver);
        let (service, source) = (create().await, create().await);
        installation::install_fixture(
            pic,
            service,
            "BLOB_AUTHORITY_PROBE_WASM",
            blob_test_protocol::fixture_wire::encode_args(
                "fixture_init",
                (tenant, fake_principal(2), gateway, source),
            )
            .expect("probe init"),
            Some(driver),
        )
        .await;
        installation::install_fixture(
            pic,
            source,
            "BLOB_GATEWAY_SOURCE_WASM",
            blob_test_protocol::fixture_wire::encode_args(
                "fixture_init",
                (service, gateway, driver),
            )
            .expect("source init"),
            Some(driver),
        )
        .await;
        let released: bool =
            fixture_calls::update_fixture(pic, service, tenant, "release", (1_u8,))
                .await
                .map(|(reply,)| reply)
                .expect("owner release");
        assert!(released);
        Self {
            harness,
            service,
            source,
            tenant,
            gateway,
            driver,
        }
    }

    fn pic(&self) -> &PocketIc {
        &self.harness.pic
    }

    async fn configure(&self, mode: SourceMode) {
        let accepted: bool = fixture_calls::update_fixture(
            self.pic(),
            self.source,
            self.driver,
            "configure",
            (mode,),
        )
        .await
        .map(|(reply,)| reply)
        .expect("source configuration");
        assert!(accepted);
    }

    async fn sync(&self) -> Result<(), SyncFailure> {
        fixture_calls::update_fixture(
            self.pic(),
            self.source,
            self.driver,
            "run_sync",
            (sync_request::read(self.pic(), self.service, self.source).await,),
        )
        .await
        .map(|(reply,)| reply)
        .expect("drive actual source-to-probe call")
    }

    async fn observation(&self) -> SourceObservation {
        let observation: Option<SourceObservation> =
            fixture_calls::query_fixture(self.pic(), self.source, self.driver, "observation", ())
                .await
                .map(|(reply,)| reply)
                .expect("source observation");
        observation.expect("authorized driver")
    }

    async fn pending(&self, actor: Principal) -> Option<Vec<u8>> {
        fixture_calls::query_fixture(self.pic(), self.service, actor, "pending", ())
            .await
            .map(|(reply,)| reply)
            .expect("pending query")
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn denied_and_failed_syncs_do_not_change_membership_or_automatically_retry() {
    let fixture = Fixture::new().await;
    let before = fixture.observation().await;
    for caller in [
        fixture.tenant,
        fixture.gateway,
        fixture.driver,
        Principal::anonymous(),
    ] {
        let result: Result<(), SyncFailure> = fixture_calls::update_fixture(
            fixture.pic(),
            fixture.service,
            caller,
            "sync_gateway",
            (sync_request::read(fixture.pic(), fixture.service, fixture.source).await,),
        )
        .await
        .map(|(reply,)| reply)
        .expect("sync denial");
        assert_eq!(result, Err(SyncFailure::Denied));
    }
    assert_eq!(fixture.observation().await, before);
    for (mode, error) in [
        (SourceMode::Malformed, SyncFailure::InvalidReply),
        (SourceMode::Oversized, SyncFailure::ReplyTooLarge),
        (SourceMode::Empty, SyncFailure::InvalidReply),
        (SourceMode::Reject, SyncFailure::Transport),
    ] {
        fixture.configure(mode).await;
        let requests = fixture.observation().await.requests;
        assert_eq!(fixture.sync().await, Err(error));
        assert_eq!(fixture.observation().await.requests, requests + 1);
        assert_eq!(fixture.pending(fixture.gateway).await, Some(vec![1]));
        fixture.configure(SourceMode::Valid).await;
        assert_eq!(fixture.sync().await, Ok(()));
        assert_eq!(fixture.observation().await.requests, requests + 2);
        assert_eq!(fixture.pending(fixture.gateway).await, Some(vec![1]));
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn reentrant_sync_cannot_overlap_or_undo_revocation_and_newer_membership() {
    let fixture = Fixture::new().await;
    fixture.configure(SourceMode::Overlap).await;
    assert_eq!(fixture.sync().await, Ok(()));
    assert_eq!(
        fixture.observation().await,
        SourceObservation {
            requests: 1,
            nested_sync: Some(Err(SyncFailure::InProgress)),
        }
    );
    assert_eq!(fixture.pending(fixture.gateway).await, Some(vec![1]));

    fixture.configure(SourceMode::Revoke).await;
    assert_eq!(fixture.sync().await, Err(SyncFailure::Stale));
    assert_eq!(fixture.pending(fixture.gateway).await, None);
    // A later explicit sync may re-add the member; revocation is not a permanent ban.
    fixture.configure(SourceMode::Valid).await;
    assert_eq!(fixture.sync().await, Ok(()));
    assert_eq!(fixture.pending(fixture.gateway).await, Some(vec![1]));

    let replacement = fake_principal(5);
    fixture.configure(SourceMode::Replace(replacement)).await;
    let requests = fixture.observation().await.requests;
    assert_eq!(fixture.sync().await, Err(SyncFailure::Stale));
    assert_eq!(
        fixture.observation().await,
        SourceObservation {
            requests: requests + 2,
            nested_sync: Some(Ok(())),
        }
    );
    assert_eq!(fixture.pending(fixture.gateway).await, None);
    assert_eq!(fixture.pending(replacement).await, Some(vec![1]));
    assert_eq!(fixture.sync().await, Ok(()));
    assert_eq!(fixture.pending(replacement).await, Some(vec![1]));
}
