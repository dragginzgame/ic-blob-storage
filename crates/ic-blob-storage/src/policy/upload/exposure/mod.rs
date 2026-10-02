//! Restricted trusted-uploader issuance; no provider economics or replay guarantee.
use crate::model::service::upload::UploadPermission;
/// Host-established facts scoped to the exact original permission. Never accept
/// these values from production ingress or infer them from manifest consistency.
/// The host must establish each fact in the same synchronous execution; this is
/// a report, not a cryptographic proof or reusable authorization token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Independent local contract prerequisites must remain separately observable"
)]
pub struct UploadExposureHostEvidence {
    /// Entire original permission, including service/tenant/namespace/uploader/deadline.
    pub permission: UploadPermission,
    /// Observation time; mismatches fail, but equal time alone cannot prove freshness.
    pub observed_at_ns: u64,
    /// Installed lifetime envelope and body meet the one-object/1 KiB contract.
    pub trial_bounds: bool,
    /// Explicit installed service/project/local namespace matches the permission.
    pub namespace_binding: bool,
    /// Original uploader is the explicitly installed trusted uploader.
    pub trusted_uploader: bool,
    /// Current owner has not entered inspection-only restoration; no backup claim.
    pub current_owner: bool,
    /// The host guarantees atomic durable exposure commit before any effect can escape.
    pub durable_commit: bool,
}
/// One independent reason to block exposure before any certificate/provider effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadExposureBlocker {
    /// Observation time differs from the current execution's clock.
    StaleObservation,
    /// Installed lifetime envelope or body exceeds the restricted contract.
    TrialBounds,
    /// Explicit installed local namespace mapping differs from the permission.
    NamespaceBinding,
    /// Original uploader lacks explicit installed trust.
    TrustedUploader,
    /// Owner is inspection-only after restoration.
    CurrentOwner,
    /// Atomic intent-before-effect durability is not established.
    Durability,
}

/// Pure local lifetime bounds for the accepted one-object prototype contract.
/// These constrain service records and admitted bytes, never provider charges.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RestrictedUploadEnvelope {
    /// Lifetime tenant slots, including suspended tenants.
    pub tenants: u32,
    /// Lifetime object slots.
    pub objects: u32,
    /// Lifetime object slots per tenant.
    pub tenant_objects: u32,
    /// Maximum admitted object bytes.
    pub object_bytes: u64,
    /// Stored bytes, including uncertain obligations.
    pub physical_bytes: u128,
    /// Continuing billing liability bytes.
    pub liability_bytes: u128,
    /// Tenant logical byte budget.
    pub tenant_bytes: u128,
    /// Lifetime references per object.
    pub references: u32,
    /// Lifetime receipt slots, including cleanup capacity.
    pub receipts: u32,
    /// Concurrent upload reservations.
    pub active: u32,
    /// Concurrent tenant reservations.
    pub tenant_active: u32,
}
impl RestrictedUploadEnvelope {
    /// Require positive bounded bytes and exactly one lifetime tenant/object/reference.
    #[must_use]
    pub fn permits(self, bytes: u64) -> bool {
        let slots = self.tenants == 1
            && self.objects == 1
            && self.tenant_objects == 1
            && self.references == 1
            && self.receipts >= 2
            && self.active == 1
            && self.tenant_active == 1;
        let budgets = (1..=1024).contains(&self.object_bytes)
            && (1..=1024).contains(&self.physical_bytes)
            && (1..=1024).contains(&self.liability_bytes)
            && (1..=1024).contains(&self.tenant_bytes);
        slots && budgets && bytes > 0 && bytes <= self.object_bytes
    }
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
        (evidence.trial_bounds, B::TrialBounds),
        (evidence.namespace_binding, B::NamespaceBinding),
        (evidence.trusted_uploader, B::TrustedUploader),
        (evidence.current_owner, B::CurrentOwner),
        (evidence.durable_commit, B::Durability),
    ] {
        if !established {
            blockers.push(blocker);
        }
    }
    UploadExposureAssessment { blockers }
}

#[cfg(test)]
mod tests;
