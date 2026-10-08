//! Explicit verifier trust, with exact immutable receipts and no on-canister file hashing.
use crate::ops::service::uploads::StableUploads;
use crate::ops::service::uploads::completion;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationFailure;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationMutation;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationRequest;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationResponse;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::completion::CompletionAuthority;
use ic_memory::ic_stable_structures::Memory;
/// Obtain installed provider mapping and original metadata for exposed unfinished content.
/// This is a snapshot, not a lease, upload/retry authority or provider availability evidence.
/// # Errors
/// Refuses wrong verifier or binding, prepared-only/confirmed content and restored owners.
pub fn verification_plan<M: Memory>(
    store: &StableUploads<M>,
    authority: CompletionAuthority,
    scope: &ic_blob_storage_contracts::download::scope::CaffeineDownloadScope,
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<
    ic_blob_storage_contracts::dto::upload::completion::UploadVerificationPlan,
    UploadAttestationFailure,
> {
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
    ic_blob_storage_contracts::dto::upload::manifest::UploadManifestResponse,
    ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure,
> {
    completion::manifest(store, authority, context, input)
}
