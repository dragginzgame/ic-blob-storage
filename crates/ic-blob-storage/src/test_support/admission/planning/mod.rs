//! Passive tenant admission headroom from the same accounting used by mutations.

use crate::model::service::upload::UploadAdmissionError;
use ic_blob_storage_contracts::tenant::TenantError;
use ic_blob_storage_contracts::upload::binding::UploadContext;

use super::UploadAdmissions;
use crate::model::service::upload::planning::{
    AdmissionCapacityLookup, AdmissionCapacityView, headroom,
};

impl UploadAdmissions {
    /// Inspect capacity for an enrolled tenant without scanning operation history.
    ///
    /// The result is a current observation, including during suspension. It issues
    /// no admission, certificate or funding authority and changes no state. Hosts
    /// still need authenticated delivery and operational restore fencing.
    /// # Errors
    /// Rejects wrong service, namespace or tenant actor, and unenrolled tenants.
    pub fn admission_capacity(
        &self,
        context: UploadContext,
        input: AdmissionCapacityLookup,
    ) -> Result<AdmissionCapacityView, UploadAdmissionError> {
        super::validation::tenant(&self.config, context, input.tenant, input.namespace)?;
        let enrollment = self
            .tenants
            .get(input.tenant)
            .ok_or(TenantError::NotEnrolled)?;
        Ok(headroom(
            self.config.limits(),
            enrollment,
            self.catalog.usage(),
            self.catalog.tenant_usage(input.tenant),
            self.remaining_manifest_chunks(input.tenant),
        ))
    }
}
