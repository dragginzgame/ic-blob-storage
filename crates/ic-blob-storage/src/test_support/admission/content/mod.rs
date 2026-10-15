//! Reference-model content and capacity observations.

use crate::model::catalog::admission::UploadPhase;
use crate::model::lifecycle::requests::ReferenceCapacityView;
use crate::model::lifecycle::requests::ReferenceRequests;
use crate::model::service::upload::UploadAdmissionError;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::history::UploadRootState;

use super::UploadAdmissions;
use crate::model::service::upload::content::{ContentLookup, TenantContentView};

impl UploadAdmissions {
    /// Inspect confirmed reference capacity with the same tenant authority as discovery.
    ///
    /// Unknown, foreign and unconfirmed objects return `None`. Retired confirmed
    /// objects remain visible with zero fresh retains. No result reserves capacity;
    /// retain/release operations still enforce current enrollment and exact identity.
    /// # Errors
    /// Returns the same service, namespace and tenant rejection as content discovery.
    pub fn reference_capacity(
        &self,
        context: UploadContext,
        input: ContentLookup,
    ) -> Result<Option<ReferenceCapacityView>, UploadAdmissionError> {
        let Some(_) = self.lookup_content(context, input)? else {
            return Ok(None);
        };
        Ok(self
            .catalog
            .confirmed()
            .get(input.root)
            .map(ReferenceRequests::capacity))
    }

    /// Find this tenant's original operation using the retained root index.
    ///
    /// Suspended tenants can still inspect their obligations. Uploaders and operators
    /// have no implicit discovery authority. Unknown and foreign-owned roots both
    /// return `None`; this neither proves global absence nor reserves fresh capacity.
    /// Cancelled and settled roots remain visible and cannot be reallocated. A live
    /// result requires an exact successful retain before a new consumer may publish
    /// a reference; the read itself consumes no reference, receipt or quota.
    /// Work uses logarithmic lookups, without scanning retained operation history.
    /// # Errors
    /// Rejects wrong service, namespace or tenant actor before looking up a root.
    /// # Panics
    /// Panics if the private index, permission or confirmed catalog disagree.
    pub fn lookup_content(
        &self,
        context: UploadContext,
        input: ContentLookup,
    ) -> Result<Option<TenantContentView>, UploadAdmissionError> {
        super::validation::tenant(&self.config, context, input.tenant, input.namespace)?;
        let Some(permission_key) = self.permission_roots.get(&input.root) else {
            return Ok(None);
        };
        if permission_key.0 != input.tenant {
            return Ok(None);
        }
        let request = self
            .permissions
            .get(permission_key)
            .expect("indexed permission")
            .original
            .request;
        let state = match self
            .catalog
            .phase(input.tenant, request)
            .expect("indexed operation")
        {
            UploadPhase::Reserved => UploadRootState::Reserved,
            UploadPhase::ExposurePossible => UploadRootState::ExposurePossible,
            UploadPhase::Cancelled => UploadRootState::Cancelled,
            UploadPhase::Confirmed => UploadRootState::Confirmed(
                self.catalog
                    .confirmed()
                    .get(input.root)
                    .expect("confirmed operation")
                    .lifecycle()
                    .phase(),
            ),
        };
        Ok(Some(TenantContentView { request, state }))
    }
}
