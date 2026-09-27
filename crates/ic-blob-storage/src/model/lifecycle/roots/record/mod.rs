//! Bounded stable root claims and reverse keys, independent of host memory IDs.
use super::ObjectBinding;
use crate::model::{
    lifecycle::binding::ObjectIdentity, service::configuration::ServiceConfiguration,
};
use candid::{CandidType, Deserialize, Principal, de::DecoderConfig, decode_one_with_config};
use ic_memory::ic_stable_structures::{Storable, storable::Bound};
use std::{borrow::Cow, num::NonZeroU128};

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
        impl Storable for $record {
            fn to_bytes(&self) -> Cow<'_, [u8]> {
                let bytes = candid::encode_one(self).expect("root record encoding");
                assert!(bytes.len() <= $bound, "root record byte bound");
                Cow::Owned(bytes)
            }
            fn into_bytes(self) -> Vec<u8> {
                self.to_bytes().into_owned()
            }
            fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
                assert!(bytes.len() <= $bound, "root record byte bound");
                let mut config = DecoderConfig::new();
                config
                    .set_decoding_quota(20_000)
                    .set_skipping_quota(1000)
                    .set_max_type_len(32)
                    .set_max_header_len($bound)
                    .set_full_error_message(false);
                decode_one_with_config(&bytes, &config).expect("same-release root record")
            }
            const BOUND: Bound = Bound::Bounded {
                max_size: $bound,
                is_fixed_size: false,
            };
        }
    };
}
bounded_record!(RootObjectKeyRecord, 256);
bounded_record!(RootClaimRecord, 512);

#[cfg(test)]
mod tests;
