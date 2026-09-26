//! Read protected checkpoint bytes from stable memory and test a disposable copy.
use super::*;
use blob_test_protocol::authority::{CheckpointProbeFailure, CheckpointProbeInput};

impl Fixture {
    pub(super) fn checkpoint_probe(
        &self,
        caller: Principal,
        upload: JourneyUpload,
        index: u64,
        bytes: &[u8],
    ) -> Result<JourneyProgress, CheckpointProbeFailure> {
        self.harness
            .pic
            .query_candid_as(
                self.service,
                caller,
                "probe_checkpoint",
                (CheckpointProbeInput {
                    root: upload.root,
                    index,
                    bytes: bytes.to_vec(),
                },),
            )
            .expect("isolated verifier probe")
    }
}

#[test]
fn stable_checkpoint_copy_completes_tails_without_advancing_live_authority() {
    for name in ["pattern-1048577", "uneven-tree-with-metadata"] {
        let f = Fixture::new();
        let v = chunks::vector(name, 1);
        assert_eq!(
            f.reserve_manifest(f.first, v.upload, v.manifest.clone()),
            Ok(())
        );
        let leaves: Vec<_> = v.bytes.chunks(CHUNK).collect();
        let last = leaves.len() - 1;
        for (index, bytes) in leaves[..last].iter().enumerate() {
            assert_eq!(
                f.append(f.first, v.upload, u64::try_from(index).unwrap(), bytes),
                Ok(())
            );
        }
        let before = f.archive();
        let progress = f.progress(f.first, v.upload).unwrap();
        let index = u64::try_from(last).unwrap();
        for actor in [f.first, f.second, f.gateway, Principal::anonymous()] {
            assert_eq!(
                f.checkpoint_probe(actor, v.upload, index, leaves[last]),
                Err(CheckpointProbeFailure::Denied)
            );
        }
        let mut corrupt = leaves[last].to_vec();
        corrupt[0] ^= 1;
        assert_eq!(
            f.checkpoint_probe(f.operator, v.upload, index, &corrupt),
            Err(CheckpointProbeFailure::Chunk(
                JourneyFailure::ContentMismatch
            ))
        );
        assert_eq!(
            f.checkpoint_probe(f.operator, v.upload, index + 1, leaves[last]),
            Err(CheckpointProbeFailure::Chunk(JourneyFailure::OutOfOrder))
        );
        assert_eq!(
            f.checkpoint_probe(f.operator, v.upload, 0, leaves[0]),
            Ok(progress)
        );
        assert_eq!(
            f.checkpoint_probe(f.operator, v.upload, index, leaves[last]),
            Ok(JourneyProgress {
                next_chunk: u64::try_from(leaves.len()).unwrap(),
                verified_bytes: v.upload.bytes,
                verification: JourneyVerification::Verified
            })
        );
        assert_eq!(f.archive(), before);
        assert_eq!(f.progress(f.first, v.upload), Ok(progress));
        assert_eq!(
            f.certificate(f.first, v.upload.root),
            Err(RejectCode::CanisterError)
        );
        assert_eq!(f.append(f.first, v.upload, index, leaves[last]), Ok(()));
        f.certificate(f.first, v.upload.root)
            .expect("only the live verifier issues authority");
        assert_eq!(f.complete(f.first, v.upload), Ok(()));
        let verified = f.progress(f.first, v.upload).unwrap();
        assert_eq!(
            f.checkpoint_probe(f.operator, v.upload, index, leaves[last]),
            Ok(verified)
        );
    }
}

#[test]
fn restored_hash_cannot_turn_a_wrong_raw_digest_into_verification() {
    let f = Fixture::new();
    let v = chunks::vector("pattern-1048577", 1);
    let upload = JourneyUpload {
        digest: [7; 32],
        ..v.upload
    };
    assert_eq!(
        f.reserve_manifest(f.first, upload, v.manifest.clone()),
        Ok(())
    );
    assert_eq!(f.append(f.first, upload, 0, &v.bytes[..CHUNK]), Ok(()));
    let before = f.archive();
    assert_eq!(
        f.checkpoint_probe(f.operator, upload, 1, &v.bytes[CHUNK..]),
        Err(CheckpointProbeFailure::Chunk(
            JourneyFailure::ContentMismatch
        ))
    );
    assert_eq!(f.archive(), before);
    assert_eq!(
        f.append(f.first, upload, 1, &v.bytes[CHUNK..]),
        Err(JourneyFailure::ContentMismatch)
    );
    let rejected = f.archive();
    assert_eq!(
        f.checkpoint_probe(f.operator, upload, 0, &v.bytes[..CHUNK]),
        Err(CheckpointProbeFailure::Chunk(
            JourneyFailure::ContentMismatch
        ))
    );
    assert_eq!(f.reserve_manifest(f.first, upload, v.manifest), Ok(()));
    assert_eq!(f.archive(), rejected);
    assert_eq!(
        f.certificate(f.first, upload.root),
        Err(RejectCode::CanisterError)
    );
}

#[test]
fn damaged_checkpoint_rejects_reconstruction_without_resetting_live_state() {
    let f = Fixture::new();
    let v = chunks::vector("pattern-1048577", 1);
    assert_eq!(f.reserve_manifest(f.first, v.upload, v.manifest), Ok(()));
    assert_eq!(f.append(f.first, v.upload, 0, &v.bytes[..CHUNK]), Ok(()));
    let before = f.archive();
    let memory = f.harness.pic.get_stable_memory(f.service);
    let mut corrupted = memory.clone();
    // Locate the protected versioned envelope, not an ic-memory implementation offset.
    let at = corrupted
        .windows(8)
        .position(|bytes| bytes == b"ICBV\0\0\0\x01")
        .expect("stored verifier record");
    corrupted[at + 72] ^= 1;
    f.replace_archive_bytes(corrupted);
    f.reject_upgrade(f.service);
    f.upgrade_skipping_outgoing_hook(f.service, false);
    assert_eq!(
        f.checkpoint_probe(f.operator, v.upload, 1, &v.bytes[CHUNK..]),
        Err(CheckpointProbeFailure::InvalidCheckpoint)
    );
    assert_eq!(f.progress(f.first, v.upload).unwrap().next_chunk, 1);
    assert_eq!(
        f.certificate(f.first, v.upload.root),
        Err(RejectCode::CanisterError)
    );
    f.replace_archive_bytes(memory);
    assert_eq!(f.archive(), before);
    assert_eq!(
        f.checkpoint_probe(f.operator, v.upload, 1, &v.bytes[CHUNK..])
            .unwrap()
            .verification,
        JourneyVerification::Verified
    );
}

#[test]
fn controller_status_cannot_use_an_operator_checkpoint_probe() {
    let f = Fixture::with_source_operator(true);
    let v = chunks::vector("abc-text", 1);
    assert_eq!(f.reserve_manifest(f.first, v.upload, v.manifest), Ok(()));
    assert_eq!(
        f.checkpoint_probe(f.operator, v.upload, 0, &v.bytes),
        Err(CheckpointProbeFailure::Denied)
    );
    assert_eq!(
        f.checkpoint_probe(f.gateway, v.upload, 0, &v.bytes)
            .unwrap()
            .verification,
        JourneyVerification::Verified
    );
    assert_eq!(
        f.progress(f.first, v.upload).unwrap().verification,
        JourneyVerification::Pending
    );
    f.upgrade_authority();
    assert_eq!(
        f.checkpoint_probe(f.operator, v.upload, 0, &v.bytes),
        Err(CheckpointProbeFailure::Denied)
    );
    let retained: Option<AuthorityArchiveView> = f
        .harness
        .pic
        .query_candid_as(f.service, f.gateway, "authority_archive", ())
        .unwrap();
    assert!(retained.unwrap().fenced);
    assert_eq!(
        f.checkpoint_probe(f.gateway, v.upload, 0, &v.bytes)
            .unwrap()
            .verification,
        JourneyVerification::Verified
    );
}
