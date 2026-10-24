//! Bounded same-release Candid mechanics; records retain schemas and budget policy.
//! The stable-tree layout is independent of the encoded/decoded byte ceiling.

/// Implement a record's existing Storable contract with explicit owner budgets.
/// No whole-owner snapshots, schema selection, migration or platform IO occurs.
macro_rules! storable {
    ($record:ty, bytes: $bytes:expr, work: $work:expr, skipping: $skipping:expr,
     types: $types:expr, header: $header:expr, layout: $layout:expr $(,)?) => {
        impl ic_storage::adapter::Storable for $record {
            fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
                let bytes = ic_platform::candid::contract::encode_one(self)
                    .expect("stable record encoding");
                assert!(bytes.len() <= $bytes as usize, "stable record byte bound");
                std::borrow::Cow::Owned(bytes)
            }
            fn into_bytes(self) -> Vec<u8> {
                self.to_bytes().into_owned()
            }
            fn from_bytes(bytes: std::borrow::Cow<'_, [u8]>) -> Self {
                assert!(bytes.len() <= $bytes as usize, "stable record byte bound");
                let config = ic_blob_storage_contracts::decoding::limits(
                    $work,
                    $skipping,
                    $types,
                    Some($header as usize),
                );
                ic_platform::candid::decoder::decode_one_with_config(&bytes, &config)
                    .expect("valid same-release record")
            }
            const BOUND: ic_storage::adapter::storable::Bound = $layout;
        }
    };
}
pub(crate) use storable;
