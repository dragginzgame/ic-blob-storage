//! Byte capacity can be reused; root, receipt and uncertain-effect history cannot.
use super::*;

async fn expose(f: &Fixture, v: &content_vectors::Vector) {
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest.clone())
            .await,
        Ok(())
    );
    for (index, bytes) in v.bytes.chunks(CHUNK).enumerate() {
        assert_eq!(
            f.append(f.first, v.upload, u64::try_from(index).unwrap(), bytes)
                .await,
            Ok(())
        );
    }
    f.certificate(f.first, v.upload.root).await.unwrap();
}

async fn release_delete(f: &Fixture, v: &content_vectors::Vector) {
    assert_eq!(
        f.root_control(f.first, "journey_release", v.upload.root)
            .await,
        Ok(())
    );
    assert_eq!(f.delete(vec![root(v.upload.root)]).await, Ok(()));
}

#[tokio::test(flavor = "multi_thread")]
#[expect(
    clippy::too_many_lines,
    reason = "Keep capacity exhaustion and its restored-fence observations in the same installed journey"
)]
async fn restoration_retains_large_reused_capacity_and_exhausted_lifetime_slots() {
    let f = Fixture::new().await;
    let settled = content_vectors::vector("uneven-tree-with-metadata", 1);
    f.confirm_bytes(&settled).await;
    release_delete(&f, &settled).await;
    assert_eq!(
        f.root_control(f.operator, "journey_settle", settled.upload.root)
            .await,
        Ok(())
    );
    let cancelled = content_vectors::vector("pattern-10485760", 2);
    assert_eq!(
        f.reserve_manifest(f.first, cancelled.upload, cancelled.manifest.clone())
            .await,
        Ok(())
    );
    assert_eq!(
        f.append(f.first, cancelled.upload, 0, &cancelled.bytes[..CHUNK])
            .await,
        Ok(())
    );
    assert_eq!(
        f.control(f.first, "journey_cancel", cancelled.upload).await,
        Ok(())
    );
    let live = content_vectors::vector("pattern-3145728", 3);
    f.confirm_bytes(&live).await;
    let exposed = content_vectors::vector("pattern-2097152", 4);
    expose(&f, &exposed).await;
    // The other tenant also retains four distinct operations; cancelled content
    // checkpoints remain present even though their byte reservations are zero.
    for (index, name) in [
        "repeated-identical-chunks",
        "pattern-1048577",
        "pattern-1048576",
        "pattern-1048575",
    ]
    .into_iter()
    .enumerate()
    {
        let v = content_vectors::vector(name, u8::try_from(index + 1).unwrap());
        assert_eq!(
            f.reserve_manifest(f.second, v.upload, v.manifest).await,
            Ok(())
        );
        assert_eq!(
            f.control(f.second, "journey_cancel", v.upload).await,
            Ok(())
        );
    }
    assert_eq!(
        f.usage(f.first).await,
        Ok(usage(
            5 * CHUNK as u128,
            5 * CHUNK as u128,
            5 * CHUNK as u128
        ))
    );
    assert_eq!(f.usage(f.second).await, Ok(usage(0, 0, 0)));
    for old in [&settled, &cancelled] {
        assert_eq!(
            f.reserve_manifest(
                f.first,
                JourneyUpload {
                    id: 5,
                    ..old.upload
                },
                old.manifest.clone()
            )
            .await,
            Err(JourneyFailure::Conflict)
        );
    }
    assert_eq!(
        f.reserve(f.first, upload(5, b"lifetime full"), b"lifetime full")
            .await,
        Err(JourneyFailure::Limit)
    );
    assert_eq!(
        f.reserve(
            f.second,
            upload(5, b"other lifetime full"),
            b"other lifetime full"
        )
        .await,
        Err(JourneyFailure::Limit)
    );
    let mut retained = f.archive().await;
    let lifetime_bytes: u64 = retained
        .objects
        .iter()
        .filter(|o| o.catalog == ArchiveCatalog::Journey)
        .map(|o| o.bytes)
        .sum();
    assert!(lifetime_bytes > 12 * CHUNK as u64);
    for (root, phase, receipt) in [
        (settled.upload.root, ArchivePhase::Settled, Some(true)),
        (cancelled.upload.root, ArchivePhase::Cancelled, None),
        (live.upload.root, ArchivePhase::Live, None),
        (exposed.upload.root, ArchivePhase::Live, None),
    ] {
        let entry = object(&retained, ArchiveCatalog::Journey, root);
        assert_eq!((entry.phase, entry.release_receipt), (phase, receipt));
    }
    f.restore(f.service).await;
    retained.fenced = true;
    assert_eq!(f.archive().await, retained);
    f.upgrade_skipping_outgoing_hook(f.service, true).await;
    assert_eq!(f.archive().await, retained);
    f.assert_authority_fenced(exposed.upload).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn physical_deletion_keeps_billing_capacity_reserved_across_old_archive_restore() {
    let f = Fixture::new().await;
    let deleted = content_vectors::vector("uneven-tree-with-metadata", 1);
    f.confirm_bytes(&deleted).await;
    release_delete(&f, &deleted).await;
    let second = content_vectors::vector("pattern-5242897", 1);
    assert_eq!(
        f.reserve_manifest(f.second, second.upload, second.manifest)
            .await,
        Ok(())
    );
    let next = content_vectors::vector("pattern-2097152", 2);
    assert_eq!(
        f.reserve_manifest(f.first, next.upload, next.manifest.clone())
            .await,
        Err(JourneyFailure::Limit)
    );
    assert_eq!(
        f.usage(f.first).await,
        Ok(usage(0, 0, u128::from(deleted.upload.bytes)))
    );
    let memory = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let mut retained = f.archive().await;
    let entry = object(&retained, ArchiveCatalog::Journey, deleted.upload.root);
    assert_eq!(
        (entry.physical, entry.liability, entry.phase),
        (0, deleted.upload.bytes, ArchivePhase::ProviderDeleted)
    );
    assert_eq!(
        f.root_control(f.operator, "journey_settle", deleted.upload.root)
            .await,
        Ok(())
    );
    assert_eq!(
        f.reserve_manifest(f.first, next.upload, next.manifest)
            .await,
        Ok(())
    );
    f.replace_archive_bytes(memory).await;
    f.upgrade_skipping_outgoing_hook(f.service, true).await;
    retained.fenced = true;
    assert_eq!(f.archive().await, retained);
    f.assert_authority_fenced(next.upload).await;
}
