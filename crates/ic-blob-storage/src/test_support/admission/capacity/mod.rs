//! Reserve future leaf-array capacity from the exact admitted content length.

use crate::model::service::upload::UploadAdmissionError;
use ic_blob_storage_contracts::identity::caffeine::CAFFEINE_CHUNK_BYTES;
use ic_platform::Principal;
use std::collections::BTreeMap;

use super::UploadAdmissions;
use super::manifest_capacity;

#[derive(Debug, Default)]
pub(super) struct ManifestAccounting {
    global: u64,
    tenants: BTreeMap<Principal, u64>,
}

impl ManifestAccounting {
    pub(super) fn admit(&mut self, tenant: Principal, bytes: u64) {
        let retained = bytes.div_ceil(CAFFEINE_CHUNK_BYTES as u64);
        self.global += retained;
        *self.tenants.entry(tenant).or_default() += retained;
    }
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
        manifest_capacity::remaining(limits, self.manifest_accounting.global, own)
    }

    pub(super) fn check_manifest_capacity(
        &self,
        tenant: Principal,
        bytes: u64,
    ) -> Result<(), UploadAdmissionError> {
        manifest_capacity::check(
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
