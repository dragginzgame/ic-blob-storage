//! Shared tenant admission and exact permission recovery over the durable owner.
use crate::{
    dto::upload::admission::{
        UploadAdmissionFailure, UploadAdmissionMutation, UploadAdmissionRequest,
        UploadAdmissionResponse,
    },
    model::service::upload::UploadContext,
    ops::service::uploads::{StableUploads, admission},
};
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
    let permission = admission::parse(context, input)?;
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
    let permission = admission::parse(context, input)?;
    let view = store
        .lookup(context, permission.request)
        .map_err(admission::failure)?;
    admission::present(input, &view)
}
