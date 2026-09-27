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
        remaining(limits, self.manifest_accounting.global, own)
    }

    pub(super) fn check_manifest_capacity(
        &self,
        tenant: Principal,
        bytes: u64,
    ) -> Result<(), UploadAdmissionError> {
        check(
            self.config.limits().manifests,
            self.manifest_accounting.global,
            self.manifest_accounting
                .tenants
                .get(&tenant)
                .copied()
                .unwrap_or(0),
            bytes,
        )
    }
}

fn chunks(bytes: u64) -> u64 {
    bytes.div_ceil(CAFFEINE_CHUNK_BYTES as u64)
}

pub(crate) fn remaining(
    limits: crate::model::service::configuration::ServiceManifestLimits,
    global: u64,
    tenant: u64,
) -> u64 {
    (limits.max_chunks.get() as u64 - global).min(limits.max_tenant_chunks.get() as u64 - tenant)
}

pub(crate) fn check(
    limits: crate::model::service::configuration::ServiceManifestLimits,
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
