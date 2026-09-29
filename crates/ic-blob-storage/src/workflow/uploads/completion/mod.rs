//! Explicit verifier trust, with exact immutable receipts and no on-canister file hashing.
use crate::{
    dto::upload::{
        admission::UploadAdmissionRequest,
        completion::{
            UploadAttestationFailure, UploadAttestationMutation, UploadAttestationRequest,
            UploadAttestationResponse,
        },
    },
    model::service::upload::{UploadContext, completion::CompletionAuthority},
    ops::service::uploads::{StableUploads, completion},
};
use ic_memory::ic_stable_structures::Memory;
/// Obtain installed provider mapping and original metadata for exposed unfinished content.
/// This is a snapshot, not a lease, upload/retry authority or provider availability evidence.
/// # Errors
/// Refuses wrong verifier or binding, prepared-only/confirmed content and restored owners.
pub fn verification_plan<M: Memory>(
    store: &StableUploads<M>,
    authority: CompletionAuthority,
    scope: &crate::model::service::read::download::CaffeineDownloadScope,
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<crate::dto::upload::completion::UploadVerificationPlan, UploadAttestationFailure> {
    completion::verification::plan(store, authority, scope, context, input)
}
/// Accept the installed verifier's statement for an already-exposed, prepared upload.
/// The verifier must independently fetch all bytes and verify the original root/metadata;
/// this workflow trusts that role, not a browser claim. It does not promise future
/// retention or billing cessation. Revocation/suspension cannot erase exposed obligations.
/// Propagate stable-write traps so receipt, first reference and accounting roll back together.
/// # Errors
/// Refuses wrong authority/scope/permission, unexposed uploads, changed evidence or restore fencing.
pub fn attest<M: Memory>(
    store: &mut StableUploads<M>,
    authority: CompletionAuthority,
    context: UploadContext,
    input: &UploadAttestationRequest,
    now: u64,
) -> Result<UploadAttestationMutation, UploadAttestationFailure> {
    completion::attest(store, authority, context, input, now)
}
/// Inspect immutable verifier evidence as that verifier, the tenant or original uploader.
/// Available after suspension, release and restoration; it grants no retry or liveness authority.
/// # Errors
/// Refuses wrong caller, scope, changed/missing permission or inconsistent state.
pub fn inspect<M: Memory>(
    store: &StableUploads<M>,
    authority: CompletionAuthority,
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadAttestationResponse, UploadAttestationFailure> {
    completion::inspect(store, authority, context, input)
}

/// Read the exact retained declaration as the installed verifier. This is historical
/// inspection through restore fencing, not permission to fetch or attest new content.
/// # Errors
/// Refuses wrong verifier/scope, changed permission and inconsistent retained state.
pub fn manifest<M: Memory>(
    store: &StableUploads<M>,
    authority: CompletionAuthority,
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<
    crate::dto::upload::manifest::UploadManifestResponse,
    crate::dto::upload::manifest::UploadManifestFailure,
> {
    completion::manifest(store, authority, context, input)
}
