//! Atomic confirmed lifecycle changes, individual reference rows and exact receipts.
use super::{
    ConfirmedLifecycleRecord, Key, LifecycleChange, Memory, ReferenceReceiptRecord,
    ReferenceRecord, StableUploads, UploadContext, UploadError, UploadPhase, UploadRequest,
    UploadStoreError, UploadStoreRecord, key, validation,
};
use crate::model::lifecycle::{
    LifecyclePhase, ReferenceState,
    binding::ReferenceKey,
    requests::{
        ReferenceOperation, ReferenceReceiptView, ReferenceRequest, ReferenceRequestError,
        ReferenceRequestOutcome,
    },
};
use candid::Principal;

/// Authenticated local confirmed-state observation, not provider evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConfirmedUploadView {
    /// Logical, physical and financial release phase.
    pub phase: LifecyclePhase,
    /// Lifetime reference identities, including released ones.
    pub reference_slots: u64,
    /// References that currently keep this object logically live.
    pub active_references: u64,
    /// Retained exact operation outcomes, including lifecycle failures.
    pub receipt_slots: u64,
}
impl<M: Memory> StableUploads<M> {
    /// Apply independently authenticated completion for the exact reserved root,
    /// size and object lifetime. This is trusted-host bookkeeping, not an endpoint.
    /// A browser digest, progress report or generic HTTP success is insufficient.
    /// Revocation after exposure cannot erase an actual completion obligation.
    /// # Errors
    /// Rejects scope, identity, wrong phase and restored owner before writes.
    /// # Panics
    /// Stable-write traps must roll back the entire IC update.
    pub fn confirm_upload(
        &mut self,
        request: UploadRequest,
    ) -> Result<LifecycleChange, UploadStoreError> {
        self.complete_upload(
            request,
            crate::model::service::upload::record::lifecycle::CompletionRecord::HostFact,
        )
    }
    pub(super) fn complete_upload(
        &mut self,
        request: UploadRequest,
        completion: crate::model::service::upload::record::lifecycle::CompletionRecord,
    ) -> Result<LifecycleChange, UploadStoreError> {
        self.trusted_request(request)?;
        self.mutable()?;
        let mut permission = self.required(request)?;
        let change = permission.confirm()?;
        if change == LifecycleChange::Unchanged {
            return Ok(change);
        }
        let tenant = request.object.first.object().tenant();
        let mut global = self.total(Principal::management_canister())?;
        let mut own = self.total(tenant)?;
        global.finish_reservation(request.object.bytes);
        own.finish_reservation(request.object.bytes);
        self.confirmed
            .insert(key(request), ConfirmedLifecycleRecord::new(completion));
        self.references.insert(
            (key(request), request.object.first.reference().get().get()),
            ReferenceRecord::new(ReferenceState::Active),
        );
        self.permissions
            .insert(key(request), UploadStoreRecord::Permission(permission));
        self.usage.insert(tenant, own);
        self.usage.insert(Principal::management_canister(), global);
        Ok(change)
    }
    /// Apply an exact tenant reference request with reserved cleanup receipt capacity.
    /// Recorded lifecycle errors consume a receipt; replays return the original result.
    /// # Errors
    /// Rejects scope, enrollment for fresh retains, changed identity, capacity or fence.
    /// # Panics
    /// Storage traps must roll back reference, receipt, lifecycle and usage together.
    pub fn apply_reference(
        &mut self,
        context: UploadContext,
        upload: UploadRequest,
        request: ReferenceRequest,
    ) -> Result<ReferenceRequestOutcome, UploadStoreError> {
        self.project(context, upload)?;
        self.mutable()?;
        let mut lifecycle = self.confirmed_record(upload)?;
        Self::reference_binding(upload, request)?;
        if let Some(receipt) = self.receipt(key(upload), context.actor, request)? {
            return Ok(ReferenceRequestOutcome::Replayed {
                result: receipt.result,
            });
        }
        if matches!(request.operation, ReferenceOperation::Retain(_)) {
            self.generation(context.actor)?;
        }
        let reference = request.operation.key().reference().get().get();
        let planned = lifecycle.plan(
            upload.object.first.object(),
            self.reference_state(key(upload), reference)?,
            request,
            self.config.limits().catalog,
        )?;
        let result = match &planned {
            Ok(Some(_)) => Ok(LifecycleChange::Changed),
            Ok(None) => Ok(LifecycleChange::Unchanged),
            Err(e) => Err(*e),
        };
        let before = lifecycle.phase();
        let mutation = planned.as_ref().ok().and_then(Option::as_ref);
        lifecycle.commit(mutation);
        let receipt = ReferenceReceiptRecord::new(context.actor, request, result);
        // Resolve all fallible reads and calculate counters before the first write.
        let mut global = self.total(Principal::management_canister())?;
        let mut own = self.total(context.actor)?;
        global.replace_lifecycle(upload.object.bytes, before, lifecycle.phase());
        own.replace_lifecycle(upload.object.bytes, before, lifecycle.phase());
        if let Some(m) = mutation {
            self.references.insert(
                (key(upload), m.reference.get().get()),
                ReferenceRecord::new(m.state),
            );
        }
        self.receipts
            .insert((key(upload), request.id.get().get()), receipt);
        self.confirmed.insert(key(upload), lifecycle);
        if before != lifecycle.phase() {
            self.usage.insert(context.actor, own);
            self.usage.insert(Principal::management_canister(), global);
        }
        Ok(ReferenceRequestOutcome::Recorded { result })
    }
    /// Inspect an exact historical reference receipt, including while restored/fenced.
    /// Historical retain success does not establish present reference liveness.
    /// # Errors
    /// Rejects wrong scope/actor, unconfirmed upload or changed request identity.
    pub fn reference_receipt(
        &self,
        context: UploadContext,
        upload: UploadRequest,
        request: ReferenceRequest,
    ) -> Result<Option<ReferenceReceiptView>, UploadStoreError> {
        self.project(context, upload)?;
        self.confirmed_record(upload)?;
        Self::reference_binding(upload, request)?;
        self.receipt(key(upload), context.actor, request)
    }
    /// Inspect one bound reference's current liveness as its tenant.
    /// # Errors
    /// Rejects wrong scope/actor, upload phase or reference object binding.
    pub fn reference_is_live(
        &self,
        context: UploadContext,
        upload: UploadRequest,
        reference: ReferenceKey,
    ) -> Result<bool, UploadStoreError> {
        self.project(context, upload)?;
        self.confirmed_record(upload)?;
        upload
            .object
            .first
            .object()
            .check(reference.object())
            .map_err(ReferenceRequestError::from)?;
        Ok(
            self.reference_state(key(upload), reference.reference().get().get())?
                == Some(ReferenceState::Active),
        )
    }
    /// Inspect confirmed history as the tenant; never grants a provider capability.
    /// # Errors
    /// Rejects scope/actor, missing/changed identity or unconfirmed phase.
    pub fn confirmed(
        &self,
        context: UploadContext,
        upload: UploadRequest,
    ) -> Result<ConfirmedUploadView, UploadStoreError> {
        self.project(context, upload)?;
        let record = self.confirmed_record(upload)?;
        let (reference_slots, active_references, receipt_slots) = record.counts();
        Ok(ConfirmedUploadView {
            phase: record.phase(),
            reference_slots,
            active_references,
            receipt_slots,
        })
    }
    /// Apply independently authenticated physical deletion for this exact object.
    /// Trusted hosts must verify provider authority and operation correlation first.
    /// # Errors
    /// Rejects live references, wrong identity/phase or restored owner.
    /// # Panics
    /// Storage traps must roll back lifecycle and both accounting rows.
    pub fn confirm_provider_deleted(
        &mut self,
        upload: UploadRequest,
    ) -> Result<LifecycleChange, UploadStoreError> {
        self.confirm_obligation(upload, false)
    }
    /// Apply independently authenticated final settlement and billing cessation.
    /// A missing object, zero balance or timeout cannot provide this evidence.
    /// # Errors
    /// Rejects unconfirmed physical deletion, wrong identity or restored owner.
    /// # Panics
    /// Storage traps must roll back lifecycle and both accounting rows.
    pub fn confirm_billing_stopped(
        &mut self,
        upload: UploadRequest,
    ) -> Result<LifecycleChange, UploadStoreError> {
        self.confirm_obligation(upload, true)
    }
    fn confirm_obligation(
        &mut self,
        upload: UploadRequest,
        billing: bool,
    ) -> Result<LifecycleChange, UploadStoreError> {
        self.trusted_request(upload)?;
        self.mutable()?;
        let mut record = self.confirmed_record(upload)?;
        let before = record.phase();
        let change = record.confirm(billing)?;
        if change == LifecycleChange::Unchanged {
            return Ok(change);
        }
        let tenant = upload.object.first.object().tenant();
        let mut global = self.total(Principal::management_canister())?;
        let mut own = self.total(tenant)?;
        global.replace_lifecycle(upload.object.bytes, before, record.phase());
        own.replace_lifecycle(upload.object.bytes, before, record.phase());
        self.confirmed.insert(key(upload), record);
        self.usage.insert(tenant, own);
        self.usage.insert(Principal::management_canister(), global);
        Ok(change)
    }
    fn trusted_request(&self, request: UploadRequest) -> Result<(), UploadStoreError> {
        validation::object(
            &self.config,
            UploadContext {
                service: self.config.bindings().service,
                actor: request.object.first.object().tenant(),
            },
            request.object.first.object(),
        )?;
        Ok(())
    }
    pub(super) fn confirmed_record(
        &self,
        upload: UploadRequest,
    ) -> Result<ConfirmedLifecycleRecord, UploadStoreError> {
        let view = self
            .required(upload)?
            .view()
            .ok_or(UploadStoreError::InvalidRecord)?;
        if view.phase != UploadPhase::Confirmed {
            return Err(UploadError::InvalidPhase(view.phase).into());
        }
        self.confirmed
            .get(&key(upload))
            .filter(|r| r.valid(self.config.limits().catalog))
            .ok_or(UploadStoreError::InvalidRecord)
    }
    fn reference_binding(
        upload: UploadRequest,
        request: ReferenceRequest,
    ) -> Result<(), UploadStoreError> {
        upload
            .object
            .first
            .object()
            .check(request.operation.key().object())
            .map_err(ReferenceRequestError::from)?;
        Ok(())
    }
    pub(super) fn reference_state(
        &self,
        owner: Key,
        reference: u128,
    ) -> Result<Option<ReferenceState>, UploadStoreError> {
        self.references
            .get(&(owner, reference))
            .map(|r| r.state().ok_or(UploadStoreError::InvalidRecord))
            .transpose()
    }
    fn receipt(
        &self,
        owner: Key,
        actor: Principal,
        request: ReferenceRequest,
    ) -> Result<Option<ReferenceReceiptView>, UploadStoreError> {
        let Some(record) = self.receipts.get(&(owner, request.id.get().get())) else {
            return Ok(None);
        };
        if !record.matches(actor, request) {
            return Err(ReferenceRequestError::RequestConflict.into());
        }
        Ok(Some(ReferenceReceiptView {
            result: record.result(),
        }))
    }
}
