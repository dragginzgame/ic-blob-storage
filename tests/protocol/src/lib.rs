//! Private fixture controls shared by local test canisters and the host harness.
//! These are not a production service API or an independently defined provider contract.

use candid::{CandidType, Principal};
use serde::Deserialize;

pub mod content;
pub mod obligations;
pub mod uploads;

/// Deliberate source behavior for one local gateway-list experiment.
#[derive(Clone, Copy, Debug, CandidType, Deserialize)]
pub enum SourceMode {
    /// Return the configured member.
    Valid,
    /// Return bytes that cannot decode as a principal list.
    Malformed,
    /// Exceed the probe's explicit reply byte budget.
    Oversized,
    /// Return an invalid empty membership candidate.
    Empty,
    /// Reject the inter-canister call.
    Reject,
    /// Attempt another sync while the original is awaiting this reply.
    Overlap,
    /// Revoke the old gateway before returning its stale membership list.
    Revoke,
    /// Revoke, complete a new sync, then return the original stale list.
    Replace(Principal),
}

/// Typed outcomes of the test-only sync orchestration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum SyncFailure {
    /// Caller lacks the explicit fixture operator role.
    Denied,
    /// Another attempt is already pending.
    InProgress,
    /// An intervening decision invalidated this attempt.
    Stale,
    /// Supplied reply failed decoding or membership validation.
    InvalidReply,
    /// Reply exceeded the configured pre-decoding byte bound.
    ReplyTooLarge,
    /// The local inter-canister call failed.
    Transport,
    /// Internal scope or sequence admission rejected the attempt.
    Admission,
}

/// Bounded observations of calls made to the controlled source.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct SourceObservation {
    /// Number of list requests observed, used to check denial before effects.
    pub requests: u64,
    /// Result of the most recent deliberate reentrant sync, if any.
    pub nested_sync: Option<Result<(), SyncFailure>>,
}
