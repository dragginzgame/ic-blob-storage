//! The transient probe uses the same capacity boundary as the durable service.
use ic_blob_storage::{
    dto::{
        tenant::TenantScope,
        upload::capacity::{UploadCapacityFailure, UploadCapacityResponse},
    },
    model::service::upload::UploadContext,
};
pub(crate) fn capacity(
    context: UploadContext,
    input: TenantScope,
) -> Result<UploadCapacityResponse, UploadCapacityFailure> {
    super::STATE.with_borrow(|state| {
        ic_blob_storage::workflow::uploads::capacity::inspect(
            &state.as_ref().expect("initialized probe").owner,
            context,
            input,
        )
    })
}
