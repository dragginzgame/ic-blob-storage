use super::*;

fn args(f: &Fixture, mode: &str, request: BalanceRefreshRequest) -> Vec<String> {
    let mut args = f.args(
        request.scope.service,
        f.driver,
        "authority",
        "--namespace",
        &request.scope.namespace.to_string(),
    );
    args[0] = mode.into();
    for (flag, value) in [
        ("--source", request.scope.source.to_text()),
        ("--account", request.scope.account.to_text()),
        ("--revision", request.revision.to_string()),
        ("--sequence", request.sequence.to_string()),
    ] {
        args.extend([flag.into(), value]);
    }
    args
}

fn refresh_command(args: &[String]) -> (i32, Value) {
    let output = Command::new(env!("CARGO_BIN_EXE_blob-fixture-refresh"))
        .args(args)
        .output()
        .unwrap();
    assert!(output.stderr.is_empty(), "command errors belong in JSON");
    (
        output.status.code().unwrap(),
        serde_json::from_slice(&output.stdout).unwrap(),
    )
}

#[test]
fn dry_run_is_passive_and_consumed_refresh_cannot_dispatch_twice() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    f.source_balance(bytes("success"), false, false);
    let request = f.refresh_request();
    let before = f.journals();
    for _ in 0..2 {
        let (code, result) = refresh_command(&args(&f, "dry-run", request));
        assert_eq!(code, 0);
        assert_eq!(result["action"]["outcome"], "eligible");
        assert_eq!(result["mode"], "query");
        assert!(result["post_status"].is_null());
        assert!(f.journals() == before, "journals must remain unchanged");
    }
    let (code, result) = refresh_command(&args(&f, "refresh", request));
    assert_eq!(code, 0);
    assert_eq!(result["action"]["outcome"], "completed");
    assert_eq!(
        result["post_status"]["status"]["balance_observation"]["usability"],
        "Observed"
    );
    assert_eq!(f.balance_source_status().requests, 1);
    let completed = f.journals();
    for mode in ["dry-run", "refresh"] {
        let (code, result) = refresh_command(&args(&f, mode, request));
        assert_eq!(code, 5);
        assert_eq!(result["action"]["failure"], "Stale");
        assert!(
            f.journals() == completed,
            "consumed request must preserve journals"
        );
    }
    assert_eq!(f.balance_source_status().requests, 1);
    assert_eq!(f.balance_status().balance_observation.attempts.len(), 1);
}

#[test]
fn preview_cannot_authorize_changed_bindings_or_another_callers_refresh() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    f.source_balance(bytes("success"), false, false);
    let request = f.refresh_request();
    assert_eq!(refresh_command(&args(&f, "dry-run", request)).0, 0);
    // The same binding at a new revision still invalidates the preview.
    f.configure_balance(f.balance_scope()).unwrap();
    let before = f.journals();
    assert_eq!(
        refresh_command(&args(&f, "refresh", request)).1["action"]["failure"],
        "Stale"
    );
    let current = f.refresh_request();
    let mut wrong = current;
    wrong.scope.account = Fake::principal(30);
    assert_eq!(
        refresh_command(&args(&f, "refresh", wrong)).1["action"]["failure"],
        "Binding"
    );
    wrong = current;
    wrong.scope.namespace = 2;
    assert_eq!(
        refresh_command(&args(&f, "refresh", wrong)).1["action"]["failure"],
        "Binding"
    );
    let mut denied = args(&f, "refresh", current);
    let caller = denied.iter().position(|a| a == "--caller").unwrap() + 1;
    denied[caller] = Principal::anonymous().to_text();
    let (code, result) = refresh_command(&denied);
    assert_eq!(code, 5);
    assert_eq!(result["action"]["failure"], "Denied");
    assert_eq!(result["post_status"]["error"], "denied");
    assert!(f.journals() == before, "journals must remain unchanged");
    assert_eq!(f.balance_source_status().requests, 0);
}

#[test]
fn failed_observation_consumes_identity_and_restoration_stays_fenced() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    f.source_balance(bytes("negative-ledger"), false, false);
    let request = f.refresh_request();
    let (code, result) = refresh_command(&args(&f, "refresh", request));
    assert_eq!(code, 5);
    assert_eq!(result["action"]["failure"], "Malformed");
    assert_eq!(f.balance_source_status().requests, 1);
    assert_eq!(
        refresh_command(&args(&f, "refresh", request)).1["action"]["failure"],
        "Stale"
    );
    f.upgrade_fixture(f.authority, false);
    let before = f.journals();
    for mode in ["dry-run", "refresh"] {
        assert_eq!(
            refresh_command(&args(&f, mode, f.refresh_request())).1["action"]["failure"],
            "Fenced"
        );
        assert!(f.journals() == before, "journals must remain unchanged");
    }
    assert_eq!(f.balance_source_status().requests, 1);
}

#[test]
fn completion_survives_failed_post_status_without_repeating_update() {
    let f = Fixture::new();
    let pic = &f.harness.pic;
    let canister = pic.create_canister();
    let reply = candid::encode_one(Ok::<(), BalanceFailure>(())).unwrap();
    pic.install_canister(
        canister,
        super::super::operator_method_mode::update_wasm("refresh_balance", &reply),
        vec![],
        None,
    );
    let mut request = BalanceRefreshRequest {
        scope: f.balance_scope(),
        revision: 1,
        sequence: 1,
    };
    request.scope.service = canister;
    let (code, result) = refresh_command(&args(&f, "refresh", request));
    assert_eq!(code, 7);
    assert_eq!(result["action"]["outcome"], "completed");
    assert_eq!(result["post_status"]["error"], "query_rejected");
    assert_eq!(pic.get_stable_memory(canister).len(), 65_536);
    // Malformed acknowledgement keeps the action uncertain even though it ran.
    let malformed = pic.create_canister();
    pic.install_canister(
        malformed,
        super::super::operator_method_mode::update_wasm("refresh_balance", b"bad"),
        vec![],
        None,
    );
    request.scope.service = malformed;
    let (code, result) = refresh_command(&args(&f, "refresh", request));
    assert_eq!(code, 6);
    assert_eq!(result["action"]["outcome"], "uncertain");
    assert_eq!(result["action"]["error"], "invalid_reply");
    assert_eq!(pic.get_stable_memory(malformed).len(), 65_536);

    // A preview method exported only as an update must never run on dry-run.
    let update_only = pic.create_canister();
    pic.install_canister(
        update_only,
        super::super::operator_method_mode::update_wasm("preview_balance_refresh", &reply),
        vec![],
        None,
    );
    request.scope.service = update_only;
    let (code, result) = refresh_command(&args(&f, "dry-run", request));
    assert_eq!(code, 6);
    assert_eq!(result["action"]["error"], "query_rejected");
    assert!(pic.get_stable_memory(update_only).is_empty());
}

#[test]
fn pending_read_blocks_preview_and_refresh_without_dispatching_another_call() {
    let f = Fixture::new();
    f.configure_balance(f.balance_scope()).unwrap();
    let request = f.refresh_request();
    let held = f.hold_balance();
    let pending = f.journals();
    for mode in ["dry-run", "refresh"] {
        let (code, result) = refresh_command(&args(&f, mode, request));
        assert_eq!(code, 5);
        assert_eq!(result["action"]["failure"], "Busy");
        assert!(
            f.journals() == pending,
            "rejection must retain the in-flight intent"
        );
    }
    f.resume_balance(held).unwrap();
    assert_eq!(
        refresh_command(&args(&f, "refresh", request)).1["action"]["failure"],
        "Stale"
    );
    assert_eq!(f.balance_source_status().requests, 1);
}

#[test]
fn invalid_action_arguments_are_rejected_before_transport() {
    let f = Fixture::new();
    let request = BalanceRefreshRequest {
        scope: f.balance_scope(),
        revision: 1,
        sequence: 1,
    };
    let base = args(&f, "refresh", request);
    for (flag, value) in [
        ("--revision", "0"),
        ("--sequence", "01"),
        ("--source", "aaaaa-aa"),
        ("--account", "2vxsx-fae"),
        ("--namespace", "0"),
    ] {
        let mut input = base.clone();
        let index = input.iter().position(|a| a == flag).unwrap() + 1;
        input[index] = value.into();
        assert_eq!(refresh_command(&input).0, 2);
    }
    for extra in [["--sequence", "1"], ["--method", "fund"], ["--retry", "1"]] {
        let mut input = base.clone();
        input.extend(extra.map(str::to_owned));
        assert_eq!(refresh_command(&input).0, 2);
    }
    assert_eq!(f.balance_source_status().requests, 0);
}
