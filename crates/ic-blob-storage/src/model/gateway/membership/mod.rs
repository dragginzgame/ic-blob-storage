//! Transient membership operations; no endpoint, provider or storage authority.
//!
//! The owning workflow must authenticate operators and sync results, bind the
//! service/provider namespace, reject stale in-flight results after revocation,
//! and persist changes. These operations alone do not authorize callbacks.

use ic_platform::Principal;

use super::GatewayList;
use super::GatewayListError;
use ic_blob_storage_contracts::configuration::limits::GatewayListLimits;

/// Whether an individual add changed membership.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GatewayAddOutcome {
    /// The principal was newly added.
    Added,
    /// The principal was already present; order and count did not change.
    AlreadyPresent,
}

/// Bounded membership that can be empty after explicit removal.
///
/// A provider sync response must still be nonempty and valid. This distinction
/// lets an operator remove the final member without treating an empty or broken
/// provider response as a successful synchronization. Nothing is persisted here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GatewayMembership {
    principals: Vec<Principal>,
    limits: GatewayListLimits,
}

impl GatewayMembership {
    pub(crate) fn from_retained(
        principals: &[Principal],
        limits: GatewayListLimits,
    ) -> Option<Self> {
        if principals.is_empty() {
            return Some(Self::new(limits));
        }
        let list = GatewayList::new(principals, limits).ok()?;
        if list.principals() != principals {
            return None;
        }
        Some(Self {
            principals: list.principals,
            limits,
        })
    }
    /// Start with no members and explicit positive processing/membership limits.
    #[must_use]
    pub const fn new(limits: GatewayListLimits) -> Self {
        Self {
            principals: Vec::new(),
            limits,
        }
    }

    /// Add one validated principal without duplicating or reordering members.
    ///
    /// An existing member remains an idempotent success at capacity. Individual
    /// additions use the distinct-member bound; the raw-entry bound applies to
    /// the size of each supplied input, including provider sync lists.
    ///
    /// # Errors
    /// Rejects invalid principals or a new member beyond the distinct bound.
    /// Failure leaves membership unchanged.
    pub fn add(&mut self, principal: Principal) -> Result<GatewayAddOutcome, GatewayListError> {
        // Reuse the same principal validator used for synchronized lists.
        GatewayList::new(&[principal], self.limits)?;
        if self.contains(principal) {
            return Ok(GatewayAddOutcome::AlreadyPresent);
        }
        if self.principals.len() == self.limits.max_unique.get() {
            return Err(GatewayListError::TooManyUnique {
                maximum: self.limits.max_unique.get(),
            });
        }
        self.principals.push(principal);
        Ok(GatewayAddOutcome::Added)
    }

    /// Remove an individual member, including the last one.
    ///
    /// Returns whether membership changed; unknown or repeated removals are
    /// no-ops. This does not reconcile callbacks or prove provider revocation.
    pub fn remove(&mut self, principal: Principal) -> bool {
        let Some(index) = self
            .principals
            .iter()
            .position(|candidate| *candidate == principal)
        else {
            return false;
        };
        self.principals.remove(index);
        true
    }

    /// Replace membership from a fully validated nonempty sync result.
    ///
    /// Removed members disappear; first-occurrence provider order is retained.
    /// This method does not verify the response's source, age or operation ID.
    ///
    /// # Errors
    /// Rejects invalid/empty/oversized input without changing existing membership.
    pub fn replace_from_sync(&mut self, principals: &[Principal]) -> Result<(), GatewayListError> {
        let replacement = GatewayList::new(principals, self.limits)?;
        self.principals = replacement.principals;
        Ok(())
    }

    /// Current distinct members in insertion or most-recent sync order.
    #[must_use]
    pub fn principals(&self) -> &[Principal] {
        &self.principals
    }

    /// Whether a principal occurs in the local membership value.
    #[must_use]
    pub fn contains(&self, principal: Principal) -> bool {
        self.principals.contains(&principal)
    }

    /// Original limits used for every operation.
    #[must_use]
    pub const fn limits(&self) -> GatewayListLimits {
        self.limits
    }
}

#[cfg(test)]
mod tests;
