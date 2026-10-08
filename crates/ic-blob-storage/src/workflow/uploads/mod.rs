//! Shared upload admission, preparation, guarded issuance and historical inspection.
pub mod admission;
pub mod capacity;
pub mod certificate;
pub mod completion;
pub mod discovery;
pub mod exposure;
pub mod history;
pub mod manifests;
use crate::ops::service::uploads::StableUploads;
use crate::ops::service::uploads::status;
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::upload::UploadStatusFailure;
use ic_blob_storage_contracts::dto::upload::UploadStatusResponse;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;
/// Inspect the actual tenant's original upload, including during suspension and
/// restore fencing. Confirmed means the first reference was created historically;
/// it does not prove current liveness or operational restore authority. The adapter
/// supplies actual context and bounds ingress. No state, receipt or effect is created.
/// # Errors
/// Rejects foreign/malformed bindings, unknown uploads and changed original arguments.
pub fn inspect<M: Memory>(
    uploads: &StableUploads<M>,
    context: UploadContext,
    upload: ReferenceUpload,
) -> Result<UploadStatusResponse, UploadStatusFailure> {
    let request = ic_blob_storage_contracts::upload::status::parse(context, upload)?;
    let view = uploads.lookup(context, request).map_err(status::failure)?;
    Ok(status::present(upload, view.phase, view.revoked))
}
