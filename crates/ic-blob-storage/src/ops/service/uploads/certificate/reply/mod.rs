//! Read-only certificate assessments bound to the saved original permission.
use crate::{
    dto::upload::{
        admission::UploadAdmissionRequest, certificate::UploadCertificateAssessmentResponse,
        exposure::UploadExposureFailure,
    },
    model::identity::ProviderRootHash,
};
use candid::{de::DecoderConfig, decode_one_with_config};
use std::num::NonZeroUsize;
use thiserror::Error;

/// Assessment decoding failure. No outcome authorizes certificate issuance or retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum UploadCertificateAssessmentReplyError {
    /// Encoded reply exceeds the caller's budget.
    #[error("assessment reply exceeds limit")]
    Limit,
    /// Malformed permission, reply or duplicate blockers.
    #[error("invalid assessment")]
    Invalid,
    /// Returned permission differs from the saved original intent.
    #[error("assessment binding mismatch")]
    Binding,
    /// Authenticated service refusal, distinct from an empty blocker list.
    #[error("assessment refused: {0:?}")]
    Remote(UploadExposureFailure),
}

/// Encode the root locator only after validating the full original permission.
/// The caller must authenticate as its uploader and validate the reply with [`assessment`].
/// # Errors
/// Rejects malformed permission identities or encoding failure.
pub fn assessment_request(
    permission: UploadAdmissionRequest,
) -> Result<Vec<u8>, UploadCertificateAssessmentReplyError> {
    super::super::admission::reply::check(permission)
        .map_err(|_| UploadCertificateAssessmentReplyError::Invalid)?;
    let root = ProviderRootHash::try_from(permission.upload.root.as_slice())
        .map_err(|_| UploadCertificateAssessmentReplyError::Invalid)?;
    candid::encode_one(root.to_string()).map_err(|_| UploadCertificateAssessmentReplyError::Invalid)
}

/// Decode one independently authenticated assessment of the exact saved permission.
/// An empty blocker list is a snapshot, never an issuance reservation or lease.
/// # Errors
/// Rejects oversized, malformed, duplicate-blocker, foreign or refused replies.
pub fn assessment(
    permission: UploadAdmissionRequest,
    bytes: &[u8],
    maximum: NonZeroUsize,
) -> Result<UploadCertificateAssessmentResponse, UploadCertificateAssessmentReplyError> {
    super::super::admission::reply::check(permission)
        .map_err(|_| UploadCertificateAssessmentReplyError::Invalid)?;
    if bytes.len() > maximum.get() {
        return Err(UploadCertificateAssessmentReplyError::Limit);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(100_000)
        .set_skipping_quota(0)
        .set_max_type_len(64)
        .set_max_header_len(maximum.get())
        .set_full_error_message(false);
    let response: Result<UploadCertificateAssessmentResponse, UploadExposureFailure> =
        decode_one_with_config(bytes, &config)
            .map_err(|_| UploadCertificateAssessmentReplyError::Invalid)?;
    let response = response.map_err(UploadCertificateAssessmentReplyError::Remote)?;
    if response.permission != permission {
        return Err(UploadCertificateAssessmentReplyError::Binding);
    }
    for (index, blocker) in response.blockers.iter().enumerate() {
        if response.blockers[..index].contains(blocker) {
            return Err(UploadCertificateAssessmentReplyError::Invalid);
        }
    }
    Ok(response)
}

#[cfg(test)]
mod tests;
