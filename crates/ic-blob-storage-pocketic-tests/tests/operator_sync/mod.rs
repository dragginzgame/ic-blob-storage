//! Actual sync commands with a controlled source; simulated caller is explicit.
use super::*;
use blob_test_protocol::status::OperatorStatusView;
use blob_test_protocol::{GatewaySyncRequest, SourceMode, SourceObservation, SyncFailure};

fn request(f: &Fixture) -> GatewaySyncRequest {
    sync_request::read(&f.harness.pic, f.authority, f.gateway)
}
fn args(f: &Fixture, mode: &str, request: GatewaySyncRequest) -> Vec<String> {
    let mut args = f.args(
        request.service,
        f.gateway,
        "authority",
        "--namespace",
        &request.namespace.to_string(),
    );
    args[0] = mode.into();
    for (flag, value) in [
        ("--source", request.source.to_text()),
        ("--revision", request.revision.to_string()),
        ("--sequence", request.sequence.to_string()),
    ] {
        args.extend([flag.into(), value]);
    }
    args
}
fn sync_command(args: &[String]) -> (i32, Value) {
    let output = Command::new(env!("CARGO_BIN_EXE_blob-fixture-sync"))
        .args(args)
        .output()
        .unwrap();
    assert!(output.stderr.is_empty(), "errors belong in JSON");
    (
        output.status.code().unwrap(),
        serde_json::from_slice(&output.stdout).unwrap(),
    )
}
fn status(f: &Fixture) -> OperatorStatusView {
    f.harness
        .pic
        .query_candid_as::<Option<OperatorStatusView>, _>(
            f.authority,
            f.gateway,
            "operator_status",
            (),
        )
        .unwrap()
        .unwrap()
}
fn observed(f: &Fixture) -> SourceObservation {
    f.harness
        .pic
        .query_candid_as::<Option<SourceObservation>, _>(f.gateway, f.driver, "observation", ())
        .unwrap()
        .unwrap()
}
fn configure(f: &Fixture, mode: SourceMode) {
    let result: bool = f
        .harness
        .pic
        .update_candid_as(f.gateway, f.driver, "configure", (mode,))
        .unwrap();
    assert!(result);
}
fn revoke(f: &Fixture) {
    let result: bool = f
        .harness
        .pic
        .update_candid_as(f.authority, f.gateway, "revoke_gateway", ())
        .unwrap();
    assert!(result);
}

#[test]
fn preview_and_consumed_sequence_cannot_undo_later_operator_decisions() {
    let f = Fixture::with_source_operator(true);
    let first = request(&f);
    let before = f.journals();
    assert_eq!(
        sync_command(&args(&f, "dry-run", first)).1["action"]["outcome"],
        "eligible"
    );
    assert!(f.journals() == before, "preview must preserve journals");
    revoke(&f);
    let absent = request(&f);
    assert_eq!(sync_command(&args(&f, "dry-run", absent)).0, 0);
    revoke(&f); // Absent member revocation is still a new operator decision.
    let revoked = f.journals();
    for old in [first, absent] {
        let (code, result) = sync_command(&args(&f, "sync", old));
        assert_eq!(code, 5);
        assert_eq!(result["action"]["failure"], "Stale");
    }
    assert!(
        f.journals() == revoked,
        "stale commands must preserve revocation"
    );
    assert_eq!(observed(&f).requests, 0);
    assert_eq!(status(&f).gateways, []);
    let current = request(&f);
    let (code, result) = sync_command(&args(&f, "sync", current));
    assert_eq!(code, 0);
    assert_eq!(result["action"]["outcome"], "completed");
    assert_eq!(result["post_status"]["status"]["last_sync"], "1");
    assert_eq!(result["post_status"]["status"]["sync_revision"], "2");
    assert_eq!(status(&f).gateways, vec![f.gateway]);
    let completed = f.journals();
    for mode in ["dry-run", "sync"] {
        assert_eq!(
            sync_command(&args(&f, mode, current)).1["action"]["failure"],
            "Stale"
        );
        assert!(
            f.journals() == completed,
            "consumed sequence cannot dispatch again"
        );
    }
    assert_eq!(observed(&f).requests, 1);
}

#[test]
fn failed_source_reply_consumes_sequence_without_changing_membership_or_retrying() {
    let f = Fixture::with_source_operator(true);
    for (mode, error) in [
        (SourceMode::Malformed, "InvalidReply"),
        (SourceMode::Oversized, "ReplyTooLarge"),
        (SourceMode::Reject, "Transport"),
    ] {
        configure(&f, mode);
        let r = request(&f);
        let count = observed(&f).requests;
        let (code, result) = sync_command(&args(&f, "sync", r));
        assert_eq!(code, 5);
        assert_eq!(result["action"]["failure"], error);
        assert_eq!(status(&f).gateways, vec![f.gateway]);
        assert_eq!(status(&f).pending_sync, None);
        assert_eq!(observed(&f).requests, count + 1);
        assert_eq!(
            sync_command(&args(&f, "sync", r)).1["action"]["failure"],
            "Stale"
        );
        assert_eq!(observed(&f).requests, count + 1);
    }
}

#[test]
fn held_reply_cannot_override_revocation_and_overlap_does_not_dispatch() {
    let f = Fixture::with_source_operator(true);
    configure(&f, SourceMode::Hold);
    let r = request(&f);
    let id = f
        .harness
        .pic
        .submit_call(
            f.authority,
            f.gateway,
            "sync_gateway",
            candid::encode_one(r).unwrap(),
        )
        .unwrap();
    for _ in 0..30 {
        f.harness.pic.tick();
        if observed(&f).requests == 1 {
            break;
        }
    }
    assert_eq!(observed(&f).requests, 1);
    for mode in ["dry-run", "sync"] {
        assert_eq!(
            sync_command(&args(&f, mode, r)).1["action"]["failure"],
            "InProgress"
        );
    }
    revoke(&f);
    let revised = status(&f).sync_revision;
    let resumed: bool = f
        .harness
        .pic
        .update_candid_as(f.gateway, f.driver, "resume_sync", ())
        .unwrap();
    assert!(resumed);
    let completed: Result<(), SyncFailure> =
        candid::decode_one(&f.harness.pic.await_call(id).unwrap()).unwrap();
    assert_eq!(completed, Err(SyncFailure::Stale));
    assert_eq!(observed(&f).requests, 1);
    assert_eq!(status(&f).gateways, []);
    assert_eq!(status(&f).sync_revision, revised);
}

#[test]
fn forced_restore_during_sync_retains_pending_evidence_when_callback_arrives() {
    let f = Fixture::with_source_operator(true);
    configure(&f, SourceMode::Hold);
    let r = request(&f);
    let id = f
        .harness
        .pic
        .submit_call(
            f.authority,
            f.gateway,
            "sync_gateway",
            candid::encode_one(r).unwrap(),
        )
        .unwrap();
    for _ in 0..30 {
        f.harness.pic.tick();
        if observed(&f).requests == 1 {
            break;
        }
    }
    assert_eq!(observed(&f).requests, 1);
    assert_eq!(status(&f).pending_sync, Some(r.sequence));
    f.upgrade_fixture(f.authority, true);
    let mut retained = status(&f);
    assert!(retained.fenced);
    let resumed: bool = f
        .harness
        .pic
        .update_candid_as(f.gateway, f.driver, "resume_sync", ())
        .unwrap();
    assert!(resumed);
    // Actual upgrade clears the async heap: the late callback can trap before
    // entering Rust. It cannot apply membership or clear the frozen journal.
    let failure = f.harness.pic.await_call(id).unwrap_err();
    assert_eq!(
        failure.reject_code,
        ic_testkit::pocket_ic::RejectCode::CanisterError
    );
    assert_eq!(status(&f), retained);
    assert_eq!(
        sync_command(&args(&f, "sync", r)).1["action"]["failure"],
        "Fenced"
    );
    retained = status(&f);
    assert_eq!(retained.pending_sync, Some(r.sequence));
    assert_eq!(observed(&f).requests, 1);
}

#[test]
fn bindings_caller_and_restoration_are_checked_before_source_calls() {
    let f = Fixture::with_source_operator(true);
    let r = request(&f);
    let before = f.journals();
    for wrong in [
        GatewaySyncRequest {
            source: f.receiver,
            ..r
        },
        GatewaySyncRequest { namespace: 2, ..r },
    ] {
        assert_eq!(
            sync_command(&args(&f, "sync", wrong)).1["action"]["failure"],
            "Binding"
        );
    }
    let mut denied = args(&f, "sync", r);
    let caller = denied.iter().position(|a| a == "--caller").unwrap() + 1;
    denied[caller] = Principal::anonymous().to_text();
    assert_eq!(sync_command(&denied).1["action"]["failure"], "Denied");
    assert!(f.journals() == before, "rejections cannot change journals");
    f.harness
        .pic
        .upgrade_canister(
            f.authority,
            std::fs::read(fixture_path("BLOB_AUTHORITY_PROBE_WASM")).unwrap(),
            candid::encode_args(()).unwrap(),
            None,
        )
        .unwrap();
    let restored = f.journals();
    for mode in ["dry-run", "sync"] {
        assert_eq!(
            sync_command(&args(&f, mode, r)).1["action"]["failure"],
            "Fenced"
        );
        assert!(
            f.journals() == restored,
            "restored authority must stay frozen"
        );
    }
    assert_eq!(observed(&f).requests, 0);
}

#[test]
fn sync_result_survives_failed_post_status_and_preview_stays_query_only() {
    let f = Fixture::with_source_operator(true);
    let pic = &f.harness.pic;
    let r = request(&f);
    for (reply, code, outcome) in [
        (
            candid::encode_one(Ok::<(), SyncFailure>(())).unwrap(),
            7,
            "completed",
        ),
        (vec![0], 6, "uncertain"),
    ] {
        let service = pic.create_canister();
        pic.install_canister(
            service,
            operator_method_mode::update_wasm("sync_gateway", &reply),
            vec![],
            None,
        );
        let (exit, result) = sync_command(&args(&f, "sync", GatewaySyncRequest { service, ..r }));
        assert_eq!(exit, code);
        assert_eq!(result["action"]["outcome"], outcome);
        assert_eq!(result["post_status"]["error"], "query_rejected");
        assert_eq!(pic.get_stable_memory(service).len(), 65_536);
    }
    let service = pic.create_canister();
    pic.install_canister(
        service,
        operator_method_mode::update_wasm("preview_gateway_sync", &[]),
        vec![],
        None,
    );
    assert_eq!(
        sync_command(&args(&f, "dry-run", GatewaySyncRequest { service, ..r })).1["action"]["error"],
        "query_rejected"
    );
    assert_eq!(pic.get_stable_memory(service), Vec::<u8>::new());
    let mut invalid = args(&f, "sync", r);
    invalid.extend(["--account".into(), f.receiver.to_text()]);
    assert_eq!(sync_command(&invalid).0, 2);
    assert_eq!(observed(&f).requests, 0);
}
