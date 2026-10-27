//! Bounded decoding of the labelled local `fixture_chunk` reply, not a provider contract.

/// Failure to admit a local fixture chunk reply.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChunkReplyError {
    /// Encoded bytes exceed the caller's pre-decoding ceiling.
    TooLarge,
    /// The reply is malformed, has the wrong type or exceeds its work budget.
    Invalid,
}

/// Decode one method-bound Wire bulk-byte response under the caller's byte limit.
///
/// This bounds application decoding after the CDK has buffered the IC reply.
/// It authenticates no source, root or chunk index and verifies no manifest.
/// # Errors
/// Refuses oversized, malformed, wrong-type and over-budget replies.
pub fn decode_reply(encoded: &[u8], maximum: usize) -> Result<Vec<u8>, ChunkReplyError> {
    if encoded.len() > maximum {
        return Err(ChunkReplyError::TooLarge);
    }
    crate::fixture_wire::decode_one::<ic_wire::ByteBuf>("fixture_chunk", encoded, maximum)
        .map(ic_wire::ByteBuf::into_vec)
        .map_err(|_| ChunkReplyError::Invalid)
}

#[cfg(test)]
mod tests;
