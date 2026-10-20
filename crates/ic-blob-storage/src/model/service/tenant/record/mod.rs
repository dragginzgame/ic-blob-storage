//! Bounded v1 enrollment records; independent of host memory IDs.
use ic_blob_storage_contracts::configuration::service::ServiceConfiguration;
use ic_blob_storage_contracts::tenant::TenantEnrollmentView;
use ic_platform::candid::contract::CandidType;
use ic_platform::candid::contract::serde::Deserialize;
use ic_platform::principal::ExternalPrincipal;
#[cfg(test)]
use ic_storage::adapter::Storable;
#[cfg(test)]
use std::borrow::Cow;
use std::num::NonZeroU64;

const MAX_BYTES: u32 = 256;

#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[candid_path("ic_platform::candid::contract")]
pub(crate) struct TenantStoreMetadataRecord {
    version: u8,
    service: ExternalPrincipal,
    operator: ExternalPrincipal,
    namespace: u128,
    max_tenants: u64,
}

impl TenantStoreMetadataRecord {
    pub(crate) fn new(config: &ServiceConfiguration) -> Self {
        Self {
            version: 1,
            service: config.bindings().service.into(),
            operator: config.bindings().operator.into(),
            namespace: config.bindings().namespace.get(),
            max_tenants: config.limits().max_tenants.get() as u64,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[candid_path("ic_platform::candid::contract")]
pub(crate) struct TenantEnrollmentRecord {
    version: u8,
    generation: u64,
    active: bool,
}

impl TenantEnrollmentRecord {
    pub(crate) const fn new(view: TenantEnrollmentView) -> Self {
        Self {
            version: 1,
            generation: view.generation.get(),
            active: view.active,
        }
    }

    pub(crate) fn view(&self) -> Option<TenantEnrollmentView> {
        if self.version != 1 {
            return None;
        }
        Some(TenantEnrollmentView {
            generation: NonZeroU64::new(self.generation)?,
            active: self.active,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[candid_path("ic_platform::candid::contract")]
pub(crate) enum TenantStoreRecord {
    Metadata(TenantStoreMetadataRecord),
    Enrollment(TenantEnrollmentRecord),
}

crate::model::record_codec::storable!(
    TenantStoreRecord, bytes: MAX_BYTES, work: 20_000, skipping: 1000,
    types: 32, header: MAX_BYTES,
    layout: ic_storage::adapter::storable::Bound::Bounded {
        max_size: MAX_BYTES, is_fixed_size: false,
    },
);

#[cfg(test)]
mod tests;
