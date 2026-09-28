//! Convert Caffeine's prepared manifest into the existing service declaration.
//! No byte hashing, tree builder, admission, transport or provider authority is added.

#[cfg(test)]
mod tests;

use crate::{
    dto::upload::manifest::{UploadManifestDeclaration, UploadManifestHeader},
    model::{
        identity::{
            HashParseError, ProviderRootHash,
            caffeine::{
                CaffeineHeader,
                manifest::{
                    CaffeineChunkHash, CaffeineChunkManifest, CaffeineManifestError,
                    CaffeineManifestLimits,
                },
            },
        },
        service::upload::manifest::{UploadMetadataError, validate_upload_metadata},
    },
};
use serde::Deserialize;
use std::num::NonZeroUsize;
use thiserror::Error;

/// Caller-selected JSON buffering and service declaration limits.
#[derive(Clone, Copy, Debug)]
pub struct PreparedManifestLimits {
    /// Checked before JSON decoding; the receiving transport must also bound buffering.
    pub max_json_bytes: NonZeroUsize,
    /// The same content/leaf/metadata envelope used by the intended service.
    pub manifest: CaffeineManifestLimits,
}

#[derive(Deserialize)]
enum TreeType {
    #[serde(rename = "DSBMTWH")]
    DomainSeparated,
}

#[derive(Deserialize)]
struct PreparedTree {
    hash: String,
}

#[derive(Deserialize)]
struct PreparedManifest {
    tree_type: TreeType,
    chunk_hashes: Vec<String>,
    tree: PreparedTree,
    headers: Vec<String>,
}

/// Decode the upstream `manifestJSON` against the selected root and declared length.
///
/// The JSON byte bound precedes allocation; leaf/header bounds precede conversion.
/// Metadata validation and root reconstruction reuse the service's existing model.
/// Redundant nested tree nodes are ignored: the ordered leaves and metadata are
/// checked independently against the expected root. This is not validation of an
/// arbitrary provider tree payload, file bytes, tenant authority or stored content.
/// # Errors
/// Rejects oversized/invalid JSON, wrong tree type/root, invalid hashes, exceeded
/// service limits, inconsistent metadata or a declaration that rebuilds another root.
pub fn decode_prepared_manifest(
    root: ProviderRootHash,
    content_bytes: u64,
    body: &[u8],
    limits: PreparedManifestLimits,
) -> Result<UploadManifestDeclaration, PreparedManifestError> {
    if body.len() > limits.max_json_bytes.get() {
        return Err(PreparedManifestError::JsonLimit);
    }
    let input: PreparedManifest =
        serde_json::from_slice(body).map_err(|_| PreparedManifestError::Json)?;
    let TreeType::DomainSeparated = input.tree_type;
    if input.tree.hash.parse::<ProviderRootHash>()? != root {
        return Err(PreparedManifestError::Root);
    }
    if input.chunk_hashes.len() > limits.manifest.max_chunks.get()
        || input.headers.len() > limits.manifest.max_headers.get()
    {
        return Err(PreparedManifestError::DeclarationLimit);
    }
    let mut remaining = limits.manifest.max_header_bytes.get();
    let headers = input
        .headers
        .iter()
        .map(|line| {
            // Upstream emits "name: value". Do not trim or normalize accepted metadata.
            remaining = remaining
                .checked_sub(line.len().saturating_add(1))
                .ok_or(PreparedManifestError::DeclarationLimit)?;
            let (name, value) = line.split_once(": ").ok_or(PreparedManifestError::Header)?;
            Ok(CaffeineHeader { name, value })
        })
        .collect::<Result<Vec<_>, PreparedManifestError>>()?;
    validate_upload_metadata(
        &headers,
        content_bytes,
        limits.manifest.max_headers.get(),
        limits.manifest.max_header_bytes.get(),
    )?;
    let chunks = input
        .chunk_hashes
        .iter()
        .map(|hash| hash.parse::<CaffeineChunkHash>())
        .collect::<Result<Vec<_>, _>>()?;
    CaffeineChunkManifest::new(root, content_bytes, &chunks, &headers, limits.manifest)?;
    Ok(UploadManifestDeclaration {
        chunks: chunks.iter().map(|hash| *hash.as_bytes()).collect(),
        headers: headers
            .iter()
            .map(|header| UploadManifestHeader {
                name: header.name.into(),
                value: header.value.into(),
            })
            .collect(),
    })
}

/// A rejected declaration; no operation was admitted or dispatched.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum PreparedManifestError {
    /// JSON exceeded the caller's byte budget before decoding.
    #[error("prepared manifest exceeds JSON limit")]
    JsonLimit,
    /// Malformed JSON, missing/duplicate known fields or an unsupported tree type.
    #[error("invalid prepared manifest JSON")]
    Json,
    /// The declared tree root differs from the selected upload root.
    #[error("prepared manifest root differs")]
    Root,
    /// Leaf/header counts or raw metadata bytes exceed service limits.
    #[error("prepared manifest exceeds declaration limit")]
    DeclarationLimit,
    /// A header does not use the upstream name/value separator.
    #[error("invalid prepared header framing")]
    Header,
    /// A provider root/leaf has invalid representation.
    #[error(transparent)]
    Hash(#[from] HashParseError),
    /// The existing service metadata checks refused the declaration.
    #[error(transparent)]
    Metadata(#[from] UploadMetadataError),
    /// The existing manifest model refused the length, leaves or reconstructed root.
    #[error(transparent)]
    Manifest(#[from] CaffeineManifestError),
}
