//! Read protected checkpoint bytes from stable memory and test a disposable copy.
use super::*;
use blob_test_protocol::authority::{CheckpointProbeFailure, CheckpointProbeInput};
use ic_blob_storage_pocketic_tests::fixture_calls;

impl Fixture {
    pub(super) async fn checkpoint_probe(
        &self,
        caller: Principal,
        upload: JourneyUpload,
        index: u64,
        bytes: &[u8],
    ) -> Result<JourneyProgress, CheckpointProbeFailure> {
        fixture_calls::query_fixture(
            &self.harness.pic,
            self.service,
            caller,
            "probe_checkpoint",
            (CheckpointProbeInput {
                root: upload.root,
                index,
                bytes: bytes.to_vec().into(),
            },),
        )
        .await
        .map(|(reply,)| reply)
        .expect("isolated verifier probe")
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn stable_checkpoint_copy_completes_tails_without_advancing_live_authority() {
    for name in ["pattern-1048577", "uneven-tree-with-metadata"] {
        let f = Fixture::new().await;
        let v = content_vectors::vector(name, 1);
        assert_eq!(
            f.reserve_manifest(f.first, v.upload, v.manifest.clone())
                .await,
            Ok(())
        );
        let leaves: Vec<_> = v.bytes.chunks(CHUNK).collect();
        let last = leaves.len() - 1;
        for (index, bytes) in leaves[..last].iter().enumerate() {
            assert_eq!(
                f.append(f.first, v.upload, u64::try_from(index).unwrap(), bytes)
                    .await,
                Ok(())
            );
        }
        let before = f.archive().await;
        let progress = f.progress(f.first, v.upload).await.unwrap();
        let index = u64::try_from(last).unwrap();
        for actor in [f.first, f.second, f.gateway, Principal::anonymous()] {
            assert_eq!(
                f.checkpoint_probe(actor, v.upload, index, leaves[last])
                    .await,
                Err(CheckpointProbeFailure::Denied)
            );
        }
        let mut corrupt = leaves[last].to_vec();
        corrupt[0] ^= 1;
        assert_eq!(
            f.checkpoint_probe(f.operator, v.upload, index, &corrupt)
                .await,
            Err(CheckpointProbeFailure::Chunk(
                JourneyFailure::ContentMismatch
            ))
        );
        assert_eq!(
            f.checkpoint_probe(f.operator, v.upload, index + 1, leaves[last])
                .await,
            Err(CheckpointProbeFailure::Chunk(JourneyFailure::OutOfOrder))
        );
        assert_eq!(
            f.checkpoint_probe(f.operator, v.upload, 0, leaves[0]).await,
            Ok(progress)
        );
        assert_eq!(
            f.checkpoint_probe(f.operator, v.upload, index, leaves[last])
                .await,
            Ok(JourneyProgress {
                next_chunk: u64::try_from(leaves.len()).unwrap(),
                verified_bytes: v.upload.bytes,
                verification: JourneyVerification::Verified
            })
        );
        assert_eq!(f.archive().await, before);
        assert_eq!(f.progress(f.first, v.upload).await, Ok(progress));
        assert_eq!(
            f.certificate(f.first, v.upload.root).await,
            Err(RejectCode::CanisterError)
        );
        assert_eq!(
            f.append(f.first, v.upload, index, leaves[last]).await,
            Ok(())
        );
        f.certificate(f.first, v.upload.root)
            .await
            .expect("only the live verifier issues authority");
        let verified = f.progress(f.first, v.upload).await.unwrap();
        assert_eq!(
            f.checkpoint_probe(f.operator, v.upload, index, leaves[last])
                .await,
            Ok(verified)
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn restored_hash_cannot_turn_a_wrong_raw_digest_into_verification() {
    let f = Fixture::new().await;
    let v = content_vectors::vector("pattern-1048577", 1);
    let upload = JourneyUpload {
        digest: [7; 32],
        ..v.upload
    };
    assert_eq!(
        f.reserve_manifest(f.first, upload, v.manifest.clone())
            .await,
        Ok(())
    );
    assert_eq!(
        f.append(f.first, upload, 0, &v.bytes[..CHUNK]).await,
        Ok(())
    );
    let before = f.archive().await;
    assert_eq!(
        f.checkpoint_probe(f.operator, upload, 1, &v.bytes[CHUNK..])
            .await,
        Err(CheckpointProbeFailure::Chunk(
            JourneyFailure::ContentMismatch
        ))
    );
    assert_eq!(f.archive().await, before);
    assert_eq!(
        f.append(f.first, upload, 1, &v.bytes[CHUNK..]).await,
        Err(JourneyFailure::ContentMismatch)
    );
    let rejected = f.archive().await;
    assert_eq!(
        f.checkpoint_probe(f.operator, upload, 0, &v.bytes[..CHUNK])
            .await,
        Err(CheckpointProbeFailure::Chunk(
            JourneyFailure::ContentMismatch
        ))
    );
    assert_eq!(
        f.reserve_manifest(f.first, upload, v.manifest).await,
        Ok(())
    );
    assert_eq!(f.archive().await, rejected);
    assert_eq!(
        f.certificate(f.first, upload.root).await,
        Err(RejectCode::CanisterError)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn damaged_checkpoint_rejects_reconstruction_without_resetting_live_state() {
    let f = Fixture::new().await;
    let v = content_vectors::vector("pattern-1048577", 1);
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest).await,
        Ok(())
    );
    assert_eq!(
        f.append(f.first, v.upload, 0, &v.bytes[..CHUNK]).await,
        Ok(())
    );
    let before = f.archive().await;
    let memory = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let mut corrupted = memory.clone();
    // Freed collection pages can retain earlier checkpoint copies. Damage every
    // retained envelope so the active record is covered without a Storage offset.
    let checkpoints: Vec<_> = corrupted
        .windows(8)
        .enumerate()
        .filter(|(_, bytes)| *bytes == b"ICBV\0\0\0\x01")
        .map(|(at, _)| at)
        .collect();
    assert!(!checkpoints.is_empty(), "stored verifier record");
    for at in checkpoints {
        corrupted[at + 72] ^= 1;
    }
    f.replace_archive_bytes(corrupted.clone()).await;
    f.reject_upgrade(f.service).await;
    f.upgrade_skipping_outgoing_hook(f.service, false).await;
    // Storage supplies no collection checksum. The owned checkpoint decoder
    // must reject the damaged inner record without repairing it or live state.
    assert_eq!(
        f.checkpoint_probe(f.operator, v.upload, 1, &v.bytes[CHUNK..])
            .await,
        Err(CheckpointProbeFailure::InvalidCheckpoint)
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        corrupted
    );
    assert_eq!(f.progress(f.first, v.upload).await.unwrap().next_chunk, 1);
    assert_eq!(
        f.certificate(f.first, v.upload.root).await,
        Err(RejectCode::CanisterError)
    );
    f.replace_archive_bytes(memory).await;
    assert_eq!(f.archive().await, before);
    assert_eq!(
        f.checkpoint_probe(f.operator, v.upload, 1, &v.bytes[CHUNK..])
            .await
            .unwrap()
            .verification,
        JourneyVerification::Verified
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn controller_status_cannot_use_an_operator_checkpoint_probe() {
    let f = Fixture::with_source_operator(true).await;
    let v = content_vectors::vector("abc-text", 1);
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest).await,
        Ok(())
    );
    assert_eq!(
        f.checkpoint_probe(f.operator, v.upload, 0, &v.bytes).await,
        Err(CheckpointProbeFailure::Denied)
    );
    assert_eq!(
        f.checkpoint_probe(f.gateway, v.upload, 0, &v.bytes)
            .await
            .unwrap()
            .verification,
        JourneyVerification::Verified
    );
    assert_eq!(
        f.progress(f.first, v.upload).await.unwrap().verification,
        JourneyVerification::Pending
    );
    f.restore(f.service).await;
    assert_eq!(
        f.checkpoint_probe(f.operator, v.upload, 0, &v.bytes).await,
        Err(CheckpointProbeFailure::Denied)
    );
    let retained: Option<AuthorityArchiveView> = fixture_calls::query_fixture(
        &f.harness.pic,
        f.service,
        f.gateway,
        "authority_archive",
        (),
    )
    .await
    .map(|(reply,)| reply)
    .unwrap();
    assert!(retained.unwrap().fenced);
    assert_eq!(
        f.checkpoint_probe(f.gateway, v.upload, 0, &v.bytes)
            .await
            .unwrap()
            .verification,
        JourneyVerification::Verified
    );
}
