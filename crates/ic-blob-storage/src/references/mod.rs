//! Bounded application references for one registered object.
//!
//! The application supplies immutable reference IDs. Successful retain and release
//! calls are idempotent; released IDs remain occupied for this owner's lifetime.
//! Final release closes the owner immediately and cannot be undone.
//!
//! Call only after application authorization. The integrating catalog must bind
//! each reference to its object and resolve each provider object to a single owner.
//! This component does not authenticate callers, serialize state, count provider
//! bytes, issue upload authority or authorize provider deletion. Closing means
//! local retention ended, not that provider obligations have settled.
//!
//! Rejected calls leave the owner unchanged and do not record a request receipt.
//! A future service protocol must separately retain any admitted failure outcomes
//! it promises to replay. Reference IDs are not provider keys or operation IDs.

use std::collections::BTreeMap;
use std::num::{NonZeroU128, NonZeroUsize};

/// An application-assigned, nonzero reference name within its object binding.
///
/// This local representation does not define a wire or persistence format.
pub type ReferenceId = NonZeroU128;

/// Current local lifetime of an admitted reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceState {
    /// The application still requests retention.
    Live,
    /// The reference was released and its identity cannot be reused.
    Released,
}

/// Local object retention, independent of provider availability or settlement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetentionState {
    /// At least one reference remains live.
    Open,
    /// The last reference was released; no new references may be acquired.
    Closing,
}

/// A refused reference mutation. Refusals never change the owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceError {
    /// New references cannot be acquired after final release.
    Closing,
    /// All lifetime reference slots are occupied, including released identities.
    CapacityExhausted,
    /// A release named a reference that was never admitted to this owner.
    UnknownReference,
}

/// One bounded owner of a registered object's application references.
///
/// Creation includes the application's predetermined first reference. Integration
/// must commit this creation with registration before exposing upload authority.
/// A release updates an existing slot: exhaustion cannot consume cleanup capacity.
/// There is no eviction, reset, cloning or reopening operation.
#[derive(Debug)]
pub struct ObjectReferences {
    references: BTreeMap<ReferenceId, ReferenceState>,
    live: usize,
    limit: NonZeroUsize,
}

impl ObjectReferences {
    /// Create an Open owner with its first live reference and a lifetime slot limit.
    ///
    /// The limit includes the first reference and all released identities. It
    /// bounds record count, not host memory bytes or provider storage liability.
    #[must_use]
    pub fn new(first: ReferenceId, limit: NonZeroUsize) -> Self {
        Self {
            references: BTreeMap::from([(first, ReferenceState::Live)]),
            live: 1,
            limit,
        }
    }

    /// Retain a named application use, or replay its original successful retain.
    ///
    /// A known ID returns success even after release or closure, without making it
    /// live again. Inspect [`Self::reference_state`] for current liveness.
    ///
    /// # Errors
    /// Returns [`ReferenceError::Closing`] for a new ID after final release, or
    /// [`ReferenceError::CapacityExhausted`] when no lifetime slots remain.
    pub fn retain(&mut self, id: ReferenceId) -> Result<(), ReferenceError> {
        if self.references.contains_key(&id) {
            return Ok(());
        }
        if self.live == 0 {
            return Err(ReferenceError::Closing);
        }
        if self.references.len() >= self.limit.get() {
            return Err(ReferenceError::CapacityExhausted);
        }
        self.references.insert(id, ReferenceState::Live);
        self.live += 1;
        Ok(())
    }

    /// Release a reference, closing the owner immediately on the final release.
    ///
    /// Repeated release succeeds without changing counts. No new slot or receipt
    /// allocation is needed, so admitted references remain releasable at capacity.
    ///
    /// # Errors
    /// Returns [`ReferenceError::UnknownReference`] for an unadmitted ID.
    pub fn release(&mut self, id: ReferenceId) -> Result<(), ReferenceError> {
        let state = self
            .references
            .get_mut(&id)
            .ok_or(ReferenceError::UnknownReference)?;
        if *state == ReferenceState::Live {
            *state = ReferenceState::Released;
            self.live -= 1;
        }
        Ok(())
    }

    /// Inspect local retention. Closing makes no provider deletion claim.
    #[must_use]
    pub const fn retention_state(&self) -> RetentionState {
        if self.live == 0 {
            RetentionState::Closing
        } else {
            RetentionState::Open
        }
    }

    /// Inspect the current state of one ID, or `None` if it was never admitted.
    #[must_use]
    pub fn reference_state(&self, id: ReferenceId) -> Option<ReferenceState> {
        self.references.get(&id).copied()
    }

    /// Number of application uses still requesting retention.
    #[must_use]
    pub const fn live_count(&self) -> usize {
        self.live
    }

    /// Lifetime slots occupied by live and released references together.
    #[must_use]
    pub fn occupied_slots(&self) -> usize {
        self.references.len()
    }
}

#[cfg(test)]
mod tests;
