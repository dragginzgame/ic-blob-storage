use ic_wire::{Encoding, Error as WireError, Limits};

/// Decode a retained positional record or frame using its original fixed body.
/// This is for historical bytes whose format is selected by their owner.
///
/// # Errors
/// Refuses oversized, malformed or over-budget input.
pub fn decode_retained<T: serde::de::DeserializeOwned>(
    bytes: &[u8],
    encoding: Encoding,
    mut limits: Limits,
) -> Result<T, WireError> {
    if bytes.len() > limits.bytes {
        return Err(WireError::Limit(ic_wire::LimitKind::Bytes));
    }
    let length = bytes
        .len()
        .checked_add(1)
        .ok_or(WireError::Limit(ic_wire::LimitKind::Bytes))?;
    let mut framed = Vec::new();
    framed
        .try_reserve_exact(length)
        .map_err(|_| WireError::Limit(ic_wire::LimitKind::Allocation))?;
    framed.extend_from_slice(bytes);
    framed.push(0);
    limits.bytes = limits.bytes.saturating_add(1);
    ic_wire::evolution::decode_projected(&framed, encoding, limits, |message, context| {
        message
            .extensions
            .visit(context, |_, _| Err(WireError::Trailing))?;
        Ok(message.core)
    })
}

/// Reconstruct positional bytes for comparing an original signed packet.
/// New requests and durable writes use the endpoint encoder.
///
/// # Errors
/// Returns a framing, serialization or resource admission error.
pub fn encode_retained<T: serde::Serialize>(
    value: &T,
    encoding: Encoding,
    mut limits: Limits,
) -> Result<Vec<u8>, WireError> {
    limits.bytes = limits.bytes.saturating_add(1);
    let mut bytes = ic_blob_storage_wire::frame::encode(value, encoding, limits)?;
    if bytes.pop() == Some(0) {
        Ok(bytes)
    } else {
        Err(WireError::Malformed)
    }
}
