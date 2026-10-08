//! Atomic archive capture and inspection-only same-release restoration.
pub(crate) mod recovery;
mod storage;
use super::{State, number};
use crate::model::{
    archive::{AuthorityArchiveRecord, ContentRecord, ObjectRecord},
    content::ContentVerdict,
};
use blob_test_protocol::{
    authority::{
        ArchiveCatalog, ArchivePhase, ArchivedObjectView, ArchivedReadView, AuthorityArchiveView,
    },
    journey::{JourneyManifest, JourneyProgress, JourneyVerification},
};
use candid::Principal;
use ic_blob_storage::model::catalog::BlobCatalog;
use ic_blob_storage::model::catalog::admission::UploadCatalog;
use ic_blob_storage::model::catalog::admission::UploadPhase;
use ic_blob_storage::model::lifecycle::LifecycleChange;
use ic_blob_storage_contracts::binding::ReferenceId;
use ic_blob_storage_contracts::binding::ReferenceKey;
use ic_blob_storage_contracts::reference::binding::ReferenceOperation;
use ic_blob_storage_contracts::reference::binding::ReferenceRequest;
use ic_blob_storage_contracts::reference::binding::ReferenceRequestId;
use ic_blob_storage_contracts::upload::binding::UploadRequest;
use ic_blob_storage_contracts::upload::history::LifecyclePhase;

pub(super) fn open() {
    storage::open();
}

pub(super) fn save(state: &State) {
    let record = capture(state);
    assert!(record.bounded(), "bounded authority inspection archive");
    storage::save(&record);
}

pub(crate) fn inspect(service: Principal, actor: Principal) -> Option<AuthorityArchiveView> {
    load(service, actor).map(view)
}

/// Status describes the current owner, not independently injected older stable
/// evidence. A restored owner can only supply its validated frozen journal.
pub(crate) fn current(service: Principal, actor: Principal) -> Option<AuthorityArchiveRecord> {
    if !recovery::inspection_allowed(service, actor) {
        return None;
    }
    Some(recovery::retained().unwrap_or_else(|| super::read(capture)))
}

fn load(service: Principal, actor: Principal) -> Option<AuthorityArchiveRecord> {
    // Authorize using the current running instance before opening old evidence.
    if !recovery::inspection_allowed(service, actor) {
        return None;
    }
    // Once restored, inspection reads the validated frozen owner. Subsequent
    // external stable-memory replacement is not a live-state input.
    if let Some(record) = recovery::retained() {
        return Some(record);
    }
    // Reopen from stable memory, avoiding the live Cell's cached bytes. A query's
    // temporary host/runtime replacement cannot change the running update heap.
    storage::open();
    let record = storage::load();
    assert!(record.bounded(), "bounded authority inspection archive");
    if record.service != service || record.operator != actor {
        return None;
    }
    Some(record)
}

fn capture(state: &State) -> AuthorityArchiveRecord {
    assert_eq!(
        state.catalog.usage().objects,
        2,
        "all sample objects covered"
    );
    assert_eq!(
        state.uploads.catalog.usage().operations,
        state.uploads.requests.len()
    );
    assert_eq!(
        state.journey.catalog.usage().operations,
        state.journey.requests.len()
    );
    let mut objects = vec![];
    for id in [1, 2] {
        let journal = state
            .catalog
            .get(super::root(id))
            .expect("sample root retained");
        let bytes = if id == 1 { 100 } else { 200 };
        let mut record = ObjectRecord {
            catalog: ArchiveCatalog::Samples,
            tenant: journal.lifecycle().binding().tenant(),
            id,
            root: [id; 32],
            bytes,
            digest: None,
            phase: ArchivePhase::Live,
            logical: 0,
            physical: 0,
            liability: 0,
            release_receipt: None,
            content: None,
        };
        confirmed(&mut record, &state.catalog);
        objects.push(record);
    }
    for request in state.uploads.requests {
        objects.push(upload(
            ArchiveCatalog::Uploads,
            &state.uploads.catalog,
            request,
        ));
    }
    for entry in &state.journey.requests {
        let mut record = upload(
            ArchiveCatalog::Journey,
            &state.journey.catalog,
            entry.request.upload,
        );
        record.digest = Some(*entry.request.content.as_bytes());
        let (next_chunk, verified_bytes, verdict) = entry.content.progress();
        record.content = Some(ContentRecord {
            checkpoint: entry.content.checkpoint(),
            chunks: entry.manifest_input.chunks.clone(),
            headers: entry.manifest_input.headers.clone(),
            next_chunk,
            verified_bytes,
            verdict: match verdict {
                ContentVerdict::Pending => JourneyVerification::Pending,
                ContentVerdict::Verified => JourneyVerification::Verified,
                ContentVerdict::Rejected => JourneyVerification::Rejected,
            },
        });
        objects.push(record);
    }
    let scope = state.registry.scope();
    let sync = state.registry.sync_view();
    let (last_read, pending) = state.journey.reads.observation();
    let pending_read = pending.map(|(token, valid)| {
        let mut intent = state
            .journey
            .read_intent
            .clone()
            .expect("pending read intent");
        assert_eq!(intent.token, token, "exact pending intent");
        intent.valid = valid;
        intent
    });
    assert_eq!(pending.is_some(), state.journey.read_intent.is_some());
    AuthorityArchiveRecord {
        release: crate::model::archive::release_binding(),
        balance: state.balance.clone(),
        fenced: false,
        service: state.catalog.service(),
        operator: state.operator,
        tenants: state.journey.tenants,
        namespace: scope.namespace().get(),
        initial_gateway: state.gateway,
        sync_source: scope.cashier(),
        gateways: state.registry.gateways().principals().to_vec(),
        last_sync: sync.last_sequence,
        pending_sync: sync.pending_sequence,
        sync_control: state.sync_control.clone(),
        last_read,
        pending_read,
        armed_read_trap: state.journey.armed_read_trap.map(|root| *root.as_bytes()),
        trap_read_token: state.journey.trap_read_token,
        objects,
    }
}

fn upload(catalog: ArchiveCatalog, owner: &UploadCatalog, request: UploadRequest) -> ObjectRecord {
    let binding = request.object.first.object();
    let identity = binding.identity();
    assert_eq!(
        (identity.namespace.get(), identity.incarnation.get()),
        (1, 1)
    );
    assert_eq!(
        request.object.first.reference(),
        ReferenceId::new(number(1))
    );
    let id = u8::try_from(identity.object.get()).expect("fixture object ID");
    assert_eq!(
        request.id,
        ic_blob_storage_contracts::upload::binding::UploadRequestId::new(number(u128::from(id)))
    );
    let phase = owner
        .phase(binding.tenant(), request)
        .expect("retained request");
    let charged = matches!(phase, UploadPhase::Reserved | UploadPhase::ExposurePossible);
    let mut record = ObjectRecord {
        catalog,
        tenant: binding.tenant(),
        id,
        root: *request.object.root.as_bytes(),
        bytes: request.object.bytes,
        digest: None,
        phase: match phase {
            UploadPhase::Reserved => ArchivePhase::Reserved,
            UploadPhase::ExposurePossible => ArchivePhase::ExposurePossible,
            UploadPhase::Confirmed => ArchivePhase::Live,
            UploadPhase::Cancelled => ArchivePhase::Cancelled,
        },
        logical: if charged { request.object.bytes } else { 0 },
        physical: if charged { request.object.bytes } else { 0 },
        liability: if charged { request.object.bytes } else { 0 },
        release_receipt: None,
        content: None,
    };
    if phase == UploadPhase::Confirmed {
        confirmed(&mut record, owner.confirmed());
    }
    record
}

fn confirmed(record: &mut ObjectRecord, owner: &BlobCatalog) {
    let root =
        ic_blob_storage_contracts::identity::ProviderRootHash::try_from(record.root.as_slice())
            .expect("root");
    let journal = owner.get(root).expect("confirmed object");
    let lifecycle = journal.lifecycle();
    let binding = lifecycle.binding();
    assert_eq!(binding.service(), owner.service());
    assert_eq!(binding.tenant(), record.tenant);
    let identity = binding.identity();
    assert_eq!(
        (
            identity.namespace.get(),
            identity.object.get(),
            identity.incarnation.get()
        ),
        (1, u128::from(record.id), 1)
    );
    record.phase = match lifecycle.phase() {
        LifecyclePhase::Live => ArchivePhase::Live,
        LifecyclePhase::DeletionPending => ArchivePhase::DeletionPending,
        LifecyclePhase::ProviderDeleted => ArchivePhase::ProviderDeleted,
        LifecyclePhase::Settled => ArchivePhase::Settled,
    };
    record.logical = lifecycle.logical_bytes();
    record.physical = lifecycle.physical_bytes();
    record.liability = lifecycle.liability_bytes();
    let receipt = journal
        .receipt(
            record.tenant,
            ReferenceRequest {
                id: ReferenceRequestId::new(number(1)),
                operation: ReferenceOperation::Release(ReferenceKey::new(
                    lifecycle.binding(),
                    ReferenceId::new(number(1)),
                )),
            },
        )
        .expect("fixture has only the exact tenant release request");
    record.release_receipt =
        receipt.map(
            |receipt| match receipt.result.expect("valid fixture release") {
                LifecycleChange::Changed => true,
                LifecycleChange::Unchanged => false,
            },
        );
    // The fixture exposes only reference 1 and release request 1. Fail closed if
    // a future endpoint adds history this archive does not yet represent.
    assert_eq!(journal.receipt_count(), usize::from(receipt.is_some()));
    assert_eq!(lifecycle.reference_slots(), 1);
}

fn view(record: AuthorityArchiveRecord) -> AuthorityArchiveView {
    AuthorityArchiveView {
        fenced: record.fenced,
        service: record.service,
        operator: record.operator,
        tenants: record.tenants,
        namespace: record.namespace,
        initial_gateway: record.initial_gateway,
        sync_source: record.sync_source,
        gateways: record.gateways,
        last_sync: record.last_sync,
        pending_sync: record.pending_sync,
        sync_revision: record.sync_control.revision,
        last_read: record.last_read,
        pending_read: record.pending_read.map(|read| ArchivedReadView {
            token: read.token,
            valid: read.valid,
            tenant: read.tenant,
            root: read.root,
            index: read.index,
            gateway: read.gateway,
        }),
        armed_read_trap: record.armed_read_trap,
        trap_read_token: record.trap_read_token,
        objects: record
            .objects
            .into_iter()
            .map(|object| {
                let (manifest, progress) = match object.content {
                    Some(content) => (
                        Some(JourneyManifest {
                            chunks: content.chunks,
                            headers: content.headers,
                        }),
                        Some(JourneyProgress {
                            next_chunk: content.next_chunk,
                            verified_bytes: content.verified_bytes,
                            verification: content.verdict,
                        }),
                    ),
                    None => (None, None),
                };
                ArchivedObjectView {
                    catalog: object.catalog,
                    tenant: object.tenant,
                    id: object.id,
                    root: object.root,
                    bytes: object.bytes,
                    digest: object.digest,
                    phase: object.phase,
                    logical: object.logical,
                    physical: object.physical,
                    liability: object.liability,
                    release_receipt: object.release_receipt,
                    manifest,
                    progress,
                }
            })
            .collect(),
    }
}

pub(crate) fn probe(
    service: Principal,
    actor: Principal,
    input: &blob_test_protocol::authority::CheckpointProbeInput,
) -> Result<JourneyProgress, blob_test_protocol::authority::CheckpointProbeFailure> {
    use crate::model::content::{ContentError, ContentSession};
    use blob_test_protocol::authority::CheckpointProbeFailure as Failure;
    use blob_test_protocol::journey::{JourneyFailure, JourneyUpload};
    let record = load(service, actor).ok_or(Failure::Denied)?;
    let object = record
        .objects
        .iter()
        .find(|object| object.catalog == ArchiveCatalog::Journey && object.root == input.root)
        .ok_or(Failure::Unknown)?;
    let content = object.content.as_ref().ok_or(Failure::InvalidCheckpoint)?;
    let request = super::journey::request(
        service,
        object.tenant,
        JourneyUpload {
            id: object.id,
            root: object.root,
            digest: object.digest.ok_or(Failure::InvalidCheckpoint)?,
            bytes: object.bytes,
        },
    )
    .map_err(|_| Failure::InvalidCheckpoint)?;
    let manifest = super::journey::manifest(
        request,
        &JourneyManifest {
            chunks: content.chunks.clone(),
            headers: content.headers.clone(),
        },
    )
    .map_err(|_| Failure::InvalidCheckpoint)?;
    let mut copy = ContentSession::from_record(manifest, request.content, content)
        .ok_or(Failure::InvalidCheckpoint)?;
    copy.append(input.index, &input.bytes).map_err(|error| {
        Failure::Chunk(match error {
            ContentError::OutOfOrder => JourneyFailure::OutOfOrder,
            ContentError::Mismatch => JourneyFailure::ContentMismatch,
        })
    })?;
    let (next_chunk, verified_bytes, verdict) = copy.progress();
    Ok(JourneyProgress {
        next_chunk,
        verified_bytes,
        verification: match verdict {
            ContentVerdict::Pending => JourneyVerification::Pending,
            ContentVerdict::Verified => JourneyVerification::Verified,
            ContentVerdict::Rejected => JourneyVerification::Rejected,
        },
    })
}
