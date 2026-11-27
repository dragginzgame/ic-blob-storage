//! Selected Motoko/legacy Caffeine callback ABI: vec blob, never vec text fallback.
//! Platform owns Candid parsing; this adapter validates the complete bounded root batch.

use ic_blob_storage::local::{Root, persistence::callbacks::MAX_ROOTS};
use ic_platform_protocol::{caffeine as c, candid::Limits};

pub(super) const BYTES: usize = 4096;
const LIMITS: Limits = Limits {
    bytes: BYTES,
    header: 512,
    types: 8,
    work: 65_536,
    allocation: 65_536,
    vector: 4096,
    skipping: 65_536,
};

#[expect(
    clippy::unnecessary_wraps,
    reason = "Runtime decoder signature; malformed provider input traps"
)]
pub(super) fn roots(bytes: &[u8]) -> Result<Vec<Root>, std::convert::Infallible> {
    let roots = c::decode_roots(bytes, LIMITS).unwrap();
    assert!(roots.len() <= MAX_ROOTS, "callback root limit");

    Ok(roots
        .into_iter()
        .map(|root| root.try_into().expect("32-byte tree root"))
        .collect())
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "Runtime decoder signature; malformed provider input traps"
)]
pub(super) fn empty(bytes: &[u8]) -> Result<(), std::convert::Infallible> {
    c::decode_empty(bytes, LIMITS).unwrap();

    Ok(())
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "Runtime response encoder receives the owned result"
)]
pub(super) fn liveness(value: Vec<bool>) -> Result<Vec<u8>, c::EncodeError> {
    c::encode_liveness(&value, BYTES)
}

pub(super) fn candidates(value: Vec<Root>) -> Result<Vec<u8>, c::EncodeError> {
    let blobs: Vec<_> = value.into_iter().map(|root| root.to_vec()).collect();

    c::encode_roots(&blobs, BYTES)
}

pub(super) fn done(_: ()) -> Result<Vec<u8>, c::EncodeError> {
    c::encode_empty(BYTES)
}
