//! Atomic stable inspection records, without pretending to resume authority.
use super::*;
use blob_test_protocol::{
    SourceMode, SyncFailure,
    authority::{ArchiveCatalog, ArchivePhase, ArchivedObjectView, AuthorityArchiveView},
    obligations::ObligationProbeFact,
};
use ic_testkit::pocket_ic::common::rest::BlobCompression;

impl Fixture {
    fn archive(&self) -> AuthorityArchiveView {
        let result: Option<AuthorityArchiveView> = self
            .harness
            .pic
            .query_candid_as(self.service, self.operator, "authority_archive", ())
            .expect("stable archive query");
        result.expect("explicit operator")
    }
    fn sample_fact(&self, fact: ObligationProbeFact) {
        let accepted: bool = self
            .harness
            .pic
            .update_candid_as(
                self.service,
                self.operator,
                "confirm_obligation",
                (1_u8, fact),
            )
            .expect("sample fact");
        assert!(accepted);
    }

    fn replace_archive_bytes(&self, bytes: Vec<u8>) {
        self.harness
            .pic
            .set_stable_memory(self.service, bytes, BlobCompression::NoCompression);
        // Direct PocketIC memory injection does not invalidate the IC query
        // cache. A real stateless update advances the instance version without
        // touching the archive, so the following query executes against the fault.
        let result: Result<
            blob_test_protocol::content::ContentProbeReport,
            blob_test_protocol::content::ContentProbeFailure,
        > = self
            .harness
            .pic
            .update_candid_as(
                self.service,
                self.operator,
                "probe_content",
                (blob_test_protocol::content::ContentProbeCase::UnicodeMetadata,),
            )
            .expect("stateless update after memory injection");
        result.expect("independent content probe");
    }
}

fn object(
    archive: &AuthorityArchiveView,
    catalog: ArchiveCatalog,
    root: [u8; 32],
) -> &ArchivedObjectView {
    archive
        .objects
        .iter()
        .find(|object| object.catalog == catalog && object.root == root)
        .expect("retained object")
}

#[test]
fn archive_preserves_all_catalogs_partial_progress_and_byte_free_history() {
    let f = Fixture::new();
    let partial = chunks::vector("pattern-1048577", 1);
    assert_eq!(
        f.reserve_manifest(f.first, partial.upload, partial.manifest.clone()),
        Ok(())
    );
    assert_eq!(
        f.append(f.first, partial.upload, 0, &partial.bytes[..CHUNK]),
        Ok(())
    );
    let deleted = chunks::vector("abc-text", 2);
    f.confirm_bytes(&deleted);
    assert_eq!(
        f.root_control(f.first, "journey_release", deleted.upload.root),
        Ok(())
    );
    assert_eq!(f.delete(vec![root(deleted.upload.root)]), Ok(()));
    let cancelled = upload(3, b"cancelled");
    assert_eq!(f.reserve(f.first, cancelled), Ok(()));
    assert_eq!(f.control(f.first, "journey_cancel", cancelled), Ok(()));
    let released: bool = f
        .harness
        .pic
        .update_candid_as(f.service, f.first, "release", (1_u8,))
        .unwrap();
    assert!(released);
    f.sample_fact(ObligationProbeFact::Deleted);
    let sample_cancel: bool = f
        .harness
        .pic
        .update_candid_as(f.service, f.first, "cancel_upload", (11_u8,))
        .unwrap();
    assert!(sample_cancel);
    let archive = f.archive();
    assert_eq!(
        (
            archive.service,
            archive.operator,
            archive.tenants,
            archive.namespace
        ),
        (f.service, f.operator, [f.first, f.second], 1)
    );
    assert_sample_history(&archive);
    let entry = object(&archive, ArchiveCatalog::Journey, partial.upload.root);
    assert_eq!(entry.manifest, Some(partial.manifest));
    assert_eq!(
        entry.progress,
        Some(f.progress(f.first, partial.upload).unwrap())
    );
    assert_eq!(entry.digest, Some(partial.upload.digest));
    assert_eq!(entry.phase, ArchivePhase::Reserved);
    let entry = object(&archive, ArchiveCatalog::Journey, deleted.upload.root);
    assert_eq!(
        (
            entry.phase,
            entry.physical,
            entry.liability,
            entry.release_receipt
        ),
        (ArchivePhase::ProviderDeleted, 0, 3, Some(true))
    );
    assert_eq!(
        object(&archive, ArchiveCatalog::Journey, cancelled.root).phase,
        ArchivePhase::Cancelled
    );
    assert_archive_usage(&f, &archive);
    f.reject_upgrade(f.service);
    f.upgrade_skipping_outgoing_hook(f.service, false);
    f.restart(f.service);
    assert_eq!(f.archive(), archive);
    f.sample_fact(ObligationProbeFact::BillingStopped);
    assert_eq!(
        f.root_control(f.operator, "journey_settle", deleted.upload.root),
        Ok(())
    );
    let settled = f.archive();
    for (catalog, root) in [
        (ArchiveCatalog::Samples, [1; 32]),
        (ArchiveCatalog::Journey, deleted.upload.root),
    ] {
        let entry = object(&settled, catalog, root);
        assert_eq!(
            (
                entry.phase,
                entry.logical,
                entry.physical,
                entry.liability,
                entry.release_receipt
            ),
            (ArchivePhase::Settled, 0, 0, 0, Some(true))
        );
    }
}

#[test]
fn invalid_deletion_batch_rolls_back_prior_archive_writes_and_replay_preserves_receipts() {
    let f = Fixture::new();
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    assert_eq!(
        f.root_control(f.first, "journey_release", v.upload.root),
        Ok(())
    );
    let before = f.archive();
    assert!(f.delete(vec![root(v.upload.root), vec![99; 32]]).is_err());
    assert_eq!(f.archive(), before);
    assert_eq!(f.delete(vec![root(v.upload.root)]), Ok(()));
    let deleted = f.archive();
    assert_eq!(f.delete(vec![root(v.upload.root)]), Ok(()));
    assert_eq!(
        f.root_control(f.first, "journey_release", v.upload.root),
        Ok(())
    );
    assert_eq!(f.archive(), deleted);
}

#[test]
fn callback_trap_preserves_exact_stable_read_intent_and_revocation_marks_it_stale() {
    let f = Fixture::new();
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, true);
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
    assert_eq!(f.archive().armed_read_trap, Some(v.upload.root));
    let id = f.hold_read(v.upload);
    let before = f.archive();
    let read = before.pending_read.as_ref().unwrap();
    assert_eq!(
        (read.tenant, read.root, read.index, read.gateway),
        (f.first, v.upload.root, 0, f.gateway)
    );
    assert_eq!(before.trap_read_token, Some(read.token));
    assert_eq!(before.last_read, read.token);
    assert!(read.valid);
    assert_eq!(before.armed_read_trap, None);
    let resumed: bool = f
        .harness
        .pic
        .update_candid_as(f.gateway, f.operator, "resume_read", ())
        .unwrap();
    assert!(resumed);
    let error = f
        .harness
        .pic
        .await_call(id)
        .expect_err("actual read callback trap");
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert_eq!(f.archive(), before);
    let revoked: bool = f
        .harness
        .pic
        .update_candid_as(f.service, f.operator, "revoke_gateway", ())
        .unwrap();
    assert!(revoked);
    let after = f.archive();
    assert!(after.gateways.is_empty());
    let pending = after.pending_read.as_ref().unwrap();
    assert!(!pending.valid);
    assert_eq!(pending.token, read.token);
    f.restart(f.service);
    assert_eq!(f.archive(), after);
    assert_eq!(f.source_observation().requests, 1);
}

#[test]
fn stable_archive_is_operator_only_and_old_or_missing_bytes_never_change_live_authority() {
    let f = Fixture::with_source_operator(true);
    // Here the source is the explicit operator; the human controller is not.
    for caller in [
        f.operator,
        f.first,
        f.second,
        Principal::anonymous(),
        f.service,
    ] {
        let denied: Option<AuthorityArchiveView> = f
            .harness
            .pic
            .query_candid_as(f.service, caller, "authority_archive", ())
            .unwrap();
        assert!(denied.is_none());
    }
    let initial: Option<AuthorityArchiveView> = f
        .harness
        .pic
        .query_candid_as(f.service, f.gateway, "authority_archive", ())
        .unwrap();
    assert!(initial.is_some());
    let f = Fixture::new();
    let initial = f.archive();
    let old = f.harness.pic.get_stable_memory(f.service);
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    let current = f.archive();
    let saved = f.harness.pic.get_stable_memory(f.service);
    f.replace_archive_bytes(old);
    assert_eq!(f.archive(), initial);
    assert_eq!(
        f.certificate(f.first, v.upload.root),
        Err(RejectCode::CanisterError)
    );
    assert_eq!(f.usage(f.first), Ok(usage(3, 3, 3)));
    f.replace_archive_bytes(vec![]);
    let failed = f
        .harness
        .pic
        .query_call(
            f.service,
            f.operator,
            "authority_archive",
            candid::encode_args(()).unwrap(),
        )
        .expect_err("missing archive rejects inspection");
    assert_eq!(failed.reject_code, RejectCode::CanisterError);
    // Repair external fault injection before any legitimate mutation.
    f.replace_archive_bytes(saved);
    assert_eq!(f.archive(), current);
}

#[test]
fn terminal_digest_failure_and_full_lifetime_root_history_are_archived() {
    let f = Fixture::new();
    let v = chunks::vector("abc-text", 1);
    let bad = JourneyUpload {
        digest: [9; 32],
        ..v.upload
    };
    assert_eq!(f.reserve_manifest(f.first, bad, v.manifest), Ok(()));
    assert_eq!(
        f.append(f.first, bad, 0, &v.bytes),
        Err(JourneyFailure::ContentMismatch)
    );
    let archived = f.archive();
    assert_eq!(
        object(&archived, ArchiveCatalog::Journey, bad.root)
            .progress
            .unwrap()
            .verification,
        JourneyVerification::Rejected
    );
    assert_eq!(f.control(f.first, "journey_cancel", bad), Ok(()));
    let names = [
        "abc-binary",
        "abc-no-metadata",
        "ecmascript-metadata",
        "pattern-1",
        "pattern-1048577",
        "pattern-2097152",
        "uneven-tree-with-metadata",
    ];
    for (index, name) in names.into_iter().enumerate() {
        let (tenant, id) = if index < 3 {
            (f.first, index + 2)
        } else {
            (f.second, index - 2)
        };
        let v = chunks::vector(name, u8::try_from(id).unwrap());
        assert_eq!(
            f.reserve_manifest(tenant, v.upload, v.manifest.clone()),
            Ok(())
        );
        assert_eq!(f.control(tenant, "journey_cancel", v.upload), Ok(()));
        let archive = f.archive();
        let entry = object(&archive, ArchiveCatalog::Journey, v.upload.root);
        assert_eq!(
            (entry.phase, entry.logical, entry.physical, entry.liability),
            (ArchivePhase::Cancelled, 0, 0, 0)
        );
        assert_eq!(entry.manifest, Some(v.manifest));
    }
    let full = f.archive();
    let next = upload(5, b"full");
    assert_eq!(f.reserve(f.first, next), Err(JourneyFailure::Limit));
    assert_eq!(f.archive(), full);
}

#[test]
fn successful_and_reentrant_syncs_archive_sequences_and_current_membership() {
    let f = Fixture::with_source_operator(true);
    let query = || -> AuthorityArchiveView {
        let value: Option<AuthorityArchiveView> = f
            .harness
            .pic
            .query_candid_as(f.service, f.gateway, "authority_archive", ())
            .unwrap();
        value.unwrap()
    };
    let initial = query();
    assert_eq!((initial.last_sync, initial.pending_sync), (0, None));
    for (mode, expected) in [
        (SourceMode::Valid, Ok(())),
        (SourceMode::Overlap, Ok(())),
        (SourceMode::Revoke, Err(SyncFailure::Stale)),
    ] {
        let configured: bool = f
            .harness
            .pic
            .update_candid_as(f.gateway, f.operator, "configure", (mode,))
            .unwrap();
        assert!(configured);
        let result: Result<(), SyncFailure> = f
            .harness
            .pic
            .update_candid_as(f.gateway, f.operator, "run_sync", ())
            .unwrap();
        assert_eq!(result, expected);
    }
    let after = query();
    assert_eq!((after.last_sync, after.pending_sync), (3, None));
    assert_eq!(after.objects, initial.objects);
    assert!(after.gateways.is_empty());
}

fn assert_sample_history(archive: &AuthorityArchiveView) {
    let sample = object(archive, ArchiveCatalog::Samples, [1; 32]);
    assert_eq!(
        (
            sample.phase,
            sample.logical,
            sample.physical,
            sample.liability,
            sample.release_receipt
        ),
        (ArchivePhase::ProviderDeleted, 0, 0, 100, Some(true))
    );
    assert_eq!(
        object(archive, ArchiveCatalog::Samples, [2; 32]).phase,
        ArchivePhase::Live
    );
    for (id, phase) in [
        (11, ArchivePhase::Cancelled),
        (12, ArchivePhase::ExposurePossible),
        (13, ArchivePhase::Live),
        (14, ArchivePhase::Cancelled),
    ] {
        let entry = object(archive, ArchiveCatalog::Uploads, [id; 32]);
        assert_eq!(entry.phase, phase);
        assert!(entry.digest.is_some());
    }
}

fn assert_archive_usage(f: &Fixture, archive: &AuthorityArchiveView) {
    let totals = archive
        .objects
        .iter()
        .filter(|o| o.catalog == ArchiveCatalog::Journey && o.tenant == f.first)
        .fold((0_u128, 0_u128, 0_u128), |(l, p, b), o| {
            (
                l + u128::from(o.logical),
                p + u128::from(o.physical),
                b + u128::from(o.liability),
            )
        });
    assert_eq!(f.usage(f.first), Ok(usage(totals.0, totals.1, totals.2)));
}
