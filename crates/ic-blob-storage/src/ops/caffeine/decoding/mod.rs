//! Common decoder mechanics; callers own byte limits, binding and reply validation.
use candid::de::DecoderConfig;
use std::num::NonZeroUsize;

pub(super) fn limits(
    work: NonZeroUsize,
    skipping: NonZeroUsize,
    types: NonZeroUsize,
) -> DecoderConfig {
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(work.get())
        .set_skipping_quota(skipping.get())
        .set_max_type_len(types.get())
        .set_full_error_message(false);
    config
}
