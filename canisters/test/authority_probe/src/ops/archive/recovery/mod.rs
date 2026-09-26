//! Restore the complete journal into a permanently fenced inspection owner.
//! Disposable catalogs validate records through shared transitions. They are
//! never installed as live state, and no sync/read token is minted from a backup.
use std::cell::RefCell;

use blob_test_protocol::{
    authority::{ArchiveCatalog, ArchivePhase},
    journey::{JourneyManifest, JourneyUpload, JourneyVerification},
};
use candid::Principal;
use ic_blob_storage::model::{
    catalog::admission::{UploadAdmission, UploadCatalog, UploadRequest},
    gateway::{GatewayListLimits, membership::GatewayMembership},
    lifecycle::{
        LifecycleChange,
        requests::{
            ReferenceOperation, ReferenceRequest, ReferenceRequestId, ReferenceRequestOutcome,
        },
    },
};

use crate::{
    model::{archive::AuthorityArchiveRecord, content::ContentSession},
    ops::{self, journey::VerifiedUpload, number},
};

thread_local! {
    static RESTORED: RefCell<Option<AuthorityArchiveRecord>> = const { RefCell::new(None) };
}

pub(crate) fn is_fenced() -> bool {
    RESTORED.with_borrow(Option::is_some)
}

pub(crate) fn require_active() {
    assert!(
        !RESTORED.with_borrow(Option::is_some),
        "restored authority is fenced"
    );
}

pub(super) fn retained() -> Option<AuthorityArchiveRecord> {
    RESTORED.with_borrow(Clone::clone)
}

pub(super) fn inspection_allowed(service: Principal, actor: Principal) -> bool {
    RESTORED.with_borrow(|record| match record {
        Some(record) => record.service == service && record.operator == actor,
        None => ops::read(|state| state.catalog.service() == service && state.operator == actor),
    })
}

pub(crate) fn prepare() {
    if RESTORED.with_borrow(Option::is_some) {
        return;
    }
    ops::read(|state| {
        assert!(
            state.registry.sync_view().pending_sequence.is_none(),
            "pending sync"
        );
        assert!(!state.journey.reads.busy(), "pending read");
        assert!(!state.balance.busy(), "pending balance read");
        // Every mutation already saved atomically. Do not replace stable history
        // here: incoming validation must detect missing or corrupt checkpoints.
    });
}

pub(crate) fn restore(service: Principal) {
    super::storage::open();
    let mut record = super::storage::load();
    assert_eq!(record.service, service, "same service required");
    validate(&record).expect("complete same-release authority journal");
    record.fenced = true;
    super::storage::save(&record);
    RESTORED.with_borrow_mut(|owner| *owner = Some(record));
}

/// Validate all stored obligations without granting them operational authority.
fn validate(record: &AuthorityArchiveRecord) -> Option<()> {
    if !record.bounded() || record.namespace != 1 || record.sync_source != record.operator {
        return None;
    }
    let [first, second] = record.tenants;
    let mut copy = ops::fresh(
        record.service,
        first,
        second,
        record.initial_gateway,
        record.operator,
    );
    let mut membership = GatewayMembership::new(GatewayListLimits {
        max_entries: ops::bound(1),
        max_unique: ops::bound(1),
    });
    for gateway in &record.gateways {
        membership.add(*gateway).ok()?;
    }
    if record
        .pending_sync
        .is_some_and(|token| token == 0 || token != record.last_sync)
    {
        return None;
    }
    samples(record, &mut copy)?;
    journey(record, &mut copy)?;
    // Compare every identity, counter, receipt, phase, manifest and checkpoint.
    // No missing/extra sample or duplicate entry can disappear during rebuild.
    if super::capture(&copy).objects != record.objects {
        return None;
    }
    record.pending_valid().then_some(())
}

fn samples(record: &AuthorityArchiveRecord, copy: &mut ops::State) -> Option<()> {
    // Samples have fixed identities and only expose release/deletion/settlement.
    let baseline = super::capture(copy);
    for sample in baseline
        .objects
        .iter()
        .filter(|o| o.catalog == ArchiveCatalog::Samples)
    {
        let retained = record
            .objects
            .iter()
            .find(|o| o.catalog == sample.catalog && o.root == sample.root)?;
        let reference = ops::reference(sample.id, record.service, sample.tenant)?;
        if retained.phase != ArchivePhase::Live {
            copy.catalog
                .apply_reference(reference.root, sample.tenant, release(reference.reference))
                .ok()?;
        }
        if matches!(
            retained.phase,
            ArchivePhase::ProviderDeleted | ArchivePhase::Settled
        ) {
            copy.catalog
                .confirm_provider_deleted(reference.root, reference.reference.object())
                .ok()?;
        }
        if retained.phase == ArchivePhase::Settled {
            copy.catalog
                .confirm_billing_stopped(reference.root, reference.reference.object())
                .ok()?;
        }
    }
    for request in copy.uploads.requests {
        let retained = record.objects.iter().find(|o| {
            o.catalog == ArchiveCatalog::Uploads && o.root == *request.object.root.as_bytes()
        })?;
        if retained.phase == ArchivePhase::Cancelled {
            copy.uploads
                .catalog
                .cancel(request.object.first.object().tenant(), request)
                .ok()?;
        }
    }
    Some(())
}

fn journey(record: &AuthorityArchiveRecord, copy: &mut ops::State) -> Option<()> {
    // Rebuild finished entries first so temporary reservation charges cannot
    // reject valid history whose capacity has since been reused. Ordering within
    // the retained journal is restored before exact comparison below.
    let mut entries: Vec<_> = record
        .objects
        .iter()
        .filter(|o| o.catalog == ArchiveCatalog::Journey)
        .collect();
    entries.sort_by_key(|o| match o.phase {
        ArchivePhase::Settled | ArchivePhase::Cancelled => 0,
        ArchivePhase::ProviderDeleted => 1,
        ArchivePhase::DeletionPending => 2,
        _ => 3,
    });
    for entry in entries {
        if !record.tenants.contains(&entry.tenant) {
            return None;
        }
        let request = ops::journey::request(
            record.service,
            entry.tenant,
            JourneyUpload {
                id: entry.id,
                root: entry.root,
                bytes: entry.bytes,
                digest: entry.digest?,
            },
        )
        .ok()?;
        let retained = entry.content.as_ref()?;
        let manifest_input = JourneyManifest {
            chunks: retained.chunks.clone(),
            headers: retained.headers.clone(),
        };
        let manifest = ops::journey::manifest(request, &manifest_input).ok()?;
        let content = ContentSession::from_record(manifest, request.content, retained)?;
        if !matches!(
            entry.phase,
            ArchivePhase::Reserved | ArchivePhase::Cancelled
        ) && retained.verdict != JourneyVerification::Verified
        {
            return None;
        }
        replay(&mut copy.journey.catalog, request, entry.phase)?;
        copy.journey.requests.push(VerifiedUpload {
            request,
            content,
            manifest_input,
        });
    }
    copy.journey.requests.sort_by_key(|entry| {
        record.objects.iter().position(|o| {
            o.catalog == ArchiveCatalog::Journey && o.root == *entry.request.object.root.as_bytes()
        })
    });
    Some(())
}

fn release(
    reference: ic_blob_storage::model::lifecycle::binding::ReferenceKey,
) -> ReferenceRequest {
    ReferenceRequest {
        id: ReferenceRequestId::new(number(1)),
        operation: ReferenceOperation::Release(reference),
    }
}

fn replay(catalog: &mut UploadCatalog, request: UploadRequest, phase: ArchivePhase) -> Option<()> {
    let tenant = request.object.first.object().tenant();
    if catalog.reserve(tenant, request).ok()? != UploadAdmission::Reserved {
        return None;
    }
    match phase {
        ArchivePhase::Reserved => return Some(()),
        ArchivePhase::Cancelled => {
            catalog.cancel(tenant, request).ok()?;
            return Some(());
        }
        _ => {}
    }
    catalog.mark_exposure_possible(tenant, request).ok()?;
    if phase == ArchivePhase::ExposurePossible {
        return Some(());
    }
    catalog.confirm_upload(request).ok()?;
    if phase == ArchivePhase::Live {
        return Some(());
    }
    if catalog
        .apply_reference(request.object.root, tenant, release(request.object.first))
        .ok()?
        != (ReferenceRequestOutcome::Recorded {
            result: Ok(LifecycleChange::Changed),
        })
    {
        return None;
    }
    if phase == ArchivePhase::DeletionPending {
        return Some(());
    }
    catalog
        .confirm_provider_deleted(request.object.root, request.object.first.object())
        .ok()?;
    if phase == ArchivePhase::Settled {
        catalog
            .confirm_billing_stopped(request.object.root, request.object.first.object())
            .ok()?;
    }
    Some(())
}

#[cfg(test)]
mod tests;
