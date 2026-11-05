//! Actual balance reads with independently encoded bytes and adversarial schedules.
use ic_blob_storage_pocketic_tests::fixture_calls;

mod refresh;
use super::*;
use blob_test_protocol::{
    balance::*,
    status::{OperatorBlockerView, OperatorStatusView},
};
use ic_testkit::pocket_ic::common::rest::BlobCompression;
use ic_testkit::pocket_ic::{CanisterInstallMode, common::rest::RawMessageId};
use sha2::{Digest, Sha256};
use std::time::Duration;

fn account() -> Principal {
    Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
}
fn bytes(name: &str) -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/ic-blob-storage/tests/fixtures/caffeine-balance")
        .join(format!("{name}.hex"));
    let hex = std::fs::read_to_string(path).unwrap();
    hex::decode(hex.trim()).expect("fixture hex")
}

impl Fixture {
    fn balance_scope(&self) -> BalanceScope {
        BalanceScope {
            service: self.authority,
            namespace: 1,
            source: self.gateway,
            account: account(),
        }
    }
    async fn configure_balance(&self, scope: BalanceScope) -> Result<(), BalanceFailure> {
        fixture_calls::update_fixture(
            &self.harness.pic,
            self.authority,
            self.driver,
            "configure_balance",
            (scope,),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap()
    }
    async fn source_balance(&self, reply: Vec<u8>, hold: bool, reject: bool) {
        let result: bool = fixture_calls::update_fixture(
            &self.harness.pic,
            self.gateway,
            self.driver,
            "configure_balance",
            (BalanceSourceConfig {
                account: account(),
                bytes: reply,
                hold,
                reject,
            },),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
        assert!(result);
    }
    async fn refresh_balance(&self) -> Result<(), BalanceFailure> {
        fixture_calls::update_fixture(
            &self.harness.pic,
            self.authority,
            self.driver,
            "refresh_balance",
            (self.refresh_request().await,),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap()
    }
    async fn refresh_request(&self) -> BalanceRefreshRequest {
        let balance = self.balance_status().await.balance_observation;
        BalanceRefreshRequest {
            scope: balance.configured.unwrap_or_else(|| self.balance_scope()),
            revision: balance.revision,
            sequence: balance.attempts.len() as u64 + 1,
        }
    }
    async fn balance_status(&self) -> OperatorStatusView {
        let status: Option<OperatorStatusView> = fixture_calls::query_fixture(
            &self.harness.pic,
            self.authority,
            self.driver,
            "operator_status",
            (),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
        status.unwrap()
    }
    async fn balance_source_status(&self) -> BalanceSourceView {
        let status: Option<BalanceSourceView> = fixture_calls::query_fixture(
            &self.harness.pic,
            self.gateway,
            self.driver,
            "balance_observation",
            (),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
        status.unwrap()
    }
    async fn inspect_balance(&self) -> OperatorStatusView {
        let before = self.journals().await;
        let status = self.balance_status().await;
        assert_eq!(
            self.journals().await,
            before,
            "inspection preserves balance and source journals"
        );
        status
    }
    async fn hold_balance(&self) -> RawMessageId {
        self.source_balance(bytes("success"), true, false).await;
        let id = self
            .harness
            .pic
            .submit_call(
                self.authority,
                self.driver,
                "refresh_balance",
                blob_test_protocol::fixture_wire::encode_one(
                    "refresh_balance",
                    self.refresh_request().await,
                )
                .unwrap(),
            )
            .await
            .unwrap();
        for _ in 0..30 {
            self.harness.pic.tick().await.unwrap();
            if self.balance_source_status().await.pending.is_some() {
                return id;
            }
        }
        panic!("balance read did not reach held source");
    }
    async fn resume_balance(&self, id: RawMessageId) -> Result<(), BalanceFailure> {
        let resumed: bool = fixture_calls::update_fixture(
            &self.harness.pic,
            self.gateway,
            self.driver,
            "resume_balance",
            (),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
        assert!(resumed);
        blob_test_protocol::fixture_wire::decode_one(
            "refresh_balance",
            &self.harness.pic.await_call(id).await.unwrap(),
            blob_test_protocol::fixture_wire::MAX_BYTES,
        )
        .unwrap()
    }
    pub(super) async fn upgrade_fixture(&self, canister: Principal, skip: bool) {
        let pic = &self.harness.pic;
        let variable = if canister == self.authority {
            "BLOB_AUTHORITY_PROBE_WASM"
        } else {
            "BLOB_GATEWAY_SOURCE_WASM"
        };
        let wasm = std::fs::read(fixture_path(variable)).unwrap();
        let hashes = {
            let mut values = Vec::new();
            for b in wasm.chunks(1024 * 1024) {
                values.push(pic.upload_chunk(canister, None, b.to_vec()).await.unwrap());
            }
            values
        };
        #[expect(
            clippy::default_trait_access,
            reason = "PocketIC does not reexport UpgradeFlags"
        )]
        let mut mode = CanisterInstallMode::Upgrade(Some(Default::default()));
        let CanisterInstallMode::Upgrade(Some(flags)) = &mut mode else {
            unreachable!()
        };
        flags.skip_pre_upgrade = Some(skip);
        pic.install_chunked_canister(
            canister,
            None,
            mode,
            canister,
            hashes,
            Sha256::digest(&wasm).to_vec(),
            blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
        )
        .await
        .unwrap();
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn configured_observations_remain_passive_and_never_supply_spendability_or_credit() {
    let f = Fixture::new().await;
    f.configure_balance(f.balance_scope()).await.unwrap();
    f.source_balance(bytes("success"), false, false).await;
    assert_eq!(f.balance_source_status().await.requests, 0);
    assert_eq!(f.inspect_balance().await.provider_balance, None);
    assert_eq!(f.balance_source_status().await.requests, 0);
    f.refresh_balance().await.unwrap();
    let status = f.balance_status().await;
    assert_eq!(status.provider_balance, Some(100));
    assert_eq!(status.available_funding_cycles, None);
    assert_eq!(status.funding_activity, None);
    assert!(!status.billing_configured && !status.provider_qualified);
    assert!(
        status
            .blockers
            .contains(&OperatorBlockerView::FundingUnknown)
    );
    assert!(
        status
            .blockers
            .contains(&OperatorBlockerView::BillingNotConfigured)
    );
    let observation = f.inspect_balance().await.balance_observation;
    assert_eq!(observation.configured.unwrap().account, account());
    let Some(BalanceOutcomeView::Reported(amounts, _)) = observation.attempts[0].outcome else {
        panic!("expected balance amounts")
    };
    assert_eq!(amounts.prepaid, 80);
    assert_eq!(f.balance_source_status().await.requests, 1);
    f.harness
        .pic
        .advance_time(Duration::from_secs(31))
        .await
        .unwrap();
    f.harness.pic.tick().await.unwrap();
    let expired = f.balance_status().await;
    assert_eq!(expired.provider_balance, None);
    assert_eq!(
        expired.balance_observation.usability,
        BalanceUsability::Expired
    );
    assert_eq!(
        expired.balance_observation.attempts,
        status.balance_observation.attempts
    );
    f.source_balance(bytes("zero"), false, false).await;
    f.refresh_balance().await.unwrap();
    assert_eq!(f.balance_status().await.provider_balance, Some(0));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn malformed_rejected_and_misbound_replies_replace_no_history_with_zero() {
    let f = Fixture::new().await;
    f.configure_balance(f.balance_scope()).await.unwrap();
    f.source_balance(bytes("success"), false, false).await;
    f.refresh_balance().await.unwrap();
    let original = f.balance_status().await.balance_observation.attempts[0];
    for (reply, reject, expected) in [
        (bytes("not-found"), false, BalanceFailure::AccountNotFound),
        (bytes("internal"), false, BalanceFailure::ProviderInternal),
        (bytes("negative-ledger"), false, BalanceFailure::Malformed),
        (bytes("unknown-error"), false, BalanceFailure::Malformed),
        (vec![0], false, BalanceFailure::Malformed),
        (vec![0; 4097], false, BalanceFailure::Oversized),
        (bytes("success"), true, BalanceFailure::Transport),
    ] {
        f.source_balance(reply, false, reject).await;
        assert_eq!(f.refresh_balance().await, Err(expected));
        let status = f.balance_status().await;
        assert_eq!(status.provider_balance, None);
        assert_eq!(status.balance_observation.attempts[0], original);
        assert_eq!(
            status.balance_observation.attempts.last().unwrap().outcome,
            Some(BalanceOutcomeView::Failed(expected))
        );
    }
    let other = fake_principal(50);
    let changed = BalanceScope {
        account: other,
        ..f.balance_scope()
    };
    f.configure_balance(changed).await.unwrap();
    let configured: bool = fixture_calls::update_fixture(
        &f.harness.pic,
        f.gateway,
        f.driver,
        "configure_balance",
        (BalanceSourceConfig {
            account: other,
            bytes: bytes("success"),
            hold: false,
            reject: false,
        },),
    )
    .await
    .map(|(reply,)| reply)
    .unwrap();
    assert!(configured);
    assert_eq!(
        f.refresh_balance().await,
        Err(BalanceFailure::AccountMismatch)
    );
    assert!(f.inspect_balance().await.provider_balance.is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn held_reply_is_invalidated_by_reconfiguration_without_releasing_its_slot() {
    let f = Fixture::new().await;
    f.configure_balance(f.balance_scope()).await.unwrap();
    let id = f.hold_balance().await;
    assert_eq!(
        f.balance_status().await.balance_observation.usability,
        BalanceUsability::Pending
    );
    assert_eq!(f.refresh_balance().await, Err(BalanceFailure::Busy));
    f.configure_balance(f.balance_scope()).await.unwrap();
    assert_eq!(
        f.inspect_balance().await.balance_observation.usability,
        BalanceUsability::Invalidated
    );
    assert_eq!(f.refresh_balance().await, Err(BalanceFailure::Busy));
    assert_eq!(f.resume_balance(id).await, Err(BalanceFailure::Stale));
    assert_eq!(f.balance_source_status().await.requests, 1);
    assert_eq!(f.balance_status().await.provider_balance, None);
    f.source_balance(bytes("success"), false, false).await;
    f.refresh_balance().await.unwrap();
    assert_eq!(f.balance_status().await.provider_balance, Some(100));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn old_pending_journals_restore_fenced_without_replay_or_fresh_balances() {
    let f = Fixture::new().await;
    f.configure_balance(f.balance_scope()).await.unwrap();
    let id = f.hold_balance().await;
    let pic = &f.harness.pic;
    let authority = pic.get_stable_memory(f.authority).await.unwrap();
    let source = pic.get_stable_memory(f.gateway).await.unwrap();
    let pending = f.balance_status().await.balance_observation.attempts;
    for (canister, variable) in [
        (f.authority, "BLOB_AUTHORITY_PROBE_WASM"),
        (f.gateway, "BLOB_GATEWAY_SOURCE_WASM"),
    ] {
        assert!(
            pic.upgrade_canister(
                canister,
                std::fs::read(fixture_path(variable)).unwrap(),
                blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
                None
            )
            .await
            .is_err()
        );
    }
    assert_eq!(f.resume_balance(id).await, Ok(()));
    pic.set_stable_memory(f.authority, authority, BlobCompression::NoCompression)
        .await
        .unwrap();
    pic.set_stable_memory(f.gateway, source, BlobCompression::NoCompression)
        .await
        .unwrap();
    f.upgrade_fixture(f.authority, true).await;
    f.upgrade_fixture(f.gateway, true).await;
    let status = f.balance_status().await;
    assert_eq!(status.balance_observation.attempts, pending);
    assert_eq!(
        status.balance_observation.usability,
        BalanceUsability::Fenced
    );
    assert_eq!(status.provider_balance, None);
    assert_eq!(f.refresh_balance().await, Err(BalanceFailure::Fenced));
    assert_eq!(
        f.configure_balance(f.balance_scope()).await,
        Err(BalanceFailure::Fenced)
    );
    assert_eq!(f.balance_source_status().await.pending, Some(account()));
    assert_eq!(f.balance_source_status().await.requests, 1);
    f.inspect_balance().await;
    f.upgrade_fixture(f.authority, true).await;
    assert_eq!(
        f.balance_status().await.balance_observation.attempts,
        pending
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn forced_upgrade_while_callback_is_live_cannot_complete_the_restored_intent() {
    let f = Fixture::new().await;
    f.configure_balance(f.balance_scope()).await.unwrap();
    let id = f.hold_balance().await;
    let pending = f.balance_status().await.balance_observation.attempts;
    f.upgrade_fixture(f.authority, true).await;
    let resumed: bool =
        fixture_calls::update_fixture(&f.harness.pic, f.gateway, f.driver, "resume_balance", ())
            .await
            .map(|(reply,)| reply)
            .unwrap();
    assert!(resumed);
    let error = f
        .harness
        .pic
        .await_call(id)
        .await
        .expect_err("old async heap cannot resume after upgrade");
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        ic_testkit::pocket_ic::RejectCode::CanisterError
    );
    assert_eq!(
        f.balance_status().await.balance_observation.attempts,
        pending
    );
    assert_eq!(f.balance_status().await.provider_balance, None);
    assert_eq!(f.balance_source_status().await.requests, 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn authority_and_capacity_checks_happen_before_any_source_call() {
    let f = Fixture::new().await;
    let before = f.journals().await;
    let denied: Result<(), BalanceFailure> = fixture_calls::update_fixture(
        &f.harness.pic,
        f.authority,
        Principal::anonymous(),
        "configure_balance",
        (f.balance_scope(),),
    )
    .await
    .map(|(reply,)| reply)
    .unwrap();
    assert_eq!(denied, Err(BalanceFailure::Denied));
    assert_eq!(
        f.configure_balance(BalanceScope {
            namespace: 2,
            ..f.balance_scope()
        })
        .await,
        Err(BalanceFailure::Binding)
    );
    assert!(
        f.journals().await == before,
        "invalid admission cannot alter history"
    );
    f.configure_balance(f.balance_scope()).await.unwrap();
    f.source_balance(bytes("success"), false, false).await;
    for _ in 0..16 {
        f.refresh_balance().await.unwrap();
    }
    let full = f.balance_status().await;
    assert_eq!(f.refresh_balance().await, Err(BalanceFailure::Limit));
    assert_eq!(f.balance_status().await, full);
    assert_eq!(f.balance_source_status().await.requests, 16);
    f.inspect_balance().await; // Maximum retained history fits the fixture decoder's bounds.
    f.upgrade_fixture(f.authority, false).await;
    assert_eq!(
        f.balance_status().await.balance_observation.attempts,
        full.balance_observation.attempts
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn delayed_reply_ages_from_dispatch_and_restored_source_cannot_answer() {
    let f = Fixture::new().await;
    f.configure_balance(f.balance_scope()).await.unwrap();
    let id = f.hold_balance().await;
    f.harness
        .pic
        .advance_time(Duration::from_secs(31))
        .await
        .unwrap();
    assert_eq!(f.resume_balance(id).await, Ok(()));
    assert_eq!(
        f.balance_status().await.balance_observation.usability,
        BalanceUsability::Expired
    );
    assert_eq!(f.balance_status().await.provider_balance, None);
    let id = f.hold_balance().await;
    f.upgrade_fixture(f.gateway, true).await;
    assert_eq!(f.balance_source_status().await.pending, Some(account()));
    // The suspended source callback observes its fence before returning configured bytes.
    let result: Result<(), BalanceFailure> = blob_test_protocol::fixture_wire::decode_one(
        "refresh_balance",
        &f.harness.pic.await_call(id).await.unwrap(),
        blob_test_protocol::fixture_wire::MAX_BYTES,
    )
    .unwrap();
    assert_eq!(result, Err(BalanceFailure::Transport));
    assert_eq!(f.balance_status().await.provider_balance, None);
    assert_eq!(f.balance_source_status().await.pending, Some(account()));
    assert_eq!(f.balance_source_status().await.requests, 2);
    assert_eq!(f.refresh_balance().await, Err(BalanceFailure::Transport));
    assert_eq!(f.balance_source_status().await.requests, 2);
}

mod billing;
