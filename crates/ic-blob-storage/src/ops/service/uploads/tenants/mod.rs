//! Enrollment authority, conversion and named endpoint contract over the shared owner.
pub mod client;
use super::{StableUploads, UploadStoreError};
use crate::ops::service::tenant::TenantStoreError;
use candid::Principal;
use ic_blob_storage_contracts::dto::tenant::TenantEnrollment;
use ic_blob_storage_contracts::dto::tenant::TenantEnrollmentResponse;
use ic_blob_storage_contracts::dto::tenant::TenantFailure;
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::tenant::TenantEnrollmentView;
use ic_blob_storage_contracts::tenant::TenantError;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;

pub(crate) fn authorize<M: Memory>(
    store: &StableUploads<M>,
    context: UploadContext,
    scope: TenantScope,
    mutation: bool,
) -> Result<(), TenantFailure> {
    let binding = store.config.bindings();
    if context.service != binding.service
        || scope.service != binding.service
        || scope.namespace != binding.namespace.get()
    {
        return Err(TenantFailure::Binding);
    }
    let operator = context.actor == binding.operator;
    let observer = operator || context.actor == scope.tenant;
    if (mutation && !operator) || (!mutation && !observer) {
        return Err(TenantFailure::Denied);
    }
    if scope.tenant == Principal::anonymous() || scope.tenant == Principal::management_canister() {
        return Err(TenantFailure::Invalid);
    }
    Ok(())
}

pub(crate) fn present(
    scope: TenantScope,
    view: Option<TenantEnrollmentView>,
    fenced: bool,
) -> TenantEnrollmentResponse {
    TenantEnrollmentResponse {
        scope,
        enrollment: view.map(|v| TenantEnrollment {
            generation: v.generation.get(),
            active: v.active,
        }),
        fenced,
    }
}

pub(crate) fn failure(error: UploadStoreError) -> TenantFailure {
    use TenantFailure as F;
    use TenantStoreError as S;
    match error {
        UploadStoreError::Binding | UploadStoreError::Tenants(S::WrongService | S::Binding) => {
            F::Binding
        }
        UploadStoreError::Tenants(S::NotOperator | S::NotObserver) => F::Denied,
        UploadStoreError::Fenced | UploadStoreError::Tenants(S::Fenced) => F::Fenced,
        UploadStoreError::Tenants(S::Tenant(TenantError::InvalidTenant)) => F::Invalid,
        UploadStoreError::Tenants(S::Tenant(TenantError::Conflict)) => F::Conflict,
        UploadStoreError::Tenants(S::Tenant(TenantError::Capacity)) => F::Capacity,
        UploadStoreError::Tenants(S::Tenant(TenantError::GenerationExhausted)) => {
            F::GenerationExhausted
        }
        _ => F::Internal,
    }
}
