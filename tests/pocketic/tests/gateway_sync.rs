//! Real inter-canister callbacks with a deliberately reentrant local list source.
//! This proves fixture composition, not the deployed Cashier or service adapters.

#![cfg(not(target_family = "wasm"))]

mod support;

use blob_test_protocol::{SourceMode, SourceObservation, SyncFailure};
use candid::Principal;
use ic_testkit::{
    Fake,
    pic::CandidCallExt,
    pocket_ic::{CanisterSettings, PocketIc},
};
use support::{Harness, fixture_path};

struct Fixture {
    harness: Harness,
    service: Principal,
    source: Principal,
    tenant: Principal,
    gateway: Principal,
    driver: Principal,
}

impl Fixture {
    fn new() -> Self {
        let harness = Harness::new();
        let pic = &harness.pic;
        let (tenant, gateway, driver) =
            (Fake::principal(1), Fake::principal(3), Fake::principal(4));
        let create = || {
            pic.create_canister_with_settings(
                Some(driver),
                Some(CanisterSettings {
                    controllers: Some(vec![driver]),
                    ..CanisterSettings::default()
                }),
            )
        };
        let (service, source) = (create(), create());
        pic.install_canister(
            service,
            std::fs::read(fixture_path("BLOB_AUTHORITY_PROBE_WASM")).expect("probe Wasm"),
            candid::encode_args((tenant, Fake::principal(2), gateway, source)).expect("probe init"),
            Some(driver),
        );
        pic.install_canister(
            source,
            std::fs::read(fixture_path("BLOB_GATEWAY_SOURCE_WASM")).expect("source Wasm"),
            candid::encode_args((service, gateway, driver)).expect("source init"),
            Some(driver),
        );
        let released: bool = pic
            .update_candid_as(service, tenant, "release", (1_u8,))
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

    fn configure(&self, mode: SourceMode) {
        let accepted: bool = self
            .pic()
            .update_candid_as(self.source, self.driver, "configure", (mode,))
            .expect("source configuration");
        assert!(accepted);
    }

    fn sync(&self) -> Result<(), SyncFailure> {
        self.pic()
            .update_candid_as(self.source, self.driver, "run_sync", ())
            .expect("drive actual source-to-probe call")
    }

    fn observation(&self) -> SourceObservation {
        let observation: Option<SourceObservation> = self
            .pic()
            .query_candid_as(self.source, self.driver, "observation", ())
            .expect("source observation");
        observation.expect("authorized driver")
    }

    fn pending(&self, actor: Principal) -> Option<Vec<u8>> {
        self.pic()
            .query_candid_as(self.service, actor, "pending", ())
            .expect("pending query")
    }
}

#[test]
fn denied_and_failed_syncs_do_not_change_membership_or_automatically_retry() {
    let fixture = Fixture::new();
    let before = fixture.observation();
    for caller in [
        fixture.tenant,
        fixture.gateway,
        fixture.driver,
        Principal::anonymous(),
    ] {
        let result: Result<(), SyncFailure> = fixture
            .pic()
            .update_candid_as(fixture.service, caller, "sync_gateway", ())
            .expect("sync denial");
        assert_eq!(result, Err(SyncFailure::Denied));
    }
    assert_eq!(fixture.observation(), before);
    for (mode, error) in [
        (SourceMode::Malformed, SyncFailure::InvalidReply),
        (SourceMode::Oversized, SyncFailure::ReplyTooLarge),
        (SourceMode::Empty, SyncFailure::InvalidReply),
        (SourceMode::Reject, SyncFailure::Transport),
    ] {
        fixture.configure(mode);
        let requests = fixture.observation().requests;
        assert_eq!(fixture.sync(), Err(error));
        assert_eq!(fixture.observation().requests, requests + 1);
        assert_eq!(fixture.pending(fixture.gateway), Some(vec![1]));
        fixture.configure(SourceMode::Valid);
        assert_eq!(fixture.sync(), Ok(()));
        assert_eq!(fixture.observation().requests, requests + 2);
        assert_eq!(fixture.pending(fixture.gateway), Some(vec![1]));
    }
}

#[test]
fn reentrant_sync_cannot_overlap_or_undo_revocation_and_newer_membership() {
    let fixture = Fixture::new();
    fixture.configure(SourceMode::Overlap);
    assert_eq!(fixture.sync(), Ok(()));
    assert_eq!(
        fixture.observation(),
        SourceObservation {
            requests: 1,
            nested_sync: Some(Err(SyncFailure::InProgress)),
        }
    );
    assert_eq!(fixture.pending(fixture.gateway), Some(vec![1]));

    fixture.configure(SourceMode::Revoke);
    assert_eq!(fixture.sync(), Err(SyncFailure::Stale));
    assert_eq!(fixture.pending(fixture.gateway), None);
    // A later explicit sync may re-add the member; revocation is not a permanent ban.
    fixture.configure(SourceMode::Valid);
    assert_eq!(fixture.sync(), Ok(()));
    assert_eq!(fixture.pending(fixture.gateway), Some(vec![1]));

    let replacement = Fake::principal(5);
    fixture.configure(SourceMode::Replace(replacement));
    let requests = fixture.observation().requests;
    assert_eq!(fixture.sync(), Err(SyncFailure::Stale));
    assert_eq!(
        fixture.observation(),
        SourceObservation {
            requests: requests + 2,
            nested_sync: Some(Ok(())),
        }
    );
    assert_eq!(fixture.pending(fixture.gateway), None);
    assert_eq!(fixture.pending(replacement), Some(vec![1]));
    assert_eq!(fixture.sync(), Ok(()));
    assert_eq!(fixture.pending(replacement), Some(vec![1]));
}
