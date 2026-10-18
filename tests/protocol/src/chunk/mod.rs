//! Bounded decoding of the labelled local `fixture_chunk` reply, not a provider contract.
use ic_platform::candid::decoder::{DecoderConfig, decode_one_with_config};

/// Failure to admit a local fixture chunk reply.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChunkReplyError {
    /// Encoded bytes exceed the caller's pre-decoding ceiling.
    TooLarge,
    /// The reply is malformed, has the wrong type or exceeds its work budget.
    Invalid,
}

/// Decode one Candid blob with caller-owned byte and work limits.
///
/// This bounds application decoding after the CDK has buffered the IC reply.
/// It authenticates no source, root or chunk index and verifies no manifest.
/// # Errors
/// Refuses oversized, malformed, wrong-type and over-budget replies.
pub fn decode_reply(
    encoded: &[u8],
    maximum: usize,
    work: usize,
) -> Result<Vec<u8>, ChunkReplyError> {
    if encoded.len() > maximum {
        return Err(ChunkReplyError::TooLarge);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(work)
        .set_skipping_quota(64)
        .set_max_type_len(8)
        .set_full_error_message(false);
    // Preserve the blob fast path: one bulk copy after bounded type/work admission.
    decode_one_with_config::<serde_bytes::ByteBuf>(encoded, &config)
        .map(serde_bytes::ByteBuf::into_vec)
        .map_err(|_| ChunkReplyError::Invalid)
}

#[cfg(test)]
mod tests;
