//! Local readback experiment protocol, not a Caffeine HTTP contract.
use candid::CandidType;
use serde::Deserialize;

/// Last completed local read, overwritten in memory and visible only to the operator.
/// Counters cover this service's call context across awaits, excluding source
/// execution and final reply encoding. This is not a production read protocol.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ReadExecutionProfile {
    /// Authenticated initiating actor, preserved across callbacks.
    pub caller: candid::Principal,
    /// Counter at workflow entry (after ingress decoding).
    pub started: u64,
    /// Immediately before the outgoing call, if admitted.
    pub sending: Option<u64>,
    /// Immediately after the outgoing call resolves.
    pub replied: Option<u64>,
    /// After callback authorization and the encoded-size check.
    pub decoding: Option<u64>,
    /// After successful bounded Candid decoding.
    pub decoded: Option<u64>,
    /// After manifest verification, including a hash mismatch.
    pub verified: Option<u64>,
    /// Before storing this observation and encoding the ingress reply.
    pub finished: u64,
    /// Encoded source reply size; zero for transport rejection or no call.
    pub received_bytes: u64,
    /// Verified bytes returned by the workflow; zero on any rejection.
    pub disclosed_bytes: u64,
    /// Typed workflow outcome; traps do not publish a completed observation.
    pub outcome: Result<(), super::JourneyFailure>,
}

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
    /// Return a valid Candid vector with a different element type.
    WrongType,
    /// Declare the full blob but omit its last encoded byte.
    TruncatedEncoding,
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
