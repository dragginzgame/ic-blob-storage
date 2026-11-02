//! Private ordered storage keys; RPC and record payloads retain their own codecs.
use ic_platform::Principal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub(crate) struct TenantKeyRecord(Principal);

impl TenantKeyRecord {
    pub(crate) fn management_canister() -> Self {
        Principal::management_canister().into()
    }
}
impl From<Principal> for TenantKeyRecord {
    fn from(value: Principal) -> Self {
        Self(value)
    }
}
impl From<TenantKeyRecord> for Principal {
    fn from(value: TenantKeyRecord) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub(crate) struct UploadKeyRecord(pub(crate) TenantKeyRecord, pub(crate) u128);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub(crate) struct ManifestKeyRecord(pub(crate) UploadKeyRecord, pub(crate) u8);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub(crate) struct ReferenceKeyRecord(pub(crate) UploadKeyRecord, pub(crate) u128);

macro_rules! key_codec {
    ($record:ty, $id:expr, $bytes:expr) => {
        crate::model::record_codec::storable!(
            $record, id: $id, bytes: $bytes,
            layout: ic_storage::adapter::Bound::Bounded { max_size: $bytes, is_fixed_size: false },
        );
    };
}
key_codec!(TenantKeyRecord, b"BLTENKY1", 64);
key_codec!(UploadKeyRecord, b"BLUPKEY1", 96);
key_codec!(ManifestKeyRecord, b"BLMANKY1", 128);
key_codec!(ReferenceKeyRecord, b"BLREFKY1", 128);

#[cfg(test)]
mod tests;
