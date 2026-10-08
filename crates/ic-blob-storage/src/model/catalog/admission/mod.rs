//! Transient upload reservations sharing one catalog's capacity and root history.
//!
//! This model issues no certificates, calls no provider and persists no intent.
//! Production must durably commit admission and exposure before authority escapes.
//! A fresh instance is not recovery. No timeout or retry releases uncertain bytes.

mod accounting;
pub(crate) mod capacity;
pub mod read;
use accounting::ReservationAccounting;
use ic_blob_storage_contracts::configuration::limits::UploadLimits;
use ic_blob_storage_contracts::upload::binding::UploadRequest;
use ic_blob_storage_contracts::upload::binding::UploadRequestId;

use super::BlobCatalog;
use super::CatalogCapacity;
use super::CatalogError;
use super::CatalogUsage;
use super::ConfirmedObject;
use super::check_bytes;
use crate::model::lifecycle::LifecycleChange;
use crate::model::lifecycle::requests::ReferenceRequestOutcome;
use crate::model::lifecycle::roots::RootClaimError;
use candid::Principal;
use ic_blob_storage_contracts::binding::ObjectBinding;
use ic_blob_storage_contracts::binding::ObjectBindingError;
use ic_blob_storage_contracts::configuration::limits::CatalogLimits;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::reference::binding::ReferenceRequest;
use std::collections::BTreeMap;
use thiserror::Error;

#[cfg(test)]
mod tests;

/// Local operation state, not a provider observation or permission to send work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadPhase {
    /// Capacity reserved; no upload authority may have escaped yet.
    Reserved,
    /// Authority may have escaped; completion and effects remain unresolved.
    ExposurePossible,
    /// Exact independently authenticated completion was applied to the catalog.
    Confirmed,
    /// Cancelled before exposure; byte capacity freed but identity history retained.
    Cancelled,
}

impl UploadPhase {
    const fn active(self) -> bool {
        matches!(self, Self::Reserved | Self::ExposurePossible)
    }
}

/// Admission result. Replays report current local state, never new upload authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadAdmission {
    /// A fresh reservation and immutable root claim were recorded together.
    Reserved,
    /// Exact operation already exists; no state or accounting changed.
    Existing(UploadPhase),
}

#[derive(Debug, Eq, PartialEq)]
struct Operation {
    request: UploadRequest,
    phase: UploadPhase,
}

/// One owner of pending uploads and confirmed lifecycles, with no mutable escape.
///
/// Every admitted operation permanently consumes one object/root/history slot,
/// including cancelled and settled uploads. Active operations reserve logical,
/// physical and liability bytes, plus the future confirmed object's metadata
/// capacity. Confirmation transfers accounting without charging twice. Counts
/// are maintained with successful transitions; receipt history is never evicted. This owner
/// deliberately cannot be cloned, imported from a catalog, or serialized.
#[derive(Debug, Eq, PartialEq)]
pub struct UploadCatalog {
    catalog: BlobCatalog,
    limits: UploadLimits,
    operations: BTreeMap<(Principal, UploadRequestId), Operation>,
    accounting: ReservationAccounting,
}

impl UploadCatalog {
    /// Construct a fresh transient service owner, never restore an installation.
    /// # Errors
    /// Rejects an invalid service principal.
    pub fn new(
        service: Principal,
        catalog: CatalogLimits,
        limits: UploadLimits,
    ) -> Result<Self, ObjectBindingError> {
        Ok(Self {
            catalog: BlobCatalog::new(service, catalog)?,
            limits,
            operations: BTreeMap::new(),
            accounting: ReservationAccounting::default(),
        })
    }

    /// Reserve all capacity before any authority can escape, without an effect.
    ///
    /// `actor` must be the independently authenticated tenant principal. Delegated
    /// actors are not supported. Exact retries succeed even at capacity and after
    /// cancellation/settlement; changing an argument never reuses the operation.
    /// # Errors
    /// Rejects wrong actor/service, conflicting operation/root/object, or exhausted
    /// capacity. Rejections leave root claims, receipts and byte accounting intact.
    pub fn reserve(
        &mut self,
        actor: Principal,
        request: UploadRequest,
    ) -> Result<UploadAdmission, UploadError> {
        self.check_actor(actor, request)?;
        let key = Self::key(request);
        if let Some(operation) = self.operations.get(&key) {
            return if operation.request == request {
                Ok(UploadAdmission::Existing(operation.phase))
            } else {
                Err(UploadError::RequestConflict)
            };
        }
        // A different operation may not repeat even the identical object/root.
        if self.catalog.claims.resolve(request.object.root).is_ok() {
            return Err(UploadError::Root(RootClaimError::RootAlreadyClaimed));
        }
        capacity::check(
            self.usage(),
            self.tenant_usage(key.0),
            self.catalog.limits,
            self.limits,
            request.object.bytes,
        )?;
        // One first reference and its reserved release receipt always fit the
        // positive per-object bounds. Root claim is the last fallible mutation.
        self.catalog
            .claims
            .claim(request.object.root, request.object.first.object())?;
        self.operations.insert(
            key,
            Operation {
                request,
                phase: UploadPhase::Reserved,
            },
        );
        self.accounting.admit(key.0, request.object.bytes);
        Ok(UploadAdmission::Reserved)
    }

    /// Mark that upload authority may escape; this must precede any actual effect.
    ///
    /// Repeating this call reports `Unchanged`, not permission to repeat an upload.
    /// There is no return to Reserved, expiry or failed-transport reset.
    /// # Errors
    /// Rejects wrong actor, unknown/conflicting request, or terminal operation.
    pub fn mark_exposure_possible(
        &mut self,
        actor: Principal,
        request: UploadRequest,
    ) -> Result<LifecycleChange, UploadError> {
        self.check_actor(actor, request)?;
        self.transition(
            request,
            UploadPhase::Reserved,
            UploadPhase::ExposurePossible,
        )
    }

    /// Cancel only while the model knows upload authority has not escaped.
    ///
    /// Byte/concurrent capacity is released; the exact operation and root claim
    /// remain forever. A possibly exposed upload requires reconciliation instead.
    /// # Errors
    /// Rejects wrong actor, unknown/conflicting request, or exposure/completion.
    pub fn cancel(
        &mut self,
        actor: Principal,
        request: UploadRequest,
    ) -> Result<LifecycleChange, UploadError> {
        self.check_actor(actor, request)?;
        self.transition(request, UploadPhase::Reserved, UploadPhase::Cancelled)
    }

    /// Apply a trusted exact completion fact, transferring reservation to catalog.
    ///
    /// The caller must independently authenticate and correlate provider completion
    /// to this entire request, including provider root and stored size. A client hash/progress,
    /// timeout or generic HTTP success cannot supply that fact. This local method
    /// defines no provider evidence format. Replays never reactivate references.
    /// # Errors
    /// Rejects unknown/conflicting, unexposed or cancelled operations. All catalog
    /// capacity was reserved up front; rejection leaves the reservation intact.
    pub fn confirm_upload(
        &mut self,
        request: UploadRequest,
    ) -> Result<LifecycleChange, UploadError> {
        self.operation(request)?;
        let operation = self
            .operations
            .get_mut(&Self::key(request))
            .ok_or(UploadError::UnknownRequest)?;
        if operation.phase == UploadPhase::Confirmed {
            return Ok(LifecycleChange::Unchanged);
        }
        if operation.phase != UploadPhase::ExposurePossible {
            return Err(UploadError::InvalidPhase(operation.phase));
        }
        self.catalog.insert_confirmed(ConfirmedObject {
            root: request.object.root,
            bytes: request.object.bytes,
            first: request.object.first,
        })?;
        operation.phase = UploadPhase::Confirmed;
        self.accounting
            .release(Self::key(request).0, request.object.bytes);
        Ok(LifecycleChange::Changed)
    }

    /// Read current state for an exact authenticated tenant request.
    /// # Errors
    /// Rejects wrong actor/service, unknown ID or changed arguments.
    pub fn phase(
        &self,
        actor: Principal,
        request: UploadRequest,
    ) -> Result<UploadPhase, UploadError> {
        self.check_actor(actor, request)?;
        Ok(self.operation(request)?.phase)
    }

    /// Read confirmed objects only. Use this owner's usage for admission totals.
    /// Tenant/gateway authorization is still required by the caller for disclosure.
    #[must_use]
    pub const fn confirmed(&self) -> &BlobCatalog {
        &self.catalog
    }

    /// Apply an independently authorized confirmed-object reference request.
    /// # Errors
    /// Propagates catalog errors; active reservations cannot be released this way.
    pub fn apply_reference(
        &mut self,
        root: ProviderRootHash,
        actor: Principal,
        request: ReferenceRequest,
    ) -> Result<ReferenceRequestOutcome, CatalogError> {
        self.catalog.apply_reference(root, actor, request)
    }

    /// Apply independently authenticated exact physical deletion evidence.
    /// # Errors
    /// Propagates catalog lookup, binding and lifecycle errors.
    pub fn confirm_provider_deleted(
        &mut self,
        root: ProviderRootHash,
        object: ObjectBinding,
    ) -> Result<LifecycleChange, CatalogError> {
        self.catalog.confirm_provider_deleted(root, object)
    }

    /// Apply independently authenticated final billing cessation evidence.
    /// # Errors
    /// Propagates catalog lookup, binding and lifecycle errors.
    pub fn confirm_billing_stopped(
        &mut self,
        root: ProviderRootHash,
        object: ObjectBinding,
    ) -> Result<LifecycleChange, CatalogError> {
        self.catalog.confirm_billing_stopped(root, object)
    }

    /// Aggregate charged capacity, including reserved and possibly exposed uploads.
    #[must_use]
    pub fn usage(&self) -> UploadUsage {
        self.accounting.usage(self.catalog.usage())
    }

    /// Aggregate charged tenant capacity across namespaces; not authorization.
    /// Reservation and confirmed totals each require one tenant lookup; neither
    /// scans retained operation or object history.
    #[must_use]
    pub fn tenant_usage(&self, tenant: Principal) -> UploadUsage {
        self.accounting
            .tenant_usage(tenant, self.catalog.tenant_usage(tenant))
    }

    fn check_actor(&self, actor: Principal, request: UploadRequest) -> Result<(), UploadError> {
        if request.object.first.object().service() != self.catalog.service() {
            return Err(UploadError::Root(RootClaimError::WrongService));
        }
        if actor != request.object.first.object().tenant() {
            return Err(UploadError::Denied);
        }
        Ok(())
    }

    fn key(request: UploadRequest) -> (Principal, UploadRequestId) {
        (request.object.first.object().tenant(), request.id)
    }

    fn operation(&self, request: UploadRequest) -> Result<&Operation, UploadError> {
        let operation = self
            .operations
            .get(&Self::key(request))
            .ok_or(UploadError::UnknownRequest)?;
        if operation.request != request {
            return Err(UploadError::RequestConflict);
        }
        Ok(operation)
    }

    fn transition(
        &mut self,
        request: UploadRequest,
        from: UploadPhase,
        to: UploadPhase,
    ) -> Result<LifecycleChange, UploadError> {
        let phase = self.operation(request)?.phase;
        if phase == to {
            return Ok(LifecycleChange::Unchanged);
        }
        if phase != from {
            return Err(UploadError::InvalidPhase(phase));
        }
        self.operations
            .get_mut(&Self::key(request))
            .expect("validated operation")
            .phase = to;
        if from.active() && !to.active() {
            self.accounting
                .release(Self::key(request).0, request.object.bytes);
        }
        Ok(LifecycleChange::Changed)
    }
}

/// Read-only capacity charged to pending operations plus confirmed lifecycles.
/// Byte liabilities are capacity units, not measured currency or a spending cap.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UploadUsage {
    /// Lifetime operation/root slots, including cancelled and settled entries.
    pub operations: usize,
    /// Reserved or possibly exposed operations, including zero-byte uploads.
    pub active_reservations: usize,
    /// Bytes reserved by active operations, counted once in each total below.
    pub reserved_bytes: u128,
    /// Tenant logical bytes plus active reservations.
    pub logical_bytes: u128,
    /// Confirmed physical bytes plus active reservations.
    pub physical_bytes: u128,
    /// Unsettled confirmed bytes plus active reservations.
    pub liability_bytes: u128,
}

/// Rejected local admission/transition. No error authorizes repeating an effect.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum UploadError {
    /// The supplied authenticated actor is not the tenant owner.
    #[error("upload actor is not tenant owner")]
    Denied,
    /// No operation was admitted with this tenant/ID.
    #[error("unknown upload request")]
    UnknownRequest,
    /// The operation ID was already used with different exact arguments.
    #[error("upload request conflicts with original input")]
    RequestConflict,
    /// Global active reservation limit reached.
    #[error("active upload limit reached")]
    ActiveLimit,
    /// Per-tenant active reservation limit reached.
    #[error("tenant active upload limit reached")]
    TenantActiveLimit,
    /// The current phase forbids this transition.
    #[error("upload transition rejected in {0:?}")]
    InvalidPhase(UploadPhase),
    /// Immutable root/object/service history rejects admission.
    #[error(transparent)]
    Root(#[from] RootClaimError),
    /// Shared catalog capacity or lifecycle admission failed.
    #[error(transparent)]
    Catalog(#[from] CatalogError),
}
