//! Trusted-uploader issuance within admitted resource quotas; no provider replay guarantee.
use ic_blob_storage_contracts::upload::binding::UploadPermission;
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
    /// Explicit installed local namespace mapping differs from the permission.
    NamespaceBinding,
    /// Original uploader lacks explicit installed trust.
    TrustedUploader,
    /// Owner is inspection-only after restoration.
    CurrentOwner,
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
