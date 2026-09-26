//! Actual lifecycle recovery preserves obligations but cannot resume authority.
use super::*;

impl Fixture {
    pub(in crate::recovery) fn upgrade_authority(&self) {
        self.harness
            .pic
            .upgrade_canister(
                self.service,
                self.wasm(self.service),
                candid::encode_args(()).unwrap(),
                Some(self.operator),
            )
            .expect("same-release fenced restore");
    }

    pub(in crate::recovery) fn assert_authority_fenced(&self, upload: JourneyUpload) {
        let before = self.archive();
        assert!(before.fenced);
        let sync: Result<(), SyncFailure> = self
            .harness
            .pic
            .update_candid_as(
                self.service,
                self.authority_operator,
                "sync_gateway",
                (self.sync_request(),),
            )
            .unwrap();
        assert_eq!(sync, Err(SyncFailure::Fenced));
        let args = |root: [u8; 32]| candid::encode_args((root.to_vec(),)).unwrap();
        let updates = [
            (
                self.first,
                "journey_reserve",
                candid::encode_args((JourneyReservation {
                    upload,
                    manifest: blob_test_protocol::journey::JourneyManifest {
                        chunks: vec![],
                        headers: vec![],
                    },
                },))
                .unwrap(),
            ),
            (
                self.first,
                "journey_append",
                candid::encode_args((root(upload.root), 0_u64, vec![1_u8])).unwrap(),
            ),
            (
                self.first,
                "journey_cancel",
                candid::encode_args((upload,)).unwrap(),
            ),
            (self.first, "journey_release", args(upload.root)),
            (self.authority_operator, "journey_settle", args(upload.root)),
            (
                self.gateway,
                "journey_complete",
                candid::encode_args((self.first, upload)).unwrap(),
            ),
            (
                self.first,
                "_immutableObjectStorageCreateCertificate",
                candid::encode_args((root_text(upload.root),)).unwrap(),
            ),
            (
                self.gateway,
                "_immutableObjectStorageConfirmBlobDeletion",
                candid::encode_args((vec![root(upload.root)],)).unwrap(),
            ),
            (
                self.first,
                "journey_read",
                candid::encode_args((root(upload.root), 0_u64)).unwrap(),
            ),
            (
                self.authority_operator,
                "journey_arm_read_callback_trap",
                args(upload.root),
            ),
            (
                self.authority_operator,
                "revoke_gateway",
                candid::encode_args(()).unwrap(),
            ),
            (
                self.first,
                "cancel_upload",
                candid::encode_args((11_u8,)).unwrap(),
            ),
            (self.first, "release", candid::encode_args((1_u8,)).unwrap()),
            (
                self.authority_operator,
                "confirm_obligation",
                candid::encode_args((1_u8, ObligationProbeFact::Deleted)).unwrap(),
            ),
        ];
        for (actor, method, input) in updates {
            let failure = self
                .harness
                .pic
                .update_call(self.service, actor, method, input)
                .expect_err(method);
            assert_eq!(failure.reject_code, RejectCode::CanisterError, "{method}");
        }
        self.assert_gateway_queries_fenced(upload);
        assert_eq!(self.archive(), before);
    }

    fn assert_gateway_queries_fenced(&self, upload: JourneyUpload) {
        for (method, input) in [
            (
                "_immutableObjectStorageBlobsAreLive",
                candid::encode_args((vec![root(upload.root)],)).unwrap(),
            ),
            (
                "_immutableObjectStorageBlobsToDelete",
                candid::encode_args(()).unwrap(),
            ),
            ("pending", candid::encode_args(()).unwrap()),
            ("upload_roots", candid::encode_args(()).unwrap()),
        ] {
            let failure = self
                .harness
                .pic
                .query_call(self.service, self.gateway, method, input)
                .expect_err(method);
            assert_eq!(failure.reject_code, RejectCode::CanisterError, "{method}");
        }
    }
}

#[test]
fn partial_verifier_and_all_catalogs_survive_repeated_and_skipped_hook_upgrades() {
    let f = Fixture::new();
    let v = chunks::vector("pattern-1048577", 1);
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest.clone()),
        Ok(())
    );
    assert_eq!(f.append(f.first, v.upload, 0, &v.bytes[..CHUNK]), Ok(()));
    let mut before = f.archive();
    for skip in [false, false, true] {
        if skip {
            f.upgrade_skipping_outgoing_hook(f.service, true);
        } else {
            f.upgrade_authority();
        }
        before.fenced = true;
        assert_eq!(f.archive(), before);
        assert_eq!(
            f.checkpoint_probe(f.operator, v.upload, 1, &v.bytes[CHUNK..])
                .unwrap()
                .verification,
            JourneyVerification::Verified
        );
        f.assert_authority_fenced(v.upload);
    }
    f.restart(f.service);
    f.harness.pic.advance_time(Duration::from_hours(24));
    f.assert_authority_fenced(v.upload);
}

#[test]
fn old_archive_is_fenced_and_missing_archive_rejects_atomically() {
    let f = Fixture::new();
    let v = chunks::vector("abc-text", 1);
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest.clone()),
        Ok(())
    );
    let old = f.harness.pic.get_stable_memory(f.service);
    let mut retained = f.archive();
    assert_eq!(f.append(f.first, v.upload, 0, &v.bytes), Ok(()));
    f.certificate(f.first, v.upload.root).unwrap();
    let saved = f.harness.pic.get_stable_memory(f.service);
    f.replace_archive_bytes(vec![]);
    f.reject_upgrade(f.service);
    f.upgrade_skipping_outgoing_hook(f.service, false);
    assert_eq!(f.usage(f.first), Ok(usage(3, 3, 3)));
    f.replace_archive_bytes(saved);
    assert_eq!(f.complete(f.first, v.upload), Ok(()));
    // The old archive predates exposure. It cannot prove no authority escaped.
    f.replace_archive_bytes(old);
    f.upgrade_skipping_outgoing_hook(f.service, true);
    retained.fenced = true;
    assert_eq!(f.archive(), retained);
    f.assert_authority_fenced(v.upload);
}

#[test]
fn trapped_read_intent_survives_forced_restore_without_callback_replay() {
    let f = Fixture::new();
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false);
    let armed: bool = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.operator,
            "journey_arm_read_callback_trap",
            (root(v.upload.root),),
        )
        .unwrap();
    assert!(armed);
    let failure = f
        .harness
        .pic
        .update_call(
            f.service,
            f.first,
            "journey_read",
            candid::encode_args((root(v.upload.root), 0_u64)).unwrap(),
        )
        .unwrap_err();
    assert_eq!(failure.reject_code, RejectCode::CanisterError);
    let mut retained = f.archive();
    assert!(retained.pending_read.is_some());
    f.reject_upgrade(f.service);
    f.upgrade_skipping_outgoing_hook(f.service, true);
    retained.fenced = true;
    assert_eq!(f.archive(), retained);
    f.upgrade_authority();
    assert_eq!(f.archive(), retained);
    f.assert_authority_fenced(v.upload);
    assert_eq!(f.source_observation().requests, 1);
}
