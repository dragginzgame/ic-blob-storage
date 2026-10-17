//! Direct exposure access for isolated interruption, resource and rollback fixtures.
use crate::ops::service::uploads::{StableUploads, UploadStoreError};
pub use crate::workflow::uploads::exposure::boundary::{
    UploadExposureResult, commit, inspect_preparation,
};
pub use crate::workflow::uploads::exposure::inspect;
use ic_blob_storage_contracts::upload::binding::{UploadContext, UploadRequest};
use ic_memory::Memory;

/// Record exposure bookkeeping without minting a certificate, for resource fixtures.
/// No provider or host-evidence claim is established by this test-only access.
/// # Errors
/// Preserves the owner's authority, phase, manifest, clock and recovery refusals.
/// # Panics
/// Stable-write traps must propagate for IC rollback.
pub fn record<M: Memory>(
    store: &mut StableUploads<M>,
    context: UploadContext,
    request: UploadRequest,
    now: u64,
) -> Result<(), UploadStoreError> {
    store.expose(context, request, now)
}
