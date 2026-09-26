//! Controls and observations for the local Wasm byte-verification experiment.

use candid::CandidType;
use serde::Deserialize;

/// Fixed independent vector and deliberate finalization conditions.
#[derive(Clone, Copy, Debug, CandidType, Deserialize)]
pub enum ContentProbeCase {
    /// One full 1 MiB leaf.
    FullChunk,
    /// One full leaf followed by a one-byte final leaf.
    PartialFinalChunk,
    /// ECMAScript metadata normalization and ordering over three content bytes.
    UnicodeMetadata,
    /// Valid manifest bytes paired with an intentionally wrong expected raw digest.
    WrongDigest,
    /// Omit the final leaf after accepting a full prefix.
    Truncated,
}

/// Results observed inside the installed Wasm, not a production completion receipt.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ContentProbeReport {
    /// Final raw digest as canonical text.
    pub raw_digest: String,
    /// Final manifest root as canonical text.
    pub provider_root: String,
    /// Unique bytes verified by the out-of-order tracker.
    pub verified_bytes: u64,
    /// Maximum measured instructions for one valid ordered leaf append.
    /// Excludes input generation, manifest construction and Candid handling.
    pub max_append_instructions: u64,
}

/// Typed expected rejections from the local probe.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ContentProbeFailure {
    /// Caller is not the explicitly configured fixture operator.
    Denied,
    /// Finalization occurred without the full byte sequence.
    Incomplete,
    /// Manifest bytes passed but the independent raw digest did not match.
    DigestMismatch,
}
