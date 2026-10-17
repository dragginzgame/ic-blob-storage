//! Per-method service codec owners during the pending 0.32 hard cut.
//!
//! Tenant methods use bounded, method-bound IC Wire contracts. Remaining owned
//! methods temporarily retain CBOR until their coordinated conversion completes.
//! Provider contracts and stable records have separate encoding owners.
pub(crate) mod decimal;
/// Bounded IC Wire tenant contracts; provider contracts remain separate.
pub mod tenant;
use serde::{Serialize, de::DeserializeOwned};
use std::io::{self, Write};

/// Absolute codec ceiling; each receiving boundary supplies its own narrower limit.
pub const MAX_MESSAGE_BYTES: usize = 4 * 1024 * 1024;
/// Maximum CBOR nesting accepted before entering application code.
pub const MAX_DEPTH: usize = 32;

/// Encoding or admission failure; never evidence that an earlier effect did not occur.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    /// Encoded bytes exceed the selected ceiling.
    #[error("Blob CBOR byte limit exceeded")]
    Limit,
    /// Malformed, excessive nesting, wrong shape or unknown/duplicate fields.
    #[error("invalid Blob CBOR value")]
    Invalid,
    /// Another value or bytes follow the expected value.
    #[error("trailing Blob CBOR input")]
    Trailing,
}

struct BoundedOutput {
    bytes: Vec<u8>,
    exceeded: bool,
}
impl Write for BoundedOutput {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > MAX_MESSAGE_BYTES.saturating_sub(self.bytes.len()) {
            self.exceeded = true;
            return Err(io::Error::other("Blob CBOR output limit"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Encode one value with bounded output; integers retain their full 128-bit width.
/// # Errors
/// Refuses unsupported serialization and oversized output without returning partial bytes.
pub fn encode<T: Serialize>(value: T) -> Result<Vec<u8>, Error> {
    let mut output = BoundedOutput {
        bytes: Vec::new(),
        exceeded: false,
    };
    if ciborium::ser::into_writer(&value, &mut output).is_err() {
        return Err(if output.exceeded {
            Error::Limit
        } else {
            Error::Invalid
        });
    }
    Ok(output.bytes)
}

/// Decode one complete value after byte admission, with bounded nesting.
/// Unknown/duplicate DTO fields refuse; trailing data is never silently skipped.
/// # Errors
/// Refuses size, nesting, shape and trailing input before returning a value.
pub fn decode<T: DeserializeOwned>(bytes: &[u8], max: usize) -> Result<T, Error> {
    if bytes.len() > max.min(MAX_MESSAGE_BYTES) {
        return Err(Error::Limit);
    }
    let mut remaining = bytes;
    let value = ciborium::de::from_reader_with_recursion_limit(&mut remaining, MAX_DEPTH)
        .map_err(|_| Error::Invalid)?;
    if !remaining.is_empty() {
        return Err(Error::Trailing);
    }
    Ok(value)
}

#[cfg(test)]
mod tests;
