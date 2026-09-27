//! Actual query-only recovery of local evidence, not provider credit reconciliation.
use super::*;
use blob_test_protocol::funding::lookup::{
    FundingLookupFailure, FundingLookupRequest, FundingLookupState, FundingLookupView,
};
use ic_testkit::pocket_ic::common::rest::BlobCompression;

fn request() -> FundingRequest {
    FundingRequest {
        id: u64::MAX,
        offered: 1_000_000,
        accept: 400_000,
        reply: FundingReplyMode::InternalError,
        trap_callback: false,
    }
}
fn args(f: &Fixture, request: FundingRequest) -> Vec<String> {
    let mut args = f.args(
        f.sender,
        f.driver,
        "funding",
        "--peer",
        &f.receiver.to_text(),
    );
    args[0] = "lookup".into();
    args.extend([
        "--id".into(),
        request.id.to_string(),
        "--amount".into(),
        request.offered.to_string(),
        "--accept".into(),
        request.accept.to_string(),
        "--reply".into(),
        format!("{:?}", request.reply),
        "--trap-callback".into(),
        request.trap_callback.to_string(),
    ]);
    args
}
fn lookup(args: &[String]) -> (i32, Value) {
    let output = Command::new(env!("CARGO_BIN_EXE_blob-fixture-funding-lookup"))
        .args(args)
        .output()
        .unwrap();
    assert!(output.stderr.is_empty(), "lookup failures belong in JSON");
    (
        output.status.code().unwrap(),
        serde_json::from_slice(&output.stdout).unwrap(),
    )
}
fn unchanged(f: &Fixture, request: FundingRequest) -> (i32, Value) {
    let before = f.journals();
    let result = lookup(&args(f, request));
    assert!(
        f.journals() == before,
        "lookup cannot mutate either side of a transfer"
    );
    result
}
fn upgrade(f: &Fixture) {
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
}
fn submit_and_discard_reply(f: &Fixture, request: FundingRequest) {
    // The ingress completes, but its response is deliberately not decoded or used.
    // A subsequent query must recover retained evidence without invoking fund again.
    f.harness
        .pic
        .update_call(
            f.sender,
            f.driver,
            "fund",
            candid::encode_one(request).unwrap(),
        )
        .unwrap();
}

#[test]
fn lost_ingress_result_is_recovered_exactly_and_conflicting_inputs_cannot_read_it() {
    let f = Fixture::new();
    let original = request();
    let (code, absent) = unchanged(&f, original);
    assert_eq!(code, 4);
    assert_eq!(absent["lookup"]["state"], "Absent");
    submit_and_discard_reply(&f, original);
    let (code, result) = unchanged(&f, original);
    assert_eq!(code, 0);
    assert_eq!(result["method"], "lookup_funding");
    assert_eq!(result["mode"], "query");
    assert_eq!(result["lookup"]["state"], "Observed");
    assert_eq!(result["lookup"]["request"]["id"], u64::MAX.to_string());
    let observed = &result["lookup"]["observation"];
    assert_eq!(observed["transport_accepted"], "400000");
    assert_eq!(observed["refunded"], "600000");
    assert_eq!(observed["outcome"]["ProviderError"], "InternalError");
    assert_eq!(observed["reconciliation"]["kind"], "CreditRequired");
    assert_eq!(unchanged(&f, original).1, result);
    for changed in [
        FundingRequest {
            offered: 1_000_001,
            ..original
        },
        FundingRequest {
            accept: 0,
            ..original
        },
        FundingRequest {
            reply: FundingReplyMode::Success,
            ..original
        },
        FundingRequest {
            trap_callback: true,
            ..original
        },
    ] {
        let (code, failure) = unchanged(&f, changed);
        assert_eq!(code, 3);
        assert_eq!(failure["error"], "request_conflict");
    }
    upgrade(&f);
    let (code, frozen) = unchanged(&f, original);
    assert_eq!(code, 0);
    assert_eq!(frozen["lookup"]["fenced"], true);
    assert_eq!(
        frozen["lookup"]["observation"],
        result["lookup"]["observation"]
    );
}

#[test]
fn old_backup_absence_cannot_erase_a_receipt_or_authorize_another_transfer() {
    let f = Fixture::new();
    let old = f.harness.pic.get_stable_memory(f.sender);
    let original = request();
    submit_and_discard_reply(&f, original);
    let receiver = f.harness.pic.get_stable_memory(f.receiver);
    f.harness
        .pic
        .set_stable_memory(f.sender, old, BlobCompression::NoCompression);
    // Until an actual restore, diagnosis uses the active owner, not old stable bytes.
    assert_eq!(unchanged(&f, original).1["lookup"]["state"], "Observed");
    upgrade(&f);
    let (code, absent) = unchanged(&f, original);
    assert_eq!(code, 4);
    assert_eq!(absent["lookup"]["state"], "Absent");
    assert_eq!(absent["lookup"]["fenced"], true);
    assert!(absent["lookup"]["observation"].is_null());
    let result: Result<FundingObservation, FundingFailure> = f
        .harness
        .pic
        .update_candid_as(f.sender, f.driver, "fund", (original,))
        .unwrap();
    assert_eq!(result, Err(FundingFailure::Fenced));
    assert!(
        f.harness.pic.get_stable_memory(f.receiver) == receiver,
        "lookup cannot erase the receiver receipt"
    );
}

#[test]
fn callback_trap_remains_pending_through_lookup_and_restore() {
    let f = Fixture::new();
    let original = FundingRequest {
        trap_callback: true,
        ..request()
    };
    let failure = f
        .harness
        .pic
        .update_call(
            f.sender,
            f.driver,
            "fund",
            candid::encode_one(original).unwrap(),
        )
        .unwrap_err();
    assert_eq!(
        failure.reject_code,
        ic_testkit::pocket_ic::RejectCode::CanisterError
    );
    for restored in [false, true] {
        if restored {
            upgrade(&f);
        }
        let (code, result) = unchanged(&f, original);
        assert_eq!(code, 4);
        assert_eq!(result["lookup"]["state"], "Pending");
        assert_eq!(result["lookup"]["fenced"], restored);
        assert!(result["lookup"]["observation"].is_null());
    }
}

#[test]
fn live_lookup_observes_pending_then_original_completion_without_repeating_it() {
    let f = Fixture::new();
    let original = FundingRequest {
        reply: FundingReplyMode::DelayedSuccess,
        ..request()
    };
    let query = FundingLookupRequest {
        service: f.sender,
        peer: f.receiver,
        attempt: original,
    };
    let call = f
        .harness
        .pic
        .submit_call(
            f.sender,
            f.driver,
            "fund",
            candid::encode_one(original).unwrap(),
        )
        .unwrap();
    let mut pending = false;
    for _ in 0..20 {
        f.harness.pic.tick();
        let result: Result<FundingLookupView, FundingLookupFailure> = f
            .harness
            .pic
            .query_candid_as(f.sender, f.driver, "lookup_funding", (query,))
            .unwrap();
        if result.unwrap().state == FundingLookupState::Pending {
            pending = true;
            break;
        }
    }
    assert!(pending, "observe committed intent before completion");
    let (code, result) = unchanged(&f, original);
    assert_eq!(code, 4);
    assert_eq!(result["lookup"]["state"], "Pending");
    f.harness.pic.await_call(call).unwrap();
    let (code, result) = unchanged(&f, original);
    assert_eq!(code, 0);
    assert_eq!(
        result["lookup"]["observation"]["transport_accepted"],
        "400000"
    );
}

#[test]
fn lookup_denies_wrong_callers_and_bindings_and_never_falls_back_to_update() {
    let f = Fixture::new();
    let before = f.journals();
    for (flag, value, error) in [
        ("--caller", Principal::anonymous().to_text(), "denied"),
        ("--peer", Fake::principal(99).to_text(), "binding_mismatch"),
        ("--amount", "10000000000001".into(), "invalid_request"),
    ] {
        let mut args = args(&f, request());
        let i = args.iter().position(|a| a == flag).unwrap();
        args[i + 1] = value;
        let (code, result) = lookup(&args);
        assert_eq!(code, 3);
        assert_eq!(result["error"], error);
    }
    assert!(
        f.journals() == before,
        "failed lookups preserve every journal"
    );
    let pic = &f.harness.pic;
    let canister = pic.create_canister();
    pic.install_canister(
        canister,
        super::operator_method_mode::update_wasm("lookup_funding", &[]),
        vec![],
        None,
    );
    let mut args = args(&f, request());
    let i = args.iter().position(|a| a == "--canister").unwrap();
    args[i + 1] = canister.to_text();
    let (code, result) = lookup(&args);
    assert_eq!(code, 3);
    assert_eq!(result["error"], "query_rejected");
    assert!(pic.get_stable_memory(canister).is_empty());
    pic.update_call(canister, f.driver, "lookup_funding", vec![])
        .unwrap();
    assert_eq!(pic.get_stable_memory(canister).len(), 65_536);
}
