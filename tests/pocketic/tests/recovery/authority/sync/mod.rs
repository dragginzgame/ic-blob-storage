//! Capture real in-flight sync journals, then restore them without fresh authority.
use super::*;
use blob_test_protocol::{SourceObservation, source::SourceHeldSyncView};
use ic_testkit::pocket_ic::common::rest::RawMessageId;

impl Fixture {
    pub(super) fn hold_sync(&self) -> RawMessageId {
        let configured: bool = self
            .harness
            .pic
            .update_candid_as(
                self.gateway,
                self.operator,
                "configure",
                (SourceMode::Hold,),
            )
            .unwrap();
        assert!(configured);
        let id = self
            .harness
            .pic
            .submit_call(
                self.gateway,
                self.operator,
                "run_sync",
                candid::encode_one(self.sync_request()).unwrap(),
            )
            .unwrap();
        for _ in 0..30 {
            self.harness.pic.tick();
            if self
                .source_recovery(self.operator)
                .unwrap()
                .held_sync
                .is_some()
            {
                return id;
            }
        }
        panic!("list did not enter its bounded hold");
    }

    pub(super) fn resume_sync(&self, id: RawMessageId) -> Result<(), SyncFailure> {
        let resumed: bool = self
            .harness
            .pic
            .update_candid_as(self.gateway, self.operator, "resume_sync", ())
            .unwrap();
        assert!(resumed);
        candid::decode_one(&self.harness.pic.await_call(id).unwrap()).unwrap()
    }

    pub(super) fn list_observation(&self) -> SourceObservation {
        let observed: Option<SourceObservation> = self
            .harness
            .pic
            .query_candid_as(self.gateway, self.operator, "observation", ())
            .unwrap();
        observed.unwrap()
    }
}

#[test]
fn actual_pending_sync_rejects_upgrade_and_old_journal_restores_only_as_evidence() {
    let f = Fixture::with_source_operator(true);
    let id = f.hold_sync();
    let mut authority = f.archive();
    assert_eq!((authority.last_sync, authority.pending_sync), (1, Some(1)));
    let mut source = f.source_recovery(f.operator).unwrap();
    assert_eq!(
        source.held_sync,
        Some(SourceHeldSyncView {
            sequence: 1,
            gateway: f.gateway,
            ready: false
        })
    );
    assert_eq!(source.effects.last().unwrap().succeeded, None);
    let authority_bytes = f.harness.pic.get_stable_memory(f.service);
    let source_bytes = f.harness.pic.get_stable_memory(f.gateway);
    for target in [f.service, f.gateway] {
        f.reject_upgrade(target);
    }
    assert_eq!(f.archive(), authority);
    // Ordinary overlap allocates no token and sends no second source request.
    let overlap: Result<(), SyncFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.gateway, "sync_gateway", (f.sync_request(),))
        .unwrap();
    assert_eq!(overlap, Err(SyncFailure::InProgress));
    assert_eq!(f.list_observation().requests, 1);
    for actor in [f.first, f.second, f.service, Principal::anonymous()] {
        let resumed: bool = f
            .harness
            .pic
            .update_candid_as(f.gateway, actor, "resume_sync", ())
            .unwrap();
        assert!(!resumed);
    }
    assert_eq!(f.resume_sync(id), Ok(()));
    assert!(f.archive().pending_sync.is_none());
    assert!(f.source_recovery(f.operator).unwrap().held_sync.is_none());
    f.replace_archive_bytes(authority_bytes);
    f.upgrade_skipping_outgoing_hook(f.service, true);
    authority.fenced = true;
    assert_eq!(f.archive(), authority);
    f.upgrade_authority();
    assert_eq!(f.archive(), authority);
    f.assert_authority_fenced(upload(1, b"fenced"));
    assert_eq!(f.list_observation().requests, 1);
    // Restore the matching source journal too. Pending reply and outgoing sync
    // remain unresolved even though the newer live instance already completed.
    f.harness
        .pic
        .set_stable_memory(f.gateway, source_bytes, BlobCompression::NoCompression);
    f.upgrade_skipping_outgoing_hook(f.gateway, true);
    source.fenced = true;
    assert_eq!(f.source_recovery(f.operator).unwrap(), source);
    let resumed: bool = f
        .harness
        .pic
        .update_candid_as(f.gateway, f.operator, "resume_sync", ())
        .unwrap();
    assert!(!resumed);
    let retry: Result<(), SyncFailure> = f
        .harness
        .pic
        .update_candid_as(f.gateway, f.operator, "run_sync", (f.sync_request(),))
        .unwrap();
    assert_eq!(retry, Err(SyncFailure::Denied));
    assert_eq!(f.source_recovery(f.operator).unwrap(), source);
}

#[test]
fn revocation_while_list_is_held_survives_stale_reply_and_fenced_upgrade() {
    let f = Fixture::with_source_operator(true);
    let id = f.hold_sync();
    let configured: bool = f
        .harness
        .pic
        .update_candid_as(f.gateway, f.operator, "configure", (SourceMode::Valid,))
        .unwrap();
    assert!(!configured);
    let revoked: bool = f
        .harness
        .pic
        .update_candid_as(f.service, f.gateway, "revoke_gateway", ())
        .unwrap();
    assert!(revoked);
    let mut revoked = f.archive();
    assert!(revoked.gateways.is_empty());
    assert_eq!((revoked.last_sync, revoked.pending_sync), (1, None));
    assert_eq!(f.resume_sync(id), Err(SyncFailure::Stale));
    assert_eq!(f.archive(), revoked);
    assert_eq!(f.list_observation().requests, 1);
    assert_eq!(
        f.source_recovery(f.operator)
            .unwrap()
            .effects
            .last()
            .unwrap()
            .succeeded,
        Some(false)
    );
    f.upgrade_authority();
    revoked.fenced = true;
    assert_eq!(f.archive(), revoked);
    f.assert_authority_fenced(upload(1, b"revoked"));
}

#[test]
fn exhausted_local_hold_releases_exact_read_only_attempt_without_automatic_retry() {
    let f = Fixture::with_source_operator(true);
    let id = f.hold_sync();
    // PocketIC's await_call stops after 100 rounds; the fixture deliberately
    // allows 128 management callbacks. Drive that bounded schedule explicitly.
    for _ in 0..256 {
        f.harness.pic.tick();
        if f.source_recovery(f.operator).unwrap().held_sync.is_none() {
            break;
        }
    }
    assert!(f.source_recovery(f.operator).unwrap().held_sync.is_none());
    let result: Result<(), SyncFailure> =
        candid::decode_one(&f.harness.pic.await_call(id).unwrap()).unwrap();
    assert_eq!(result, Err(SyncFailure::Transport));
    assert_eq!((f.archive().last_sync, f.archive().pending_sync), (1, None));
    let source = f.source_recovery(f.operator).unwrap();
    assert!(source.held_sync.is_none());
    assert_eq!(source.effects.last().unwrap().succeeded, Some(false));
    assert_eq!(f.list_observation().requests, 1);
    let configured: bool = f
        .harness
        .pic
        .update_candid_as(f.gateway, f.operator, "configure", (SourceMode::Valid,))
        .unwrap();
    assert!(configured);
    let explicit: Result<(), SyncFailure> = f
        .harness
        .pic
        .update_candid_as(f.gateway, f.operator, "run_sync", (f.sync_request(),))
        .unwrap();
    assert_eq!(explicit, Ok(()));
    assert_eq!((f.archive().last_sync, f.archive().pending_sync), (2, None));
    assert_eq!(f.list_observation().requests, 2);
}
