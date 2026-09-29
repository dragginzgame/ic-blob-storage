//! Enrollment authority, conversion and named endpoint contract over the shared owner.
pub mod client;
pub mod reply;
use super::{StableUploads, UploadStoreError};
use crate::{
    dto::tenant::{
        TenantEnrollment, TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest,
    },
    model::service::{
        tenant::{TenantEnrollmentView, TenantError, TenantUpdate},
        upload::UploadContext,
    },
    ops::service::tenant::TenantStoreError,
};
use candid::Principal;
use ic_memory::ic_stable_structures::Memory;
use std::num::NonZeroU64;

/// Explicit operator compare-and-set update; linking exports no endpoint.
pub const TENANT_UPDATE_METHOD: &str = "blob_update_tenant";
/// Scoped enrollment inspection for the operator or original tenant.
pub const TENANT_INSPECTION_METHOD: &str = "blob_tenant";

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

pub(crate) fn parse(input: TenantUpdateRequest) -> Result<TenantUpdate, TenantFailure> {
    Ok(TenantUpdate {
        tenant: input.scope.tenant,
        expected: input
            .expected
            .map(|v| {
                Ok(TenantEnrollmentView {
                    generation: NonZeroU64::new(v.generation).ok_or(TenantFailure::Invalid)?,
                    active: v.active,
                })
            })
            .transpose()?,
        active: input.active,
    })
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
