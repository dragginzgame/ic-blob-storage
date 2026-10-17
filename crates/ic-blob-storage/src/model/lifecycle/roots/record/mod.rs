//! Bounded stable root claims and reverse keys, independent of host memory IDs.
use super::ObjectBinding;
use candid::{CandidType, Deserialize, Principal};
use ic_blob_storage_contracts::binding::ObjectIdentity;
use ic_blob_storage_contracts::configuration::service::ServiceConfiguration;
#[cfg(test)]
use ic_storage::adapter::Storable;
use std::num::NonZeroU128;

/// All non-service object identity fields; service is fixed by store metadata.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, CandidType, Deserialize)]
pub(crate) struct RootObjectKeyRecord {
    version: u8,
    tenant: Principal,
    namespace: u128,
    object: u128,
    incarnation: u128,
}

impl RootObjectKeyRecord {
    pub(crate) fn new(binding: ObjectBinding) -> Self {
        let identity = binding.identity();
        Self {
            version: 1,
            tenant: binding.tenant(),
            namespace: identity.namespace.get(),
            object: identity.object.get(),
            incarnation: identity.incarnation.get(),
        }
    }

    pub(crate) fn binding(&self, service: Principal) -> Option<ObjectBinding> {
        if self.version != 1 {
            return None;
        }
        ObjectBinding::new(
            service,
            self.tenant,
            ObjectIdentity {
                namespace: NonZeroU128::new(self.namespace)?,
                object: NonZeroU128::new(self.object)?,
                incarnation: NonZeroU128::new(self.incarnation)?,
            },
        )
        .ok()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct RootStoreMetadataRecord {
    version: u8,
    service: Principal,
    namespace: u128,
    max_roots: u64,
}

impl RootStoreMetadataRecord {
    pub(crate) fn new(config: &ServiceConfiguration) -> Self {
        Self {
            version: 1,
            service: config.bindings().service,
            namespace: config.bindings().namespace.get(),
            max_roots: config.limits().catalog.max_objects.get() as u64,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) enum RootClaimRecord {
    Metadata(RootStoreMetadataRecord),
    Claim(RootObjectKeyRecord),
}

// Both schemas are small fixed-shape records. Bound the encoded input before
// Candid decoding; never store unbounded maps or serialize an entire owner.
macro_rules! bounded_record {
    ($record:ty, $bound:expr) => {
        crate::model::record_codec::storable!(
            $record, bytes: $bound, work: 20_000, skipping: 1000,
            types: 32, header: $bound, layout: ic_storage::adapter::storable::Bound::Bounded { max_size: $bound as u32, is_fixed_size: false },
        );
    };
}
bounded_record!(RootObjectKeyRecord, 256);
bounded_record!(RootClaimRecord, 512);

#[cfg(test)]
mod tests;
