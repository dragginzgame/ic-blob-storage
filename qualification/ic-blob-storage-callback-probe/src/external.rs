//! Selected Motoko/legacy Caffeine callback ABI: vec blob, never vec text fallback.
//! Platform owns Candid parsing; this adapter validates the complete bounded root batch.

use ic_blob_storage::local::{Root, persistence::callbacks::MAX_ROOTS};
use ic_platform_protocol::{caffeine as c, candid::Limits};
use ic_platform_runtime::raw as ic0;

const BYTES: usize = 4096;
const LIMITS: Limits = Limits {
    bytes: BYTES,
    header: 512,
    types: 8,
    work: 65_536,
    allocation: 65_536,
    vector: 4096,
    skipping: 65_536,
};

/// Bound the provider request before decoding. Each callback applies its own access policy.
pub(super) fn admit() -> Result<(), String> {
    if ic0::msg_arg_data_size() > BYTES {
        Err("callback input limit".into())
    } else {
        Ok(())
    }
}

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

#[expect(
    clippy::unnecessary_wraps,
    reason = "Runtime decoder signature; malformed provider input traps"
)]
pub(super) fn certificate_root(bytes: &[u8]) -> Result<Root, std::convert::Infallible> {
    let text = c::decode_root(bytes, LIMITS).unwrap();
    let hex = text.strip_prefix("sha256:").expect("sha256 tree root");
    assert!(
        hex.len() == 64
            && hex
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "canonical root"
    );
    Ok(std::array::from_fn(|i| {
        u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap()
    }))
}

pub(super) fn certificate_reply(root: Root) -> Result<Vec<u8>, c::EncodeError> {
    let hex: String = root.iter().map(|byte| format!("{byte:02x}")).collect();
    // This plain record is not the IC certificate. Request-status proof is owned
    // by the replica/Agent, and provider acceptance of that proof remains unqualified.
    c::encode_certificate(
        &c::CertificateReply {
            method: "upload".into(),
            blob_hash: format!("sha256:{hex}"),
        },
        1024,
    )
}
