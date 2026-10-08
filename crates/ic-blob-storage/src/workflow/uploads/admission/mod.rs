//! Shared tenant admission and exact permission recovery over the durable owner.
use crate::ops::service::uploads::StableUploads;
use crate::ops::service::uploads::admission;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionMutation;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;
/// Reserve one exact permission synchronously using actual host caller/service/time.
/// Persist consumer intent before dispatch. This issues no certificate or provider effect.
/// The adapter must propagate traps to preserve IC atomic rollback of stable writes.
/// # Errors
/// Refuses wrong authority, conflicting identities, invalid fresh permissions,
/// exhausted capacity and every mutation on a restored owner, including replay.
pub fn admit<M: Memory>(
    store: &mut StableUploads<M>,
    context: UploadContext,
    input: UploadAdmissionRequest,
    now: u64,
) -> Result<UploadAdmissionMutation, UploadAdmissionFailure> {
    let permission = ic_blob_storage_contracts::upload::admission::parse(context, input)?;
    let outcome = store
        .admit(context, permission, now)
        .map_err(admission::failure)?;
    let observed = inspect(store, context, input)?;
    Ok(admission::mutation(observed, outcome))
}
/// Inspect the full original permission, including uploader and deadline. Available
/// under suspension/expiry/restore; no observation renews issuance authority.
/// # Errors
/// Rejects foreign, missing or changed permissions. Unknown is not retry authority.
pub fn inspect<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, UploadAdmissionFailure> {
    let permission = ic_blob_storage_contracts::upload::admission::parse(context, input)?;
    let view = store
        .lookup(context, permission.request)
        .map_err(admission::failure)?;
    admission::present(input, &view)
}

/// Withdraw the tenant's exact original permission, including uploader and expiry.
/// Cancellation before exposure releases its reservation; possible exposure stays
/// charged. Confirmed references, provider deletion and billing remain separate.
/// Suspension and expiry permit cleanup. Restore rejects all mutation, even replay.
/// The host must propagate stable-write traps for atomic IC rollback.
/// # Errors
/// Rejects foreign, missing or changed permissions and restored owners.
pub fn revoke<M: Memory>(
    store: &mut StableUploads<M>,
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<
    ic_blob_storage_contracts::dto::upload::admission::UploadRevocationResponse,
    UploadAdmissionFailure,
> {
    let permission = ic_blob_storage_contracts::upload::admission::parse(context, input)?;
    // Check the full original permission before withdrawing any authority.
    inspect(store, context, input)?;
    let change = store
        .revoke(context, permission.request)
        .map_err(admission::failure)?;
    let observed = inspect(store, context, input)?;
    Ok(admission::revocation(observed, change))
}
