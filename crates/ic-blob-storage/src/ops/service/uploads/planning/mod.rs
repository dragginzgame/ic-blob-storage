//! Capacity observations from maintained counters, without loading histories.
use super::{
    Memory, StableUploads, UploadAdmissionError, UploadContext, UploadStoreError, validation,
};
use crate::model::{
    catalog::admission::read::UploadRootState,
    lifecycle::requests::{ReferenceCapacityView, reference_headroom},
    service::{
        tenant::TenantError,
        upload::{
            capacity,
            content::ContentLookup,
            planning::{AdmissionCapacityLookup, AdmissionCapacityView, headroom},
        },
    },
};
use candid::Principal;

impl<M: Memory> StableUploads<M> {
    /// Observe independent admission dimensions using fixed enrollment/counter reads.
    /// Suspension and restore fencing preserve inspection. Headroom is not a
    /// reservation, fresh identity, provider permission or authority to unfence.
    /// # Errors
    /// Rejects wrong service/namespace/tenant, unenrolled tenants or invalid counters.
    pub fn admission_capacity(
        &self,
        execution: UploadContext,
        input: AdmissionCapacityLookup,
    ) -> Result<AdmissionCapacityView, UploadStoreError> {
        validation::tenant(&self.config, execution, input.tenant, input.namespace)?;
        let enrollment = self
            .tenants
            .enrollment_for_owner(input.tenant)?
            .ok_or(UploadAdmissionError::Tenant(TenantError::NotEnrolled))?;
        let global = self.total(Principal::management_canister())?;
        let tenant = self.total(input.tenant)?;
        Ok(headroom(
            self.config.limits(),
            enrollment,
            global.view().ok_or(UploadStoreError::InvalidRecord)?,
            tenant.view().ok_or(UploadStoreError::InvalidRecord)?,
            capacity::remaining(
                self.config.limits().manifests,
                global.chunks(),
                tenant.chunks(),
            ),
        ))
    }

    /// Observe confirmed reference/receipt headroom with cleanup slots reserved.
    /// Unknown, foreign and unconfirmed roots return `None`. Retired objects have
    /// zero fresh retains. Reads do not bypass suspension or restored mutation fences.
    /// # Errors
    /// Rejects wrong service/namespace/tenant or inconsistent retained state.
    pub fn reference_capacity(
        &self,
        execution: UploadContext,
        input: ContentLookup,
    ) -> Result<Option<ReferenceCapacityView>, UploadStoreError> {
        let Some(content) = self.lookup_content(execution, input)? else {
            return Ok(None);
        };
        let UploadRootState::Confirmed(phase) = content.state else {
            return Ok(None);
        };
        let record = self.confirmed_record(content.request)?;
        let (references, active, receipts) = record.counts();
        let references =
            usize::try_from(references).map_err(|_| UploadStoreError::InvalidRecord)?;
        let active = usize::try_from(active).map_err(|_| UploadStoreError::InvalidRecord)?;
        let receipts = usize::try_from(receipts).map_err(|_| UploadStoreError::InvalidRecord)?;
        let limits = self.config.limits().catalog;
        Ok(Some(reference_headroom(
            phase,
            limits.max_references_per_object.get() - references,
            active,
            limits.max_receipts_per_object.get() - receipts,
        )))
    }
}
