//! Actual balance reads with independently encoded bytes and adversarial schedules.
use super::*;
use blob_test_protocol::{
    balance::*,
    status::{OperatorBlockerView, OperatorStatusView},
};
use ic_testkit::pocket_ic::{
    CanisterInstallMode,
    common::rest::{BlobCompression, RawMessageId},
};
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
    hex.trim()
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
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
    fn configure_balance(&self, scope: BalanceScope) -> Result<(), BalanceFailure> {
        self.harness
            .pic
            .update_candid_as(self.authority, self.driver, "configure_balance", (scope,))
            .unwrap()
    }
    fn source_balance(&self, reply: Vec<u8>, hold: bool, reject: bool) {
        let result: bool = self
            .harness
            .pic
            .update_candid_as(
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
            .unwrap();
        assert!(result);
    }
    fn refresh_balance(&self) -> Result<(), BalanceFailure> {
        self.harness
            .pic
            .update_candid_as(self.authority, self.driver, "refresh_balance", ())
            .unwrap()
    }
    fn balance_status(&self) -> OperatorStatusView {
        let status: Option<OperatorStatusView> = self
            .harness
            .pic
            .query_candid_as(self.authority, self.driver, "operator_status", ())
            .unwrap();
        status.unwrap()
    }
    fn balance_source_status(&self) -> BalanceSourceView {
        let status: Option<BalanceSourceView> = self
            .harness
            .pic
            .query_candid_as(self.gateway, self.driver, "balance_observation", ())
            .unwrap();
        status.unwrap()
    }
    fn balance_cli(&self) -> Value {
        let before = self.journals();
        let (code, json) =
            command(&self.args(self.authority, self.driver, "authority", "--namespace", "1"));
        assert_eq!(code, 0);
        assert!(
            self.journals() == before,
            "passive CLI must preserve balance and source journals"
        );
        json
    }
    fn hold_balance(&self) -> RawMessageId {
        self.source_balance(bytes("success"), true, false);
        let id = self
            .harness
            .pic
            .submit_call(
                self.authority,
                self.driver,
                "refresh_balance",
                candid::encode_args(()).unwrap(),
            )
            .unwrap();
        for _ in 0..30 {
            self.harness.pic.tick();
            if self.balance_source_status().pending.is_some() {
                return id;
            }
        }
        panic!("balance read did not reach held source");
    }
    fn resume_balance(&self, id: RawMessageId) -> Result<(), BalanceFailure> {
        let resumed: bool = self
            .harness
            .pic
            .update_candid_as(self.gateway, self.driver, "resume_balance", ())
            .unwrap();
        assert!(resumed);
        candid::decode_one(&self.harness.pic.await_call(id).unwrap()).unwrap()
    }
    fn upgrade_balance(&self, canister: Principal, skip: bool) {
        let pic = &self.harness.pic;
        let variable = if canister == self.authority {
            "BLOB_AUTHORITY_PROBE_WASM"
        } else {
            "BLOB_GATEWAY_SOURCE_WASM"
        };
        let wasm = std::fs::read(fixture_path(variable)).unwrap();
        let hashes = wasm
            .chunks(1024 * 1024)
            .map(|b| pic.upload_chunk(canister, None, b.to_vec()).unwrap())
            .collect();
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
            candid::encode_args(()).unwrap(),
        )
        .unwrap();
    }
}

#[test]
fn configured_observations_remain_passive_and_never_supply_spendability_or_credit() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    f.source_balance(bytes("success"), false, false);
    assert_eq!(f.balance_source_status().requests, 0);
    assert_eq!(f.balance_cli()["status"]["provider_balance"], Value::Null);
    assert_eq!(f.balance_source_status().requests, 0);
    f.refresh_balance().unwrap();
    let status = f.balance_status();
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
    let json = f.balance_cli();
    assert_eq!(json["status"]["provider_balance"], "100");
    let observation = &json["status"]["balance_observation"];
    assert_eq!(observation["scope"], "controlled_source");
    assert_eq!(observation["configured"]["account"], account().to_text());
    assert_eq!(observation["attempts"][0]["outcome"]["prepaid"], "80");
    assert_eq!(f.balance_source_status().requests, 1);
    f.harness.pic.advance_time(Duration::from_secs(31));
    f.harness.pic.tick();
    let expired = f.balance_status();
    assert_eq!(expired.provider_balance, None);
    assert_eq!(
        expired.balance_observation.usability,
        BalanceUsability::Expired
    );
    assert_eq!(
        expired.balance_observation.attempts,
        status.balance_observation.attempts
    );
    f.source_balance(bytes("zero"), false, false);
    f.refresh_balance().unwrap();
    assert_eq!(f.balance_status().provider_balance, Some(0));
}

#[test]
fn malformed_rejected_and_misbound_replies_replace_no_history_with_zero() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    f.source_balance(bytes("success"), false, false);
    f.refresh_balance().unwrap();
    let original = f.balance_status().balance_observation.attempts[0];
    for (reply, reject, expected) in [
        (bytes("not-found"), false, BalanceFailure::AccountNotFound),
        (bytes("internal"), false, BalanceFailure::ProviderInternal),
        (bytes("negative-ledger"), false, BalanceFailure::Malformed),
        (bytes("unknown-error"), false, BalanceFailure::Malformed),
        (vec![0], false, BalanceFailure::Malformed),
        (vec![0; 4097], false, BalanceFailure::Oversized),
        (bytes("success"), true, BalanceFailure::Transport),
    ] {
        f.source_balance(reply, false, reject);
        assert_eq!(f.refresh_balance(), Err(expected));
        let status = f.balance_status();
        assert_eq!(status.provider_balance, None);
        assert_eq!(status.balance_observation.attempts[0], original);
        assert_eq!(
            status.balance_observation.attempts.last().unwrap().outcome,
            Some(BalanceOutcomeView::Failed(expected))
        );
    }
    let other = Fake::principal(50);
    let changed = BalanceScope {
        account: other,
        ..f.balance_scope()
    };
    f.configure_balance(changed).unwrap();
    let configured: bool = f
        .harness
        .pic
        .update_candid_as(
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
        .unwrap();
    assert!(configured);
    assert_eq!(f.refresh_balance(), Err(BalanceFailure::AccountMismatch));
    assert!(f.balance_cli()["status"]["provider_balance"].is_null());
}

#[test]
fn held_reply_is_invalidated_by_reconfiguration_without_releasing_its_slot() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    let id = f.hold_balance();
    assert_eq!(
        f.balance_status().balance_observation.usability,
        BalanceUsability::Pending
    );
    assert_eq!(f.refresh_balance(), Err(BalanceFailure::Busy));
    f.configure_balance(f.balance_scope()).unwrap();
    assert_eq!(
        f.balance_cli()["status"]["balance_observation"]["usability"],
        "Invalidated"
    );
    assert_eq!(f.refresh_balance(), Err(BalanceFailure::Busy));
    assert_eq!(f.resume_balance(id), Err(BalanceFailure::Stale));
    assert_eq!(f.balance_source_status().requests, 1);
    assert_eq!(f.balance_status().provider_balance, None);
    f.source_balance(bytes("success"), false, false);
    f.refresh_balance().unwrap();
    assert_eq!(f.balance_status().provider_balance, Some(100));
}

#[test]
fn old_pending_journals_restore_fenced_without_replay_or_fresh_balances() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    let id = f.hold_balance();
    let pic = &f.harness.pic;
    let authority = pic.get_stable_memory(f.authority);
    let source = pic.get_stable_memory(f.gateway);
    let pending = f.balance_status().balance_observation.attempts;
    for (canister, variable) in [
        (f.authority, "BLOB_AUTHORITY_PROBE_WASM"),
        (f.gateway, "BLOB_GATEWAY_SOURCE_WASM"),
    ] {
        assert!(
            pic.upgrade_canister(
                canister,
                std::fs::read(fixture_path(variable)).unwrap(),
                candid::encode_args(()).unwrap(),
                None
            )
            .is_err()
        );
    }
    assert_eq!(f.resume_balance(id), Ok(()));
    pic.set_stable_memory(f.authority, authority, BlobCompression::NoCompression);
    pic.set_stable_memory(f.gateway, source, BlobCompression::NoCompression);
    f.upgrade_balance(f.authority, true);
    f.upgrade_balance(f.gateway, true);
    let status = f.balance_status();
    assert_eq!(status.balance_observation.attempts, pending);
    assert_eq!(
        status.balance_observation.usability,
        BalanceUsability::Fenced
    );
    assert_eq!(status.provider_balance, None);
    assert_eq!(f.refresh_balance(), Err(BalanceFailure::Fenced));
    assert_eq!(
        f.configure_balance(f.balance_scope()),
        Err(BalanceFailure::Fenced)
    );
    assert_eq!(f.balance_source_status().pending, Some(account()));
    assert_eq!(f.balance_source_status().requests, 1);
    f.balance_cli();
    f.upgrade_balance(f.authority, true);
    assert_eq!(f.balance_status().balance_observation.attempts, pending);
}

#[test]
fn forced_upgrade_while_callback_is_live_cannot_complete_the_restored_intent() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    let id = f.hold_balance();
    let pending = f.balance_status().balance_observation.attempts;
    f.upgrade_balance(f.authority, true);
    let resumed: bool = f
        .harness
        .pic
        .update_candid_as(f.gateway, f.driver, "resume_balance", ())
        .unwrap();
    assert!(resumed);
    let error = f
        .harness
        .pic
        .await_call(id)
        .expect_err("old async heap cannot resume after upgrade");
    assert_eq!(
        error.reject_code,
        ic_testkit::pocket_ic::RejectCode::CanisterError
    );
    assert_eq!(f.balance_status().balance_observation.attempts, pending);
    assert_eq!(f.balance_status().provider_balance, None);
    assert_eq!(f.balance_source_status().requests, 1);
}

#[test]
fn authority_and_capacity_checks_happen_before_any_source_call() {
    let f = Fixture::new();
    let before = f.journals();
    let denied: Result<(), BalanceFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.authority,
            Principal::anonymous(),
            "configure_balance",
            (f.balance_scope(),),
        )
        .unwrap();
    assert_eq!(denied, Err(BalanceFailure::Denied));
    assert_eq!(
        f.configure_balance(BalanceScope {
            namespace: 2,
            ..f.balance_scope()
        }),
        Err(BalanceFailure::Binding)
    );
    assert!(
        f.journals() == before,
        "invalid admission cannot alter history"
    );
    f.configure_balance(f.balance_scope()).unwrap();
    f.source_balance(bytes("success"), false, false);
    for _ in 0..16 {
        f.refresh_balance().unwrap();
    }
    let full = f.balance_status();
    assert_eq!(f.refresh_balance(), Err(BalanceFailure::Limit));
    assert_eq!(f.balance_status(), full);
    assert_eq!(f.balance_source_status().requests, 16);
    f.balance_cli(); // Maximum retained history fits the client decoder's bounds.
    f.upgrade_balance(f.authority, false);
    assert_eq!(
        f.balance_status().balance_observation.attempts,
        full.balance_observation.attempts
    );
}

#[test]
fn delayed_reply_ages_from_dispatch_and_restored_source_cannot_answer() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    let id = f.hold_balance();
    f.harness.pic.advance_time(Duration::from_secs(31));
    assert_eq!(f.resume_balance(id), Ok(()));
    assert_eq!(
        f.balance_status().balance_observation.usability,
        BalanceUsability::Expired
    );
    assert_eq!(f.balance_status().provider_balance, None);
    let id = f.hold_balance();
    f.upgrade_balance(f.gateway, true);
    assert_eq!(f.balance_source_status().pending, Some(account()));
    // The suspended source callback observes its fence before returning configured bytes.
    let result: Result<(), BalanceFailure> =
        candid::decode_one(&f.harness.pic.await_call(id).unwrap()).unwrap();
    assert_eq!(result, Err(BalanceFailure::Transport));
    assert_eq!(f.balance_status().provider_balance, None);
    assert_eq!(f.balance_source_status().pending, Some(account()));
    assert_eq!(f.balance_source_status().requests, 2);
    assert_eq!(f.refresh_balance(), Err(BalanceFailure::Transport));
    assert_eq!(f.balance_source_status().requests, 2);
}

mod billing;
