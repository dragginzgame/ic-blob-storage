//! Private controls for testing the durable upload owner, not a service API.
pub mod funding;
pub mod gateways;
pub mod read;
use crate::{
    admission::{
        Permission, Phase, Request,
        input::{PreparationInput, ReferenceInput},
    },
    journey::JourneyUsage,
};
use candid::{CandidType, Deserialize};

/// Test-only stable write boundary; no production fault control is exported.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum WriteFault {
    /// Reverse root index, after the forward root write.
    Objects,
    /// Permission write, after root admission or manifest preparation.
    Permissions,
    /// Accounting write, after root and permission admission.
    Usage,
    /// Manifest write.
    Manifests,
    /// Confirmed lifecycle metadata write.
    Confirmed,
    /// Individual reference write.
    References,
    /// Exact reference receipt write.
    Receipts,
    /// Root-to-request index, after root claim and permission insertion.
    RootRequests,
    /// Funding accounting, after the exact intent row write.
    FundingAccounting,
    /// Funding intent/attempt/outcome row.
    FundingIntents,
    /// Whole gateway membership and sync record.
    Gateways,
    /// Read-session row write/removal.
    ReadSessions,
    /// Per-tenant read occupancy, after session row mutation.
    ReadTenants,
    /// Aggregate read occupancy and sequence, after session/tenant mutation.
    ReadJournal,
}
/// One admission that must trap at the selected stable write.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FaultAdmission {
    /// Exact permission, without caller or clock override.
    pub permission: Permission,
    /// Write that traps within this IC update.
    pub fault: WriteFault,
}
/// One manifest binding that must trap at the selected stable write.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FaultPreparation {
    /// Exact declaration to prepare.
    pub preparation: PreparationInput,
    /// Write that traps within this IC update.
    pub fault: WriteFault,
}
/// Typed boundary rejection for a fixture store operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Failure {
    /// Returned bytes differ from the admitted leaf hash or exact length.
    ContentMismatch,
    /// Labelled local read-only transport failed.
    Transport,
    /// Caller lacks the operation's role.
    Denied,
    /// Service or namespace differs.
    Binding,
    /// Invalid identity or record.
    Invalid,
    /// Configured capacity exhausted.
    Capacity,
    /// Reopening enforces inspection only.
    Fenced,
    /// Changed operation or stale enrollment precondition.
    Conflict,
    /// Unknown operation.
    Unknown,
    /// Enrollment absent, suspended, or permission from an earlier activation.
    Inactive,
    /// Permission revoked.
    Revoked,
    /// Manifest required before exposure.
    Unprepared,
    /// Phase does not allow the requested transition.
    Phase,
    /// Remaining receipts are reserved for releasing live references.
    ReceiptCapacity,
    /// Scan cursor differs from the requested scope or filter.
    CursorScope,
}
/// Exact retained permission, without provider or completion authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Observation {
    /// Original immutable permission.
    pub permission: Permission,
    /// Original activation generation.
    pub generation: u64,
    /// Tenant has revoked this permission.
    pub revoked: bool,
    /// Immutable declaration is retained.
    pub prepared: bool,
    /// Current upload phase.
    pub phase: Phase,
}
/// Operator-only observations; no restore authority is implied.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Status {
    /// Lifetime permissions and root claims retained.
    pub operations: u64,
    /// Active reservations, including possible exposure.
    pub active: u64,
    /// Bytes reserved by unfinished uploads.
    pub bytes: u128,
    /// Charged bytes, including pending and confirmed obligations.
    pub usage: JourneyUsage,
    /// The complete owner enforces the restored fence.
    pub fenced: bool,
}

/// Labelled local provider-fact substitute; never a deployed provider callback.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ProviderFact {
    /// Exact upload completion substitute.
    Uploaded,
    /// Exact physical deletion substitute.
    Deleted,
    /// Final settlement and billing cessation substitute.
    Settled,
}
/// Operator-only fact injection, used to exercise the shared transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct FactInput {
    /// Complete original upload identity.
    pub request: Request,
    /// Explicit substitute fact.
    pub fact: ProviderFact,
    /// Optional fixture-only interrupted write.
    pub fault: Option<WriteFault>,
}
/// Exact reference request with an optional fixture-only interrupted write.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ReferenceMutationInput {
    /// Full operation identity and action.
    pub request: ReferenceInput,
    /// Optional stable write fault.
    pub fault: Option<WriteFault>,
}
/// Original recorded reference result; failure is distinct from admission rejection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ReferenceResult {
    /// Applied a reference transition.
    Changed,
    /// Exact reference state already held.
    Unchanged,
    /// Unknown reference could not be released.
    UnknownReference,
    /// Released reference cannot be reactivated.
    Released,
    /// Lifetime references exhausted.
    Limit,
    /// No new references after deletion queues.
    DeletionQueued,
}
/// Receipt result and whether it was read from retained history.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ReferenceOutcome {
    /// True if no new receipt was recorded.
    pub replayed: bool,
    /// Exact original result.
    pub result: ReferenceResult,
}
