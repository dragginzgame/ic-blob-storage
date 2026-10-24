//! Shared passive reference capacity, including reserved cleanup receipts.
use crate::model::lifecycle::requests::ReferenceCapacityView;
use crate::model::service::upload::UploadAdmissionError;
use crate::model::service::upload::content::ContentLookup;
use crate::ops::service::uploads::UploadStoreError;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityFailure as F;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityRequest;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityResponse;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceHeadroom;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::upload::binding::UploadContext;

use crate::ops::service::references::capacity::ReferenceCapacitySource;

fn failure(error: UploadStoreError) -> F {
    use UploadAdmissionError as A;
    match error {
        UploadStoreError::Binding
        | UploadStoreError::Admission(
            UploadAdmissionError::WrongService | UploadAdmissionError::WrongNamespace,
        ) => F::Binding,
        UploadStoreError::Admission(UploadAdmissionError::NotProject) => F::Denied,
        UploadStoreError::AlreadyAllocated
        | UploadStoreError::Missing
        | UploadStoreError::InvalidRecord
        | UploadStoreError::UnsupportedEnvelope
        | UploadStoreError::Fenced
        | UploadStoreError::Tenants(_)
        | UploadStoreError::Roots(_)
        | UploadStoreError::Reference(_)
        | UploadStoreError::Lifecycle(_)
        | UploadStoreError::CursorScope
        | UploadStoreError::Admission(
            A::ManifestCapacity(_)
            | A::ManifestNotPrepared
            | A::Manifest(_)
            | A::Metadata(_)
            | A::EmptyObject
            | A::NotOperator
            | A::Reference(_)
            | A::InvalidUploader
            | A::UnknownPermission
            | A::PermissionConflict
            | A::NotUploader
            | A::NotObserver
            | A::Revoked
            | A::Expired
            | A::ClockReversed
            | A::ObjectTooLarge
            | A::NotReserved(_)
            | A::Catalog(_)
            | A::Tenant(_),
        ) => F::Internal,
    }
}
fn present(view: ReferenceCapacityView) -> Result<ReferenceHeadroom, F> {
    let count = |v| u64::try_from(v).map_err(|_| F::Internal);
    Ok(ReferenceHeadroom {
        reference_slots: count(view.reference_slots)?,
        unreserved_receipts: count(view.unreserved_receipts)?,
        release_reserved_receipts: count(view.release_reserved_receipts)?,
        fresh_retains: count(view.fresh_retains)?,
    })
}
/// Inspect tenant-scoped reference headroom using authenticated service/caller context.
/// Hosts bound ingress and retain one synchronous owner borrow. Suspension and restore
/// preserve inspection; positive counts do not authorize retain, retry or publication.
/// Unknown/foreign/unconfirmed content shares the same absence response.
/// # Errors
/// Rejects invalid/foreign scope, unrelated callers or inconsistent retained counters.
/// # Panics
/// Panics if the fixed-width root type no longer matches the request contract.
pub fn inspect<S: ReferenceCapacitySource>(
    source: &S,
    context: UploadContext,
    request: ReferenceCapacityRequest,
) -> Result<ReferenceCapacityResponse, F> {
    let scope = request.scope;
    let namespace = crate::policy::tenant::assess_scope(
        context.service,
        context.actor,
        scope.service,
        scope.tenant,
        scope.namespace,
    )
    .map_err(|error| match error {
        crate::policy::tenant::TenantScopeError::Binding => F::Binding,
        crate::policy::tenant::TenantScopeError::Denied => F::Denied,
        crate::policy::tenant::TenantScopeError::Invalid => F::Invalid,
    })?;
    let input = ContentLookup {
        tenant: scope.tenant,
        namespace,
        root: ProviderRootHash::try_from(request.root.as_slice()).expect("fixed root width"),
    };
    let observed = source
        .reference_capacity_view(context, input)
        .map_err(failure)?;
    Ok(ReferenceCapacityResponse {
        request,
        headroom: observed.headroom.map(present).transpose()?,
        fenced: observed.fenced,
    })
}
