//! Explicit shared resource limits; each decoder owns binding and reply validation.
use ic_platform::candid::decoder::DecoderConfig;
use std::num::NonZeroUsize;

/// Explicit resource bounds for one Caffeine reply; no production budget is inferred.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaffeineReplyLimits {
    /// Bound supplied bytes before decoding; transport buffering needs its own bound.
    pub max_bytes: NonZeroUsize,
    /// Maximum Candid decoding work under the decoder's cost model.
    pub decoding_quota: NonZeroUsize,
    /// Maximum work spent skipping additional fields or arguments.
    pub skipping_quota: NonZeroUsize,
    /// Maximum entries in the Candid type table.
    pub max_type_entries: NonZeroUsize,
}

impl CaffeineReplyLimits {
    pub(super) fn config(self) -> DecoderConfig {
        ic_blob_storage_contracts::decoding::limits(
            self.decoding_quota.get(),
            self.skipping_quota.get(),
            self.max_type_entries.get(),
            None,
        )
    }
}
