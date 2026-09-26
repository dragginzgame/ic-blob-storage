//! Execute diagnostics as actual queries, including while calls are outstanding.
use super::*;
use blob_test_protocol::status::{
    CatalogStatusView, OperatorBlockerView as Blocker, OperatorStatusView,
    OperatorWarningView as Warning,
};

impl Fixture {
    fn status(&self, actor: Principal) -> Option<OperatorStatusView> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "operator_status", ())
            .expect("read-only operator query")
    }

    fn status_unchanged(&self) -> OperatorStatusView {
        let authority = self.harness.pic.get_stable_memory(self.service);
        let source = self.harness.pic.get_stable_memory(self.gateway);
        let effects = self.source_recovery(self.operator);
        let before = self.status(self.authority_operator).unwrap();
        assert_eq!(self.status(self.authority_operator), Some(before.clone()));
        assert_eq!(self.harness.pic.get_stable_memory(self.service), authority);
        assert_eq!(self.harness.pic.get_stable_memory(self.gateway), source);
        assert_eq!(self.source_recovery(self.operator), effects);
        before
    }
}

fn catalog(view: &OperatorStatusView, owner: ArchiveCatalog) -> &CatalogStatusView {
    view.catalogs
        .iter()
        .find(|c| c.catalog == owner)
        .expect("catalog retained separately")
}

fn phase(view: &CatalogStatusView, phase: ArchivePhase) -> u64 {
    view.phases
        .iter()
        .find(|p| p.phase == phase)
        .unwrap()
        .objects
}

#[test]
fn operator_query_denies_controller_and_reports_missing_economics_without_effects() {
    let f = Fixture::with_source_operator(true);
    for actor in [
        f.operator,
        f.first,
        f.second,
        f.service,
        Principal::anonymous(),
    ] {
        assert_eq!(f.status(actor), None);
    }
    let view = f.status_unchanged();
    assert_eq!(
        (view.service, view.namespace, view.fenced),
        (f.service, 1, false)
    );
    assert!(!view.provider_qualified);
    assert!(!view.billing_configured);
    assert_eq!(view.provider_balance, None);
    assert_eq!(view.available_funding_cycles, None);
    assert_eq!(view.funding_activity, None);
    assert_eq!(
        view.blockers,
        vec![
            Blocker::ProviderUnqualified,
            Blocker::FundingUnknown,
            Blocker::BillingNotConfigured
        ]
    );
    assert_eq!(view.warnings, vec![Warning::UploadCompletionUnknown]);
    let samples = catalog(&view, ArchiveCatalog::Samples);
    assert_eq!(
        (
            samples.objects,
            samples.logical,
            samples.physical,
            samples.liability
        ),
        (2, 300, 300, 300)
    );
    assert_eq!(phase(samples, ArchivePhase::Live), 2);
    let uploads = catalog(&view, ArchiveCatalog::Uploads);
    assert_eq!(
        (
            uploads.objects,
            uploads.logical,
            uploads.physical,
            uploads.liability
        ),
        (4, 300, 300, 300)
    );
    assert_eq!(phase(uploads, ArchivePhase::ExposurePossible), 1);
    assert_eq!(phase(uploads, ArchivePhase::Cancelled), 1);
    let journey = catalog(&view, ArchiveCatalog::Journey);
    assert_eq!(
        (
            journey.objects,
            journey.logical,
            journey.physical,
            journey.liability
        ),
        (0, 0, 0, 0)
    );
    f.upgrade_authority();
    assert_eq!(f.status(f.operator), None);
    let restored = f.status_unchanged();
    assert!(restored.fenced);
    assert_eq!(restored.catalogs, view.catalogs);
    assert!(restored.blockers.contains(&Blocker::RecoveryFenced));
    f.assert_authority_fenced(upload(1, b"fenced"));
    assert_eq!(f.status_unchanged(), restored);
}

#[test]
fn diagnostic_counts_keep_logical_physical_and_billing_release_separate() {
    let f = Fixture::new();
    let v = chunks::vector("abc-text", 1);
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest.clone()),
        Ok(())
    );
    assert_eq!(
        phase(
            catalog(&f.status_unchanged(), ArchiveCatalog::Journey),
            ArchivePhase::Reserved
        ),
        1
    );
    assert_eq!(f.append(f.first, v.upload, 0, &v.bytes), Ok(()));
    f.certificate(f.first, v.upload.root).unwrap();
    assert_eq!(
        phase(
            catalog(&f.status_unchanged(), ArchiveCatalog::Journey),
            ArchivePhase::ExposurePossible
        ),
        1
    );
    assert_eq!(f.complete(f.first, v.upload), Ok(()));
    assert_eq!(
        f.root_control(f.first, "journey_release", v.upload.root),
        Ok(())
    );
    let released = f.status_unchanged();
    let entry = catalog(&released, ArchiveCatalog::Journey);
    assert_eq!(
        (
            entry.logical,
            entry.physical,
            entry.liability,
            entry.release_receipts
        ),
        (0, 3, 3, 1)
    );
    assert!(released.warnings.contains(&Warning::DeletionPending));
    assert_eq!(f.delete(vec![root(v.upload.root)]), Ok(()));
    let deleted = f.status_unchanged();
    let entry = catalog(&deleted, ArchiveCatalog::Journey);
    assert_eq!((entry.logical, entry.physical, entry.liability), (0, 0, 3));
    assert!(!deleted.warnings.contains(&Warning::DeletionPending));
    assert!(deleted.warnings.contains(&Warning::BillingCessationPending));
    assert_eq!(
        f.root_control(f.operator, "journey_settle", v.upload.root),
        Ok(())
    );
    let settled = f.status_unchanged();
    let entry = catalog(&settled, ArchiveCatalog::Journey);
    assert_eq!(
        (
            entry.objects,
            entry.logical,
            entry.physical,
            entry.liability,
            entry.release_receipts
        ),
        (1, 0, 0, 0, 1)
    );
    assert_eq!(phase(entry, ArchivePhase::Settled), 1);
    assert!(!settled.warnings.contains(&Warning::BillingCessationPending));
    f.upgrade_authority();
    assert_eq!(f.status_unchanged().catalogs, settled.catalogs);
}

#[test]
fn old_or_missing_stable_evidence_cannot_replace_live_status_before_restore() {
    let f = Fixture::new();
    let old = f.harness.pic.get_stable_memory(f.service);
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    let current = f.status_unchanged();
    f.replace_archive_bytes(vec![]);
    assert_eq!(f.status_unchanged(), current);
    f.replace_archive_bytes(old);
    assert_eq!(f.status_unchanged(), current);
    assert!(
        !f.archive()
            .objects
            .iter()
            .any(|o| o.catalog == ArchiveCatalog::Journey)
    );
    f.upgrade_skipping_outgoing_hook(f.service, true);
    let restored = f.status_unchanged();
    assert!(restored.fenced);
    assert!(restored.blockers.contains(&Blocker::RecoveryFenced));
    assert_eq!(catalog(&restored, ArchiveCatalog::Journey).objects, 0);
    f.assert_authority_fenced(v.upload);
}

#[test]
fn pending_sync_and_invalidated_reads_remain_visible_without_diagnostic_retry() {
    let f = Fixture::with_source_operator(true);
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, true);
    let read = f.hold_read(v.upload);
    let sync = f.hold_sync();
    let pending = f.status_unchanged();
    assert_eq!(pending.pending_sync, Some(1));
    assert!(pending.pending_read.as_ref().unwrap().valid);
    assert!(pending.warnings.contains(&Warning::SyncPending));
    assert!(pending.warnings.contains(&Warning::ReadPending));
    assert_eq!(f.list_observation().requests, 1);
    assert_eq!(f.source_observation().requests, 1);
    let memory = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(f.resume_sync(sync), Ok(()));
    let stale = f.status_unchanged();
    assert_eq!(stale.pending_sync, None);
    assert!(!stale.pending_read.as_ref().unwrap().valid);
    assert!(stale.warnings.contains(&Warning::ReadPending));
    assert_eq!(f.resume_read(read), Err(JourneyFailure::StaleRead));
    assert_eq!(f.status_unchanged().pending_read, None);
    let revoked: bool = f
        .harness
        .pic
        .update_candid_as(f.service, f.authority_operator, "revoke_gateway", ())
        .unwrap();
    assert!(revoked);
    let no_gateway = f.status_unchanged();
    assert!(no_gateway.gateways.is_empty());
    assert!(no_gateway.blockers.contains(&Blocker::GatewaysMissing));
    f.replace_archive_bytes(memory);
    f.upgrade_skipping_outgoing_hook(f.service, true);
    let restored = f.status_unchanged();
    assert_eq!(restored.pending_sync, pending.pending_sync);
    assert_eq!(restored.pending_read, pending.pending_read);
    assert_eq!(restored.catalogs, pending.catalogs);
    assert!(restored.blockers.contains(&Blocker::RecoveryFenced));
    assert_eq!(f.list_observation().requests, 1);
    assert_eq!(f.source_observation().requests, 1);
}
