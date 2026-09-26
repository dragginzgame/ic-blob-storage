//! Local readback experiment protocol, not a Caffeine HTTP contract.
use candid::CandidType;
use serde::Deserialize;

/// One verified manifest leaf; no whole-file read or durable receipt is implied.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct JourneyReadChunk {
    /// Position in the reserved manifest.
    pub index: u64,
    /// Byte offset in the file.
    pub offset: u64,
    /// Exact bytes checked against that leaf.
    pub bytes: Vec<u8>,
}

/// Deliberate local-source reply faults.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ReadSourceMode {
    /// Return the configured bytes.
    Valid,
    /// Change one byte without changing length.
    Corrupt,
    /// Omit the last byte.
    Truncated,
    /// Exceed the service's encoded-reply budget.
    Oversized,
    /// Return invalid Candid.
    Malformed,
    /// Reject the actual call.
    Reject,
}

/// Driver-controlled single-leaf substitute for provider storage.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ReadSourceConfig {
    /// Root expected in the service request.
    pub root: [u8; 32],
    /// Index expected in the service request.
    pub index: u64,
    /// At most one 1 MiB leaf, never a production stored object.
    pub bytes: Vec<u8>,
    /// Deliberate source behavior.
    pub mode: ReadSourceMode,
    /// Hold the reply until the driver resumes it.
    pub hold: bool,
}

/// Driver-only observations for deterministic callback scheduling tests.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ReadSourceObservation {
    /// Accepted source reads, useful for proving denial before effects.
    pub requests: u64,
    /// A captured reply is currently held across messages.
    pub waiting: bool,
}
