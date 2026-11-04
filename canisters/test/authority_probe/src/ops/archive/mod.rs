//! Atomic archive capture and inspection-only same-release restoration.
pub(crate) mod recovery;
mod storage;
use super::durable::{Owner, context};
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
use ic_blob_storage::model::catalog::admission::UploadPhase;
use ic_blob_storage::model::lifecycle::LifecycleChange;
use ic_blob_storage_contracts::binding::ReferenceId;
use ic_blob_storage_contracts::reference::binding::ReferenceOperation;
use ic_blob_storage_contracts::reference::binding::ReferenceRequest;
use ic_blob_storage_contracts::reference::binding::ReferenceRequestId;
use ic_blob_storage_contracts::upload::binding::UploadRequest;
use ic_blob_storage_contracts::upload::history::LifecyclePhase;
use ic_platform::Principal;

pub(super) fn create() {
    storage::create();
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
    if record.service != service.into() || record.operator != actor.into() {
        return None;
    }
    Some(record)
}

fn capture(state: &State) -> AuthorityArchiveRecord {
    assert_eq!(
        state.catalog.usage().unwrap().operations,
        2,
        "all sample objects covered"
    );
    assert_eq!(
        state.uploads.catalog.usage().unwrap().operations,
        state.uploads.requests.len()
    );
    assert_eq!(
        state.journey.catalog.usage().unwrap().operations,
        state.journey.requests.len()
    );
    let mut objects = vec![];
    for request in state.samples {
        objects.push(upload(ArchiveCatalog::Samples, &state.catalog, request));
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
        service: state.service.into(),
        operator: state.operator.into(),
        tenants: state.journey.tenants.map(Into::into),
        namespace: scope.namespace().get(),
        initial_gateway: state.gateway.into(),
        sync_source: scope.cashier().into(),
        gateways: state
            .registry
            .gateways()
            .principals()
            .iter()
            .copied()
            .map(Into::into)
            .collect(),
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

fn upload(catalog: ArchiveCatalog, owner: &Owner, request: UploadRequest) -> ObjectRecord {
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
        .lookup(context(binding.service(), binding.tenant()), request)
        .expect("retained request")
        .phase;
    let charged = matches!(phase, UploadPhase::Reserved | UploadPhase::Registered);
    let mut record = ObjectRecord {
        catalog,
        tenant: binding.tenant().into(),
        id,
        root: *request.object.root.as_bytes(),
        bytes: request.object.bytes,
        digest: None,
        phase: match phase {
            UploadPhase::Reserved => ArchivePhase::Reserved,
            UploadPhase::Registered => ArchivePhase::Live,
            UploadPhase::Cancelled => ArchivePhase::Cancelled,
        },
        logical: if charged { request.object.bytes } else { 0 },
        physical: if charged { request.object.bytes } else { 0 },
        liability: if charged { request.object.bytes } else { 0 },
        release_receipt: None,
        content: None,
    };
    if phase == UploadPhase::Registered {
        confirmed(&mut record, owner, request);
    }
    record
}

fn confirmed(record: &mut ObjectRecord, owner: &Owner, request: UploadRequest) {
    let binding = request.object.first.object();
    let ctx = context(binding.service(), record.tenant.into());
    let view = owner.confirmed(ctx, request).expect("confirmed object");
    record.phase = match view.phase {
        LifecyclePhase::Live => ArchivePhase::Live,
        LifecyclePhase::DeletionPending => ArchivePhase::DeletionPending,
        LifecyclePhase::ProviderDeleted => ArchivePhase::ProviderDeleted,
        LifecyclePhase::Settled => ArchivePhase::Settled,
    };
    record.logical = if view.phase == LifecyclePhase::Live {
        record.bytes
    } else {
        0
    };
    record.physical = if matches!(
        view.phase,
        LifecyclePhase::Live | LifecyclePhase::DeletionPending
    ) {
        record.bytes
    } else {
        0
    };
    record.liability = if view.phase == LifecyclePhase::Settled {
        0
    } else {
        record.bytes
    };
    let receipt = owner
        .reference_receipt(
            ctx,
            request,
            ReferenceRequest {
                id: ReferenceRequestId::new(number(1)),
                operation: ReferenceOperation::Release(request.object.first),
            },
        )
        .expect("exact tenant release request");
    record.release_receipt =
        receipt.map(|r| r.result.expect("fixture release") == LifecycleChange::Changed);
    assert_eq!(view.receipt_slots, u64::from(receipt.is_some()));
    assert_eq!(view.reference_slots, 1);
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
        object.tenant.into(),
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
