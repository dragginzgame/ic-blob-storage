//! Caffeine's update reply payload. The IC supplies the ingress response certificate.
use crate::dto::upload::admission::UploadAdmissionRequest;
use crate::dto::upload::exposure::UploadExposureBlocker;
use serde::Deserialize;

/// Exact uploader-only preparation assessment. This snapshot cannot authorize a
/// later update; issuance rechecks the original permission and current host facts.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UploadCertificateAssessmentResponse {
    /// Original retained permission resolved from the provider root locator.
    pub permission: UploadAdmissionRequest,
    /// Host clock for this inspection; not a provider observation or lease.
    pub assessed_at_ns: u64,
    /// Every missing prerequisite, independently represented.
    pub blockers: Vec<UploadExposureBlocker>,
}

/// Successful `_immutableObjectStorageCreateCertificate` response, matching the
/// reviewed Caffeine Mixin. This record alone is neither a signed certificate nor
/// proof of provider acceptance, completed upload, or billing behavior.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
#[derive(ic_platform::candid::contract::CandidType)]
#[candid_path("ic_platform::candid::contract")]
pub struct CaffeineUploadCertificateResponse {
    /// Exactly `upload` for the supported operation.
    pub method: String,
    /// Canonical provider root bound to the original admitted upload.
    pub blob_hash: String,
}
