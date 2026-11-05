//! Connected chunk sessions using the retained independent Caffeine vectors.
use super::*;

const CHUNK: usize = 1024 * 1024;

use super::content_vectors::vector;

fn progress(
    next_chunk: u64,
    verified_bytes: u64,
    verification: JourneyVerification,
) -> JourneyProgress {
    JourneyProgress {
        next_chunk,
        verified_bytes,
        verification,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered chunk journey proves exact retries, tenant isolation, completion and settlement on the same fixture"
)]
async fn metadata_and_multiple_chunks_retain_exact_progress_until_separate_completion() {
    let f = Fixture::new().await;
    let v = vector("uneven-tree-with-metadata", 1);
    let a = v.upload;
    let amount = u128::from(a.bytes);
    assert_eq!(
        f.reserve_manifest(f.first, a, v.manifest.clone()).await,
        Ok(())
    );
    assert_eq!(
        f.progress(f.first, a).await.expect("tenant progress"),
        progress(0, 0, JourneyVerification::Pending)
    );
    for caller in [f.second, f.gateway, f.operator, Principal::anonymous()] {
        assert_eq!(f.progress(caller, a).await, Err(JourneyFailure::Denied));
        assert_eq!(
            f.append(caller, a, 0, &v.bytes[..CHUNK]).await,
            Err(JourneyFailure::Denied)
        );
    }
    assert_eq!(
        f.append(f.first, a, 1, &v.bytes[CHUNK..2 * CHUNK]).await,
        Err(JourneyFailure::OutOfOrder)
    );
    assert_eq!(f.append(f.first, a, 0, &v.bytes[..CHUNK]).await, Ok(()));
    let prefix = progress(1, CHUNK as u64, JourneyVerification::Pending);
    assert_eq!(
        f.progress(f.first, a).await.expect("tenant progress"),
        prefix
    );
    // Simulate a lost append reply: exact retry checks bytes without hashing twice.
    assert_eq!(f.append(f.first, a, 0, &v.bytes[..CHUNK]).await, Ok(()));
    assert_eq!(
        f.reserve_manifest(f.first, a, v.manifest.clone()).await,
        Ok(())
    );
    assert_eq!(
        f.progress(f.first, a).await.expect("tenant progress"),
        prefix
    );
    let mut corrupt = v.bytes[..CHUNK].to_vec();
    corrupt[10] ^= 1;
    assert_eq!(
        f.append(f.first, a, 0, &corrupt).await,
        Err(JourneyFailure::ContentMismatch)
    );
    assert_eq!(
        f.append(f.first, a, 1, &v.bytes[CHUNK..2 * CHUNK - 1])
            .await,
        Err(JourneyFailure::ContentMismatch)
    );
    assert_eq!(
        f.append(f.first, a, 1, &v.bytes[CHUNK..=2 * CHUNK]).await,
        Err(JourneyFailure::ContentMismatch)
    );
    assert_eq!(
        f.progress(f.first, a).await.expect("tenant progress"),
        prefix
    );
    assert_eq!(
        f.certificate(f.first, a.root).await,
        Err(RejectCode::CanisterError)
    );
    assert_eq!(f.usage(f.first).await, Ok(usage(amount, amount, amount)));

    // Another tenant's completed metadata-bound file cannot alter this prefix.
    let other = vector("abc-text", 1);
    assert_eq!(
        f.reserve_manifest(f.second, other.upload, other.manifest)
            .await,
        Ok(())
    );
    assert_eq!(
        f.append(f.second, other.upload, 0, &other.bytes).await,
        Ok(())
    );
    f.certificate(f.second, other.upload.root)
        .await
        .expect("other certificate");
    assert_eq!(
        f.progress(f.first, a).await.expect("tenant progress"),
        prefix
    );
    for (index, bytes) in v.bytes.chunks(CHUNK).enumerate().skip(1) {
        let index = u64::try_from(index).expect("index");
        assert_eq!(f.append(f.first, a, index, bytes).await, Ok(()));
        if index < 5 {
            assert_eq!(
                f.progress(f.first, a).await.expect("tenant progress"),
                progress(
                    index + 1,
                    (index + 1) * CHUNK as u64,
                    JourneyVerification::Pending
                )
            );
            assert_eq!(
                f.certificate(f.first, a.root).await,
                Err(RejectCode::CanisterError)
            );
        }
    }
    let verified = progress(6, a.bytes, JourneyVerification::Verified);
    assert_eq!(
        f.progress(f.first, a).await.expect("tenant progress"),
        verified
    );
    assert_eq!(f.append(f.first, a, 5, &v.bytes[5 * CHUNK..]).await, Ok(()));
    assert_eq!(f.append(f.first, a, 0, &v.bytes[..CHUNK]).await, Ok(()));
    assert_eq!(
        f.append(f.first, a, 6, b"").await,
        Err(JourneyFailure::ContentMismatch)
    );
    assert_eq!(
        f.progress(f.first, a).await.expect("tenant progress"),
        verified
    );
    f.certificate(f.first, a.root)
        .await
        .expect("whole content verified");
    assert_eq!(
        f.root_control(f.first, "journey_release", a.root).await,
        Ok(())
    );
    assert_eq!(f.usage(f.first).await, Ok(usage(0, amount, amount)));
    assert_eq!(f.delete(vec![root(a.root)]).await, Ok(()));
    assert_eq!(f.usage(f.first).await, Ok(usage(0, 0, amount)));
    assert_eq!(
        f.root_control(f.operator, "journey_settle", a.root).await,
        Ok(())
    );
    assert_eq!(f.usage(f.first).await, Ok(usage(0, 0, 0)));
    assert_eq!(f.usage(f.second).await, Ok(usage(3, 3, 3)));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn invalid_manifests_and_metadata_cannot_consume_or_replace_a_reservation() {
    let f = Fixture::new().await;
    let v = vector("abc-text", 1);
    let mut invalid = vec![
        JourneyManifest {
            chunks: vec![],
            headers: v.manifest.headers.clone(),
        },
        JourneyManifest {
            chunks: vec![[0; 32]; 11],
            headers: vec![],
        },
        JourneyManifest {
            chunks: v.manifest.chunks.clone(),
            headers: vec![("X".to_owned(), "a".to_owned()); 9],
        },
        JourneyManifest {
            chunks: v.manifest.chunks.clone(),
            headers: vec![("X".to_owned(), "a".repeat(1024))],
        },
    ];
    let mut changed = v.manifest.clone();
    changed.headers[1].1 = "application/json".to_owned();
    invalid.push(changed);
    let mut duplicate = v.manifest.clone();
    duplicate.headers.push(duplicate.headers[0].clone());
    invalid.push(duplicate);
    for manifest in &invalid {
        assert_eq!(
            f.reserve_manifest(f.first, v.upload, manifest.clone())
                .await,
            Err(JourneyFailure::InvalidInput)
        );
        assert_eq!(f.usage(f.first).await, Ok(usage(0, 0, 0)));
    }
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest.clone())
            .await,
        Ok(())
    );
    for manifest in invalid {
        assert_eq!(
            f.reserve_manifest(f.first, v.upload, manifest).await,
            Err(JourneyFailure::InvalidInput)
        );
    }
    assert_eq!(
        f.progress(f.first, v.upload)
            .await
            .expect("tenant progress"),
        progress(0, 0, JourneyVerification::Pending)
    );
    assert_eq!(f.append(f.first, v.upload, 0, &v.bytes).await, Ok(()));
    let mut reordered = v.manifest;
    reordered.headers.reverse();
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, reordered).await,
        Ok(())
    );
    assert_eq!(
        f.progress(f.first, v.upload)
            .await
            .expect("tenant progress"),
        progress(1, 3, JourneyVerification::Verified)
    );
    f.certificate(f.first, v.upload.root)
        .await
        .expect("unchanged canonical metadata");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unfinished_sessions_remain_charged_until_explicit_unexposed_cancellation() {
    let f = Fixture::new().await;
    let v = vector("pattern-9437215", 1);
    let other = vector("pattern-2097152", 2);
    let amount = u128::from(v.upload.bytes);
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest.clone())
            .await,
        Ok(())
    );
    assert_eq!(
        f.append(f.first, v.upload, 0, &v.bytes[..CHUNK]).await,
        Ok(())
    );
    assert_eq!(
        f.reserve_manifest(f.first, other.upload, other.manifest.clone())
            .await,
        Err(JourneyFailure::Limit)
    );
    assert_eq!(f.usage(f.first).await, Ok(usage(amount, amount, amount)));
    assert_eq!(f.control(f.first, "journey_cancel", v.upload).await, Ok(()));
    assert_eq!(f.usage(f.first).await, Ok(usage(0, 0, 0)));
    assert_eq!(
        f.append(f.first, v.upload, 1, &v.bytes[CHUNK..2 * CHUNK])
            .await,
        Err(JourneyFailure::InvalidPhase)
    );
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest).await,
        Ok(())
    );
    assert_eq!(
        f.certificate(f.first, v.upload.root).await,
        Err(RejectCode::CanisterError)
    );
    assert_eq!(
        f.reserve_manifest(f.first, other.upload, other.manifest)
            .await,
        Ok(())
    );
    assert_eq!(
        f.progress(f.first, other.upload)
            .await
            .expect("tenant progress"),
        progress(0, 0, JourneyVerification::Pending)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn final_digest_rejection_retains_the_completed_leaf_progress_and_reservation() {
    let f = Fixture::new().await;
    let v = vector("pattern-1048577", 1);
    let wrong = JourneyUpload {
        digest: [0; 32],
        ..v.upload
    };
    assert_eq!(
        f.reserve_manifest(f.first, wrong, v.manifest.clone()).await,
        Ok(())
    );
    assert_eq!(f.append(f.first, wrong, 0, &v.bytes[..CHUNK]).await, Ok(()));
    assert_eq!(
        f.append(f.first, wrong, 1, &v.bytes[CHUNK..]).await,
        Err(JourneyFailure::ContentMismatch)
    );
    let rejected = progress(2, wrong.bytes, JourneyVerification::Rejected);
    assert_eq!(
        f.progress(f.first, wrong).await.expect("tenant progress"),
        rejected
    );
    assert_eq!(
        f.append(f.first, wrong, 1, &v.bytes[CHUNK..]).await,
        Err(JourneyFailure::ContentMismatch)
    );
    assert_eq!(
        f.reserve_manifest(f.first, wrong, v.manifest.clone()).await,
        Ok(())
    );
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest).await,
        Err(JourneyFailure::Conflict)
    );
    assert_eq!(
        f.progress(f.first, wrong).await.expect("tenant progress"),
        rejected
    );
    assert_eq!(
        f.certificate(f.first, wrong.root).await,
        Err(RejectCode::CanisterError)
    );
    let amount = u128::from(wrong.bytes);
    assert_eq!(f.usage(f.first).await, Ok(usage(amount, amount, amount)));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ten_mib_media_boundary_verifies_across_messages_and_keeps_billing_until_settlement() {
    let f = Fixture::new().await;
    let v = vector("pattern-10485760", 1);
    let too_large = vector("pattern-10485761", 2);
    assert_eq!(
        f.reserve_manifest(f.first, too_large.upload, too_large.manifest)
            .await,
        Err(JourneyFailure::InvalidInput)
    );
    assert_eq!(f.usage(f.first).await, Ok(usage(0, 0, 0)));
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest.clone())
            .await,
        Ok(())
    );
    for (index, bytes) in v.bytes.chunks(CHUNK).enumerate() {
        let index = u64::try_from(index).unwrap();
        assert_eq!(f.append(f.first, v.upload, index, bytes).await, Ok(()));
        // Recover a lost ingress reply at the largest admitted file size.
        assert_eq!(f.append(f.first, v.upload, index, bytes).await, Ok(()));
        let expected = if index == 9 {
            JourneyVerification::Verified
        } else {
            JourneyVerification::Pending
        };
        assert_eq!(
            f.progress(f.first, v.upload).await.unwrap(),
            progress(index + 1, (index + 1) * CHUNK as u64, expected)
        );
        if index < 9 {
            assert_eq!(
                f.certificate(f.first, v.upload.root).await,
                Err(RejectCode::CanisterError)
            );
        }
    }
    f.certificate(f.first, v.upload.root)
        .await
        .expect("verified full manifest");
    assert_eq!(
        f.root_control(f.first, "journey_release", v.upload.root)
            .await,
        Ok(())
    );
    let bytes = u128::from(v.upload.bytes);
    assert_eq!(f.usage(f.first).await, Ok(usage(0, bytes, bytes)));
    assert_eq!(f.delete(vec![root(v.upload.root)]).await, Ok(()));
    assert_eq!(f.usage(f.first).await, Ok(usage(0, 0, bytes)));
    assert_eq!(
        f.root_control(f.operator, "journey_settle", v.upload.root)
            .await,
        Ok(())
    );
    assert_eq!(f.usage(f.first).await, Ok(usage(0, 0, 0)));
}
