//! Labelled host-evidence substitutes for exposure tests. Never accept these controls in production.
use candid::{CandidType, Deserialize};
use ic_blob_storage::dto::upload::admission::{UploadAdmissionRequest, UploadAdmissionResponse};
/// Scenario selected by the local test host, not evidence from Caffeine.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ExposureScenario {
    /// Every provider/recovery/durability fact is unknown.
    Unknown,
    /// All facts are explicitly simulated as established in this execution.
    QualifiedSubstitute,
    /// Established substitute facts from an earlier execution.
    StaleSubstitute,
    /// Established substitute facts for a changed permission.
    ForeignSubstitute,
}
/// Private exposure control, also used for operator-only certificate test setup.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ExposureInput {
    /// Exact original permission.
    pub permission: UploadAdmissionRequest,
    /// Labelled substitute host facts.
    pub scenario: ExposureScenario,
    /// Trap during the permission write.
    pub trap_write: bool,
    /// Trap after the shared handler returns; the whole IC update must roll back.
    pub trap_after: bool,
}
/// Private wire representation of independent shared-policy blockers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ExposureBlocker {
    /// Observation predates this execution.
    StaleObservation,
    /// Pre-charge size enforcement unknown.
    PrechargeLimits,
    /// Provider namespace enforcement unknown.
    ProviderNamespace,
    /// Replay charging unknown.
    ReplayCharging,
    /// Recovery eligibility unknown.
    Recovery,
    /// Durable commit unknown.
    Durability,
}
/// Simulated exposure or missing prerequisites; never a provider certificate.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ExposureOutcome {
    /// No mutation took place.
    Blocked(Vec<ExposureBlocker>),
    /// Durable possible-exposure state, without any provider effect.
    Exposed(Box<UploadAdmissionResponse>),
}
