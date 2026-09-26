//! Source reconstruction and refusal to act from retained or older stable state.
use super::*;
use blob_test_protocol::{
    SourceMode, SyncFailure,
    journey::readback::ReadSourceConfig,
    source::{SourceAction, SourceEffectView, SourceRecoveryView},
};
use ic_testkit::pocket_ic::common::rest::BlobCompression;

impl Fixture {
    fn source_recovery(&self, caller: Principal) -> Option<SourceRecoveryView> {
        self.harness
            .pic
            .query_candid_as(self.gateway, caller, "recovery_observation", ())
            .expect("journal inspection")
    }

    fn upgrade_source(&self) {
        self.harness
            .pic
            .upgrade_canister(
                self.gateway,
                self.wasm(self.gateway),
                candid::encode_args(()).expect("no args"),
                None,
            )
            .expect("fenced source restore");
    }

    fn assert_source_fenced(&self, v: &chunks::Vector) {
        let before = self.source_recovery(self.operator).expect("driver journal");
        assert!(before.fenced);
        assert_eq!(
            (before.service, before.gateway, before.driver),
            (self.service, self.gateway, self.operator)
        );
        assert_eq!(
            self.read_chunk(self.first, v.upload, 0),
            Err(JourneyFailure::Transport)
        );
        let configured: bool = self
            .harness
            .pic
            .update_candid_as(
                self.gateway,
                self.operator,
                "configure_read",
                (ReadSourceConfig {
                    root: v.upload.root,
                    index: 0,
                    bytes: b"replacement".to_vec(),
                    mode: ReadSourceMode::Valid,
                    hold: false,
                },),
            )
            .expect("fenced configuration");
        assert!(!configured);
        let configured: bool = self
            .harness
            .pic
            .update_candid_as(
                self.gateway,
                self.operator,
                "configure",
                (SourceMode::Reject,),
            )
            .expect("fenced list configuration");
        assert!(!configured);
        let resumed: bool = self
            .harness
            .pic
            .update_candid_as(self.gateway, self.operator, "resume_read", ())
            .expect("fenced resume");
        assert!(!resumed);
        let sync: Result<(), SyncFailure> = self
            .harness
            .pic
            .update_candid_as(self.gateway, self.operator, "run_sync", ())
            .expect("fenced outgoing sync");
        assert_eq!(sync, Err(SyncFailure::Denied));
        let deletion = self
            .harness
            .pic
            .update_call(
                self.gateway,
                self.operator,
                "run_deletion",
                candid::encode_args((vec![root(v.upload.root)],)).expect("deletion input"),
            )
            .expect_err("fenced outgoing deletion");
        assert_eq!(deletion.reject_code, RejectCode::CanisterError);
        let list = self
            .harness
            .pic
            .update_call(
                self.gateway,
                self.service,
                "fixture_gateways",
                candid::encode_args(()).expect("list input"),
            )
            .expect_err("fenced list response");
        assert_eq!(list.reject_code, RejectCode::CanisterReject);
        let after = self.source_recovery(self.operator).expect("driver journal");
        assert_eq!(before, after);
    }
}

#[test]
fn source_restores_maximum_leaf_and_history_but_all_effects_stay_fenced() {
    let f = Fixture::with_source_operator(true);
    let v = chunks::vector("pattern-1048576", 1);
    f.confirm_bytes(&v);
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false);
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0)
            .expect("initial read")
            .bytes,
        v.bytes
    );
    let result: Result<(), SyncFailure> = f
        .harness
        .pic
        .update_candid_as(f.gateway, f.operator, "run_sync", ())
        .expect("actual source sync");
    assert_eq!(result, Ok(()));
    assert_eq!(f.delete(vec![]), Ok(()));
    let rejected_root = [99; 32];
    assert!(f.delete(vec![root(rejected_root)]).is_err());
    let before = f.source_recovery(f.operator).expect("driver journal");
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
    let reads = f.source_observation();
    let usage = f.usage(f.first);
    f.upgrade_source();
    let after = f
        .source_recovery(f.operator)
        .expect("restored driver journal");
    assert_eq!(after.effects, before.effects);
    assert_eq!(after.read.expect("retained leaf").bytes, v.bytes);
    assert_eq!(f.source_observation(), reads);
    for caller in [
        f.first,
        f.second,
        f.service,
        f.gateway,
        Principal::anonymous(),
    ] {
        assert!(f.source_recovery(caller).is_none());
    }
    f.assert_source_fenced(&v);
    f.restart(f.gateway);
    f.harness.pic.advance_time(Duration::from_hours(24));
    f.upgrade_source();
    f.upgrade_skipping_outgoing_hook(f.gateway, true);
    f.assert_source_fenced(&v);
    assert_eq!(f.source_observation(), reads);
    assert_eq!(f.usage(f.first), usage);
}

#[test]
fn source_intents_have_a_lifetime_budget_and_restore_does_not_reset_it() {
    let f = Fixture::new();
    let v = chunks::vector("pattern-1048576", 1);
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false);
    // Fill with the largest supported action payload alongside the maximum leaf.
    let roots = vec![vec![99; 32]; 8];
    for _ in 0..64 {
        assert!(f.delete(roots.clone()).is_err());
    }
    let before = f.source_recovery(f.operator).expect("full journal");
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
            candid::encode_args((roots,)).unwrap(),
        )
        .expect_err("lifetime capacity exhausted");
    assert_eq!(rejected.reject_code, RejectCode::CanisterError);
    assert_eq!(
        f.source_recovery(f.operator).unwrap().effects,
        before.effects
    );
    f.upgrade_source();
    assert_eq!(
        f.source_recovery(f.operator).unwrap().effects,
        before.effects
    );
}

#[test]
fn missing_journal_rejects_upgrade_without_discarding_the_old_heap() {
    let f = Fixture::new();
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false);
    let saved = f.harness.pic.get_stable_memory(f.gateway);
    f.harness
        .pic
        .set_stable_memory(f.gateway, vec![], BlobCompression::NoCompression);
    f.reject_upgrade(f.gateway);
    let old = f
        .source_recovery(f.operator)
        .expect("old heap survives failed restore");
    assert!(!old.fenced);
    assert_eq!(old.read.unwrap().bytes, v.bytes);
    // Restore the injected fault before resuming any mutations against the old store.
    f.harness
        .pic
        .set_stable_memory(f.gateway, saved, BlobCompression::NoCompression);
    assert_eq!(f.read_chunk(f.first, v.upload, 0).unwrap().bytes, v.bytes);
    f.upgrade_source();
    f.assert_source_fenced(&v);
}

#[test]
fn restoring_older_stable_bytes_never_reactivates_the_source() {
    let f = Fixture::new();
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false);
    let old = f.harness.pic.get_stable_memory(f.gateway);
    assert_eq!(f.read_chunk(f.first, v.upload, 0).unwrap().bytes, v.bytes);
    assert_eq!(f.source_observation().requests, 1);
    f.harness
        .pic
        .set_stable_memory(f.gateway, old, BlobCompression::NoCompression);
    f.upgrade_skipping_outgoing_hook(f.gateway, true);
    // This counter really is stale. It cannot authorize work after reconstruction.
    assert_eq!(f.source_observation().requests, 0);
    f.assert_source_fenced(&v);
}

#[test]
fn unresolved_source_call_survives_a_forced_fenced_restore() {
    let f = Fixture::with_source_operator(true);
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false);
    f.harness
        .pic
        .stop_canister(f.service, Some(f.operator))
        .expect("stop target before dispatch");
    let failure = f
        .harness
        .pic
        .update_call(
            f.gateway,
            f.operator,
            "run_sync",
            candid::encode_args(()).unwrap(),
        )
        .expect_err("real failed call traps source callback");
    assert_eq!(failure.reject_code, RejectCode::CanisterError);
    let before = f.source_recovery(f.operator).unwrap();
    assert_eq!(
        before.effects,
        vec![SourceEffectView {
            action: SourceAction::Sync,
            succeeded: None
        }]
    );
    // Ordinary lifecycle refuses unresolved work. Even a forced restore retains it.
    f.reject_upgrade(f.gateway);
    f.upgrade_skipping_outgoing_hook(f.gateway, true);
    assert_eq!(
        f.source_recovery(f.operator).unwrap().effects,
        before.effects
    );
    f.harness
        .pic
        .start_canister(f.service, Some(f.operator))
        .expect("restart target");
    f.assert_source_fenced(&v);
}

#[test]
fn restored_pending_read_is_retained_for_inspection_without_releasing_it() {
    let f = Fixture::new();
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, true);
    let id = f.hold_read(v.upload);
    let pending = f.harness.pic.get_stable_memory(f.gateway);
    assert_eq!(f.resume_read(id).unwrap().bytes, v.bytes);
    f.harness
        .pic
        .set_stable_memory(f.gateway, pending, BlobCompression::NoCompression);
    f.upgrade_skipping_outgoing_hook(f.gateway, true);
    let restored = f.source_recovery(f.operator).unwrap();
    assert!(restored.read_pending);
    assert!(!restored.read_ready);
    assert_eq!(f.source_observation().requests, 1);
    assert!(f.source_observation().waiting);
    f.assert_source_fenced(&v);
    // The fence does not pretend that historical pending work was resolved.
    f.reject_upgrade(f.gateway);
}
