//! Reserve future leaf-array capacity from the exact admitted content length.

use super::{UploadAdmissionError, UploadAdmissions};
use crate::model::identity::caffeine::CAFFEINE_CHUNK_BYTES;
use candid::Principal;

/// Which retained manifest budget rejected fresh admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadManifestLimit {
    /// Sum of all tenants' lifetime admitted chunk slots.
    Global,
    /// One tenant's lifetime admitted chunk slots.
    Tenant,
}

impl UploadAdmissions {
    pub(super) fn check_manifest_capacity(
        &self,
        tenant: Principal,
        bytes: u64,
    ) -> Result<(), UploadAdmissionError> {
        let requested = chunks(bytes);
        // Derive from the same immutable permissions as admission, avoiding a
        // separately mutable counter. Config bounds both map size and leaf count;
        // even their maximum portable product fits u64. Cancellation/settlement
        // cannot remove a permission and therefore cannot refund this capacity.
        let mut global = requested;
        let mut own = requested;
        for permission in self.permissions.values() {
            let object = permission.original.request.object;
            let retained = chunks(object.bytes);
            global += retained;
            if object.first.object().tenant() == tenant {
                own += retained;
            }
        }
        let limits = self.config.limits().manifests;
        if own > limits.max_tenant_chunks.get() as u64 {
            return Err(UploadAdmissionError::ManifestCapacity(
                UploadManifestLimit::Tenant,
            ));
        }
        if global > limits.max_chunks.get() as u64 {
            return Err(UploadAdmissionError::ManifestCapacity(
                UploadManifestLimit::Global,
            ));
        }
        Ok(())
    }
}

fn chunks(bytes: u64) -> u64 {
    bytes.div_ceil(CAFFEINE_CHUNK_BYTES as u64)
}
