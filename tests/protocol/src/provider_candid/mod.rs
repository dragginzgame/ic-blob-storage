//! Codec hooks only for emulated external Candid provider contracts.
//!
//! Preserve the former generated tuple decoder, 10,000 skipping quota, empty
//! argument fallback and reply arity. Private fixture RPC uses `fixture_wire`;
//! these hooks do not select a new protocol
//! or replace the narrower decoders used by bounded fixtures and Blob endpoints.

use ic_platform::candid::{contract, decoder};

/// Decode the positional arguments of an existing multi-argument fixture.
/// # Panics
/// Rejects malformed arguments and exhausted skipping work before dispatch.
#[must_use]
#[expect(
    clippy::needless_pass_by_value,
    reason = "entrypoint hooks own ingress bytes"
)]
pub fn decode_args<T: for<'de> contract::ArgumentDecoder<'de>>(bytes: Vec<u8>) -> T {
    let bytes: &[u8] = if bytes.is_empty() {
        b"DIDL\x00\x00"
    } else {
        &bytes
    };
    let mut config = decoder::DecoderConfig::new();
    config.set_skipping_quota(10_000);
    decoder::decode_args_with_config(bytes, &config).unwrap()
}

/// Project the single argument from the same positional decoder.
/// # Panics
/// Rejects malformed arguments and exhausted skipping work before dispatch.
#[must_use]
pub fn decode_one<T: contract::CandidType + for<'de> serde::Deserialize<'de>>(bytes: Vec<u8>) -> T {
    let (value,) = decode_args(bytes);
    value
}

/// Preserve the old zero-argument fixtures' deliberate lack of input decoding.
pub fn ignore_arguments(_: Vec<u8>) {}

/// Encode one existing fixture result, including an explicit unit-valued result.
/// # Panics
/// Traps on serialization failure rather than delivering a partial reply.
#[must_use]
pub fn encode_one<T: contract::CandidType>(value: T) -> Vec<u8> {
    contract::encode_one(value).unwrap()
}

/// Preserve the former implicit unit reply of a function with no return type.
#[must_use]
pub fn encode_unit() -> Vec<u8> {
    encode_one(())
}

#[cfg(test)]
mod tests;
