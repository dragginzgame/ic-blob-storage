//! Selected Motoko/legacy Caffeine callback ABI: vec blob, never vec text fallback.
//! Platform owns Candid parsing; this adapter validates the complete bounded root batch.

use ic_blob_storage::local::{Root, persistence::callbacks::MAX_ROOTS};
use ic_platform::candid_codec::{
    arguments::encode_args,
    decoder::{Arity, Limits, decode_args},
};
use ic_platform_runtime::ic0;

const BYTES: usize = 4096;
const LIMITS: Limits = Limits {
    bytes: BYTES,
    header: 512,
    types: 8,
    work: 65_536,
    allocation: 65_536,
};

pub(super) fn admit() -> Result<(), String> {
    if ic0::msg_arg_data_size() > BYTES {
        Err("callback input limit".into())
    } else {
        Ok(())
    }
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "Runtime owns the argument buffer"
)]
pub(super) fn roots(bytes: Vec<u8>) -> Vec<Root> {
    let (roots,): (Vec<Vec<u8>>,) = decode_args(&bytes, LIMITS, Arity::Exact).unwrap();
    assert!(roots.len() <= MAX_ROOTS, "callback root limit");
    roots
        .into_iter()
        .map(|root| root.try_into().expect("32-byte tree root"))
        .collect()
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "Runtime owns the argument buffer"
)]
pub(super) fn empty(bytes: Vec<u8>) {
    decode_args::<()>(&bytes, LIMITS, Arity::Exact).unwrap();
}

pub(super) fn liveness(value: Vec<bool>) -> Vec<u8> {
    encode_args(&(value,), BYTES).unwrap()
}
pub(super) fn candidates(value: Vec<Root>) -> Vec<u8> {
    let blobs: Vec<_> = value.into_iter().map(|root| root.to_vec()).collect();
    encode_args(&(blobs,), BYTES).unwrap()
}
pub(super) fn done() -> Vec<u8> {
    encode_args(&(), BYTES).unwrap()
}

ic_platform::candid_record! {
    struct CertificateReply {
        156956385 => method: String,
        278101296 => blob_hash: String,
    }
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "Runtime owns the argument buffer"
)]
pub(super) fn certificate_root(bytes: Vec<u8>) -> Root {
    let (text,): (String,) = decode_args(&bytes, LIMITS, Arity::Exact).unwrap();
    let hex = text.strip_prefix("sha256:").expect("sha256 tree root");
    assert!(
        hex.len() == 64
            && hex
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "canonical root"
    );
    std::array::from_fn(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap())
}

pub(super) fn certificate_reply(root: Root) -> Vec<u8> {
    let hex: String = root.iter().map(|byte| format!("{byte:02x}")).collect();
    // This plain record is not the IC certificate. Request-status proof is owned
    // by the replica/Agent, and provider acceptance of that proof remains unqualified.
    encode_args(
        &(CertificateReply {
            method: "upload".into(),
            blob_hash: format!("sha256:{hex}"),
        },),
        1024,
    )
    .unwrap()
}
