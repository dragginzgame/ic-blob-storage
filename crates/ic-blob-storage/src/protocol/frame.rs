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
    let mut output = Vec::new();
    evolution::encode_projected_into(&mut output, encoding, limits, |_| {
        Ok(evolution::Projected::empty(value))
    })?;
    Ok(output)
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

/// Reopen a versioned durable record. Version 1 used a positional payload.
/// The caller validates the decoded record version and its domain invariants.
///
/// # Errors
/// Refuses oversized, malformed or over-budget records.
pub fn decode_record<T: serde::de::DeserializeOwned>(
    bytes: &[u8],
    limits: Limits,
) -> Result<T, Error> {
    if bytes.first() == Some(&1) {
        decode_retained(bytes, Encoding::Payload, limits)
    } else {
        decode(bytes, Encoding::Payload, limits)
    }
}

/// Decode a retained positional record or frame using its original fixed body.
/// This is for historical bytes whose format is selected by their owner.
///
/// # Errors
/// Refuses oversized, malformed or over-budget input.
pub fn decode_retained<T: serde::de::DeserializeOwned>(
    bytes: &[u8],
    encoding: Encoding,
    mut limits: Limits,
) -> Result<T, Error> {
    if bytes.len() > limits.bytes {
        return Err(Error::Limit(ic_wire::LimitKind::Bytes));
    }
    let length = bytes
        .len()
        .checked_add(1)
        .ok_or(Error::Limit(ic_wire::LimitKind::Bytes))?;
    let mut framed = Vec::new();
    framed
        .try_reserve_exact(length)
        .map_err(|_| Error::Limit(ic_wire::LimitKind::Allocation))?;
    framed.extend_from_slice(bytes);
    framed.push(0);
    limits.bytes = limits.bytes.saturating_add(1);
    evolution::decode_projected(&framed, encoding, limits, |message, context| {
        message
            .extensions
            .visit(context, |_, _| Err(Error::Trailing))?;
        Ok(message.core)
    })
}

/// Reconstruct positional bytes for comparing an original signed packet.
/// New requests and durable writes use [encode].
///
/// # Errors
/// Returns a framing, serialization or resource admission error.
pub fn encode_retained<T: Serialize>(
    value: &T,
    encoding: Encoding,
    mut limits: Limits,
) -> Result<Vec<u8>, Error> {
    limits.bytes = limits.bytes.saturating_add(1);
    let mut bytes = encode(value, encoding, limits)?;
    if bytes.pop() == Some(0) {
        Ok(bytes)
    } else {
        Err(Error::Malformed)
    }
}
