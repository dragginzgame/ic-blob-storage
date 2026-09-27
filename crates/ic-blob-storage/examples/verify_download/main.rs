//! Local streaming verification and optional file publication; no gateway calls.
//! Run with `CLAIM.json MAX_BYTES [OUTPUT]` and supply the body on stdin. The claim's root,
//! length and original metadata must already be trusted by the application.

mod output;

use ic_blob_storage::model::identity::caffeine::{
    CaffeineContentHashes, CaffeineHashError, CaffeineHashLimits, CaffeineHeader,
    verification::CaffeineRootVerifier,
};
use serde::Deserialize;
use std::{
    error::Error,
    fs::File,
    io::{self, Read, Write},
    num::{NonZeroU64, NonZeroUsize},
};
use thiserror::Error;

const FRAME: usize = 65_536;
const CLAIM_BYTES: u64 = 16_384;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DownloadClaim {
    root: String,
    bytes: u64,
    headers: Vec<Header>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Header {
    name: String,
    value: String,
}

#[derive(Debug, Error)]
enum BodyError {
    #[error(transparent)]
    Read(#[from] io::Error),
    #[error("staging write failed: {0}")]
    Stage(io::Error),
    #[error(transparent)]
    Integrity(#[from] CaffeineHashError),
}

// EOF, rather than received_bytes == expected_bytes, owns completion here.
fn verify_body(
    mut body: impl Read,
    mut verifier: CaffeineRootVerifier,
    mut staging: impl Write,
) -> Result<CaffeineContentHashes, BodyError> {
    let mut frame = vec![0; FRAME];
    loop {
        match body.read(&mut frame) {
            Ok(0) => {
                let hashes = verifier.finish()?;
                staging.flush().map_err(BodyError::Stage)?;
                return Ok(hashes);
            }
            Ok(count) => {
                verifier.append(verifier.received_bytes(), &frame[..count])?;
                staging
                    .write_all(&frame[..count])
                    .map_err(BodyError::Stage)?;
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(error.into()),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let claim_path = args
        .next()
        .ok_or("usage: verify_download CLAIM.json MAX_BYTES [OUTPUT] < body")?;
    let maximum: NonZeroU64 = args
        .next()
        .ok_or("missing MAX_BYTES")?
        .to_str()
        .ok_or("MAX_BYTES must be UTF-8")?
        .parse()?;
    let destination = args.next();
    if args.next().is_some() {
        return Err("unexpected argument".into());
    }
    let mut encoded = Vec::new();
    File::open(claim_path)?
        .take(CLAIM_BYTES + 1)
        .read_to_end(&mut encoded)?;
    if encoded.len() as u64 > CLAIM_BYTES {
        return Err("claim exceeds example input bound".into());
    }
    let claim: DownloadClaim = serde_json::from_slice(&encoded)?;
    let headers: Vec<_> = claim
        .headers
        .iter()
        .map(|header| CaffeineHeader {
            name: &header.name,
            value: &header.value,
        })
        .collect();
    let verifier = CaffeineRootVerifier::new(
        claim.root.parse()?,
        claim.bytes,
        &headers,
        CaffeineHashLimits {
            max_content_bytes: maximum,
            max_append_bytes: NonZeroUsize::new(FRAME).unwrap(),
            max_headers: NonZeroUsize::new(16).unwrap(),
            max_header_bytes: NonZeroUsize::new(4096).unwrap(),
        },
    )?;
    let actual = if let Some(destination) = destination {
        output::save_verified(io::stdin().lock(), verifier, destination.as_ref())?
    } else {
        verify_body(io::stdin().lock(), verifier, io::sink())?
    };
    println!(
        "{}",
        serde_json::json!({
            "provider_root":actual.provider_root.to_string(),
            "computed_content_digest":actual.content_digest.to_string(),
            "bytes":claim.bytes,
        })
    );
    Ok(())
}

#[cfg(test)]
mod tests;
