//! Bounded verification plans from an independently authenticated installed service.
use crate::{
    dto::upload::{
        admission::UploadAdmissionRequest,
        completion::{UploadAttestationFailure, UploadVerificationPlan},
    },
    model::service::{
        read::download::CaffeineDownloadScope, upload::completion::CompletionAuthority,
    },
    ops::service::uploads::{
        completion::reply::{UploadAttestationReplyError, inspection_request},
        manifests::reply::{UploadManifestReplyError, UploadManifestReplyLimits, validate},
    },
};
use candid::{de::DecoderConfig, decode_one_with_config};

/// Authenticate the selected service before using this decoder. The returned project
/// is the service's installed mapping; only the caller may select an approved origin.
/// # Errors
/// Rejects invalid scope, excessive/malformed replies, changed identity or bad declarations.
pub fn decode(
    authority: CompletionAuthority,
    permission: UploadAdmissionRequest,
    bytes: &[u8],
    limits: UploadManifestReplyLimits,
) -> Result<UploadVerificationPlan, UploadAttestationReplyError> {
    inspection_request(authority, permission)?;
    if bytes.len() > limits.max_reply_bytes.get() {
        return Err(UploadAttestationReplyError::Limit);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(2_000_000)
        .set_skipping_quota(0)
        .set_max_type_len(64)
        .set_max_header_len(limits.max_reply_bytes.get())
        .set_full_error_message(false);
    let response: Result<UploadVerificationPlan, UploadAttestationFailure> =
        decode_one_with_config(bytes, &config).map_err(|_| UploadAttestationReplyError::Invalid)?;
    let response = response.map_err(UploadAttestationReplyError::Remote)?;
    if response.permission != permission
        || response.verifier != authority.verifier()
        || response.owner != authority.service()
    {
        return Err(UploadAttestationReplyError::Binding);
    }
    CaffeineDownloadScope::new(response.owner, authority.namespace(), &response.project)
        .map_err(|_| UploadAttestationReplyError::Invalid)?;
    validate(permission, &response.declaration, limits.declaration).map_err(|e| match e {
        UploadManifestReplyError::Limit => UploadAttestationReplyError::Limit,
        _ => UploadAttestationReplyError::Invalid,
    })?;
    Ok(response)
}
