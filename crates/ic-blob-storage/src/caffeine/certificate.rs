//! Candid boundary for the SDK's `_immutableObjectStorageCreateCertificate` update.
//! The embedding host owns the update export, captured caller and clock. Execute
//! `Command::Register(root)` synchronously, trap on refusal, then encode the same
//! root in that update. An encoding failure must trap so registration rolls back.

use crate::local::Root;
use ic_platform_protocol::{caffeine, candid::Limits};

/// Maximum Candid request size; apply this cap before copying ingress arguments.
pub const MAX_REQUEST_BYTES: usize = 4096;

/// Decode the SDK's single text argument into the exact tree root.
///
/// # Errors
/// Rejects malformed or over-budget Candid and noncanonical SHA-256 root text.
pub fn decode_request(bytes: &[u8]) -> Result<Root, &'static str> {
    let text = caffeine::decode_root(
        bytes,
        Limits {
            bytes: MAX_REQUEST_BYTES,
            header: 512,
            types: 8,
            work: 65_536,
            allocation: 65_536,
            vector: 4096,
            skipping: 65_536,
        },
    )
    .map_err(|_| "invalid Candid root argument")?;
    let hex = text.strip_prefix("sha256:").ok_or("expected sha256 root")?;
    if hex.len() != 64
        || !hex
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("expected 64 lowercase hexadecimal digits");
    }
    let mut root = [0; 32];
    for (i, byte) in root.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).map_err(|_| "invalid root digit")?;
    }
    Ok(root)
}

/// Encode the provider's plain upload reply after successful registration.
/// The replica certifies the ingress response.
///
/// # Errors
/// Returns the Platform encoder error if it cannot produce the bounded reply.
pub fn encode_reply(root: Root) -> Result<Vec<u8>, caffeine::EncodeError> {
    let hex: String = root.iter().map(|byte| format!("{byte:02x}")).collect();
    caffeine::encode_certificate(
        &caffeine::CertificateReply {
            method: "upload".into(),
            blob_hash: format!("sha256:{hex}"),
        },
        1024,
    )
}
