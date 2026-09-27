//! Reserve future leaf-array capacity from the exact admitted content length.

use super::{UploadAdmissionError, UploadAdmissions};
use crate::model::identity::caffeine::CAFFEINE_CHUNK_BYTES;
use candid::Principal;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub(super) struct ManifestAccounting {
    global: u64,
    tenants: BTreeMap<Principal, u64>,
}

impl ManifestAccounting {
    pub(super) fn admit(&mut self, tenant: Principal, bytes: u64) {
        let retained = chunks(bytes);
        self.global += retained;
        *self.tenants.entry(tenant).or_default() += retained;
    }
}

/// Which retained manifest budget rejected fresh admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadManifestLimit {
    /// Sum of all tenants' lifetime admitted chunk slots.
    Global,
    /// One tenant's lifetime admitted chunk slots.
    Tenant,
}

impl UploadAdmissions {
    pub(super) fn remaining_manifest_chunks(&self, tenant: Principal) -> u64 {
        let limits = self.config.limits().manifests;
        let own = self
            .manifest_accounting
            .tenants
            .get(&tenant)
            .copied()
            .unwrap_or(0);
        (limits.max_chunks.get() as u64 - self.manifest_accounting.global)
            .min(limits.max_tenant_chunks.get() as u64 - own)
    }

    pub(super) fn check_manifest_capacity(
        &self,
        tenant: Principal,
        bytes: u64,
    ) -> Result<(), UploadAdmissionError> {
        let requested = chunks(bytes);
        // Admission alone consumes slots; cancellation and settlement keep them.
        // Totals change only after the catalog and permission insert succeed.
        let global = self.manifest_accounting.global + requested;
        let own = self
            .manifest_accounting
            .tenants
            .get(&tenant)
            .copied()
            .unwrap_or(0)
            + requested;
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
