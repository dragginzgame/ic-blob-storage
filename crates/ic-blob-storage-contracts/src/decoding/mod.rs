//! Shared decoder construction; each boundary selects its own unchanged budgets.
use candid::de::DecoderConfig;

/// Configure bounded work and header parsing without granting reply provenance.
pub(crate) fn limits(work: usize, skipping: usize, types: usize, header: usize) -> DecoderConfig {
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(work)
        .set_skipping_quota(skipping)
        .set_max_type_len(types)
        .set_max_header_len(header)
        .set_full_error_message(false);
    config
}
