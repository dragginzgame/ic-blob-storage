//! Local headless preparation, without service/provider calls or publication.
//! Supply `DECLARATION.json MAX_BYTES MAX_CHUNKS` and the exact body on stdin.
//! The resulting claim is locally computed data, not an authenticated descriptor.

mod inventory;
mod snapshot;

use ic_blob_storage::model::{
    identity::caffeine::{
        CaffeineHashError, CaffeineHashLimits, CaffeineHeader,
        manifest::builder::{CaffeineManifestBuilder, CaffeineManifestBuilderError},
    },
    service::upload::manifest::{UploadMetadataError, validate_upload_metadata},
};
use ic_host_artifacts::artifact::{ArtifactError, read_reader};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::File,
    io::{self, Read, Write},
    num::{NonZeroU64, NonZeroUsize},
};
use thiserror::Error;

const DECLARATION_BYTES: usize = 16_384;
const FRAME: usize = 65_536;
const MAX_HEADERS: usize = 16;
const MAX_HEADER_BYTES: usize = 4096;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Declaration {
    bytes: u64,
    headers: Vec<Header>,
}

fn read_declaration(reader: impl Read) -> Result<Declaration, Box<dyn Error>> {
    let encoded = read_reader(reader, DECLARATION_BYTES).map_err(|error| -> Box<dyn Error> {
        match error {
            ArtifactError::LimitExceeded { .. } => "declaration exceeds example input bound".into(),
            ArtifactError::Io(error) => error.into(),
            ArtifactError::Allocation(error) => error.into(),
            error => error.into(),
        }
    })?;
    Ok(serde_json::from_slice(&encoded)?)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Header {
    name: String,
    value: String,
}

#[derive(Clone, Copy)]
struct PreparationLimits {
    bytes: NonZeroU64,
    chunks: NonZeroUsize,
}

#[derive(Debug, Serialize)]
struct PreparedUpload {
    claim: DownloadClaim,
    chunk_hashes: Vec<String>,
    computed_content_digest: String,
}

#[derive(Debug, Serialize)]
struct DownloadClaim {
    root: String,
    bytes: u64,
    headers: Vec<Header>,
}

#[derive(Debug, Error)]
enum PreparationError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Metadata(#[from] UploadMetadataError),
    #[error(transparent)]
    Builder(#[from] CaffeineManifestBuilderError),
    #[error(transparent)]
    Hash(#[from] CaffeineHashError),
}

fn prepare(
    body: impl Read,
    declaration: Declaration,
    limits: PreparationLimits,
) -> Result<PreparedUpload, PreparationError> {
    prepare_to(body, declaration, limits, io::sink())
}

// Store the very buffers supplied to the hasher; never reopen a changing source.
fn prepare_to(
    mut body: impl Read,
    declaration: Declaration,
    limits: PreparationLimits,
    mut output: impl Write,
) -> Result<PreparedUpload, PreparationError> {
    let headers: Vec<_> = declaration
        .headers
        .iter()
        .map(|header| CaffeineHeader {
            name: &header.name,
            value: &header.value,
        })
        .collect();
    validate_upload_metadata(&headers, declaration.bytes, MAX_HEADERS, MAX_HEADER_BYTES)?;
    let mut builder = CaffeineManifestBuilder::new(
        declaration.bytes,
        &headers,
        CaffeineHashLimits {
            max_content_bytes: limits.bytes,
            max_append_bytes: NonZeroUsize::new(FRAME).unwrap(),
            max_headers: NonZeroUsize::new(MAX_HEADERS).unwrap(),
            max_header_bytes: NonZeroUsize::new(MAX_HEADER_BYTES).unwrap(),
        },
        limits.chunks,
    )?;
    let mut frame = vec![0; FRAME];
    loop {
        match body.read(&mut frame) {
            Ok(0) => break,
            Ok(count) => {
                builder.append(builder.received_bytes(), &frame[..count])?;
                output.write_all(&frame[..count])?;
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(error.into()),
        }
    }
    let prepared = builder.finish()?;
    output.flush()?;
    Ok(PreparedUpload {
        claim: DownloadClaim {
            root: prepared.hashes().provider_root.to_string(),
            bytes: declaration.bytes,
            headers: declaration.headers,
        },
        chunk_hashes: prepared
            .manifest()
            .chunks()
            .iter()
            .map(ToString::to_string)
            .collect(),
        computed_content_digest: prepared.hashes().content_digest.to_string(),
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let input = args
        .next()
        .ok_or("usage: prepare_upload DECLARATION.json MAX_BYTES MAX_CHUNKS < body")?;
    if input == "--inventory" {
        let path = args.next().ok_or("missing inventory file")?;
        if let Some(option) = args.next() {
            if option != "--snapshot" {
                return Err("expected --snapshot PARENT_DIRECTORY".into());
            }
            let parent = args.next().ok_or("missing snapshot parent directory")?;
            if args.next().is_some() {
                return Err("unexpected snapshot argument".into());
            }
            let saved = snapshot::run(path.as_ref(), parent.as_ref())?;
            let mut output = io::stdout().lock();
            serde_json::to_writer(&mut output, &saved)?;
            writeln!(output)?;
            return Ok(());
        }
        let report = inventory::run(path.as_ref())?;
        let mut output = io::stdout().lock();
        serde_json::to_writer(&mut output, &report)?;
        writeln!(output)?;
        return Ok(());
    }
    let maximum = args.next().ok_or("missing MAX_BYTES")?;
    let chunks = args.next().ok_or("missing MAX_CHUNKS")?;
    if args.next().is_some() {
        return Err("unexpected argument".into());
    }
    let limits = PreparationLimits {
        bytes: maximum.to_str().ok_or("MAX_BYTES must be UTF-8")?.parse()?,
        chunks: chunks.to_str().ok_or("MAX_CHUNKS must be UTF-8")?.parse()?,
    };
    let declaration = read_declaration(File::open(input)?)?;
    let prepared = prepare(io::stdin().lock(), declaration, limits)?;
    let mut output = io::stdout().lock();
    serde_json::to_writer(&mut output, &prepared)?;
    writeln!(output)?;
    Ok(())
}

#[cfg(test)]
mod tests;
