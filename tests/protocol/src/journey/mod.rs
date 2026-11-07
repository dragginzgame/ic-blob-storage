//! Local journey controls and wire-shaped observations, never provider evidence.
use serde::Deserialize;

/// Immutable upload identity and capacity declaration; no real provider upload.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct JourneyUpload {
    /// Positive tenant-scoped operation and object identifier.
    pub id: u8,
    /// Expected provider root, including explicitly supplied metadata.
    pub root: [u8; 32],
    /// Independent expected SHA-256 of the complete content.
    pub digest: [u8; 32],
    /// Capacity to reserve before authority can escape.
    pub bytes: u64,
}

/// Bounded manifest supplied with admission, never an authorization credential.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct JourneyManifest {
    /// Ordered domain-separated leaf hashes.
    pub chunks: Vec<[u8; 32]>,
    /// Exact header names and values; normalization follows the shared hasher.
    pub headers: Vec<(String, String)>,
}

/// Charged tenant capacity, including uncertain uploads.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct JourneyUsage {
    /// Logical bytes plus reservations.
    pub logical: u128,
    /// Physical bytes plus reservations.
    pub physical: u128,
    /// Unsettled billing bytes plus reservations.
    pub liability: u128,
}
