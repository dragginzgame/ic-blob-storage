//! Passive recovery outcomes; ingress supplies no freshness evidence.
use candid::CandidType;
use candid::Deserialize;

/// Recovery refusal never clears an existing fence or discards an obligation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum CurrentInstanceRecoveryFailure {
    /// Caller is not the installed operator.
    Denied,
    /// Actual service, installation or platform execution changed.
    Binding,
    /// Call must execute as a replicated update.
    Execution,
    /// Bounded IC history call failed; no retry was performed.
    Platform,
    /// Platform reply could not be bounded or validated.
    Reply,
    /// No qualified platform anchor was retained at installation.
    MissingAnchor,
    /// History no longer covers every management change since installation.
    IncompleteHistory,
    /// Returned history has inconsistent counts, ordering or versions.
    InvalidHistory,
    /// Snapshot loading may have lost later obligations; activation is refused.
    SnapshotRestored,
    /// State replacement requires retirement and reinstall rather than activation.
    Replaced,
    /// Unknown or renamed platform state cannot establish continuity.
    UnqualifiedChange,
}
