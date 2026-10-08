//! Tenant-bound indexed discovery over the maintained upload owner.
use crate::model::service::upload::UploadAdmissionError;
use crate::model::service::upload::UploadAdmissions;
use crate::model::service::upload::content::ContentLookup;
use crate::model::service::upload::content::TenantContentView;
use crate::ops::service::uploads::StableUploads;
use crate::ops::service::uploads::UploadStoreError;
use candid::Principal;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryFailure as F;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryRequest;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryResponse;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;
use std::num::NonZeroU128;

/// One synchronous owner observation, without an allocation or lifetime-history scan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadDiscoveryObservation {
    /// Absent for roots unavailable to this tenant.
    pub content: Option<TenantContentView>,
    /// The same owner's restore fence, independent of content visibility.
    pub fenced: bool,
}
/// Read access to one owner. Hosts keep the complete observation under one borrow.
pub trait UploadDiscoverySource {
    /// Authenticate context/scope and inspect retained content without writing.
    /// # Errors
    /// Rejects foreign scope/caller or inconsistent retained state.
    fn discovery_view(
        &self,
        context: UploadContext,
        input: ContentLookup,
    ) -> Result<UploadDiscoveryObservation, UploadStoreError>;
}
impl<M: Memory> UploadDiscoverySource for StableUploads<M> {
    fn discovery_view(
        &self,
        context: UploadContext,
        input: ContentLookup,
    ) -> Result<UploadDiscoveryObservation, UploadStoreError> {
        Ok(UploadDiscoveryObservation {
            content: self.lookup_content(context, input)?,
            fenced: self.is_fenced(),
        })
    }
}
impl UploadDiscoverySource for UploadAdmissions {
    fn discovery_view(
        &self,
        context: UploadContext,
        input: ContentLookup,
    ) -> Result<UploadDiscoveryObservation, UploadStoreError> {
        // The heap owner has no restore path; recreation is not operational recovery.
        Ok(UploadDiscoveryObservation {
            content: self.lookup_content(context, input)?,
            fenced: false,
        })
    }
}
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
pub(crate) fn inspect<S: UploadDiscoverySource>(
    source: &S,
    context: UploadContext,
    request: UploadDiscoveryRequest,
) -> Result<UploadDiscoveryResponse, F> {
    let scope = request.scope;
    if scope.service != context.service {
        return Err(F::Binding);
    }
    if scope.tenant != context.actor {
        return Err(F::Denied);
    }
    if [Principal::anonymous(), Principal::management_canister()].contains(&scope.tenant) {
        return Err(F::Invalid);
    }
    let input = ContentLookup {
        tenant: scope.tenant,
        namespace: NonZeroU128::new(scope.namespace).ok_or(F::Invalid)?,
        root: ProviderRootHash::try_from(request.root.as_slice()).expect("fixed root width"),
    };
    let observed = source.discovery_view(context, input).map_err(failure)?;
    Ok(UploadDiscoveryResponse {
        request,
        content: observed.content.map(super::history::present),
        fenced: observed.fenced,
    })
}
