//! Local journey controls and wire-shaped observations, never provider evidence.
use candid::CandidType;
use serde::Deserialize;
pub mod readback;

/// Immutable upload identity and capacity declaration; no real provider upload.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct JourneyManifest {
    /// Ordered domain-separated leaf hashes.
    pub chunks: Vec<[u8; 32]>,
    /// Exact header names and values; normalization follows the shared hasher.
    pub headers: Vec<(String, String)>,
}

/// Admission binds both the upload identity and its validated manifest.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct JourneyReservation {
    /// Immutable operation and expected content identities.
    pub upload: JourneyUpload,
    /// Expected chunks and explicit metadata.
    pub manifest: JourneyManifest,
}

/// Local content check, distinct from provider completion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum JourneyVerification {
    /// More manifest-bound chunks are required.
    Pending,
    /// All chunks and the independent raw digest matched.
    Verified,
    /// All chunks matched, but the independent raw digest did not.
    Rejected,
}

/// Tenant-only progress across local messages; never a restart checkpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct JourneyProgress {
    /// Next required index, or the chunk count after all chunks were checked.
    pub next_chunk: u64,
    /// Bytes checked against manifest leaves, even if the final raw digest failed.
    pub verified_bytes: u64,
    /// Independent whole-content verification outcome.
    pub verification: JourneyVerification,
}

/// Current source's certificate response shape, without gateway verification.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct JourneyCertificate {
    /// The literal upload method marker.
    pub method: String,
    /// Canonical provider root.
    pub blob_hash: String,
}

/// Observable rejection category for local controls.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum JourneyFailure {
    /// Actual caller or service is not authorized.
    Denied,
    /// Unknown local operation/root.
    Unknown,
    /// Changed operation arguments or claimed root ownership.
    Conflict,
    /// Current state forbids the requested transition.
    InvalidPhase,
    /// Admission capacity is exhausted.
    Limit,
    /// Malformed identifiers, root or oversized batch.
    InvalidInput,
    /// Bytes do not match the reserved manifest or raw digest.
    ContentMismatch,
    /// A chunk skipped the required position; progress is unchanged.
    OutOfOrder,
    /// The bounded local read slot is already occupied.
    ReadInProgress,
    /// Authority changed while a read was in flight.
    StaleRead,
    /// The local source call failed without yielding content.
    Transport,
    /// Source reply exceeds the application byte budget.
    ReplyTooLarge,
    /// Source reply is malformed or exceeds decoding budgets.
    InvalidReply,
}

/// Charged tenant capacity, including uncertain uploads.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct JourneyUsage {
    /// Logical bytes plus reservations.
    pub logical: u128,
    /// Physical bytes plus reservations.
    pub physical: u128,
    /// Unsettled billing bytes plus reservations.
    pub liability: u128,
}
