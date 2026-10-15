//! Passive prepared-inventory JSON shared by local producers and observers.
//! Every value is an untrusted declaration. Consumers independently check totals,
//! roots and budgets; a successful decode establishes no upload or authority.
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Complete preparation output; it makes no service-capacity or payment claim.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryReport {
    /// Declared work and deduplicated sizes, to be independently recomputed.
    pub totals: InventoryTotals,
    /// Original asset-to-root mapping, including distinct assets sharing a root.
    pub assets: Vec<PreparedAsset>,
    /// One preparation result per distinct provider root.
    pub blobs: Vec<PreparedUpload>,
}

/// Declared inventory counts; source work is counted before deduplication.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryTotals {
    /// Number of named assets.
    pub assets: usize,
    /// Sum of bytes across all source entries, including duplicates.
    pub source_bytes: u64,
    /// Sum of leaves across all source entries, including duplicates.
    pub source_chunks: u64,
    /// Number of distinct provider roots.
    pub distinct_blobs: usize,
    /// Sum of bytes across distinct roots.
    pub distinct_bytes: u64,
    /// Sum of leaves across distinct roots.
    pub distinct_chunks: u64,
}

/// Asset provenance and its declared provider root, never a file-access grant.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PreparedAsset {
    /// Caller-assigned asset name, to be checked for uniqueness and bounds.
    pub asset: String,
    /// Passive original path; observers must never resolve or open it.
    pub source: PathBuf,
    /// Declared provider root, to be correlated with a validated blob.
    pub root: String,
}

/// Original metadata and locally computed hashes; source bytes need rechecking.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PreparedUpload {
    /// Declared provider root, byte length and original headers.
    pub claim: DownloadClaim,
    /// Ordered leaf hashes used to construct the declared provider root.
    pub chunk_hashes: Vec<String>,
    /// Raw content digest, distinct from the provider root.
    pub computed_content_digest: String,
}

/// Original download metadata; decoding does not authenticate its contents.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DownloadClaim {
    /// Declared provider root, distinct from a raw content digest.
    pub root: String,
    /// Declared complete body length.
    pub bytes: u64,
    /// Original ordered headers, including any duplicates pending validation.
    pub headers: Vec<Header>,
}

/// One original header pair; consumers apply their own metadata validation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Header {
    /// Original header name, before validation or case folding.
    pub name: String,
    /// Original value, before metadata validation.
    pub value: String,
}
