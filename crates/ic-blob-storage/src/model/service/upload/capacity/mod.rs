//! Reserve future leaf-array capacity from the exact admitted content length.

use super::UploadAdmissionError;
use ic_blob_storage_contracts::identity::caffeine::CAFFEINE_CHUNK_BYTES;

/// Which retained manifest budget rejected fresh admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadManifestLimit {
    /// Sum of all tenants' lifetime admitted chunk slots.
    Global,
    /// One tenant's lifetime admitted chunk slots.
    Tenant,
}

fn chunks(bytes: u64) -> u64 {
    bytes.div_ceil(CAFFEINE_CHUNK_BYTES as u64)
}

pub(crate) fn remaining(
    limits: ic_blob_storage_contracts::configuration::service::ServiceManifestLimits,
    global: u64,
    tenant: u64,
) -> u64 {
    (limits.max_chunks.get() as u64 - global).min(limits.max_tenant_chunks.get() as u64 - tenant)
}

pub(crate) fn check(
    limits: ic_blob_storage_contracts::configuration::service::ServiceManifestLimits,
    global: u64,
    tenant: u64,
    bytes: u64,
) -> Result<(), UploadAdmissionError> {
    let requested = chunks(bytes);
    if tenant + requested > limits.max_tenant_chunks.get() as u64 {
        return Err(UploadAdmissionError::ManifestCapacity(
            UploadManifestLimit::Tenant,
        ));
    }
    if global + requested > limits.max_chunks.get() as u64 {
        return Err(UploadAdmissionError::ManifestCapacity(
            UploadManifestLimit::Global,
        ));
    }
    Ok(())
}
