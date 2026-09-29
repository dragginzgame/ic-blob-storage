//! Bounded completion replies from an independently authenticated service.
use crate::{
    dto::upload::{
        admission::UploadAdmissionRequest,
        completion::{
            UploadAttestationFailure, UploadAttestationLookup, UploadAttestationMutation,
            UploadAttestationReceipt, UploadAttestationRequest, UploadAttestationResponse,
        },
    },
    model::service::upload::completion::CompletionAuthority,
    ops::service::uploads::admission,
};
use candid::{CandidType, Deserialize, de::DecoderConfig, decode_one_with_config};
use std::num::NonZeroUsize;

/// Unusable evidence. No error or absent receipt authorizes a provider retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum UploadAttestationReplyError {
    /// Encoded reply exceeds the caller's bound.
    #[error("attestation reply exceeds limit")]
    Limit,
    /// Malformed data, invalid permission or impossible receipt chronology.
    #[error("invalid attestation reply")]
    Invalid,
    /// Service, namespace, permission, verifier or exact statement mismatch.
    #[error("attestation reply binding mismatch")]
    Binding,
    /// Authenticated service refusal, preserved separately from transport failure.
    #[error("attestation refused: {0:?}")]
    Remote(UploadAttestationFailure),
}

fn validate(
    authority: CompletionAuthority,
    permission: UploadAdmissionRequest,
) -> Result<(), UploadAttestationReplyError> {
    if permission.upload.service != authority.service()
        || permission.upload.namespace != authority.namespace().get()
    {
        return Err(UploadAttestationReplyError::Binding);
    }
    admission::parse_binding(authority.service(), permission)
        .map_err(|_| UploadAttestationReplyError::Invalid)?;
    Ok(())
}

/// Encode an exact historical lookup after checking the independently selected scope.
/// The caller must authenticate the service and the expected installed verifier.
/// # Errors
/// Rejects invalid permissions and service/namespace mismatches before transport.
pub fn inspection_request(
    authority: CompletionAuthority,
    permission: UploadAdmissionRequest,
) -> Result<Vec<u8>, UploadAttestationReplyError> {
    validate(authority, permission)?;
    candid::encode_one(permission).map_err(|_| UploadAttestationReplyError::Invalid)
}

fn decode<T: CandidType + for<'de> Deserialize<'de>>(
    bytes: &[u8],
    maximum: NonZeroUsize,
) -> Result<T, UploadAttestationReplyError> {
    if bytes.len() > maximum.get() {
        return Err(UploadAttestationReplyError::Limit);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(100_000)
        .set_skipping_quota(0)
        .set_max_type_len(64)
        .set_max_header_len(maximum.get())
        .set_full_error_message(false);
    let result: Result<T, UploadAttestationFailure> =
        decode_one_with_config(bytes, &config).map_err(|_| UploadAttestationReplyError::Invalid)?;
    result.map_err(UploadAttestationReplyError::Remote)
}

fn receipt(
    authority: CompletionAuthority,
    permission: UploadAdmissionRequest,
    receipt: &UploadAttestationReceipt,
) -> Result<(), UploadAttestationReplyError> {
    if receipt.request.permission != permission || receipt.verifier != authority.verifier() {
        return Err(UploadAttestationReplyError::Binding);
    }
    // Completion can arrive after permission expiry. Neither time nor a receipt
    // establishes current retention, reference liveness or permission to dispatch.
    if receipt.request.observed_at_ns > receipt.accepted_at_ns {
        return Err(UploadAttestationReplyError::Invalid);
    }
    Ok(())
}

/// Inspect exact historical evidence, preserving absence, conflicts and restore fences.
/// Compare a found receipt's statement with the saved intent before claiming recovery.
/// Authentication of these bytes and selection of `authority` belong to the caller.
/// # Errors
/// Rejects excessive/malformed replies, changed bindings and impossible chronology.
pub fn inspection(
    authority: CompletionAuthority,
    permission: UploadAdmissionRequest,
    bytes: &[u8],
    maximum: NonZeroUsize,
) -> Result<UploadAttestationResponse, UploadAttestationReplyError> {
    validate(authority, permission)?;
    let response: UploadAttestationResponse = decode(bytes, maximum)?;
    if response.permission != permission {
        return Err(UploadAttestationReplyError::Binding);
    }
    if let UploadAttestationLookup::Found(found) = &response.attestation {
        receipt(authority, permission, found)?;
    }
    Ok(response)
}

/// Validate acknowledgment of the exact saved statement, including replay acknowledgments.
/// This does not dispatch, retry or grant current object/reference liveness.
/// # Errors
/// Also rejects any digest or observation time unlike the submitted statement.
pub fn mutation(
    authority: CompletionAuthority,
    statement: &UploadAttestationRequest,
    bytes: &[u8],
    maximum: NonZeroUsize,
) -> Result<UploadAttestationMutation, UploadAttestationReplyError> {
    validate(authority, statement.permission)?;
    let response: UploadAttestationMutation = decode(bytes, maximum)?;
    receipt(authority, statement.permission, &response.receipt)?;
    if response.receipt.request != *statement {
        return Err(UploadAttestationReplyError::Binding);
    }
    Ok(response)
}

#[cfg(test)]
mod tests;
