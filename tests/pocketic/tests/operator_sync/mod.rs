//! Typed sync calls with a controlled source; simulated caller is explicit.
use super::*;
use blob_test_protocol::status::OperatorStatusView;
use blob_test_protocol::{GatewaySyncRequest, SourceMode, SourceObservation, SyncFailure};
use ic_blob_storage_pocketic_tests::fixture_calls;

async fn request(f: &Fixture) -> GatewaySyncRequest {
    sync_request::read(&f.harness.pic, f.authority, f.gateway).await
}
async fn sync(f: &Fixture, preview: bool, input: GatewaySyncRequest) -> Result<(), SyncFailure> {
    let result = if preview {
        fixture_calls::query_fixture(
            &f.harness.pic,
            input.service,
            f.gateway,
            "preview_gateway_sync",
            (input,),
        )
        .await
    } else {
        fixture_calls::update_fixture(
            &f.harness.pic,
            input.service,
            f.gateway,
            "sync_gateway",
            (input,),
        )
        .await
    };
    let (reply,): (Result<(), SyncFailure>,) = result.unwrap();
    reply
}

async fn status(f: &Fixture) -> OperatorStatusView {
    fixture_calls::query_fixture::<(Option<OperatorStatusView>,), _>(
        &f.harness.pic,
        f.authority,
        f.gateway,
        "operator_status",
        (),
    )
    .await
    .map(|(reply,)| reply)
    .unwrap()
    .unwrap()
}
async fn observed(f: &Fixture) -> SourceObservation {
    fixture_calls::query_fixture::<(Option<SourceObservation>,), _>(
        &f.harness.pic,
        f.gateway,
        f.driver,
        "observation",
        (),
    )
    .await
    .map(|(reply,)| reply)
    .unwrap()
    .unwrap()
}
async fn configure(f: &Fixture, mode: SourceMode) {
    let result: bool =
        fixture_calls::update_fixture(&f.harness.pic, f.gateway, f.driver, "configure", (mode,))
            .await
            .map(|(reply,)| reply)
            .unwrap();
    assert!(result);
}
async fn revoke(f: &Fixture) {
    let result: bool =
        fixture_calls::update_fixture(&f.harness.pic, f.authority, f.gateway, "revoke_gateway", ())
            .await
            .map(|(reply,)| reply)
            .unwrap();
    assert!(result);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn preview_and_consumed_sequence_cannot_undo_later_operator_decisions() {
    let f = Fixture::with_source_operator(true).await;
    let first = request(&f).await;
    let before = f.journals().await;
    assert_eq!(sync(&f, true, first).await, Ok(()));
    assert!(
        f.journals().await == before,
        "preview must preserve journals"
    );
    revoke(&f).await;
    let absent = request(&f).await;
    assert_eq!(sync(&f, true, absent).await, Ok(()));
    revoke(&f).await; // Absent member revocation is still a new operator decision.
    let revoked = f.journals().await;
    for old in [first, absent] {
        assert_eq!(sync(&f, false, old).await, Err(SyncFailure::Stale));
    }
    assert!(
        f.journals().await == revoked,
        "stale commands must preserve revocation"
    );
    assert_eq!(observed(&f).await.requests, 0);
    assert_eq!(status(&f).await.gateways, []);
    let current = request(&f).await;
    assert_eq!(sync(&f, false, current).await, Ok(()));
    assert_eq!(status(&f).await.last_sync, 1);
    assert_eq!(status(&f).await.sync_revision, Some(2));
    assert_eq!(status(&f).await.gateways, vec![f.gateway]);
    let completed = f.journals().await;
    for preview in [true, false] {
        assert_eq!(sync(&f, preview, current).await, Err(SyncFailure::Stale));
        assert!(
            f.journals().await == completed,
            "consumed sequence cannot dispatch again"
        );
    }
    assert_eq!(observed(&f).await.requests, 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn failed_source_reply_consumes_sequence_without_changing_membership_or_retrying() {
    let f = Fixture::with_source_operator(true).await;
    for (mode, error) in [
        (SourceMode::Malformed, SyncFailure::InvalidReply),
        (SourceMode::Oversized, SyncFailure::ReplyTooLarge),
        (SourceMode::Reject, SyncFailure::Transport),
    ] {
        configure(&f, mode).await;
        let r = request(&f).await;
        let count = observed(&f).await.requests;
        assert_eq!(sync(&f, false, r).await, Err(error));
        assert_eq!(status(&f).await.gateways, vec![f.gateway]);
        assert_eq!(status(&f).await.pending_sync, None);
        assert_eq!(observed(&f).await.requests, count + 1);
        assert_eq!(sync(&f, false, r).await, Err(SyncFailure::Stale));
        assert_eq!(observed(&f).await.requests, count + 1);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn held_reply_cannot_override_revocation_and_overlap_does_not_dispatch() {
    let f = Fixture::with_source_operator(true).await;
    configure(&f, SourceMode::Hold).await;
    let r = request(&f).await;
    let id = f
        .harness
        .pic
        .submit_call(
            f.authority,
            f.gateway,
            "sync_gateway",
            blob_test_protocol::fixture_wire::encode_one("sync_gateway", r).unwrap(),
        )
        .await
        .unwrap();
    for _ in 0..30 {
        f.harness.pic.tick().await.unwrap();
        if observed(&f).await.requests == 1 {
            break;
        }
    }
    assert_eq!(observed(&f).await.requests, 1);
    for preview in [true, false] {
        assert_eq!(sync(&f, preview, r).await, Err(SyncFailure::InProgress));
    }
    revoke(&f).await;
    let revised = status(&f).await.sync_revision;
    let resumed: bool =
        fixture_calls::update_fixture(&f.harness.pic, f.gateway, f.driver, "resume_sync", ())
            .await
            .map(|(reply,)| reply)
            .unwrap();
    assert!(resumed);
    let completed: Result<(), SyncFailure> = blob_test_protocol::fixture_wire::decode_one(
        "sync_gateway",
        &f.harness.pic.await_call(id).await.unwrap(),
        blob_test_protocol::fixture_wire::MAX_BYTES,
    )
    .unwrap();
    assert_eq!(completed, Err(SyncFailure::Stale));
    assert_eq!(observed(&f).await.requests, 1);
    assert_eq!(status(&f).await.gateways, []);
    assert_eq!(status(&f).await.sync_revision, revised);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn forced_restore_during_sync_retains_pending_evidence_when_callback_arrives() {
    let f = Fixture::with_source_operator(true).await;
    configure(&f, SourceMode::Hold).await;
    let r = request(&f).await;
    let id = f
        .harness
        .pic
        .submit_call(
            f.authority,
            f.gateway,
            "sync_gateway",
            blob_test_protocol::fixture_wire::encode_one("sync_gateway", r).unwrap(),
        )
        .await
        .unwrap();
    for _ in 0..30 {
        f.harness.pic.tick().await.unwrap();
        if observed(&f).await.requests == 1 {
            break;
        }
    }
    assert_eq!(observed(&f).await.requests, 1);
    assert_eq!(status(&f).await.pending_sync, Some(r.sequence));
    f.upgrade_fixture(f.authority, true).await;
    let mut retained = status(&f).await;
    assert!(retained.fenced);
    let resumed: bool =
        fixture_calls::update_fixture(&f.harness.pic, f.gateway, f.driver, "resume_sync", ())
            .await
            .map(|(reply,)| reply)
            .unwrap();
    assert!(resumed);
    // Actual upgrade clears the async heap: the late callback can trap before
    // entering Rust. It cannot apply membership or clear the frozen journal.
    let failure = f.harness.pic.await_call(id).await.unwrap_err();
    assert_eq!(
        crate::canister_rejection(&failure).reject_code,
        ic_testkit::pocket_ic::RejectCode::CanisterError
    );
    assert_eq!(status(&f).await, retained);
    assert_eq!(sync(&f, false, r).await, Err(SyncFailure::Fenced));
    retained = status(&f).await;
    assert_eq!(retained.pending_sync, Some(r.sequence));
    assert_eq!(observed(&f).await.requests, 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn bindings_caller_and_restoration_are_checked_before_source_calls() {
    let f = Fixture::with_source_operator(true).await;
    let r = request(&f).await;
    let before = f.journals().await;
    for wrong in [
        GatewaySyncRequest {
            source: f.receiver,
            ..r
        },
        GatewaySyncRequest { namespace: 2, ..r },
    ] {
        assert_eq!(sync(&f, false, wrong).await, Err(SyncFailure::Binding));
    }
    let (denied,): (Result<(), SyncFailure>,) = fixture_calls::update_fixture(
        &f.harness.pic,
        f.authority,
        Principal::anonymous(),
        "sync_gateway",
        (r,),
    )
    .await
    .unwrap();
    assert_eq!(denied, Err(SyncFailure::Denied));
    assert!(
        f.journals().await == before,
        "rejections cannot change journals"
    );
    f.harness
        .pic
        .upgrade_canister(
            f.authority,
            std::fs::read(fixture_path("BLOB_AUTHORITY_PROBE_WASM")).unwrap(),
            blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
            None,
        )
        .await
        .unwrap();
    let restored = f.journals().await;
    for preview in [true, false] {
        assert_eq!(sync(&f, preview, r).await, Err(SyncFailure::Fenced));
        assert!(
            f.journals().await == restored,
            "restored authority must stay frozen"
        );
    }
    assert_eq!(observed(&f).await.requests, 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn acknowledged_and_malformed_updates_remain_distinct_when_status_is_unavailable() {
    let f = Fixture::with_source_operator(true).await;
    let pic = &f.harness.pic;
    let r = request(&f).await;
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
        let service = pic.create_canister().await.unwrap();
        pic.install_canister(
            service,
            operator_method_mode::update_wasm("sync_gateway", &reply),
            vec![],
            None,
        )
        .await
        .unwrap();
        let result = fixture_calls::update_fixture::<(Result<(), SyncFailure>,), _>(
            pic,
            service,
            f.gateway,
            "sync_gateway",
            (GatewaySyncRequest { service, ..r },),
        )
        .await;
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
            .await
            .is_err()
        );
        assert_eq!(pic.get_stable_memory(service).await.unwrap().len(), 65_536);
    }
    let service = pic.create_canister().await.unwrap();
    pic.install_canister(
        service,
        operator_method_mode::update_wasm("preview_gateway_sync", &[]),
        vec![],
        None,
    )
    .await
    .unwrap();
    assert!(
        fixture_calls::query_fixture::<(Result<(), SyncFailure>,), _>(
            pic,
            service,
            f.gateway,
            "preview_gateway_sync",
            (GatewaySyncRequest { service, ..r },)
        )
        .await
        .is_err()
    );
    assert_eq!(
        pic.get_stable_memory(service).await.unwrap(),
        Vec::<u8>::new()
    );
    assert_eq!(observed(&f).await.requests, 0);
}
