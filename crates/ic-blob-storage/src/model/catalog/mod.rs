//! Bounded transient ownership of confirmed objects, root claims and receipts.
//!
//! No upload admission, persistence or provider effects occur here. Insert only
//! independently confirmed objects whose upload was already safely admitted.
//! Production must reserve capacity and root history before upload authority is
//! exposed; discovering a full catalog after uploading is not safe admission.
//! One authoritative instance must own this state. Cloning/reconstruction does
//! not establish restore safety, fresh identities or permission to discard history.

pub mod pending;

#[cfg(test)]
mod tests;

use std::{
    collections::BTreeMap,
    num::{NonZeroU128, NonZeroUsize},
};

use candid::Principal;
use thiserror::Error;

use crate::model::{
    identity::ProviderRootHash,
    lifecycle::{
        BlobLifecycle, LifecycleChange, LifecycleError, LifecyclePhase,
        binding::{ObjectBinding, ObjectBindingError, ReferenceKey},
        requests::{
            ReferenceRequest, ReferenceRequestError, ReferenceRequestOutcome, ReferenceRequests,
        },
        roots::{RootClaimError, RootClaims},
    },
};

/// Explicit lifetime and outstanding-obligation bounds. No defaults are inferred.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CatalogLimits {
    /// Lifetime object/root slots, including settled objects and their history.
    pub max_objects: NonZeroUsize,
    /// Lifetime object/root slots per tenant across namespaces, including zero bytes.
    pub max_tenant_objects: NonZeroUsize,
    /// Physical bytes across every tenant, including logically released objects.
    pub max_physical_bytes: NonZeroU128,
    /// Bytes with outstanding billing obligations, including physically deleted objects.
    pub max_liability_bytes: NonZeroU128,
    /// Logical bytes per tenant, counted once per live object across namespaces.
    pub max_tenant_logical_bytes: NonZeroU128,
    /// Lifetime reference slots per object, including released identities.
    pub max_references_per_object: NonZeroUsize,
    /// Lifetime request receipts per object; active-reference release slots are reserved.
    pub max_receipts_per_object: NonZeroUsize,
}

/// Local bookkeeping input for an independently confirmed object, not upload proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConfirmedObject {
    /// Root already bound exclusively to this object before provider effects.
    pub root: ProviderRootHash,
    /// Exact confirmed byte length, including zero-length objects.
    pub bytes: u64,
    /// First reference, carrying service/tenant/namespace/object/incarnation identity.
    pub first: ReferenceKey,
}

/// Reference lookup qualified by its provider root and complete local binding.
///
/// Neither the root nor the reference establishes caller authority. The catalog
/// must resolve trusted ownership before interpreting the supplied binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CatalogReferenceKey {
    /// Root used to locate the catalog entry.
    pub root: ProviderRootHash,
    /// Complete expected object incarnation and reference identity.
    pub reference: ReferenceKey,
}

/// Outcome of recording a confirmed object; neither variant authorizes upload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CatalogInsertOutcome {
    /// New object, root history and reference journal were recorded together.
    Inserted,
    /// Exact original input was already recorded; no references are reactivated.
    Existing,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Entry {
    original: ConfirmedObject,
    requests: ReferenceRequests,
}

/// One service's confirmed-object collection with no independent mutable escape.
///
/// Settled entries remain counted against lifetime capacity. No remove/reset API
/// erases the only root or receipt evidence. Derived usage avoids independently
/// mutable counters drifting from object state. Scans are bounded by `max_objects`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlobCatalog {
    service: Principal,
    limits: CatalogLimits,
    claims: RootClaims,
    entries: BTreeMap<ProviderRootHash, Entry>,
}

impl BlobCatalog {
    /// Construct a fresh local catalog, never restore an existing installation.
    /// # Errors
    /// Rejects anonymous or management service principals.
    pub fn new(service: Principal, limits: CatalogLimits) -> Result<Self, ObjectBindingError> {
        Ok(Self {
            service,
            limits,
            claims: RootClaims::new(service, limits.max_objects)?,
            entries: BTreeMap::new(),
        })
    }

    /// Record a confirmed object with exact replay and all-or-nothing admission.
    ///
    /// Bounds are checked before root claims change. Exact replay is accepted at
    /// capacity and after release/settlement, without reactivating its reference.
    /// Physical deletion frees physical capacity; only settlement frees liability
    /// bytes. Neither frees lifetime object/root slots. Tenant logical capacity
    /// releases only after the last live reference.
    /// # Errors
    /// Rejects wrong service, conflicting identity/payload, exhausted lifetime or
    /// byte capacity, or journal admission. Any rejection leaves the catalog intact.
    pub fn insert_confirmed(
        &mut self,
        input: ConfirmedObject,
    ) -> Result<CatalogInsertOutcome, CatalogError> {
        if input.first.object().service() != self.service {
            return Err(RootClaimError::WrongService.into());
        }
        if let Some(existing) = self.entries.get(&input.root) {
            if existing.original.first.object() != input.first.object() {
                return Err(RootClaimError::RootAlreadyClaimed.into());
            }
            return if existing.original == input {
                Ok(CatalogInsertOutcome::Existing)
            } else {
                Err(CatalogError::RegistrationConflict)
            };
        }
        let usage = self.usage();
        if usage.objects >= self.limits.max_objects.get() {
            return Err(CatalogError::Capacity(CatalogCapacity::Objects));
        }
        let tenant_usage = self.tenant_usage(input.first.object().tenant());
        if tenant_usage.objects >= self.limits.max_tenant_objects.get() {
            return Err(CatalogError::Capacity(CatalogCapacity::TenantObjects));
        }
        check_bytes(
            usage.physical_bytes,
            input.bytes,
            self.limits.max_physical_bytes,
            CatalogCapacity::PhysicalBytes,
        )?;
        check_bytes(
            usage.liability_bytes,
            input.bytes,
            self.limits.max_liability_bytes,
            CatalogCapacity::LiabilityBytes,
        )?;
        check_bytes(
            tenant_usage.logical_bytes,
            input.bytes,
            self.limits.max_tenant_logical_bytes,
            CatalogCapacity::TenantLogicalBytes,
        )?;
        let requests = ReferenceRequests::new(
            BlobLifecycle::from_confirmed_upload(
                input.bytes,
                input.first,
                self.limits.max_references_per_object,
            ),
            self.limits.max_receipts_per_object,
        )?;
        // This is the last fallible step: no partial root claim on a capacity error.
        self.claims.claim(input.root, input.first.object())?;
        self.entries.insert(
            input.root,
            Entry {
                original: input,
                requests,
            },
        );
        Ok(CatalogInsertOutcome::Inserted)
    }

    /// Apply an already authorized reference request, retaining exact receipts.
    ///
    /// Authenticate even retries before calling. Logical bytes can only stay
    /// unchanged or decrease: retains are allowed only while the object is live.
    /// Per-object reference/receipt bounds and `max_objects` bound aggregate metadata.
    /// # Errors
    /// Unknown root or journal admission fails without mutation. Recorded lifecycle
    /// failures remain in the returned outcome and consume their receipt slot.
    pub fn apply_reference(
        &mut self,
        root: ProviderRootHash,
        actor: Principal,
        request: ReferenceRequest,
    ) -> Result<ReferenceRequestOutcome, CatalogError> {
        Ok(self.entry_mut(root)?.requests.apply(actor, request)?)
    }

    /// Apply independently authenticated, exact physical deletion evidence.
    /// # Errors
    /// Unknown root, binding mismatch or live references leave state unchanged.
    pub fn confirm_provider_deleted(
        &mut self,
        root: ProviderRootHash,
        object: ObjectBinding,
    ) -> Result<LifecycleChange, CatalogError> {
        Ok(self
            .entry_mut(root)?
            .requests
            .confirm_provider_deleted(object)?)
    }

    /// Apply independently authenticated final billing cessation evidence.
    /// # Errors
    /// Unknown root, binding mismatch or unconfirmed deletion leaves state unchanged.
    pub fn confirm_billing_stopped(
        &mut self,
        root: ProviderRootHash,
        object: ObjectBinding,
    ) -> Result<LifecycleChange, CatalogError> {
        Ok(self
            .entry_mut(root)?
            .requests
            .confirm_billing_stopped(object)?)
    }

    /// Read an owned journal; callers still need scope and actor authorization.
    #[must_use]
    pub fn get(&self, root: ProviderRootHash) -> Option<&ReferenceRequests> {
        self.entries.get(&root).map(|entry| &entry.requests)
    }

    /// Service whose complete lifetime history this catalog owns.
    #[must_use]
    pub const fn service(&self) -> Principal {
        self.service
    }

    /// Aggregate local usage across all tenants and namespaces.
    #[must_use]
    pub fn usage(&self) -> CatalogUsage {
        usage(self.entries.values())
    }

    /// Local tenant usage across namespaces; this accessor is not authorization.
    #[must_use]
    pub fn tenant_usage(&self, tenant: Principal) -> CatalogUsage {
        usage(
            self.entries
                .values()
                .filter(|entry| entry.original.first.object().tenant() == tenant),
        )
    }

    fn entry_mut(&mut self, root: ProviderRootHash) -> Result<&mut Entry, CatalogError> {
        self.entries.get_mut(&root).ok_or(CatalogError::UnknownRoot)
    }
}

fn check_bytes(
    current: u128,
    additional: u64,
    maximum: NonZeroU128,
    capacity: CatalogCapacity,
) -> Result<(), CatalogError> {
    if current
        .checked_add(u128::from(additional))
        .is_none_or(|total| total > maximum.get())
    {
        return Err(CatalogError::Capacity(capacity));
    }
    Ok(())
}

/// Read-only local counters. Byte totals are not currency or provider evidence.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CatalogUsage {
    /// Lifetime entries, including settled records with retained history.
    pub objects: usize,
    /// Objects with at least one active reference.
    pub live_objects: usize,
    /// Objects awaiting confirmed physical deletion.
    pub pending_deletions: usize,
    /// Objects with unresolved physical or billing obligations, including zero bytes.
    pub unsettled_objects: usize,
    /// Bytes of objects with live references, counted once per object.
    pub logical_bytes: u128,
    /// Bytes not yet physically deleted, including released objects.
    pub physical_bytes: u128,
    /// Bytes not yet financially settled, including physically deleted objects.
    pub liability_bytes: u128,
    /// Active references across the selected objects.
    pub active_references: u128,
    /// Lifetime reference slots, including released identities.
    pub reference_slots: u128,
    /// Retained request results, including rejected transitions.
    pub receipt_slots: u128,
}

fn usage<'a>(entries: impl Iterator<Item = &'a Entry>) -> CatalogUsage {
    let mut result = CatalogUsage::default();
    for entry in entries {
        let lifecycle = entry.requests.lifecycle();
        result.objects += 1;
        result.live_objects += usize::from(lifecycle.phase() == LifecyclePhase::Live);
        result.pending_deletions +=
            usize::from(lifecycle.phase() == LifecyclePhase::DeletionPending);
        result.unsettled_objects += usize::from(lifecycle.has_unsettled_obligations());
        // On supported 32/64-bit targets, at most usize::MAX objects each hold
        // u64 bytes and usize metadata slots; their products fit u128 exactly.
        result.logical_bytes += u128::from(lifecycle.logical_bytes());
        result.physical_bytes += u128::from(lifecycle.physical_bytes());
        result.liability_bytes += u128::from(lifecycle.liability_bytes());
        result.active_references += lifecycle.active_references() as u128;
        result.reference_slots += lifecycle.reference_slots() as u128;
        result.receipt_slots += entry.requests.receipt_count() as u128;
    }
    result
}

/// Exhausted catalog resource; no partial object admission occurs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CatalogCapacity {
    /// Lifetime object/root history capacity.
    Objects,
    /// Per-tenant lifetime object/root history capacity, independent of byte usage.
    TenantObjects,
    /// Global physical byte capacity.
    PhysicalBytes,
    /// Global unresolved billing-byte capacity.
    LiabilityBytes,
    /// Per-tenant logical byte capacity across namespaces.
    TenantLogicalBytes,
}

/// Rejected catalog operation; lifecycle failures inside recorded receipts are separate.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CatalogError {
    /// Root has no confirmed object in this catalog.
    #[error("unknown catalog root")]
    UnknownRoot,
    /// Repeated registration changed length or initial reference.
    #[error("confirmed object registration conflicts with original input")]
    RegistrationConflict,
    /// A configured capacity does not admit a fresh confirmed object.
    #[error("catalog capacity exhausted: {0:?}")]
    Capacity(CatalogCapacity),
    /// Root/service/object history rejects reassignment.
    #[error(transparent)]
    Root(#[from] RootClaimError),
    /// Exact reference request admission failed without recording a result.
    #[error(transparent)]
    Request(#[from] ReferenceRequestError),
    /// A physical or financial confirmation failed.
    #[error(transparent)]
    Lifecycle(#[from] LifecycleError),
}
