//! Current bounded Wire records over Storage's collection interface.
//! Allocation keys name slots, while each record has its own storage-only identity.
//! Candid records and foreign frames refuse; restoration never selects a fallback.

/// Wire admission units, independently of stable-tree layout bounds and heap use.
/// Finite headroom covers structured records, full-width counters and manifests;
/// these are not equivalents of the retired Candid work quotas.
pub(crate) const fn limits(bytes: usize) -> ic_wire::Limits {
    ic_wire::Limits {
        bytes,
        allocation: bytes * 256,
        nodes: bytes * 8,
        depth: 32,
    }
}

/// Implement a record's existing Storable contract with explicit owner budgets.
/// No whole-owner snapshots, schema selection, migration or platform IO occurs.
macro_rules! storable {
    ($record:ty, id: $id:expr, bytes: $bytes:expr, layout: $layout:expr $(,)?) => {
        impl $record {
            // Candidate admission shares this fallible path before persistence.
            pub(crate) fn encode_record(&self) -> Result<Vec<u8>, ic_wire::Error> {
                ic_wire::native::encode_request(
                    u64::from_le_bytes(*$id),
                    self,
                    $crate::model::record_codec::limits($bytes as usize),
                )
            }
        }
        impl ic_storage::adapter::Storable for $record {
            fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
                std::borrow::Cow::Owned(self.encode_record().expect("bounded Wire stable record"))
            }
            fn into_bytes(self) -> Vec<u8> {
                self.encode_record().expect("bounded Wire stable record")
            }
            fn from_bytes(bytes: std::borrow::Cow<'_, [u8]>) -> Self {
                ic_wire::native::decode_request(
                    u64::from_le_bytes(*$id),
                    &bytes,
                    $crate::model::record_codec::limits($bytes as usize),
                )
                .expect("current Wire stable record")
            }
            const BOUND: ic_storage::adapter::storable::Bound = $layout;
        }
    };
}
pub(crate) use storable;

#[cfg(test)]
mod tests;
