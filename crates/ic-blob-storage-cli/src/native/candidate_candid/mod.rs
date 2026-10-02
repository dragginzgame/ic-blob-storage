//! Exact, work-bounded decoding of offline installation candidates.
use super::Failure;
use candid::{CandidType, DecoderConfig, de::IDLDeserialize};

pub(super) const MAX_BYTES: usize = 16 * 1024;

pub(super) fn decode<T>(bytes: &[u8]) -> Result<T, Failure>
where
    T: CandidType + for<'de> serde::Deserialize<'de>,
{
    let mut limits = DecoderConfig::new();
    limits
        .set_decoding_quota(100_000)
        .set_skipping_quota(0)
        .set_max_type_len(64)
        .set_max_header_len(MAX_BYTES)
        .set_full_error_message(false);
    let mut decoder =
        IDLDeserialize::new_with_config(bytes, &limits).map_err(|_| Failure::Arguments)?;
    let input = decoder.get_value().map_err(|_| Failure::Arguments)?;
    if !decoder.is_done() {
        return Err(Failure::Arguments);
    }
    decoder.done().map_err(|_| Failure::Arguments)?;
    Ok(input)
}
