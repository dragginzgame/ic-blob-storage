//! Validate prepared declarations without trusting supplied totals or opening bodies.

use crate::operator::Failure;
use ic_blob_storage::model::{
    identity::{
        ContentDigest, ProviderRootHash,
        caffeine::{
            CaffeineHeader,
            manifest::{CaffeineChunkHash, CaffeineChunkManifest, CaffeineManifestLimits},
        },
        verification::ContentVerifier,
    },
    service::upload::manifest::validate_upload_metadata,
};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::Read,
    num::{NonZeroU64, NonZeroUsize},
    path::Path,
};

const MAX_BYTES: u64 = 8 * 1024 * 1024;
const MAX_LEAVES: usize = 8192;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EncodedInventory {
    totals: Totals,
    assets: Vec<AssetEntry>,
    blobs: Vec<Blob>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
struct Totals {
    assets: usize,
    source_bytes: u64,
    source_chunks: u64,
    distinct_blobs: usize,
    distinct_bytes: u64,
    distinct_chunks: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AssetEntry {
    asset: String,
    source: String,
    root: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Blob {
    claim: Claim,
    chunk_hashes: Vec<String>,
    computed_content_digest: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Claim {
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

pub(super) struct PreparedBlob {
    pub root: ProviderRootHash,
    pub bytes: u64,
    pub chunks: u64,
    pub headers: usize,
    pub header_bytes: usize,
    pub assets: Vec<String>,
}

pub(super) struct Inventory {
    pub blobs: Vec<PreparedBlob>,
}

pub(super) fn load(path: &Path) -> Result<Inventory, Failure> {
    let mut encoded = Vec::new();
    File::open(path)
        .map_err(|_| Failure::InvalidRequest)?
        .take(MAX_BYTES + 1)
        .read_to_end(&mut encoded)
        .map_err(|_| Failure::InvalidRequest)?;
    parse(&encoded)
}

fn parse(encoded: &[u8]) -> Result<Inventory, Failure> {
    if encoded.len() as u64 > MAX_BYTES {
        return Err(Failure::InvalidRequest);
    }
    let input: EncodedInventory =
        serde_json::from_slice(encoded).map_err(|_| Failure::InvalidRequest)?;
    if input.blobs.is_empty()
        || input.blobs.len() > 2048
        || input.assets.is_empty()
        || input.assets.len() > 4096
    {
        return Err(Failure::InvalidRequest);
    }
    let mut totals = Totals {
        assets: input.assets.len(),
        source_bytes: 0,
        source_chunks: 0,
        distinct_blobs: input.blobs.len(),
        distinct_bytes: 0,
        distinct_chunks: 0,
    };
    let mut blobs = BTreeMap::new();
    for blob in input.blobs {
        totals.distinct_chunks = add(totals.distinct_chunks, blob.chunk_hashes.len() as u64)?;
        if totals.distinct_chunks > MAX_LEAVES as u64 {
            return Err(Failure::InvalidRequest);
        }
        let checked = validate(&blob)?;
        totals.distinct_bytes = add(totals.distinct_bytes, checked.bytes)?;
        if blobs.insert(checked.root.to_string(), checked).is_some() {
            return Err(Failure::InvalidRequest);
        }
    }
    let mut names = BTreeSet::new();
    for asset in input.assets {
        if asset.asset.is_empty()
            || asset.asset.len() > 256
            || asset.asset.chars().any(char::is_control)
            || !names.insert(asset.asset.clone())
            || asset.source.is_empty()
        {
            return Err(Failure::InvalidRequest);
        }
        // `source` is provenance only; never resolve or open this untrusted path.
        let blob = blobs.get_mut(&asset.root).ok_or(Failure::InvalidRequest)?;
        totals.source_bytes = add(totals.source_bytes, blob.bytes)?;
        totals.source_chunks = add(totals.source_chunks, blob.chunks)?;
        blob.assets.push(asset.asset);
    }
    if totals != input.totals || blobs.values().any(|blob| blob.assets.is_empty()) {
        return Err(Failure::InvalidRequest);
    }
    Ok(Inventory {
        blobs: blobs.into_values().collect(),
    })
}

fn add(a: u64, b: u64) -> Result<u64, Failure> {
    a.checked_add(b).ok_or(Failure::InvalidRequest)
}

fn validate(blob: &Blob) -> Result<PreparedBlob, Failure> {
    let invalid = |_| Failure::InvalidRequest;
    let root: ProviderRootHash = blob.claim.root.parse().map_err(invalid)?;
    // Canonical keys prevent case aliases from bypassing duplicate detection.
    if root.to_string() != blob.claim.root {
        return Err(Failure::InvalidRequest);
    }
    blob.computed_content_digest
        .parse::<ContentDigest>()
        .map_err(invalid)?;
    let chunks: Vec<CaffeineChunkHash> = blob
        .chunk_hashes
        .iter()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()
        .map_err(invalid)?;
    let headers: Vec<_> = blob
        .claim
        .headers
        .iter()
        .map(|h| CaffeineHeader {
            name: &h.name,
            value: &h.value,
        })
        .collect();
    validate_upload_metadata(&headers, blob.claim.bytes, 16, 4096)
        .map_err(|_| Failure::InvalidRequest)?;
    CaffeineChunkManifest::new(
        root,
        blob.claim.bytes,
        &chunks,
        &headers,
        CaffeineManifestLimits {
            max_content_bytes: NonZeroU64::new(ContentVerifier::MAX_BYTES).unwrap(),
            max_chunks: NonZeroUsize::new(MAX_LEAVES).unwrap(),
            max_headers: NonZeroUsize::new(16).unwrap(),
            max_header_bytes: NonZeroUsize::new(4096).unwrap(),
        },
    )
    .map_err(|_| Failure::InvalidRequest)?;
    Ok(PreparedBlob {
        root,
        bytes: blob.claim.bytes,
        chunks: chunks.len() as u64,
        headers: headers.len(),
        header_bytes: headers
            .iter()
            .map(|h| h.name.len() + h.value.len() + 3)
            .sum(),
        assets: vec![],
    })
}

#[cfg(test)]
pub(super) mod tests;
