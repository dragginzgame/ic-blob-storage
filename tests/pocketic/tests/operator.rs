//! Actual CLI subprocesses against local fixtures; never a production provider.
#![cfg(not(target_family = "wasm"))]

mod operator_balance;
mod operator_funding_preview;
mod operator_method_mode;
mod operator_sync;
mod support;
mod sync_request;

use blob_test_protocol::funding::{
    FundingFailure, FundingObservation, FundingReplyMode, FundingRequest, FundingUpgradeArgs,
};
use candid::Principal;
use ic_testkit::{Fake, pic::CandidCallExt, pocket_ic::CreateCanisterParams};
use serde_json::Value;
use std::process::Command;
use support::{Harness, fixture_path};

struct Fixture {
    harness: Harness,
    authority: Principal,
    gateway: Principal,
    sender: Principal,
    receiver: Principal,
    driver: Principal,
}

impl Fixture {
    fn new() -> Self {
        Self::with_source_operator(false)
    }

    fn with_source_operator(source_operator: bool) -> Self {
        let harness = Harness::new();
        let pic = &harness.pic;
        let create = || {
            pic.create_canister_with_params(
                None,
                CreateCanisterParams {
                    cycles: Some(2_000_000_000_000),
                    ..CreateCanisterParams::default()
                },
            )
            .unwrap()
        };
        let authority = create();
        let gateway = create();
        let sender = create();
        let receiver = create();
        let driver = Fake::principal(11);
        pic.install_canister(
            authority,
            std::fs::read(fixture_path("BLOB_AUTHORITY_PROBE_WASM")).unwrap(),
            candid::encode_args((
                Fake::principal(1),
                Fake::principal(2),
                gateway,
                if source_operator { gateway } else { driver },
            ))
            .unwrap(),
            None,
        );
        pic.install_canister(
            gateway,
            std::fs::read(fixture_path("BLOB_GATEWAY_SOURCE_WASM")).unwrap(),
            candid::encode_args((authority, gateway, driver)).unwrap(),
            None,
        );
        let funding = std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).unwrap();
        for (canister, peer) in [(sender, receiver), (receiver, sender)] {
            pic.install_canister(
                canister,
                funding.clone(),
                candid::encode_args((
                    peer,
                    driver,
                    blob_test_protocol::funding::budget::FundingBudgetInput {
                        operating_reserve: 1_000_000_000,
                        other_liabilities: 0,
                        allocated: 100_000_000_000_000,
                        reserve: 1_000_000_000_000,
                    },
                ))
                .unwrap(),
                None,
            );
        }
        Self {
            harness,
            authority,
            gateway,
            sender,
            receiver,
            driver,
        }
    }

    fn args(
        &self,
        canister: Principal,
        caller: Principal,
        kind: &str,
        flag: &str,
        binding: &str,
    ) -> Vec<String> {
        let url = self.harness.pic.get_server_url();
        [
            "status".into(),
            "--server".into(),
            format!("{}:{}", url.host_str().unwrap(), url.port().unwrap()),
            "--instance".into(),
            self.harness.pic.instance_id().to_string(),
            "--canister".into(),
            canister.to_text(),
            "--caller".into(),
            caller.to_text(),
            "--kind".into(),
            kind.into(),
            flag.into(),
            binding.into(),
        ]
        .to_vec()
    }

    fn journals(&self) -> Vec<Vec<u8>> {
        [self.authority, self.gateway, self.sender, self.receiver]
            .into_iter()
            .map(|id| self.harness.pic.get_stable_memory(id))
            .collect()
    }
}

fn command(args: &[String]) -> (i32, Value) {
    let output = Command::new(env!("CARGO_BIN_EXE_blob-fixture-status"))
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.stderr.is_empty(),
        "unexpected CLI stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value = serde_json::from_slice(&output.stdout).expect("one JSON report");
    (output.status.code().expect("normal child exit"), value)
}

#[test]
fn status_and_check_preserve_journals_unknowns_and_instance_ownership() {
    let f = Fixture::new();
    let args = f.args(f.authority, f.driver, "authority", "--namespace", "1");
    let before = f.journals();
    let (code, report) = command(&args);
    assert_eq!(code, 0);
    assert_eq!(report["scope"], "pocketic_fixture");
    assert_eq!(report["mode"], "query");
    assert_eq!(report["target"]["canister"], f.authority.to_text());
    assert_eq!(report["status"]["namespace"], "1");
    assert!(report["status"]["provider_balance"].is_null());
    assert!(report["status"]["funding_activity"].is_null());
    let catalogs = report["status"]["catalogs"].as_array().unwrap();
    assert!(
        catalogs
            .iter()
            .any(|c| c["catalog"] == "Samples" && c["physical_bytes"] == "300")
    );
    assert!(
        catalogs
            .iter()
            .any(|c| c["catalog"] == "Journey" && c["physical_bytes"] == "0")
    );
    let mut check = args.clone();
    check[0] = "check".into();
    let (code, checked) = command(&check);
    assert_eq!(code, 4);
    assert_eq!(checked, report);
    assert!(
        f.journals() == before,
        "queries must preserve all fixture journals"
    );
    f.harness
        .pic
        .upgrade_canister(
            f.authority,
            std::fs::read(fixture_path("BLOB_AUTHORITY_PROBE_WASM")).unwrap(),
            candid::encode_args(()).unwrap(),
            None,
        )
        .unwrap();
    let frozen = f.journals();
    let (_, report) = command(&check);
    assert_eq!(report["status"]["fenced"], true);
    assert!(
        report["status"]["blockers"]
            .as_array()
            .unwrap()
            .contains(&Value::from("RecoveryFenced"))
    );
    assert!(
        f.journals() == frozen,
        "fenced inspection must preserve journals"
    );
}

#[test]
fn funding_report_keeps_accepted_cycles_separate_from_credit_after_restore() {
    let f = Fixture::new();
    let request = FundingRequest {
        id: u64::MAX,
        offered: 1_000_000_000,
        accept: 400_000_000,
        reply: FundingReplyMode::Success,
        trap_callback: false,
    };
    let result: Result<FundingObservation, FundingFailure> = f
        .harness
        .pic
        .update_candid_as(f.sender, f.driver, "fund", (request,))
        .unwrap();
    result.unwrap();
    let args = f.args(
        f.sender,
        f.driver,
        "funding",
        "--peer",
        &f.receiver.to_text(),
    );
    let before = f.journals();
    let (code, report) = command(&args);
    assert_eq!(code, 0);
    let attempt = &report["status"]["attempts"][0];
    assert_eq!(attempt["id"], u64::MAX.to_string());
    assert_eq!(attempt["transport_accepted"], "400000000");
    assert_eq!(attempt["refunded"], "600000000");
    assert!(attempt["provider_credit"].is_null());
    assert_eq!(report["status"]["funding_activity"], "Uncertain");
    assert!(
        f.journals() == before,
        "funding query must preserve receipts and attempts"
    );
    f.harness
        .pic
        .upgrade_canister(
            f.sender,
            std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).unwrap(),
            candid::encode_args((FundingUpgradeArgs {
                trap_after_restore: false,
            },))
            .unwrap(),
            None,
        )
        .unwrap();
    let (_, frozen) = command(&args);
    assert_eq!(frozen["status"]["fenced"], true);
    assert_eq!(frozen["status"]["attempts"], report["status"]["attempts"]);
}

#[test]
fn denied_misbound_and_missing_methods_never_become_success() {
    let f = Fixture::new();
    let before = f.journals();
    for (args, error) in [
        (
            f.args(
                f.authority,
                Principal::anonymous(),
                "authority",
                "--namespace",
                "1",
            ),
            "denied",
        ),
        (
            f.args(f.authority, f.driver, "authority", "--namespace", "2"),
            "binding_mismatch",
        ),
        (
            f.args(
                f.sender,
                f.driver,
                "funding",
                "--peer",
                &f.gateway.to_text(),
            ),
            "binding_mismatch",
        ),
        (
            f.args(f.gateway, f.driver, "authority", "--namespace", "1"),
            "query_rejected",
        ),
    ] {
        let (code, report) = command(&args);
        assert_eq!(code, 3);
        assert_eq!(report["error"], error);
    }
    assert!(
        f.journals() == before,
        "failed inspections cannot change fixture history"
    );
    let mut args = f.args(f.authority, f.driver, "authority", "--namespace", "1");
    args[4] = usize::MAX.to_string();
    let (code, report) = command(&args);
    assert_eq!(code, 3);
    assert_eq!(report["error"], "transport_failure");
    args.push("--update".into());
    let (code, report) = command(&args);
    assert_eq!(code, 2);
    assert_eq!(report["error"], "invalid_arguments");
    assert!(
        f.journals() == before,
        "failed transport leaves owned instance usable"
    );
}
