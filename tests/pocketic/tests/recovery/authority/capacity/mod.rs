//! Byte capacity can be reused; root, receipt and uncertain-effect history cannot.
use super::*;

fn expose(f: &Fixture, v: &chunks::Vector) {
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest.clone()),
        Ok(())
    );
    for (index, bytes) in v.bytes.chunks(CHUNK).enumerate() {
        assert_eq!(
            f.append(f.first, v.upload, u64::try_from(index).unwrap(), bytes),
            Ok(())
        );
    }
    f.certificate(f.first, v.upload.root).unwrap();
}

fn release_delete(f: &Fixture, v: &chunks::Vector) {
    assert_eq!(
        f.root_control(f.first, "journey_release", v.upload.root),
        Ok(())
    );
    assert_eq!(f.delete(vec![root(v.upload.root)]), Ok(()));
}

#[test]
fn restoration_retains_large_reused_capacity_and_exhausted_lifetime_slots() {
    let f = Fixture::new();
    let settled = chunks::vector("uneven-tree-with-metadata", 1);
    f.confirm_bytes(&settled);
    release_delete(&f, &settled);
    assert_eq!(
        f.root_control(f.operator, "journey_settle", settled.upload.root),
        Ok(())
    );
    let cancelled = chunks::vector("pattern-5242897", 2);
    assert_eq!(
        f.reserve_manifest(f.first, cancelled.upload, cancelled.manifest.clone()),
        Ok(())
    );
    assert_eq!(
        f.append(f.first, cancelled.upload, 0, &cancelled.bytes[..CHUNK]),
        Ok(())
    );
    assert_eq!(
        f.control(f.first, "journey_cancel", cancelled.upload),
        Ok(())
    );
    let live = chunks::vector("pattern-3145728", 3);
    f.confirm_bytes(&live);
    let exposed = chunks::vector("pattern-2097152", 4);
    expose(&f, &exposed);
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
        let v = chunks::vector(name, u8::try_from(index + 1).unwrap());
        assert_eq!(f.reserve_manifest(f.second, v.upload, v.manifest), Ok(()));
        assert_eq!(f.control(f.second, "journey_cancel", v.upload), Ok(()));
    }
    assert_eq!(
        f.usage(f.first),
        Ok(usage(
            5 * CHUNK as u128,
            5 * CHUNK as u128,
            5 * CHUNK as u128
        ))
    );
    assert_eq!(f.usage(f.second), Ok(usage(0, 0, 0)));
    for old in [&settled, &cancelled] {
        assert_eq!(
            f.reserve_manifest(
                f.first,
                JourneyUpload {
                    id: 5,
                    ..old.upload
                },
                old.manifest.clone()
            ),
            Err(JourneyFailure::Conflict)
        );
    }
    assert_eq!(
        f.reserve(f.first, upload(5, b"lifetime full")),
        Err(JourneyFailure::Limit)
    );
    assert_eq!(
        f.reserve(f.second, upload(5, b"other lifetime full")),
        Err(JourneyFailure::Limit)
    );
    let mut retained = f.archive();
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
        (exposed.upload.root, ArchivePhase::ExposurePossible, None),
    ] {
        let entry = object(&retained, ArchiveCatalog::Journey, root);
        assert_eq!((entry.phase, entry.release_receipt), (phase, receipt));
    }
    f.upgrade_authority();
    retained.fenced = true;
    assert_eq!(f.archive(), retained);
    f.upgrade_skipping_outgoing_hook(f.service, true);
    assert_eq!(f.archive(), retained);
    f.assert_authority_fenced(exposed.upload);
}

#[test]
fn physical_deletion_keeps_billing_capacity_reserved_across_old_archive_restore() {
    let f = Fixture::new();
    let deleted = chunks::vector("uneven-tree-with-metadata", 1);
    f.confirm_bytes(&deleted);
    release_delete(&f, &deleted);
    let second = chunks::vector("pattern-5242897", 1);
    assert_eq!(
        f.reserve_manifest(f.second, second.upload, second.manifest),
        Ok(())
    );
    let next = chunks::vector("pattern-2097152", 2);
    assert_eq!(
        f.reserve_manifest(f.first, next.upload, next.manifest.clone()),
        Err(JourneyFailure::Limit)
    );
    assert_eq!(
        f.usage(f.first),
        Ok(usage(0, 0, u128::from(deleted.upload.bytes)))
    );
    let memory = f.harness.pic.get_stable_memory(f.service);
    let mut retained = f.archive();
    let entry = object(&retained, ArchiveCatalog::Journey, deleted.upload.root);
    assert_eq!(
        (entry.physical, entry.liability, entry.phase),
        (0, deleted.upload.bytes, ArchivePhase::ProviderDeleted)
    );
    assert_eq!(
        f.root_control(f.operator, "journey_settle", deleted.upload.root),
        Ok(())
    );
    assert_eq!(
        f.reserve_manifest(f.first, next.upload, next.manifest),
        Ok(())
    );
    f.replace_archive_bytes(memory);
    f.upgrade_skipping_outgoing_hook(f.service, true);
    retained.fenced = true;
    assert_eq!(f.archive(), retained);
    f.assert_authority_fenced(next.upload);
}
