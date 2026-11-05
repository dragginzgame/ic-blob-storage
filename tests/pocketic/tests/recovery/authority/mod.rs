//! Atomic stable inspection records, without pretending to resume authority.

use ic_blob_storage_pocketic_tests::fixture_calls;
mod capacity;
mod checkpoint;
mod restore;
mod status;
mod sync;
use super::*;
use blob_test_protocol::{
    SourceMode, SyncFailure,
    authority::{ArchiveCatalog, ArchivePhase, ArchivedObjectView, AuthorityArchiveView},
    obligations::ObligationProbeFact,
};
use ic_testkit::pocket_ic::common::rest::BlobCompression;

impl Fixture {
    pub(super) async fn archive(&self) -> AuthorityArchiveView {
        let result: Option<AuthorityArchiveView> = fixture_calls::query_fixture(
            &self.harness.pic,
            self.service,
            self.authority_operator,
            "authority_archive",
            (),
        )
        .await
        .map(|(reply,)| reply)
        .expect("stable archive query");
        result.expect("explicit operator")
    }
    async fn sample_fact(&self, fact: ObligationProbeFact) {
        let accepted: bool = fixture_calls::update_fixture(
            &self.harness.pic,
            self.service,
            self.authority_operator,
            "confirm_obligation",
            (1_u8, fact),
        )
        .await
        .map(|(reply,)| reply)
        .expect("sample fact");
        assert!(accepted);
    }

    async fn replace_archive_bytes(&self, bytes: Vec<u8>) {
        self.harness
            .pic
            .set_stable_memory(self.service, bytes, BlobCompression::NoCompression)
            .await
            .unwrap();
        // Direct PocketIC memory injection does not invalidate the IC query
        // cache. A real stateless update advances the instance version without
        // touching the archive, so the following query executes against the fault.
        let result: Result<
            blob_test_protocol::content::ContentProbeReport,
            blob_test_protocol::content::ContentProbeFailure,
        > = fixture_calls::update_fixture(
            &self.harness.pic,
            self.service,
            self.authority_operator,
            "probe_content",
            (blob_test_protocol::content::ContentProbeCase::UnicodeMetadata,),
        )
        .await
        .map(|(reply,)| reply)
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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One archive journey checks all catalogs and partial progress against the same original identities"
)]
async fn archive_preserves_all_catalogs_partial_progress_and_byte_free_history() {
    let f = Fixture::new().await;
    let partial = content_vectors::vector("pattern-1048577", 1);
    assert_eq!(
        f.reserve_manifest(f.first, partial.upload, partial.manifest.clone())
            .await,
        Ok(())
    );
    assert_eq!(
        f.append(f.first, partial.upload, 0, &partial.bytes[..CHUNK])
            .await,
        Ok(())
    );
    let deleted = content_vectors::vector("abc-text", 2);
    f.confirm_bytes(&deleted).await;
    assert_eq!(
        f.root_control(f.first, "journey_release", deleted.upload.root)
            .await,
        Ok(())
    );
    assert_eq!(f.delete(vec![root(deleted.upload.root)]).await, Ok(()));
    let cancelled = upload(3, b"cancelled");
    assert_eq!(f.reserve(f.first, cancelled, b"cancelled").await, Ok(()));
    assert_eq!(
        f.control(f.first, "journey_cancel", cancelled).await,
        Ok(())
    );
    let (released,): (bool,) =
        fixture_calls::update_fixture(&f.harness.pic, f.service, f.first, "release", (1_u8,))
            .await
            .unwrap();
    assert!(released);
    f.sample_fact(ObligationProbeFact::Deleted).await;
    let (sample_cancel,): (bool,) = fixture_calls::update_fixture(
        &f.harness.pic,
        f.service,
        f.first,
        "cancel_upload",
        (11_u8,),
    )
    .await
    .unwrap();
    assert!(sample_cancel);
    let archive = f.archive().await;
    assert_eq!(archive.service, f.service);
    assert_eq!(archive.operator, f.operator);
    assert_eq!(archive.tenants, [f.first, f.second]);
    assert_eq!(archive.namespace, 1);
    assert_sample_history(&archive);
    let entry = object(&archive, ArchiveCatalog::Journey, partial.upload.root);
    assert_eq!(entry.manifest, Some(partial.manifest));
    assert_eq!(
        entry.progress,
        Some(f.progress(f.first, partial.upload).await.unwrap())
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
    assert_archive_usage(&f, &archive).await;
    f.restart(f.service).await;
    assert_eq!(f.archive().await, archive);
    f.sample_fact(ObligationProbeFact::BillingStopped).await;
    assert_eq!(
        f.root_control(f.operator, "journey_settle", deleted.upload.root)
            .await,
        Ok(())
    );
    let settled = f.archive().await;
    for (catalog, root) in [
        (ArchiveCatalog::Samples, sample_object(&settled, 1).root),
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
    let mut retained = f.archive().await;
    f.restore(f.service).await;
    retained.fenced = true;
    assert_eq!(f.archive().await, retained);
    f.assert_authority_fenced(partial.upload).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn invalid_deletion_batch_rolls_back_prior_archive_writes_and_replay_preserves_receipts() {
    let f = Fixture::new().await;
    let v = content_vectors::vector("abc-text", 1);
    f.confirm_bytes(&v).await;
    assert_eq!(
        f.root_control(f.first, "journey_release", v.upload.root)
            .await,
        Ok(())
    );
    let before = f.archive().await;
    assert!(
        f.delete(vec![root(v.upload.root), vec![99; 32]])
            .await
            .is_err()
    );
    assert_eq!(f.archive().await, before);
    assert_eq!(f.delete(vec![root(v.upload.root)]).await, Ok(()));
    let deleted = f.archive().await;
    assert_eq!(f.delete(vec![root(v.upload.root)]).await, Ok(()));
    assert_eq!(
        f.root_control(f.first, "journey_release", v.upload.root)
            .await,
        Ok(())
    );
    assert_eq!(f.archive().await, deleted);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn callback_trap_preserves_exact_stable_read_intent_and_revocation_marks_it_stale() {
    let f = Fixture::new().await;
    let v = content_vectors::vector("abc-text", 1);
    f.confirm_bytes(&v).await;
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, true)
        .await;
    let (armed,): (bool,) = fixture_calls::update_fixture(
        &f.harness.pic,
        f.service,
        f.operator,
        "journey_arm_read_callback_trap",
        (root(v.upload.root),),
    )
    .await
    .unwrap();
    assert!(armed);
    assert_eq!(f.archive().await.armed_read_trap, Some(v.upload.root));
    let id = f.hold_read(v.upload).await;
    let before = f.archive().await;
    let read = before.pending_read.as_ref().unwrap();
    assert_eq!(
        (read.tenant, read.root, read.index, read.gateway),
        (f.first, v.upload.root, 0, f.gateway)
    );
    assert_eq!(before.trap_read_token, Some(read.token));
    assert_eq!(before.last_read, read.token);
    assert!(read.valid);
    assert_eq!(before.armed_read_trap, None);
    let (resumed,): (bool,) =
        fixture_calls::update_fixture(&f.harness.pic, f.gateway, f.operator, "resume_read", ())
            .await
            .unwrap();
    assert!(resumed);
    let error = f
        .harness
        .pic
        .await_call(id)
        .await
        .expect_err("actual read callback trap");
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(f.archive().await, before);
    let (revoked,): (bool,) =
        fixture_calls::update_fixture(&f.harness.pic, f.service, f.operator, "revoke_gateway", ())
            .await
            .unwrap();
    assert!(revoked);
    let after = f.archive().await;
    assert_eq!(after.gateways, []);
    let pending = after.pending_read.as_ref().unwrap();
    assert!(!pending.valid);
    assert_eq!(pending.token, read.token);
    f.restart(f.service).await;
    assert_eq!(f.archive().await, after);
    assert_eq!(f.source_observation().await.requests, 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn stable_archive_is_operator_only_and_old_or_missing_bytes_never_change_live_authority() {
    let f = Fixture::with_source_operator(true).await;
    // Here the source is the explicit operator; the human controller is not.
    for caller in [
        f.operator,
        f.first,
        f.second,
        Principal::anonymous(),
        f.service,
    ] {
        let denied: Option<AuthorityArchiveView> = fixture_calls::query_fixture(
            &f.harness.pic,
            f.service,
            caller,
            "authority_archive",
            (),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
        assert!(denied.is_none());
    }
    let (initial,): (Option<AuthorityArchiveView>,) = fixture_calls::query_fixture(
        &f.harness.pic,
        f.service,
        f.gateway,
        "authority_archive",
        (),
    )
    .await
    .unwrap();
    assert!(initial.is_some());
    let f = Fixture::new().await;
    let initial = f.archive().await;
    let old = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let v = content_vectors::vector("abc-text", 1);
    f.confirm_bytes(&v).await;
    let current = f.archive().await;
    let saved = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    f.replace_archive_bytes(old).await;
    assert_eq!(f.archive().await, initial);
    assert_eq!(
        f.certificate(f.first, v.upload.root).await,
        Err(RejectCode::CanisterError)
    );
    assert_eq!(f.usage(f.first).await, Ok(usage(3, 3, 3)));
    f.replace_archive_bytes(vec![]).await;
    let failed = f
        .harness
        .pic
        .query_call(
            f.service,
            f.operator,
            "authority_archive",
            blob_test_protocol::fixture_wire::encode_args("authority_archive", ()).unwrap(),
        )
        .await
        .expect_err("missing archive rejects inspection");
    assert_eq!(
        crate::canister_rejection(&failed).reject_code,
        RejectCode::CanisterError
    );
    // Repair external fault injection before any legitimate mutation.
    f.replace_archive_bytes(saved).await;
    assert_eq!(f.archive().await, current);
}

#[tokio::test(flavor = "multi_thread")]
async fn invalid_upload_metadata_cannot_allocate_capacity_or_change_the_archive() {
    let f = Fixture::new().await;
    let v = content_vectors::vector("ecmascript-metadata", 1);
    let before = f.archive().await;
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest).await,
        Err(JourneyFailure::InvalidInput)
    );
    assert_eq!(f.usage(f.first).await, Ok(usage(0, 0, 0)));
    assert_eq!(f.archive().await, before);
}

#[tokio::test(flavor = "multi_thread")]
async fn terminal_digest_failure_and_full_lifetime_root_history_are_archived() {
    let f = Fixture::new().await;
    let v = content_vectors::vector("abc-text", 1);
    let bad = JourneyUpload {
        digest: [9; 32],
        ..v.upload
    };
    assert_eq!(f.reserve_manifest(f.first, bad, v.manifest).await, Ok(()));
    assert_eq!(
        f.append(f.first, bad, 0, &v.bytes).await,
        Err(JourneyFailure::ContentMismatch)
    );
    let archived = f.archive().await;
    assert_eq!(
        object(&archived, ArchiveCatalog::Journey, bad.root)
            .progress
            .unwrap()
            .verification,
        JourneyVerification::Rejected
    );
    assert_eq!(f.control(f.first, "journey_cancel", bad).await, Ok(()));
    let names = [
        "abc-binary",
        "abc-no-metadata",
        "media-1048577",
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
        let v = content_vectors::vector(name, u8::try_from(id).unwrap());
        assert_eq!(
            f.reserve_manifest(tenant, v.upload, v.manifest.clone())
                .await,
            Ok(())
        );
        assert_eq!(f.control(tenant, "journey_cancel", v.upload).await, Ok(()));
        let archive = f.archive().await;
        let entry = object(&archive, ArchiveCatalog::Journey, v.upload.root);
        assert_eq!(
            (entry.phase, entry.logical, entry.physical, entry.liability),
            (ArchivePhase::Cancelled, 0, 0, 0)
        );
        assert_eq!(entry.manifest, Some(v.manifest));
    }
    let full = f.archive().await;
    let next = upload(5, b"full");
    assert_eq!(
        f.reserve(f.first, next, b"full").await,
        Err(JourneyFailure::Limit)
    );
    assert_eq!(f.archive().await, full);
    let mut full = full;
    f.restore(f.service).await;
    full.fenced = true;
    assert_eq!(f.archive().await, full);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn successful_and_reentrant_syncs_archive_sequences_and_current_membership() {
    let f = Fixture::with_source_operator(true).await;
    let query = async || -> AuthorityArchiveView {
        let value: Option<AuthorityArchiveView> = fixture_calls::query_fixture(
            &f.harness.pic,
            f.service,
            f.gateway,
            "authority_archive",
            (),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
        value.unwrap()
    };
    let initial = query().await;
    assert_eq!((initial.last_sync, initial.pending_sync), (0, None));
    for (mode, expected) in [
        (SourceMode::Valid, Ok(())),
        (SourceMode::Overlap, Ok(())),
        (SourceMode::Revoke, Err(SyncFailure::Stale)),
    ] {
        let configured: bool = fixture_calls::update_fixture(
            &f.harness.pic,
            f.gateway,
            f.operator,
            "configure",
            (mode,),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
        assert!(configured);
        let result: Result<(), SyncFailure> = fixture_calls::update_fixture(
            &f.harness.pic,
            f.gateway,
            f.operator,
            "run_sync",
            (f.sync_request().await,),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
        assert_eq!(result, expected);
    }
    let after = query().await;
    assert_eq!((after.last_sync, after.pending_sync), (3, None));
    assert_eq!(after.objects, initial.objects);
    assert_eq!(after.gateways, []);
}

fn sample_object(archive: &AuthorityArchiveView, id: u8) -> &ArchivedObjectView {
    archive
        .objects
        .iter()
        .find(|o| o.catalog == ArchiveCatalog::Samples && o.id == id)
        .expect("sample identity")
}

fn assert_sample_history(archive: &AuthorityArchiveView) {
    let sample = sample_object(archive, 1);
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
    assert_eq!(sample_object(archive, 2).phase, ArchivePhase::Live);
    for (id, phase) in [
        (11, ArchivePhase::Cancelled),
        (12, ArchivePhase::Live),
        (13, ArchivePhase::Live),
        (14, ArchivePhase::Cancelled),
    ] {
        let entry = archive
            .objects
            .iter()
            .find(|o| o.catalog == ArchiveCatalog::Uploads && o.id == id)
            .expect("initial upload identity");
        assert_eq!(entry.phase, phase);
    }
}

async fn assert_archive_usage(f: &Fixture, archive: &AuthorityArchiveView) {
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
    assert_eq!(
        f.usage(f.first).await,
        Ok(usage(totals.0, totals.1, totals.2))
    );
}
