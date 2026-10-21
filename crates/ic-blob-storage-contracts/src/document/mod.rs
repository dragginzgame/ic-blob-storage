//! Bounded Wire value documents, separate from method-bound service packets.
//!
//! Callers select the expected DTO and validate its application meaning. The
//! document identity is never accepted as an owned service request. Native field
//! and variant order are frozen with the current release, as for service calls.
//! Values carry no Rust type or field names: byte-identical schemas cannot be
//! distinguished. These documents are input data, never authentication evidence.
use ic_wire::{Limits, native};
use serde::{Deserialize, Serialize};

pub use ic_wire::Error;

/// Absolute document ceiling; receiving owners may impose a narrower bound.
pub const MAX_MESSAGE_BYTES: usize = 4 * 1024 * 1024;
/// Maximum native traversal depth, including the value at the root.
pub const MAX_DEPTH: usize = 32;
const IDENTITY: u64 = 0x424c_4f42_0036_ff10;
const LIMITS: Limits = Limits {
    bytes: MAX_MESSAGE_BYTES,
    allocation: 64 * 1024 * 1024,
    nodes: 262_144,
    depth: MAX_DEPTH,
};

/// Encode a passive value; retain its exact bytes when binding durable intent.
/// # Errors
/// Refuses serialization, traversal, allocation and output-budget failures.
pub fn encode<T: Serialize>(value: T) -> Result<Vec<u8>, Error> {
    native::encode_request(IDENTITY, &value, LIMITS)
}

/// Decode a complete canonical document under the caller's byte ceiling.
/// # Errors
/// Refuses foreign frames, trailing/noncanonical bytes and resource violations.
pub fn decode<'de, T: Serialize + Deserialize<'de>>(
    bytes: &'de [u8],
    max: usize,
) -> Result<T, Error> {
    native::decode_request(
        IDENTITY,
        bytes,
        Limits {
            bytes: max.min(MAX_MESSAGE_BYTES),
            ..LIMITS
        },
    )
}

#[cfg(test)]
mod tests;
