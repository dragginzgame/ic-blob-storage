//! Local lifecycle of one confirmed object; no persistence or provider effects.
//!
//! This value carries an immutable service, tenant, provider-namespace identity
//! and object incarnation. The workflow authenticates actors and resolves the
//! namespace; knowing an ID or hash never grants authority. Confirmations consume
//! facts already authenticated and correlated to the exact operation elsewhere.

pub mod binding;
pub mod requests;
pub mod roots;

use std::{
    collections::BTreeMap,
    num::{NonZeroU128, NonZeroUsize},
};

use thiserror::Error;

use self::binding::{ObjectBinding, ObjectBindingMismatch, ReferenceKey};

/// Reference identity within one bound object incarnation, not an authorization token.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ReferenceId(NonZeroU128);

impl ReferenceId {
    /// Wrap an independently allocated identity; allocation/reuse safety is external.
    #[must_use]
    pub const fn new(value: NonZeroU128) -> Self {
        Self(value)
    }
}

/// Distinct logical, physical and economic stages of a confirmed object's release.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecyclePhase {
    /// At least one live reference remains.
    Live,
    /// No live references remain; physical storage and billing are unresolved.
    DeletionPending,
    /// Physical deletion is confirmed; financial obligations remain unresolved.
    ProviderDeleted,
    /// Deletion and final billing cessation are both confirmed.
    Settled,
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
enum ReferenceState {
    Active,
    Released,
}

/// Bounded reference bookkeeping and release phases for one confirmed object.
///
/// Released reference IDs retain their slots to reject reuse and make duplicate
/// release harmless. Reaching the lifetime slot bound rejects new references;
/// this value never evicts replay evidence. It deliberately has no serialization,
/// stable schema, expiry or restore mechanism. A settled object still retains
/// its IDs; discarding it requires a separate proven retirement boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlobLifecycle {
    binding: ObjectBinding,
    bytes: u64,
    phase: LifecyclePhase,
    references: BTreeMap<ReferenceId, ReferenceState>,
    active_references: usize,
    reference_limit: NonZeroUsize,
}

impl BlobLifecycle {
    /// Start bookkeeping after independently verified upload completion.
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
        self.binding.check(key.object())?;
        let reference = key.reference();
        match self.references.get(&reference) {
            Some(ReferenceState::Active) => return Ok(LifecycleChange::Unchanged),
            Some(ReferenceState::Released) => return Err(LifecycleError::ReferenceReleased),
            None => {}
        }
        if self.phase != LifecyclePhase::Live {
            return Err(LifecycleError::DeletionAlreadyQueued);
        }
        if self.references.len() >= self.reference_limit.get() {
            return Err(LifecycleError::ReferenceLimitReached);
        }
        self.references.insert(reference, ReferenceState::Active);
        self.active_references += 1;
        Ok(LifecycleChange::Changed)
    }

    /// Release exactly one known reference, queuing deletion only after the last.
    ///
    /// # Errors
    /// A mismatched binding or unknown reference rejects without changing state.
    /// Repeated release of a known reference is a no-op in every later phase.
    pub fn release(&mut self, key: ReferenceKey) -> Result<LifecycleChange, LifecycleError> {
        self.binding.check(key.object())?;
        let reference = key.reference();
        let state = self
            .references
            .get_mut(&reference)
            .ok_or(LifecycleError::UnknownReference)?;
        if *state == ReferenceState::Released {
            return Ok(LifecycleChange::Unchanged);
        }
        *state = ReferenceState::Released;
        self.active_references -= 1;
        if self.active_references == 0 {
            self.phase = LifecyclePhase::DeletionPending;
        }
        Ok(LifecycleChange::Changed)
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
        match self.phase {
            LifecyclePhase::Live => Err(LifecycleError::LiveReferencesRemain),
            LifecyclePhase::DeletionPending => {
                self.phase = LifecyclePhase::ProviderDeleted;
                Ok(LifecycleChange::Changed)
            }
            LifecyclePhase::ProviderDeleted | LifecyclePhase::Settled => {
                Ok(LifecycleChange::Unchanged)
            }
        }
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
        match self.phase {
            LifecyclePhase::Live | LifecyclePhase::DeletionPending => {
                Err(LifecycleError::DeletionNotConfirmed)
            }
            LifecyclePhase::ProviderDeleted => {
                self.phase = LifecyclePhase::Settled;
                Ok(LifecycleChange::Changed)
            }
            LifecyclePhase::Settled => Ok(LifecycleChange::Unchanged),
        }
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

    /// Logical tenant bytes: counted once for this object while references remain.
    #[must_use]
    pub const fn logical_bytes(&self) -> u64 {
        if matches!(self.phase, LifecyclePhase::Live) {
            self.bytes
        } else {
            0
        }
    }

    /// Physical bytes still charged to global capacity until confirmed deletion.
    #[must_use]
    pub const fn physical_bytes(&self) -> u64 {
        match self.phase {
            LifecyclePhase::Live | LifecyclePhase::DeletionPending => self.bytes,
            LifecyclePhase::ProviderDeleted | LifecyclePhase::Settled => 0,
        }
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
        if matches!(self.phase, LifecyclePhase::Settled) {
            0
        } else {
            self.bytes
        }
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
mod tests {
    use super::*;

    fn key(value: u128) -> ReferenceKey {
        use binding::ObjectIdentity;
        use candid::Principal;
        let one = NonZeroU128::new(1).expect("positive identity");
        let object = ObjectBinding::new(
            Principal::from_slice(&[1, 1]),
            Principal::from_slice(&[2, 1]),
            ObjectIdentity {
                namespace: one,
                object: one,
                incarnation: one,
            },
        )
        .expect("valid binding");
        ReferenceKey::new(
            object,
            ReferenceId::new(NonZeroU128::new(value).expect("positive ID")),
        )
    }
    fn object() -> BlobLifecycle {
        BlobLifecycle::from_confirmed_upload(
            100,
            key(1),
            NonZeroUsize::new(2).expect("positive bound"),
        )
    }

    #[test]
    fn release_deletion_and_billing_discharge_separate_obligations() {
        let mut blob = object();
        assert_eq!(blob.retain(key(2)), Ok(LifecycleChange::Changed));
        assert_eq!(blob.release(key(1)), Ok(LifecycleChange::Changed));
        assert_eq!(blob.active_references(), 1);
        assert_eq!(
            (
                blob.logical_bytes(),
                blob.physical_bytes(),
                blob.liability_bytes()
            ),
            (100, 100, 100)
        );
        assert_eq!(blob.release(key(2)), Ok(LifecycleChange::Changed));
        assert_eq!(blob.phase(), LifecyclePhase::DeletionPending);
        assert_eq!(blob.active_references(), 0);
        assert_eq!(
            (
                blob.logical_bytes(),
                blob.physical_bytes(),
                blob.liability_bytes()
            ),
            (0, 100, 100)
        );
        assert_eq!(
            blob.confirm_provider_deleted(blob.binding()),
            Ok(LifecycleChange::Changed)
        );
        assert_eq!(blob.phase(), LifecyclePhase::ProviderDeleted);
        assert_eq!(
            (
                blob.logical_bytes(),
                blob.physical_bytes(),
                blob.liability_bytes()
            ),
            (0, 0, 100)
        );
        assert_eq!(
            blob.confirm_billing_stopped(blob.binding()),
            Ok(LifecycleChange::Changed)
        );
        assert_eq!(blob.phase(), LifecyclePhase::Settled);
        assert_eq!(
            (
                blob.logical_bytes(),
                blob.physical_bytes(),
                blob.liability_bytes()
            ),
            (0, 0, 0)
        );
        assert_eq!(blob.reference_slots(), 2);
    }

    #[test]
    fn replayed_and_unknown_releases_do_not_delete_another_reference() {
        let mut blob = object();
        blob.retain(key(2)).expect("second reference");
        blob.release(key(1)).expect("first release");
        let before = blob.clone();
        assert_eq!(blob.release(key(1)), Ok(LifecycleChange::Unchanged));
        assert_eq!(blob.release(key(3)), Err(LifecycleError::UnknownReference));
        assert_eq!(
            blob.confirm_provider_deleted(blob.binding()),
            Err(LifecycleError::LiveReferencesRemain)
        );
        assert_eq!(
            blob.confirm_billing_stopped(blob.binding()),
            Err(LifecycleError::DeletionNotConfirmed)
        );
        assert_eq!(blob, before);
    }

    #[test]
    fn reference_slots_include_tombstones_and_rejections_preserve_state() {
        let mut blob = object();
        blob.retain(key(2)).expect("second reference");
        blob.release(key(1)).expect("retain release evidence");
        let before = blob.clone();
        assert_eq!(blob.retain(key(2)), Ok(LifecycleChange::Unchanged));
        assert_eq!(blob.retain(key(1)), Err(LifecycleError::ReferenceReleased));
        assert_eq!(
            blob.retain(key(3)),
            Err(LifecycleError::ReferenceLimitReached)
        );
        assert_eq!(blob, before);
    }

    #[test]
    fn zero_bytes_and_maximum_length_both_require_explicit_settlement() {
        for bytes in [0, u64::MAX] {
            let mut blob = BlobLifecycle::from_confirmed_upload(
                bytes,
                key(1),
                NonZeroUsize::new(1).expect("positive bound"),
            );
            assert!(blob.has_unsettled_obligations());
            blob.release(key(1)).expect("last release");
            assert_eq!(blob.physical_bytes(), bytes);
            assert!(blob.has_unsettled_obligations());
            blob.confirm_provider_deleted(blob.binding())
                .expect("deletion evidence");
            assert_eq!(blob.physical_bytes(), 0);
            assert_eq!(blob.liability_bytes(), bytes);
            assert!(blob.has_unsettled_obligations());
            blob.confirm_billing_stopped(blob.binding())
                .expect("billing evidence");
            assert!(!blob.has_unsettled_obligations());
            assert_eq!(blob.liability_bytes(), 0);
        }
    }

    #[test]
    fn queued_deletion_cannot_be_cancelled_by_retain_and_repeated_confirmation_is_harmless() {
        let mut blob = object();
        blob.release(key(1)).expect("last release");
        let pending = blob.clone();
        assert_eq!(
            blob.retain(key(2)),
            Err(LifecycleError::DeletionAlreadyQueued)
        );
        assert_eq!(
            blob.confirm_billing_stopped(blob.binding()),
            Err(LifecycleError::DeletionNotConfirmed)
        );
        assert_eq!(blob, pending);
        blob.confirm_provider_deleted(blob.binding())
            .expect("deletion evidence");
        for settled in [false, true] {
            if settled {
                blob.confirm_billing_stopped(blob.binding())
                    .expect("billing evidence");
            }
            let before = blob.clone();
            assert_eq!(blob.release(key(1)), Ok(LifecycleChange::Unchanged));
            assert_eq!(
                blob.confirm_provider_deleted(blob.binding()),
                Ok(LifecycleChange::Unchanged)
            );
            assert_eq!(blob.retain(key(1)), Err(LifecycleError::ReferenceReleased));
            assert_eq!(
                blob.retain(key(2)),
                Err(LifecycleError::DeletionAlreadyQueued)
            );
            if settled {
                assert_eq!(
                    blob.confirm_billing_stopped(blob.binding()),
                    Ok(LifecycleChange::Unchanged)
                );
            }
            assert_eq!(blob, before);
        }
    }
}
