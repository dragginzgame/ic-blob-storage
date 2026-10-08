//! Shared declaration preparation/recovery; tenant and uploader roles stay distinct.
use crate::ops::service::uploads::StableUploads;
use crate::ops::service::uploads::manifests;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestMutation;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestRequest;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestResponse;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;
/// Prepare as the actual admitted uploader after bounding ingress decoding. Raw
/// declaration budgets precede conversion; the shared owner checks activation,
/// time, phase and root/metadata consistency. Persist intent before dispatch and
/// propagate stable-write traps for IC rollback. No file or provider call is sent.
/// # Errors
/// Refuses wrong/full changed permissions, fences and invalid/oversized declarations.
pub fn prepare<M: Memory>(
    store: &mut StableUploads<M>,
    context: UploadContext,
    input: &UploadManifestRequest,
    now: u64,
) -> Result<UploadManifestMutation, UploadManifestFailure> {
    manifests::prepare(store, context, input, now)
}
/// Recover the original bounded declaration as tenant or original uploader, even
/// after expiry/revocation/completion or restore. Explicit Unprepared is not retry
/// authority; Prepared grants no certificate, content verification or publication.
/// # Errors
/// Rejects foreign, missing or changed permissions and inconsistent stored state.
pub fn inspect<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadManifestResponse, UploadManifestFailure> {
    manifests::inspect(store, context, input)
}
