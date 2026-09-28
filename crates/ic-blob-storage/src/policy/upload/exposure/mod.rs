//! Missing provider or recovery evidence never becomes issuance authority.
use crate::model::service::upload::UploadPermission;
/// Host-established facts scoped to the exact original permission. Never accept
/// these values from production ingress or infer them from manifest consistency.
/// The host must establish each fact in the same synchronous execution; this is
/// a report, not a cryptographic proof or reusable authorization token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Independent provider and recovery blockers must remain separately observable"
)]
pub struct UploadExposureHostEvidence {
    /// Entire original permission, including service/tenant/namespace/uploader/deadline.
    pub permission: UploadPermission,
    /// Observation time; mismatches fail, but equal time alone cannot prove freshness.
    pub observed_at_ns: u64,
    /// Deployed provider enforces admitted length/tree/chunk bounds before charging.
    pub precharge_limits: bool,
    /// Provider owner/project/bucket binding is qualified for this service namespace.
    pub provider_namespace: bool,
    /// Certificate lifetime, replay and charging semantics are qualified for this effect.
    pub replay_charging: bool,
    /// Complete surviving obligations and independent freshness authority permit effects.
    pub recovery_ready: bool,
    /// The host guarantees atomic durable exposure commit before any effect can escape.
    pub durable_commit: bool,
}
/// One independent reason to block exposure before any certificate/provider effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadExposureBlocker {
    /// Observation time differs from the current execution's clock.
    StaleObservation,
    /// Pre-charge body/tree/chunk enforcement is unqualified.
    PrechargeLimits,
    /// Provider owner/project/bucket binding is unqualified.
    ProviderNamespace,
    /// Certificate replay, lifetime or charging is unqualified.
    ReplayCharging,
    /// Operational recovery/independent freshness eligibility is not established.
    Recovery,
    /// Atomic intent-before-effect durability is not established.
    Durability,
}
/// All missing host prerequisites. Local authorization/phase checks remain separate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UploadExposureAssessment {
    /// Empty only when all supplied host facts meet this policy.
    pub blockers: Vec<UploadExposureBlocker>,
}
/// Evaluate all host blockers without storage, mutation or issuance. The calling
/// boundary must bind the evidence's permission and recheck current local state.
#[must_use]
pub fn assess_exposure(
    evidence: UploadExposureHostEvidence,
    now_ns: u64,
) -> UploadExposureAssessment {
    use UploadExposureBlocker as B;
    let mut blockers = Vec::new();
    for (established, blocker) in [
        (evidence.observed_at_ns == now_ns, B::StaleObservation),
        (evidence.precharge_limits, B::PrechargeLimits),
        (evidence.provider_namespace, B::ProviderNamespace),
        (evidence.replay_charging, B::ReplayCharging),
        (evidence.recovery_ready, B::Recovery),
        (evidence.durable_commit, B::Durability),
    ] {
        if !established {
            blockers.push(blocker);
        }
    }
    UploadExposureAssessment { blockers }
}
