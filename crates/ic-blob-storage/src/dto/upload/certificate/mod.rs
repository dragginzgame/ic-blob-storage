//! Caffeine's update reply payload. The IC supplies the ingress response certificate.
use candid::{CandidType, Deserialize};

/// Successful `_immutableObjectStorageCreateCertificate` response, matching the
/// reviewed Caffeine Mixin. This record alone is neither a signed certificate nor
/// proof of provider acceptance, completed upload, or billing behavior.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct CaffeineUploadCertificateResponse {
    /// Exactly `upload` for the supported operation.
    pub method: String,
    /// Canonical provider root bound to the original admitted upload.
    pub blob_hash: String,
}
