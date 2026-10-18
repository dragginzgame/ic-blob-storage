//! Explicit Candid decoder configuration shared by service and native boundaries.
use ic_platform::candid::decoder::DecoderConfig;

/// Configure work, skipping and type limits, optionally bounding the Candid header.
///
/// `None` preserves Candid's default header behavior. A zero skipping quota is
/// valid for exact input boundaries. Callers must bound supplied bytes separately,
/// authenticate their source, check argument exhaustion where required and map
/// errors locally. This configuration grants neither provenance nor retry authority.
#[must_use]
pub fn limits(work: usize, skipping: usize, types: usize, header: Option<usize>) -> DecoderConfig {
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(work)
        .set_skipping_quota(skipping)
        .set_max_type_len(types)
        .set_full_error_message(false);
    if let Some(header) = header {
        config.set_max_header_len(header);
    }
    config
}
