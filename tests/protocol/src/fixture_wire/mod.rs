//! Method- and direction-bound Wire for private fixture RPC and lifecycle inputs.
//! Provider substitutes and saved Wire journals have separate framing.
use ic_wire::{Error, Limits, native};
use serde::{Serialize, de::DeserializeOwned};

/// Absolute ceiling for private fixture frames; owners can narrow it.
pub const MAX_BYTES: usize = 4 * 1024 * 1024;

/// Stable private method identity, not a signature or authentication mechanism.
/// FNV-1a covers the fixed namespace followed by the exact UTF-8 method name.
#[must_use]
pub const fn identity(method: &str) -> u64 {
    fingerprint(
        fingerprint(0xcbf2_9ce4_8422_2325, b"blob.private.fixture.rpc:"),
        method.as_bytes(),
    )
}
const fn fingerprint(mut value: u64, bytes: &[u8]) -> u64 {
    let mut index = 0;
    while index < bytes.len() {
        value = (value ^ bytes[index] as u64).wrapping_mul(0x0000_0100_0000_01b3);
        index += 1;
    }
    value
}
fn limits(maximum: usize) -> Limits {
    Limits {
        bytes: maximum.min(MAX_BYTES),
        allocation: 64 * 1024 * 1024,
        nodes: 262_144,
        depth: 32,
    }
}

/// Encode the complete positional request, including explicit unit for no arguments.
/// # Errors
/// Refuses values outside the finite Wire envelope.
pub fn encode_args<T: Serialize>(method: &str, value: T) -> Result<Vec<u8>, Error> {
    native::encode_request(identity(method), &value, limits(MAX_BYTES))
}
/// Encode a single argument in its positional tuple.
/// # Errors
/// Refuses values outside the finite Wire envelope.
pub fn encode_one<T: Serialize>(method: &str, value: T) -> Result<Vec<u8>, Error> {
    encode_args(method, (value,))
}
/// Decode a complete positional request with no fallback or trailing values.
/// # Errors
/// Refuses old Candid, wrong method/direction, malformed or over-budget frames.
pub fn request<T: DeserializeOwned>(
    method: &str,
    bytes: &[u8],
    maximum: usize,
) -> Result<T, Error> {
    request_id(identity(method), bytes, maximum)
}
/// Decode with an explicitly bound entrypoint identity, without IC method lookup.
/// # Errors
/// Refuses foreign, malformed, trailing or over-budget frames.
pub fn request_id<T: DeserializeOwned>(id: u64, bytes: &[u8], maximum: usize) -> Result<T, Error> {
    native::decode_request(id, bytes, limits(maximum))
}
/// Encode the complete response tuple; this is never a request frame.
/// # Errors
/// Refuses values outside the finite Wire envelope.
pub fn response<T: Serialize>(method: &str, value: T) -> Result<Vec<u8>, Error> {
    response_id(identity(method), value)
}
/// Encode with an explicitly bound entrypoint identity.
/// # Errors
/// Refuses output outside the finite envelope.
pub fn response_id<T: Serialize>(id: u64, value: T) -> Result<Vec<u8>, Error> {
    native::encode_response(id, &value, limits(MAX_BYTES))
}
/// Encode one returned value in its positional response tuple.
/// # Errors
/// Refuses values outside the finite Wire envelope.
pub fn encode_reply<T: Serialize>(method: &str, value: T) -> Result<Vec<u8>, Error> {
    response(method, (value,))
}
/// Decode the complete response tuple without retrying the operation.
/// # Errors
/// Refuses foreign, trailing, malformed and over-budget frames.
pub fn decode_args<T: DeserializeOwned>(
    method: &str,
    bytes: &[u8],
    maximum: usize,
) -> Result<T, Error> {
    native::decode_response(identity(method), bytes, limits(maximum))
}
/// Decode one response value from its positional tuple.
/// # Errors
/// Refuses responses with any other envelope or shape.
pub fn decode_one<T: DeserializeOwned>(
    method: &str,
    bytes: &[u8],
    maximum: usize,
) -> Result<T, Error> {
    decode_args::<(T,)>(method, bytes, maximum).map(|(value,)| value)
}

/// Entrypoint hooks share exactly the host's codec and explicit tuple arity.
pub mod hooks;

#[cfg(test)]
mod tests;
