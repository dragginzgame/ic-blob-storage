//! Local lifecycle of one confirmed object; no persistence or provider effects.
//!
//! This value carries an immutable service, tenant, provider-namespace identity
//! and object incarnation. The workflow authenticates actors and resolves the
//! namespace; knowing an ID or hash never grants authority. Confirmations consume
//! facts already authenticated and correlated to the exact operation elsewhere.

use ic_blob_storage_contracts::binding::ObjectBindingMismatch;
use ic_blob_storage_contracts::binding::ReferenceId;
use ic_blob_storage_contracts::upload::history::LifecyclePhase;
use thiserror::Error;
#[cfg(any(test, feature = "test-support"))]
use {
    ic_blob_storage_contracts::binding::ObjectBinding,
    ic_blob_storage_contracts::binding::ReferenceKey, std::collections::BTreeMap,
    std::num::NonZeroUsize,
};

pub mod requests;
pub mod roots;
pub(crate) mod transition;

/// Logical, physical and liable bytes for one phase; zero bytes do not prove settlement.
pub(crate) const fn contribution(bytes: u64, phase: LifecyclePhase) -> (u64, u64, u64) {
    match phase {
        LifecyclePhase::Live => (bytes, bytes, bytes),
        LifecyclePhase::DeletionPending => (0, bytes, bytes),
        LifecyclePhase::ProviderDeleted => (0, 0, bytes),
        LifecyclePhase::Settled => (0, 0, 0),
    }
}

/// Whether an idempotent operation changed the local value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleChange {
    /// The requested transition occurred.
    Changed,
    /// The exact reference or confirmation was already applied.
    Unchanged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ReferenceState {
    Active,
    Released,
}

// Private, constant-size transition. The owner checks it and publishes it
// synchronously, with no await or intervening lifecycle mutation. It cannot be
// exported, persisted or replayed as authority against another lifecycle.
pub(crate) struct ReferenceMutation {
    pub(crate) reference: ReferenceId,
    pub(crate) state: ReferenceState,
    pub(crate) active_references: usize,
}

/// Bounded reference bookkeeping and release phases for one confirmed object.
///
/// Released reference IDs retain their slots to reject reuse and make duplicate
/// release harmless. Reaching the lifetime slot bound rejects new references;
/// this value never evicts replay evidence. It deliberately has no serialization,
/// stable schema, expiry or restore mechanism. A settled object still retains
/// its IDs; discarding it requires a separate proven retirement boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg(any(test, feature = "test-support"))]
pub struct BlobLifecycle {
    binding: ObjectBinding,
    bytes: u64,
    phase: LifecyclePhase,
    references: BTreeMap<ReferenceId, ReferenceState>,
    active_references: usize,
    reference_limit: NonZeroUsize,
}

#[cfg(any(test, feature = "test-support"))]
impl BlobLifecycle {
    /// Start bookkeeping for trusted retention intent and its first reference.
    ///
    /// The caller must already have admitted object size, tenant/global capacity,
    /// the reference identity and its authority. This constructor does not turn
    /// registration or a certificate request into proof of upload completion.
    #[must_use]
    pub fn from_confirmed_upload(
        bytes: u64,
        first: ReferenceKey,
        reference_limit: NonZeroUsize,
    ) -> Self {
        Self {
            binding: first.object(),
            bytes,
            phase: LifecyclePhase::Live,
            references: BTreeMap::from([(first.reference(), ReferenceState::Active)]),
            active_references: 1,
            reference_limit,
        }
    }

    /// Retain a live object under a fresh reference or replay an active reference.
    ///
    /// # Errors
    /// Rejects mismatched bindings, released identities, exhausted lifetime slots
    /// or new references after deletion queues. Rejection leaves the value unchanged.
    pub fn retain(&mut self, key: ReferenceKey) -> Result<LifecycleChange, LifecycleError> {
        let mutation = self.plan_retain(key)?;
        Ok(self.commit_reference(mutation))
    }

    fn plan_retain(&self, key: ReferenceKey) -> Result<Option<ReferenceMutation>, LifecycleError> {
        self.reference_view()
            .plan(true, self.references.get(&key.reference()).copied(), key)
    }

    fn reference_view(&self) -> transition::ReferenceStateView {
        transition::ReferenceStateView {
            binding: self.binding,
            phase: self.phase,
            slots: self.references.len(),
            active: self.active_references,
            limit: self.reference_limit.get(),
        }
    }

    /// Release exactly one known reference, queuing deletion only after the last.
    ///
    /// # Errors
    /// A mismatched binding or unknown reference rejects without changing state.
    /// Repeated release of a known reference is a no-op in every later phase.
    pub fn release(&mut self, key: ReferenceKey) -> Result<LifecycleChange, LifecycleError> {
        let mutation = self.plan_release(key)?;
        Ok(self.commit_reference(mutation))
    }

    fn plan_release(&self, key: ReferenceKey) -> Result<Option<ReferenceMutation>, LifecycleError> {
        self.reference_view()
            .plan(false, self.references.get(&key.reference()).copied(), key)
    }

    fn commit_reference(&mut self, mutation: Option<ReferenceMutation>) -> LifecycleChange {
        let Some(mutation) = mutation else {
            return LifecycleChange::Unchanged;
        };
        self.references.insert(mutation.reference, mutation.state);
        self.active_references = mutation.active_references;
        if self.active_references == 0 {
            self.phase = LifecyclePhase::DeletionPending;
        }
        LifecycleChange::Changed
    }

    /// Apply independently authenticated deletion evidence for this exact incarnation.
    ///
    /// # Errors
    /// Rejects a mismatched binding or confirmation while live references remain. A provider callback
    /// alone must not invoke this until its authority and operation binding pass.
    pub fn confirm_provider_deleted(
        &mut self,
        object: ObjectBinding,
    ) -> Result<LifecycleChange, LifecycleError> {
        self.binding.check(object)?;
        let (phase, change) = transition::deleted(self.phase)?;
        self.phase = phase;
        Ok(change)
    }

    /// Apply exact evidence of final charge settlement and future billing cessation.
    ///
    /// # Errors
    /// The binding must match and physical deletion must already be confirmed.
    /// An empty balance, elapsed
    /// timeout or absent read response is not evidence for this transition.
    pub fn confirm_billing_stopped(
        &mut self,
        object: ObjectBinding,
    ) -> Result<LifecycleChange, LifecycleError> {
        self.binding.check(object)?;
        let (phase, change) = transition::settled(self.phase)?;
        self.phase = phase;
        Ok(change)
    }

    /// Immutable service, tenant, namespace and object-incarnation binding.
    #[must_use]
    pub const fn binding(&self) -> ObjectBinding {
        self.binding
    }

    /// Current release phase.
    #[must_use]
    pub const fn phase(&self) -> LifecyclePhase {
        self.phase
    }

    /// Number of live references, excluding retained release evidence.
    #[must_use]
    pub const fn active_references(&self) -> usize {
        self.active_references
    }

    /// Read the logical liveness of one fully bound reference.
    ///
    /// Unknown and released references return false even when another reference
    /// keeps the object live. This does not authenticate a reader, verify provider
    /// availability or authorize deletion; physical/billing obligations are separate.
    /// # Errors
    /// Rejects a different service, tenant, namespace, object or incarnation.
    pub fn reference_is_live(&self, key: ReferenceKey) -> Result<bool, ObjectBindingMismatch> {
        self.binding.check(key.object())?;
        Ok(matches!(
            self.references.get(&key.reference()),
            Some(ReferenceState::Active)
        ))
    }

    /// Slots occupied by live references and retained released identities.
    #[must_use]
    pub fn reference_slots(&self) -> usize {
        self.references.len()
    }

    /// Unused lifetime reference identities; releasing references does not refill them.
    #[must_use]
    pub fn remaining_reference_slots(&self) -> usize {
        self.reference_limit.get() - self.references.len()
    }

    /// Logical tenant bytes: counted once for this object while references remain.
    #[must_use]
    pub const fn logical_bytes(&self) -> u64 {
        contribution(self.bytes, self.phase).0
    }

    /// Physical bytes still charged to global capacity until confirmed deletion.
    #[must_use]
    pub const fn physical_bytes(&self) -> u64 {
        contribution(self.bytes, self.phase).1
    }

    /// Whether any physical or financial obligation remains unresolved.
    ///
    /// A zero-length object can still incur request fees or retain obligations.
    /// Neither zero byte counters nor an empty account proves settlement.
    #[must_use]
    pub const fn has_unsettled_obligations(&self) -> bool {
        !matches!(self.phase, LifecyclePhase::Settled)
    }

    /// Bytes with unresolved billing obligations, not a monetary cost estimate.
    ///
    /// Also account for object/liability counts: zero bytes do not prove settlement.
    #[must_use]
    pub const fn liability_bytes(&self) -> u64 {
        contribution(self.bytes, self.phase).2
    }
}

/// Rejected local lifecycle transition; no state changes on rejection.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum LifecycleError {
    /// A reference from another scope cannot mutate this lifecycle.
    #[error(transparent)]
    BindingMismatch(#[from] ObjectBindingMismatch),
    /// Unknown IDs cannot release another reference.
    #[error("reference is unknown")]
    UnknownReference,
    /// A released reference identity must never be reactivated.
    #[error("reference was already released")]
    ReferenceReleased,
    /// Lifetime reference slots, including released IDs, are exhausted.
    #[error("reference limit reached")]
    ReferenceLimitReached,
    /// No new references may race a pending or completed deletion.
    #[error("deletion was already queued")]
    DeletionAlreadyQueued,
    /// A deletion confirmation cannot remove a referenced object.
    #[error("live references remain")]
    LiveReferencesRemain,
    /// Financial completion cannot discard unresolved physical deletion.
    #[error("provider deletion is not confirmed")]
    DeletionNotConfirmed,
}

#[cfg(test)]
mod tests;
