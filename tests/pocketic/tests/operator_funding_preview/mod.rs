//! Passive funding admission against actual transfer journals, not a payment client.
use super::*;
use blob_test_protocol::funding::preview::{
    FundingPreviewFailure, FundingPreviewRequest, FundingPreviewView,
};

fn args(f: &Fixture, id: u64, amount: u128) -> Vec<String> {
    let status: Option<blob_test_protocol::funding::FundingOperatorStatusView> = f
        .harness
        .pic
        .query_candid_as(f.sender, f.driver, "operator_status", ())
        .unwrap();
    let mut args = f.args(
        f.sender,
        f.driver,
        "funding",
        "--peer",
        &f.receiver.to_text(),
    );
    args[0] = "dry-run".into();
    args.extend([
        "--revision".into(),
        status.unwrap().budget.revision.to_string(),
        "--id".into(),
        id.to_string(),
        "--amount".into(),
        amount.to_string(),
    ]);
    args
}
fn preview(args: &[String]) -> (i32, Value) {
    let output = Command::new(env!("CARGO_BIN_EXE_blob-fixture-funding-preview"))
        .args(args)
        .output()
        .unwrap();
    assert!(output.stderr.is_empty(), "errors belong in JSON");
    (
        output.status.code().unwrap(),
        serde_json::from_slice(&output.stdout).unwrap(),
    )
}
fn blocked(f: &Fixture, id: u64, amount: u128) -> Vec<Value> {
    let before = f.journals();
    let (code, value) = preview(&args(f, id, amount));
    assert_eq!(code, 4);
    assert_eq!(value["mode"], "query");
    assert_eq!(value["preview"]["id"], id.to_string());
    assert_eq!(value["preview"]["requested_cycles"], amount.to_string());
    assert!(value["preview"]["available_cycles"].is_null());
    assert!(
        f.journals() == before,
        "preview cannot consume an identity or change a receipt"
    );
    value["preview"]["blockers"].as_array().unwrap().clone()
}
fn transfer(
    f: &Fixture,
    id: u64,
    accept: u128,
    trap: bool,
) -> Result<Result<FundingObservation, FundingFailure>, ic_testkit::pocket_ic::RejectResponse> {
    f.harness
        .pic
        .update_call(
            f.sender,
            f.driver,
            "fund",
            candid::encode_one(FundingRequest {
                id,
                offered: 1_000_000,
                accept,
                reply: FundingReplyMode::Success,
                trap_callback: trap,
            })
            .unwrap(),
        )
        .map(|bytes| candid::decode_one(&bytes).unwrap())
}

#[test]
fn empty_journal_and_gross_cycles_cannot_supply_missing_funding_evidence() {
    let f = Fixture::new();
    for _ in 0..2 {
        let blockers = blocked(&f, u64::MAX, u128::MAX);
        for name in [
            "ProviderUnqualified",
            "RecoveryUnknown",
            "NotConfigured",
            "SpendabilityUnknown",
        ] {
            assert!(blockers.contains(&json_value(name)));
        }
        assert!(!blockers.contains(&json_value("FundingUncertain")));
        assert!(!blockers.contains(&json_value("AlreadyAdmitted")));
    }
    f.harness.pic.add_cycles(f.sender, 5_000_000_000_000);
    let blockers = blocked(&f, u64::MAX, 1);
    assert!(blockers.contains(&json_value("SpendabilityUnknown")));
    assert!(
        !blockers
            .iter()
            .any(|b| b.get("ReserveWouldBeViolated").is_some())
    );
}

#[test]
fn budget_revision_changes_on_admission_and_completion_even_for_full_refunds() {
    let f = Fixture::new();
    let before = args(&f, 3, 1);
    let initial = preview(&before).1["preview"]["budget"].clone();
    transfer(&f, 1, 0, false).unwrap().unwrap();
    let after = preview(&before).1;
    assert!(
        after["preview"]["blockers"]
            .as_array()
            .unwrap()
            .contains(&json_value("BudgetRevisionStale"))
    );
    assert_eq!(after["preview"]["budget"]["revision"], "2");
    assert_eq!(
        after["preview"]["budget"]["available"],
        initial["available"]
    );
    assert_eq!(after["preview"]["budget"]["refunded"], "1000000");
    let current = preview(&args(&f, 3, 1)).1;
    assert!(
        !current["preview"]["blockers"]
            .as_array()
            .unwrap()
            .contains(&json_value("BudgetRevisionStale"))
    );
    assert!(current["preview"]["available_cycles"].is_null());
}
fn json_value(value: &str) -> Value {
    Value::from(value)
}

#[test]
fn credit_uncertainty_and_identity_reuse_survive_later_refunds_and_restore() {
    let f = Fixture::new();
    transfer(&f, 1, 400_000, false).unwrap().unwrap();
    transfer(&f, 2, 0, false).unwrap().unwrap();
    let blockers = blocked(&f, 3, 1);
    assert!(blockers.contains(&json_value("FundingUncertain")));
    let reused = blocked(&f, 1, u128::MAX);
    assert!(reused.contains(&json_value("AlreadyAdmitted")));
    f.harness
        .pic
        .upgrade_canister(
            f.sender,
            std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).unwrap(),
            candid::encode_one(FundingUpgradeArgs {
                trap_after_restore: false,
            })
            .unwrap(),
            None,
        )
        .unwrap();
    let restored = blocked(&f, 1, 1);
    for name in [
        "AlreadyAdmitted",
        "FundingUncertain",
        "RecoveryFenced",
        "SpendabilityUnknown",
    ] {
        assert!(restored.contains(&json_value(name)));
    }
    assert!(!restored.contains(&json_value("RecoveryUnknown")));
}

#[test]
fn trapped_callback_keeps_the_original_identity_and_full_transfer_uncertain() {
    let f = Fixture::new();
    let failure = transfer(&f, 7, 500_000, true).unwrap_err();
    assert_eq!(
        failure.reject_code,
        ic_testkit::pocket_ic::RejectCode::CanisterError
    );
    for id in [7, 8] {
        let blockers = blocked(&f, id, 1);
        assert!(blockers.contains(&json_value("FundingUncertain")));
        assert_eq!(blockers.contains(&json_value("AlreadyAdmitted")), id == 7);
    }
}

#[test]
fn exhausted_no_transfer_history_still_blocks_new_identities() {
    let f = Fixture::new();
    for id in 0..16 {
        transfer(&f, id, 0, false).unwrap().unwrap();
    }
    let blockers = blocked(&f, 17, 1);
    assert!(blockers.contains(&json_value("JournalFull")));
    assert!(!blockers.contains(&json_value("FundingUncertain")));
    assert!(blockers.contains(&json_value("RecoveryUnknown")));
}

#[test]
fn wrong_binding_denied_callers_and_invalid_amounts_never_change_journals() {
    let f = Fixture::new();
    let before = f.journals();
    let request = FundingPreviewRequest {
        revision: 0,
        service: f.sender,
        peer: f.receiver,
        id: 1,
        requested_cycles: 1,
    };
    for (caller, input, expected) in [
        (
            Principal::anonymous(),
            request,
            FundingPreviewFailure::Denied,
        ),
        (
            f.driver,
            FundingPreviewRequest {
                service: f.receiver,
                ..request
            },
            FundingPreviewFailure::Binding,
        ),
        (
            f.driver,
            FundingPreviewRequest {
                peer: f.authority,
                ..request
            },
            FundingPreviewFailure::Binding,
        ),
        (
            f.driver,
            FundingPreviewRequest {
                requested_cycles: 0,
                ..request
            },
            FundingPreviewFailure::InvalidAmount,
        ),
    ] {
        let result: Result<FundingPreviewView, FundingPreviewFailure> = f
            .harness
            .pic
            .query_candid_as(f.sender, caller, "preview_funding", (input,))
            .unwrap();
        assert_eq!(result, Err(expected));
    }
    let mut denied = args(&f, 1, 1);
    let caller = denied.iter().position(|a| a == "--caller").unwrap() + 1;
    denied[caller] = Principal::anonymous().to_text();
    assert_eq!(preview(&denied).1["error"], "denied");
    for (flag, value) in [
        ("--amount", "0"),
        ("--id", "01"),
        ("--id", "18446744073709551616"),
    ] {
        let mut invalid = args(&f, 1, 1);
        let index = invalid.iter().position(|a| a == flag).unwrap() + 1;
        invalid[index] = value.into();
        assert_eq!(preview(&invalid).0, 2);
    }
    for tail in [
        ["--available-cycles", "100"],
        ["--reserve", "1"],
        ["--method", "fund"],
        ["--retry", "1"],
        ["--amount", "1"],
    ] {
        let mut invalid = args(&f, 1, 1);
        invalid.extend(tail.map(str::to_owned));
        assert_eq!(preview(&invalid).0, 2);
    }
    let mut invalid = args(&f, 1, 1);
    invalid[0] = "fund".into();
    assert_eq!(preview(&invalid).0, 2);
    assert!(
        f.journals() == before,
        "rejected previews must remain passive"
    );
}

#[test]
fn update_only_preview_method_cannot_trigger_a_transfer_fallback() {
    let f = Fixture::new();
    let pic = &f.harness.pic;
    let canister = pic.create_canister();
    pic.install_canister(
        canister,
        operator_method_mode::update_wasm("preview_funding", &[]),
        vec![],
        None,
    );
    let mut input = args(&f, 1, 1);
    let target = input.iter().position(|a| a == "--canister").unwrap() + 1;
    input[target] = canister.to_text();
    let (code, result) = preview(&input);
    assert_eq!(code, 3);
    assert_eq!(result["error"], "query_rejected");
    assert_eq!(pic.get_stable_memory(canister), Vec::<u8>::new());
}

#[test]
fn preview_reports_the_same_amount_bound_enforced_before_update_admission() {
    let f = Fixture::new();
    let before = f.journals();
    let blockers = blocked(&f, 1, u128::MAX);
    let maximum = blockers
        .iter()
        .find_map(|b| b.get("AmountLimitExceeded"))
        .unwrap()["maximum_cycles"]
        .as_str()
        .unwrap()
        .parse::<u128>()
        .unwrap();
    assert!(
        !blocked(&f, 1, maximum)
            .iter()
            .any(|b| b.get("AmountLimitExceeded").is_some())
    );
    assert!(
        blocked(&f, 1, maximum + 1)
            .iter()
            .any(|b| b.get("AmountLimitExceeded").is_some())
    );
    let result: Result<FundingObservation, FundingFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.sender,
            f.driver,
            "fund",
            (FundingRequest {
                id: 1,
                offered: maximum + 1,
                accept: 0,
                reply: FundingReplyMode::Success,
                trap_callback: true,
            },),
        )
        .unwrap();
    assert_eq!(result, Err(FundingFailure::Limit));
    assert!(
        f.journals() == before,
        "over-limit previews and updates cannot retain partial intents"
    );
}
