//! Validate complete upload state without repairing missing evidence or counters.
use super::{ConfirmedLifecycleRecord, UploadRequest};
use super::{
    MANIFEST_BYTES, Memory, Principal, ServiceConfiguration, StableUploads, UploadAdmissionError,
    UploadConfigurationRecord, UploadContext, UploadManifestState, UploadPhase, UploadStoreError,
    UploadStoreRecord, UploadUsageRecord, key, metadata, validation,
};
use crate::model::lifecycle::{LifecyclePhase, ReferenceState};
use std::collections::BTreeMap as HeapMap;

pub(crate) fn envelope(config: &ServiceConfiguration) -> Result<(), UploadStoreError> {
    let limits = config.manifest_limits();
    // Fixed Candid type/framing allowance, 33 bytes per encoded leaf, and a
    // conservative per-header string-length overhead beyond the raw byte budget.
    let worst = 1024u128
        + 33 * limits.max_chunks.get() as u128
        + limits.max_header_bytes.get() as u128
        + 8 * limits.max_headers.get() as u128;
    if worst > MANIFEST_BYTES as u128 {
        return Err(UploadStoreError::UnsupportedEnvelope);
    }
    Ok(())
}
pub(super) fn manifest_capacity(
    config: &ServiceConfiguration,
    global: UploadUsageRecord,
    tenant: UploadUsageRecord,
    bytes: u64,
) -> Result<(), UploadAdmissionError> {
    crate::model::service::upload::capacity::check(
        config.limits().manifests,
        global.chunks(),
        tenant.chunks(),
        bytes,
    )
}
impl<M: Memory> StableUploads<M> {
    pub(super) fn validate(&self) -> Result<(), UploadStoreError> {
        if self.permissions.get(&metadata())
            != Some(UploadStoreRecord::Configuration(
                UploadConfigurationRecord::new(&self.config),
            ))
        {
            return Err(UploadStoreError::Binding);
        }
        let limit = self.config.limits();
        let count = self.permissions.len() - 1;
        if count > limit.catalog.max_objects.get() as u64
            || self.roots.slots() != count
            || self.root_requests.len() != count
            || self.manifests.len() > count
            || self.usage.len() > limit.max_tenants.get() as u64 + 1
            || self.confirmed.len() > count
            || self.references.len() > count * limit.catalog.max_references_per_object.get() as u64
            || self.receipts.len() > count * limit.catalog.max_receipts_per_object.get() as u64
        {
            return Err(UploadStoreError::InvalidRecord);
        }
        let mut totals =
            HeapMap::from([(Principal::management_canister(), UploadUsageRecord::empty())]);
        let mut manifest_count = 0;
        let mut confirmed_count = 0;
        let mut reference_count = 0;
        let mut receipt_count = 0;
        for entry in self.permissions.iter() {
            if *entry.key() == metadata() {
                continue;
            }
            let UploadStoreRecord::Permission(record) = entry.value() else {
                return Err(UploadStoreError::InvalidRecord);
            };
            let view = record.view().ok_or(UploadStoreError::InvalidRecord)?;
            let input = view.permission;
            let object = input.request.object.first.object();
            let context = UploadContext {
                service: self.config.bindings().service,
                actor: object.tenant(),
            };
            if key(input.request) != *entry.key() {
                return Err(UploadStoreError::InvalidRecord);
            }
            // The opened root owner binds each root to one exact object/tenant.
            // This index binds it to one request ID, and the permission key binds
            // that tenant/ID pair. Together they rule out duplicate roots without
            // retaining another per-object heap index during restoration.
            if self.root_requests.get(input.request.object.root.as_bytes())
                != Some(input.request.id.get().get())
            {
                return Err(UploadStoreError::InvalidRecord);
            }
            validation::object(&self.config, context, object)
                .map_err(|_| UploadStoreError::InvalidRecord)?;
            validation::fresh(&self.config, input, view.admitted_at_ns)
                .map_err(|_| UploadStoreError::InvalidRecord)?;
            let tenant = self
                .tenants
                .enrollment_for_owner(object.tenant())?
                .ok_or(UploadStoreError::InvalidRecord)?;
            if view.tenant_generation > tenant.generation
                || self.roots.lookup(context, input.request.object.root)? != Some(object)
            {
                return Err(UploadStoreError::InvalidRecord);
            }
            manifest_count += u64::from(self.validate_manifest(input.request, view.manifest)?);
            let confirmed =
                self.validate_confirmed(input.request, view.phase, view.admitted_at_ns)?;
            if let Some(record) = confirmed {
                confirmed_count += 1;
                reference_count += record.counts().0;
                receipt_count += record.counts().2;
            }
            for tenant in [Principal::management_canister(), object.tenant()] {
                let total = totals
                    .entry(tenant)
                    .or_insert_with(UploadUsageRecord::empty);
                total.admit(input.request.object.bytes);
                if view.phase == UploadPhase::Cancelled {
                    total.cancel(input.request.object.bytes);
                }
                if let Some(record) = confirmed {
                    total.finish_reservation(input.request.object.bytes);
                    total.replace_lifecycle(
                        input.request.object.bytes,
                        LifecyclePhase::Live,
                        record.phase(),
                    );
                }
            }
        }
        if manifest_count != self.manifests.len()
            || totals.len() as u64 != self.usage.len()
            || confirmed_count != self.confirmed.len()
            || reference_count != self.references.len()
            || receipt_count != self.receipts.len()
        {
            return Err(UploadStoreError::InvalidRecord);
        }
        self.validate_totals(totals)
    }
    fn validate_manifest(
        &self,
        request: UploadRequest,
        state: UploadManifestState,
    ) -> Result<bool, UploadStoreError> {
        match (state, self.manifests.get(&key(request))) {
            (UploadManifestState::Unprepared, None) => Ok(false),
            (UploadManifestState::Bound, Some(manifest))
                if manifest.validate(&self.config, request) =>
            {
                Ok(true)
            }
            _ => Err(UploadStoreError::InvalidRecord),
        }
    }
    fn validate_totals(
        &self,
        totals: HeapMap<Principal, UploadUsageRecord>,
    ) -> Result<(), UploadStoreError> {
        let limit = self.config.limits();
        for (tenant, expected) in totals {
            if self.usage.get(&tenant) != Some(expected) {
                return Err(UploadStoreError::InvalidRecord);
            }
            let view = expected.view().ok_or(UploadStoreError::InvalidRecord)?;
            let valid = if tenant == Principal::management_canister() {
                view.operations <= limit.catalog.max_objects.get()
                    && view.active_reservations <= limit.uploads.max_active.get()
                    && view.physical_bytes <= limit.catalog.max_physical_bytes.get()
                    && view.liability_bytes <= limit.catalog.max_liability_bytes.get()
                    && expected.chunks() <= limit.manifests.max_chunks.get() as u64
            } else {
                view.operations <= limit.catalog.max_tenant_objects.get()
                    && view.active_reservations <= limit.uploads.max_tenant_active.get()
                    && view.logical_bytes <= limit.catalog.max_tenant_logical_bytes.get()
                    && expected.chunks() <= limit.manifests.max_tenant_chunks.get() as u64
            };
            if !valid {
                return Err(UploadStoreError::InvalidRecord);
            }
        }
        Ok(())
    }
    fn validate_confirmed(
        &self,
        upload: UploadRequest,
        phase: UploadPhase,
        admitted_at_ns: u64,
    ) -> Result<Option<ConfirmedLifecycleRecord>, UploadStoreError> {
        let owner = key(upload);
        let retained = self.confirmed.get(&owner);
        if phase != UploadPhase::Confirmed {
            return if retained.is_none() {
                Ok(None)
            } else {
                Err(UploadStoreError::InvalidRecord)
            };
        }
        let record = retained
            .filter(|r| r.valid(self.config.limits().catalog))
            .ok_or(UploadStoreError::InvalidRecord)?;
        if let crate::model::service::upload::record::lifecycle::CompletionRecord::Attested(
            evidence,
        ) = record.completion()
            && evidence.observed_at_ns < admitted_at_ns
        {
            return Err(UploadStoreError::InvalidRecord);
        }
        let mut references = 0;
        let mut active = 0;
        for entry in self.references.range((owner, 0)..=(owner, u128::MAX)) {
            if entry.key().1 == 0 {
                return Err(UploadStoreError::InvalidRecord);
            }
            let state = entry
                .value()
                .state()
                .ok_or(UploadStoreError::InvalidRecord)?;
            references += 1;
            active += u64::from(state == ReferenceState::Active);
        }
        if self
            .reference_state(owner, upload.object.first.reference().get().get())?
            .is_none()
        {
            return Err(UploadStoreError::InvalidRecord);
        }
        let mut receipts = 0;
        for entry in self.receipts.range((owner, 0)..=(owner, u128::MAX)) {
            let receipt = entry.value();
            if entry.key().1 == 0
                || !receipt.valid(owner.0, self.reference_state(owner, receipt.reference())?)
            {
                return Err(UploadStoreError::InvalidRecord);
            }
            receipts += 1;
        }
        if record.counts() != (references, active, receipts) {
            return Err(UploadStoreError::InvalidRecord);
        }
        Ok(Some(record))
    }
}
