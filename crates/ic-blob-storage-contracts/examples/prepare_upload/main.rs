//! Local headless preparation, without service/provider calls or publication.
//! Supply `DECLARATION.json MAX_BYTES MAX_CHUNKS` and the exact body on stdin.
//! The resulting claim is locally computed data, not an authenticated descriptor.

mod inventory;
mod snapshot;

use ic_blob_storage_contracts::identity::caffeine::CaffeineHashError;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHashLimits;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHeader;
use ic_blob_storage_contracts::identity::caffeine::manifest::builder::CaffeineManifestBuilder;
use ic_blob_storage_contracts::identity::caffeine::manifest::builder::CaffeineManifestBuilderError;
use ic_blob_storage_contracts::upload::inventory::{DownloadClaim, Header, PreparedUpload};
use ic_blob_storage_contracts::upload::metadata::UploadMetadataError;
use ic_blob_storage_contracts::upload::metadata::validate_upload_metadata;
use ic_host_artifacts::artifact::ArtifactError;
use ic_host_artifacts::artifact::read_reader;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::num::NonZeroU64;
use std::num::NonZeroUsize;
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

#[derive(Clone, Copy)]
struct PreparationLimits {
    bytes: NonZeroU64,
    chunks: NonZeroUsize,
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

fn cli() -> clap::Command {
    use clap::{Arg, Command, value_parser};
    use std::path::PathBuf;
    Command::new("prepare_upload")
        .about("Prepare a local upload from a declaration and stdin, or an explicit inventory")
        .arg(
            Arg::new("inventory")
                .long("inventory")
                .value_parser(value_parser!(PathBuf))
                .conflicts_with_all(["declaration", "maximum", "chunks"]),
        )
        .arg(
            Arg::new("snapshot")
                .long("snapshot")
                .value_parser(value_parser!(PathBuf))
                .requires("inventory"),
        )
        .arg(
            Arg::new("declaration")
                .value_name("DECLARATION.json")
                .value_parser(value_parser!(PathBuf))
                .required_unless_present("inventory"),
        )
        .arg(
            Arg::new("maximum")
                .value_name("MAX_BYTES")
                .value_parser(value_parser!(NonZeroU64))
                .required_unless_present("inventory"),
        )
        .arg(
            Arg::new("chunks")
                .value_name("MAX_CHUNKS")
                .value_parser(value_parser!(NonZeroUsize))
                .required_unless_present("inventory"),
        )
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli().get_matches();
    if let Some(path) = args.get_one::<std::path::PathBuf>("inventory") {
        let mut output = io::stdout().lock();
        if let Some(parent) = args.get_one::<std::path::PathBuf>("snapshot") {
            serde_json::to_writer(&mut output, &snapshot::run(path, parent)?)?;
        } else {
            serde_json::to_writer(&mut output, &inventory::run(path)?)?;
        }
        writeln!(output)?;
        return Ok(());
    }
    let limits = PreparationLimits {
        bytes: *args
            .get_one::<NonZeroU64>("maximum")
            .expect("required maximum"),
        chunks: *args
            .get_one::<NonZeroUsize>("chunks")
            .expect("required chunks"),
    };
    let input = args
        .get_one::<std::path::PathBuf>("declaration")
        .expect("required declaration");
    let declaration = read_declaration(File::open(input)?)?;
    let prepared = prepare(io::stdin().lock(), declaration, limits)?;
    let mut output = io::stdout().lock();
    serde_json::to_writer(&mut output, &prepared)?;
    writeln!(output)?;
    Ok(())
}

#[cfg(test)]
mod tests;
