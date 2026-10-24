//! Shared indexed tenant content discovery.
use crate::model::service::upload::UploadAdmissionError;
use crate::model::service::upload::content::ContentLookup;
use crate::ops::service::uploads::UploadStoreError;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryFailure as F;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryRequest;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryResponse;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::upload::binding::UploadContext;

use crate::ops::service::uploads::discovery::UploadDiscoverySource;

fn failure(error: UploadStoreError) -> F {
    match error {
        UploadStoreError::Binding
        | UploadStoreError::Admission(
            UploadAdmissionError::WrongService | UploadAdmissionError::WrongNamespace,
        ) => F::Binding,
        UploadStoreError::Admission(UploadAdmissionError::NotProject) => F::Denied,
        _ => F::Internal,
    }
}
/// Discover complete original identities and current local state under one owner borrow.
/// Suspension and restoration preserve passive inspection. Unknown and foreign roots
/// share absence; neither absence nor confirmed history authorizes upload or serving.
/// # Errors
/// Rejects invalid/foreign scope, unrelated callers or inconsistent retained state.
/// # Panics
/// Panics if the fixed-width root type no longer matches the request contract.
pub fn inspect<S: UploadDiscoverySource>(
    source: &S,
    context: UploadContext,
    request: UploadDiscoveryRequest,
) -> Result<UploadDiscoveryResponse, F> {
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
    let observed = source.discovery_view(context, input).map_err(failure)?;
    Ok(UploadDiscoveryResponse {
        request,
        content: observed
            .content
            .map(crate::ops::service::uploads::history::present),
        fenced: observed.fenced,
    })
}
