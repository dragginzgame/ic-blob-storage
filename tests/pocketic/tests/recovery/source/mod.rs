//! Source reconstruction and refusal to act from retained or older stable state.
use super::*;
use blob_test_protocol::{
    SourceMode, SyncFailure,
    journey::readback::ReadSourceConfig,
    source::{SourceAction, SourceEffectView, SourceRecoveryView},
};
use ic_blob_storage_pocketic_tests::fixture_calls;

use ic_testkit::pocket_ic::common::rest::BlobCompression;

impl Fixture {
    pub(in crate::recovery) async fn source_recovery(
        &self,
        caller: Principal,
    ) -> Option<SourceRecoveryView> {
        fixture_calls::query_fixture(
            &self.harness.pic,
            self.gateway,
            caller,
            "recovery_observation",
            (),
        )
        .await
        .map(|(reply,)| reply)
        .expect("journal inspection")
    }

    #[expect(
        clippy::too_many_lines,
        reason = "One restored source must reject every authority and mutation path without changing its journals"
    )]
    async fn assert_source_fenced(&self, v: &content_vectors::Vector) {
        let before = self
            .source_recovery(self.operator)
            .await
            .expect("driver journal");
        assert!(before.fenced);
        assert_eq!(
            (before.service, before.gateway, before.driver),
            (self.service, self.gateway, self.operator)
        );
        assert_eq!(
            self.read_chunk(self.first, v.upload, 0).await,
            Err(JourneyFailure::Transport)
        );
        let configured: bool = fixture_calls::update_fixture(
            &self.harness.pic,
            self.gateway,
            self.operator,
            "configure_read",
            (ReadSourceConfig {
                root: v.upload.root,
                index: 0,
                bytes: b"replacement".to_vec().into(),
                mode: ReadSourceMode::Valid,
                hold: false,
            },),
        )
        .await
        .map(|(reply,)| reply)
        .expect("fenced configuration");
        assert!(!configured);
        let configured: bool = fixture_calls::update_fixture(
            &self.harness.pic,
            self.gateway,
            self.operator,
            "configure",
            (SourceMode::Reject,),
        )
        .await
        .map(|(reply,)| reply)
        .expect("fenced list configuration");
        assert!(!configured);
        let resumed: bool = fixture_calls::update_fixture(
            &self.harness.pic,
            self.gateway,
            self.operator,
            "resume_read",
            (),
        )
        .await
        .map(|(reply,)| reply)
        .expect("fenced resume");
        assert!(!resumed);
        let sync: Result<(), SyncFailure> = fixture_calls::update_fixture(
            &self.harness.pic,
            self.gateway,
            self.operator,
            "run_sync",
            (self.sync_request().await,),
        )
        .await
        .map(|(reply,)| reply)
        .expect("fenced outgoing sync");
        assert_eq!(sync, Err(SyncFailure::Denied));
        let deletion = self
            .harness
            .pic
            .update_call(
                self.gateway,
                self.operator,
                "run_deletion",
                blob_test_protocol::fixture_wire::encode_args(
                    "run_deletion",
                    (vec![root(v.upload.root)],),
                )
                .expect("deletion input"),
            )
            .await
            .expect_err("fenced outgoing deletion");
        assert_eq!(
            crate::canister_rejection(&deletion).reject_code,
            RejectCode::CanisterError
        );
        let list = self
            .harness
            .pic
            .update_call(
                self.gateway,
                self.service,
                "fixture_gateways",
                blob_test_protocol::fixture_wire::encode_one(
                    "fixture_gateways",
                    self.sync_request().await,
                )
                .expect("list input"),
            )
            .await
            .expect_err("fenced list response");
        assert_eq!(
            crate::canister_rejection(&list).reject_code,
            RejectCode::CanisterReject
        );
        let after = self
            .source_recovery(self.operator)
            .await
            .expect("driver journal");
        assert_eq!(before, after);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn source_restores_maximum_leaf_and_history_but_all_effects_stay_fenced() {
    let f = Fixture::with_source_operator(true).await;
    let v = content_vectors::vector("pattern-1048576", 1);
    f.confirm_bytes(&v).await;
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false)
        .await;
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0)
            .await
            .expect("initial read")
            .bytes,
        v.bytes
    );
    let result: Result<(), SyncFailure> = fixture_calls::update_fixture(
        &f.harness.pic,
        f.gateway,
        f.operator,
        "run_sync",
        (f.sync_request().await,),
    )
    .await
    .map(|(reply,)| reply)
    .expect("actual source sync");
    assert_eq!(result, Ok(()));
    assert_eq!(f.delete(vec![]).await, Ok(()));
    let rejected_root = [99; 32];
    assert!(f.delete(vec![root(rejected_root)]).await.is_err());
    let before = f.source_recovery(f.operator).await.expect("driver journal");
    assert!(!before.fenced);
    assert_eq!(
        before.effects,
        vec![
            SourceEffectView {
                action: SourceAction::Sync,
                succeeded: Some(true)
            },
            SourceEffectView {
                action: SourceAction::Delete(vec![]),
                succeeded: Some(true)
            },
            SourceEffectView {
                action: SourceAction::Delete(vec![rejected_root]),
                succeeded: Some(false)
            },
        ]
    );
    let reads = f.source_observation().await;
    let usage = f.usage(f.first).await;
    f.restore(f.gateway).await;
    let after = f
        .source_recovery(f.operator)
        .await
        .expect("restored driver journal");
    assert_eq!(after.effects, before.effects);
    assert_eq!(after.read.expect("retained leaf").bytes, v.bytes);
    assert_eq!(f.source_observation().await, reads);
    for caller in [
        f.first,
        f.second,
        f.service,
        f.gateway,
        Principal::anonymous(),
    ] {
        assert!(f.source_recovery(caller).await.is_none());
    }
    f.assert_source_fenced(&v).await;
    f.restart(f.gateway).await;
    f.harness
        .pic
        .advance_time(Duration::from_hours(24))
        .await
        .unwrap();
    f.restore(f.gateway).await;
    f.upgrade_skipping_outgoing_hook(f.gateway, true).await;
    f.assert_source_fenced(&v).await;
    assert_eq!(f.source_observation().await, reads);
    assert_eq!(f.usage(f.first).await, usage);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn source_intents_have_a_lifetime_budget_and_restore_does_not_reset_it() {
    let f = Fixture::new().await;
    let v = content_vectors::vector("pattern-1048576", 1);
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false)
        .await;
    // Fill with the largest supported action payload alongside the maximum leaf.
    let roots = vec![vec![99; 32]; 8];
    for _ in 0..64 {
        assert!(f.delete(roots.clone()).await.is_err());
    }
    let before = f.source_recovery(f.operator).await.expect("full journal");
    assert!(
        before
            .effects
            .iter()
            .all(|effect| effect.succeeded == Some(false))
    );
    let rejected = f
        .harness
        .pic
        .update_call(
            f.gateway,
            f.operator,
            "run_deletion",
            blob_test_protocol::fixture_wire::encode_args("run_deletion", (roots,)).unwrap(),
        )
        .await
        .expect_err("lifetime capacity exhausted");
    assert_eq!(
        crate::canister_rejection(&rejected).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(
        f.source_recovery(f.operator).await.unwrap().effects,
        before.effects
    );
    f.restore(f.gateway).await;
    assert_eq!(
        f.source_recovery(f.operator).await.unwrap().effects,
        before.effects
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn missing_journal_rejects_upgrade_without_discarding_the_old_heap() {
    let f = Fixture::new().await;
    let v = content_vectors::vector("abc-text", 1);
    f.confirm_bytes(&v).await;
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false)
        .await;
    let saved = f.harness.pic.get_stable_memory(f.gateway).await.unwrap();
    f.harness
        .pic
        .set_stable_memory(f.gateway, vec![], BlobCompression::NoCompression)
        .await
        .unwrap();
    f.reject_upgrade(f.gateway).await;
    let old = f
        .source_recovery(f.operator)
        .await
        .expect("old heap survives failed restore");
    assert!(!old.fenced);
    assert_eq!(old.read.unwrap().bytes, v.bytes);
    // Restore the injected fault before resuming any mutations against the old store.
    f.harness
        .pic
        .set_stable_memory(f.gateway, saved, BlobCompression::NoCompression)
        .await
        .unwrap();
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0).await.unwrap().bytes,
        v.bytes
    );
    f.restore(f.gateway).await;
    f.assert_source_fenced(&v).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn restoring_older_stable_bytes_never_reactivates_the_source() {
    let f = Fixture::new().await;
    let v = content_vectors::vector("abc-text", 1);
    f.confirm_bytes(&v).await;
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false)
        .await;
    let old = f.harness.pic.get_stable_memory(f.gateway).await.unwrap();
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0).await.unwrap().bytes,
        v.bytes
    );
    assert_eq!(f.source_observation().await.requests, 1);
    f.harness
        .pic
        .set_stable_memory(f.gateway, old, BlobCompression::NoCompression)
        .await
        .unwrap();
    f.upgrade_skipping_outgoing_hook(f.gateway, true).await;
    // This counter really is stale. It cannot authorize work after reconstruction.
    assert_eq!(f.source_observation().await.requests, 0);
    f.assert_source_fenced(&v).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unresolved_source_call_survives_a_forced_fenced_restore() {
    let f = Fixture::with_source_operator(true).await;
    let v = content_vectors::vector("abc-text", 1);
    f.confirm_bytes(&v).await;
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false)
        .await;
    let request = f.sync_request().await;
    f.harness
        .pic
        .stop_canister(f.service, Some(f.operator))
        .await
        .expect("stop target before dispatch");
    let failure = f
        .harness
        .pic
        .update_call(
            f.gateway,
            f.operator,
            "run_sync",
            blob_test_protocol::fixture_wire::encode_one("run_sync", request).unwrap(),
        )
        .await
        .expect_err("real failed call traps source callback");
    assert_eq!(
        crate::canister_rejection(&failure).reject_code,
        RejectCode::CanisterError
    );
    let before = f.source_recovery(f.operator).await.unwrap();
    assert_eq!(
        before.effects,
        vec![SourceEffectView {
            action: SourceAction::Sync,
            succeeded: None
        }]
    );
    // Ordinary lifecycle refuses unresolved work. Even a forced restore retains it.
    f.reject_upgrade(f.gateway).await;
    f.upgrade_skipping_outgoing_hook(f.gateway, true).await;
    assert_eq!(
        f.source_recovery(f.operator).await.unwrap().effects,
        before.effects
    );
    f.harness
        .pic
        .start_canister(f.service, Some(f.operator))
        .await
        .expect("restart target");
    f.assert_source_fenced(&v).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn restored_pending_read_is_retained_for_inspection_without_releasing_it() {
    let f = Fixture::new().await;
    let v = content_vectors::vector("abc-text", 1);
    f.confirm_bytes(&v).await;
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, true)
        .await;
    let id = f.hold_read(v.upload).await;
    let pending = f.harness.pic.get_stable_memory(f.gateway).await.unwrap();
    assert_eq!(f.resume_read(id).await.unwrap().bytes, v.bytes);
    f.harness
        .pic
        .set_stable_memory(f.gateway, pending, BlobCompression::NoCompression)
        .await
        .unwrap();
    f.upgrade_skipping_outgoing_hook(f.gateway, true).await;
    let restored = f.source_recovery(f.operator).await.unwrap();
    assert!(restored.read_pending);
    assert!(!restored.read_ready);
    assert_eq!(f.source_observation().await.requests, 1);
    assert!(f.source_observation().await.waiting);
    f.assert_source_fenced(&v).await;
    // The fence does not pretend that historical pending work was resolved.
    f.reject_upgrade(f.gateway).await;
}
