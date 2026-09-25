//! Transient membership operations; no endpoint, provider or storage authority.
//!
//! The owning workflow must authenticate operators and sync results, bind the
//! service/provider namespace, reject stale in-flight results after revocation,
//! and persist changes. These operations alone do not authorize callbacks.

use candid::Principal;

use super::{GatewayList, GatewayListError, GatewayListLimits};

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
mod tests {
    use std::num::NonZeroUsize;

    use super::*;

    fn p(id: u8) -> Principal {
        Principal::from_slice(&[id, 1])
    }

    fn membership() -> GatewayMembership {
        GatewayMembership::new(GatewayListLimits {
            max_entries: NonZeroUsize::new(4).expect("positive limit"),
            max_unique: NonZeroUsize::new(2).expect("positive limit"),
        })
    }

    #[test]
    fn repeated_add_at_capacity_is_idempotent_and_rejection_preserves_state() {
        let mut members = membership();
        assert!(members.principals().is_empty());
        assert_eq!(members.add(p(2)), Ok(GatewayAddOutcome::Added));
        assert_eq!(members.add(p(1)), Ok(GatewayAddOutcome::Added));
        let full = members.clone();
        assert_eq!(members.add(p(2)), Ok(GatewayAddOutcome::AlreadyPresent));
        assert_eq!(members, full);
        assert_eq!(
            members.add(p(3)),
            Err(GatewayListError::TooManyUnique { maximum: 2 })
        );
        assert_eq!(members, full);
        for principal in [Principal::anonymous(), Principal::management_canister()] {
            assert_eq!(
                members.add(principal),
                Err(GatewayListError::InvalidPrincipal {
                    index: 0,
                    principal
                })
            );
            assert_eq!(members, full);
        }
    }

    #[test]
    fn removals_release_capacity_and_can_remove_the_final_member() {
        let mut members = membership();
        members.add(p(1)).expect("first member");
        members.add(p(2)).expect("second member");
        assert!(!members.remove(p(3)));
        assert!(members.remove(p(1)));
        assert!(!members.remove(p(1)));
        assert!(!members.contains(p(1)));
        assert_eq!(members.add(p(3)), Ok(GatewayAddOutcome::Added));
        assert_eq!(members.principals(), &[p(2), p(3)]);
        assert!(members.remove(p(2)));
        assert!(members.remove(p(3)));
        assert!(members.principals().is_empty());
        assert_eq!(members.replace_from_sync(&[]), Err(GatewayListError::Empty));
        members.add(p(1)).expect("explicit re-add after removal");
        assert_eq!(members.principals(), &[p(1)]);
    }

    #[test]
    fn complete_sync_replaces_members_and_failed_sync_leaves_them_intact() {
        let mut members = membership();
        members.add(p(1)).expect("first member");
        members.add(p(2)).expect("second member");
        members
            .replace_from_sync(&[p(3), p(2), p(3)])
            .expect("valid sync input");
        assert_eq!(members.principals(), &[p(3), p(2)]);
        assert!(!members.contains(p(1)));
        let synced = members.clone();
        for (input, error) in [
            (vec![], GatewayListError::Empty),
            (
                vec![p(1); 5],
                GatewayListError::TooManyEntries {
                    actual: 5,
                    maximum: 4,
                },
            ),
            (
                vec![p(1), p(2), p(3)],
                GatewayListError::TooManyUnique { maximum: 2 },
            ),
            (
                vec![p(1), Principal::anonymous()],
                GatewayListError::InvalidPrincipal {
                    index: 1,
                    principal: Principal::anonymous(),
                },
            ),
        ] {
            assert_eq!(members.replace_from_sync(&input), Err(error));
            assert_eq!(members, synced);
        }
        members
            .replace_from_sync(&[p(3), p(2), p(3)])
            .expect("repeat sync input");
        assert_eq!(members, synced);
        members
            .replace_from_sync(&[p(1)])
            .expect("successful subsequent sync input");
        assert_eq!(members.principals(), &[p(1)]);
        assert_eq!(members.limits(), synced.limits());
    }
}
