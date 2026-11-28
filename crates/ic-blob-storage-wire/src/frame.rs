//! Sparse Wire messages with an explicitly fixed Serde body.

use ic_wire::{Encoding, Error, Limits, evolution};
use serde::{Deserialize, Serialize};

/// Encode a fixed body followed by the Wire sparse root extension block.
///
/// # Errors
/// Returns the Wire framing, serialization or resource admission error.
pub fn encode<T: Serialize>(
    value: &T,
    encoding: Encoding,
    limits: Limits,
) -> Result<Vec<u8>, Error> {
    evolution::encode_closed_core(value, encoding, limits)
}

/// Decode a fixed body and validate the Wire sparse root extension block.
///
/// # Errors
/// Returns the Wire framing, decoding or resource admission error.
pub fn decode<'de, T: Deserialize<'de>>(
    bytes: &'de [u8],
    encoding: Encoding,
    limits: Limits,
) -> Result<T, Error> {
    evolution::decode_projected(bytes, encoding, limits, |message, context| {
        message.extensions.visit(context, |_, _| Ok(()))?;
        Ok(message.core)
    })
}
