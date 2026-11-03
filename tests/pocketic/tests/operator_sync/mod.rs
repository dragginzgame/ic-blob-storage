//! Typed sync calls with a controlled source; simulated caller is explicit.
use super::*;
use blob_test_protocol::status::OperatorStatusView;
use blob_test_protocol::{GatewaySyncRequest, SourceMode, SourceObservation, SyncFailure};
use ic_blob_storage_pocketic_tests::fixture_calls;

fn request(f: &Fixture) -> GatewaySyncRequest {
    sync_request::read(&f.harness.pic, f.authority, f.gateway)
}
fn sync(f: &Fixture, preview: bool, input: GatewaySyncRequest) -> Result<(), SyncFailure> {
    let call = if preview {
        fixture_calls::query_fixture
    } else {
        fixture_calls::update_fixture
    };
    let (reply,): (Result<(), SyncFailure>,) = call(
        &f.harness.pic,
        input.service,
        f.gateway,
        if preview {
            "preview_gateway_sync"
        } else {
            "sync_gateway"
        },
        (input,),
    )
    .unwrap();
    reply
}
fn status(f: &Fixture) -> OperatorStatusView {
    fixture_calls::query_fixture::<(Option<OperatorStatusView>,), _>(
        &f.harness.pic,
        f.authority,
        f.gateway,
        "operator_status",
        (),
    )
    .map(|(reply,)| reply)
    .unwrap()
    .unwrap()
}
fn observed(f: &Fixture) -> SourceObservation {
    fixture_calls::query_fixture::<(Option<SourceObservation>,), _>(
        &f.harness.pic,
        f.gateway,
        f.driver,
        "observation",
        (),
    )
    .map(|(reply,)| reply)
    .unwrap()
    .unwrap()
}
fn configure(f: &Fixture, mode: SourceMode) {
    let result: bool =
        fixture_calls::update_fixture(&f.harness.pic, f.gateway, f.driver, "configure", (mode,))
            .map(|(reply,)| reply)
            .unwrap();
    assert!(result);
}
fn revoke(f: &Fixture) {
    let result: bool =
        fixture_calls::update_fixture(&f.harness.pic, f.authority, f.gateway, "revoke_gateway", ())
            .map(|(reply,)| reply)
            .unwrap();
    assert!(result);
}

#[test]
fn preview_and_consumed_sequence_cannot_undo_later_operator_decisions() {
    let f = Fixture::with_source_operator(true);
    let first = request(&f);
    let before = f.journals();
    assert_eq!(sync(&f, true, first), Ok(()));
    assert!(f.journals() == before, "preview must preserve journals");
    revoke(&f);
    let absent = request(&f);
    assert_eq!(sync(&f, true, absent), Ok(()));
    revoke(&f); // Absent member revocation is still a new operator decision.
    let revoked = f.journals();
    for old in [first, absent] {
        assert_eq!(sync(&f, false, old), Err(SyncFailure::Stale));
    }
    assert!(
        f.journals() == revoked,
        "stale commands must preserve revocation"
    );
    assert_eq!(observed(&f).requests, 0);
    assert_eq!(status(&f).gateways, []);
    let current = request(&f);
    assert_eq!(sync(&f, false, current), Ok(()));
    assert_eq!(status(&f).last_sync, 1);
    assert_eq!(status(&f).sync_revision, Some(2));
    assert_eq!(status(&f).gateways, vec![f.gateway]);
    let completed = f.journals();
    for preview in [true, false] {
        assert_eq!(sync(&f, preview, current), Err(SyncFailure::Stale));
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
        (SourceMode::Malformed, SyncFailure::InvalidReply),
        (SourceMode::Oversized, SyncFailure::ReplyTooLarge),
        (SourceMode::Reject, SyncFailure::Transport),
    ] {
        configure(&f, mode);
        let r = request(&f);
        let count = observed(&f).requests;
        assert_eq!(sync(&f, false, r), Err(error));
        assert_eq!(status(&f).gateways, vec![f.gateway]);
        assert_eq!(status(&f).pending_sync, None);
        assert_eq!(observed(&f).requests, count + 1);
        assert_eq!(sync(&f, false, r), Err(SyncFailure::Stale));
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
            blob_test_protocol::fixture_wire::encode_one("sync_gateway", r).unwrap(),
        )
        .unwrap();
    for _ in 0..30 {
        f.harness.pic.tick().unwrap();
        if observed(&f).requests == 1 {
            break;
        }
    }
    assert_eq!(observed(&f).requests, 1);
    for preview in [true, false] {
        assert_eq!(sync(&f, preview, r), Err(SyncFailure::InProgress));
    }
    revoke(&f);
    let revised = status(&f).sync_revision;
    let resumed: bool =
        fixture_calls::update_fixture(&f.harness.pic, f.gateway, f.driver, "resume_sync", ())
            .map(|(reply,)| reply)
            .unwrap();
    assert!(resumed);
    let completed: Result<(), SyncFailure> = blob_test_protocol::fixture_wire::decode_one(
        "sync_gateway",
        &f.harness.pic.await_call(id).unwrap(),
        blob_test_protocol::fixture_wire::MAX_BYTES,
    )
    .unwrap();
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
            blob_test_protocol::fixture_wire::encode_one("sync_gateway", r).unwrap(),
        )
        .unwrap();
    for _ in 0..30 {
        f.harness.pic.tick().unwrap();
        if observed(&f).requests == 1 {
            break;
        }
    }
    assert_eq!(observed(&f).requests, 1);
    assert_eq!(status(&f).pending_sync, Some(r.sequence));
    f.upgrade_fixture(f.authority, true);
    let mut retained = status(&f);
    assert!(retained.fenced);
    let resumed: bool =
        fixture_calls::update_fixture(&f.harness.pic, f.gateway, f.driver, "resume_sync", ())
            .map(|(reply,)| reply)
            .unwrap();
    assert!(resumed);
    // Actual upgrade clears the async heap: the late callback can trap before
    // entering Rust. It cannot apply membership or clear the frozen journal.
    let failure = f.harness.pic.await_call(id).unwrap_err();
    assert_eq!(
        crate::canister_rejection(&failure).reject_code,
        ic_platform::pocket_ic::RejectCode::CanisterError
    );
    assert_eq!(status(&f), retained);
    assert_eq!(sync(&f, false, r), Err(SyncFailure::Fenced));
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
        assert_eq!(sync(&f, false, wrong), Err(SyncFailure::Binding));
    }
    let (denied,): (Result<(), SyncFailure>,) = fixture_calls::update_fixture(
        &f.harness.pic,
        f.authority,
        Principal::anonymous(),
        "sync_gateway",
        (r,),
    )
    .unwrap();
    assert_eq!(denied, Err(SyncFailure::Denied));
    assert!(f.journals() == before, "rejections cannot change journals");
    f.harness
        .pic
        .upgrade_canister(
            f.authority,
            std::fs::read(fixture_path("BLOB_AUTHORITY_PROBE_WASM")).unwrap(),
            blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
            None,
        )
        .unwrap();
    let restored = f.journals();
    for preview in [true, false] {
        assert_eq!(sync(&f, preview, r), Err(SyncFailure::Fenced));
        assert!(
            f.journals() == restored,
            "restored authority must stay frozen"
        );
    }
    assert_eq!(observed(&f).requests, 0);
}

#[test]
fn acknowledged_and_malformed_updates_remain_distinct_when_status_is_unavailable() {
    let f = Fixture::with_source_operator(true);
    let pic = &f.harness.pic;
    let r = request(&f);
    for acknowledged in [true, false] {
        let reply = if acknowledged {
            blob_test_protocol::fixture_wire::encode_reply(
                "sync_gateway",
                Ok::<(), SyncFailure>(()),
            )
            .unwrap()
        } else {
            vec![0]
        };
        let service = pic.create_canister().unwrap();
        pic.install_canister(
            service,
            operator_method_mode::update_wasm("sync_gateway", &reply),
            vec![],
            None,
        )
        .unwrap();
        let result = fixture_calls::update_fixture::<(Result<(), SyncFailure>,), _>(
            pic,
            service,
            f.gateway,
            "sync_gateway",
            (GatewaySyncRequest { service, ..r },),
        );
        if acknowledged {
            assert_eq!(result.unwrap().0, Ok(()));
        } else {
            assert!(
                result.is_err(),
                "malformed acknowledgement leaves the executed effect uncertain"
            );
        }
        assert!(
            fixture_calls::query_fixture::<(Option<OperatorStatusView>,), _>(
                pic,
                service,
                f.gateway,
                "operator_status",
                ()
            )
            .is_err()
        );
        assert_eq!(pic.get_stable_memory(service).unwrap().len(), 65_536);
    }
    let service = pic.create_canister().unwrap();
    pic.install_canister(
        service,
        operator_method_mode::update_wasm("preview_gateway_sync", &[]),
        vec![],
        None,
    )
    .unwrap();
    assert!(
        fixture_calls::query_fixture::<(Result<(), SyncFailure>,), _>(
            pic,
            service,
            f.gateway,
            "preview_gateway_sync",
            (GatewaySyncRequest { service, ..r },)
        )
        .is_err()
    );
    assert_eq!(pic.get_stable_memory(service).unwrap(), Vec::<u8>::new());
    assert_eq!(observed(&f).requests, 0);
}
