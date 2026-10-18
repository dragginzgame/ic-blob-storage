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
                let mut config = ic_platform::candid::decoder::DecoderConfig::new();
                config
                    .set_decoding_quota($work)
                    .set_skipping_quota($skipping)
                    .set_max_type_len($types)
                    .set_max_header_len($header as usize)
                    .set_full_error_message(false);
                ic_platform::candid::decoder::decode_one_with_config(&bytes, &config)
                    .expect("valid same-release record")
            }
            const BOUND: ic_storage::adapter::storable::Bound = $layout;
        }
    };
}
pub(crate) use storable;
