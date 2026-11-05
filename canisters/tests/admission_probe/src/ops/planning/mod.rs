//! The transient probe uses the same capacity boundary as the durable service.
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityFailure;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityResponse;
use ic_blob_storage_contracts::upload::binding::UploadContext;
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
