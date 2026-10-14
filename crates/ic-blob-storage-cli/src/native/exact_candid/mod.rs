//! Exact, work-bounded decoding of native inputs and immutable installation replies.
use super::Failure;
use candid::{CandidType, DecoderConfig, de::IDLDeserialize};

pub(super) const INSTALLATION_BYTES: usize = 16 * 1024;

/// Callers retain their command-specific ceilings; no skipped fields or extra arguments.
pub(super) fn decode<T>(
    bytes: &[u8],
    max_bytes: usize,
    max_types: usize,
    work: usize,
) -> Result<T, Failure>
where
    T: CandidType + for<'de> serde::Deserialize<'de>,
{
    if bytes.len() > max_bytes {
        return Err(Failure::Arguments);
    }
    let limits = limits(work, 0, max_types, Some(max_bytes));
    let mut decoder =
        IDLDeserialize::new_with_config(bytes, &limits).map_err(|_| Failure::Arguments)?;
    let input = decoder.get_value().map_err(|_| Failure::Arguments)?;
    if !decoder.is_done() {
        return Err(Failure::Arguments);
    }
    decoder.done().map_err(|_| Failure::Arguments)?;
    Ok(input)
}

/// Preserve each boundary's work, skipping, type and header budgets.
pub(super) fn limits(
    work: usize,
    skipping: usize,
    types: usize,
    header: Option<usize>,
) -> DecoderConfig {
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
