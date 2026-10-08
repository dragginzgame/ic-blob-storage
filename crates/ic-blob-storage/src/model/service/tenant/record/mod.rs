//! Bounded v1 enrollment records; independent of host memory IDs.
use candid::{CandidType, Deserialize, Principal, de::DecoderConfig, decode_one_with_config};
use ic_blob_storage_contracts::configuration::service::ServiceConfiguration;
use ic_blob_storage_contracts::tenant::TenantEnrollmentView;
use ic_memory::ic_stable_structures::{Storable, storable::Bound};
use std::{borrow::Cow, num::NonZeroU64};

const MAX_BYTES: u32 = 256;

#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct TenantStoreMetadataRecord {
    version: u8,
    service: Principal,
    operator: Principal,
    namespace: u128,
    max_tenants: u64,
}

impl TenantStoreMetadataRecord {
    pub(crate) fn new(config: &ServiceConfiguration) -> Self {
        Self {
            version: 1,
            service: config.bindings().service,
            operator: config.bindings().operator,
            namespace: config.bindings().namespace.get(),
            max_tenants: config.limits().max_tenants.get() as u64,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
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
pub(crate) enum TenantStoreRecord {
    Metadata(TenantStoreMetadataRecord),
    Enrollment(TenantEnrollmentRecord),
}

impl Storable for TenantStoreRecord {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(self.clone().into_bytes())
    }

    fn into_bytes(self) -> Vec<u8> {
        let bytes = candid::encode_one(self).expect("bounded tenant record encoding");
        assert!(
            bytes.len() <= MAX_BYTES as usize,
            "tenant record byte bound"
        );
        bytes
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        assert!(
            bytes.len() <= MAX_BYTES as usize,
            "tenant record byte bound"
        );
        let mut config = DecoderConfig::new();
        config
            .set_decoding_quota(20_000)
            .set_skipping_quota(1000)
            .set_max_type_len(32)
            .set_max_header_len(MAX_BYTES as usize)
            .set_full_error_message(false);
        decode_one_with_config(&bytes, &config).expect("valid same-release tenant record")
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_BYTES,
        is_fixed_size: false,
    };
}

#[cfg(test)]
mod tests;
