//! Artifact-owned bounds applied before managed access checks and dispatch.
use canic::endpoint::ArgumentLimits;

pub(crate) const REQUEST: ArgumentLimits = ArgumentLimits {
    max_bytes: 4096,
    decoding_quota: 100_000,
    skipping_quota: 20_000,
    max_type_len: 128,
    max_header_len: 2048,
};

// Preserve the existing manifest transport envelope; shared policy still enforces
// semantic metadata/chunk limits independently of Candid decoding work.
pub(crate) const MANIFEST: ArgumentLimits = ArgumentLimits {
    max_bytes: 131_072,
    decoding_quota: 2_000_000,
    skipping_quota: 200_000,
    max_type_len: 128,
    max_header_len: 8192,
};

// Bound the complete protected framework carrier before any lifecycle participant.
// The shared adapter separately bounds/validates the nested application bytes.
pub(crate) const LIFECYCLE: ArgumentLimits = ArgumentLimits {
    max_bytes: ic_blob_storage_canic::arguments::CARRIER_BYTES,
    decoding_quota: 2_000_000,
    skipping_quota: 200_000,
    max_type_len: 256,
    max_header_len: 16_384,
};
