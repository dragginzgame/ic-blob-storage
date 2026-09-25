//! Bounded gateway-list validation, independent of storage and provider calls.
//!
//! Membership in this value is not callback authority. A workflow must establish
//! the source's authority, service/provider namespace and freshness before using
//! the list, and separately enforce revocation and recovery fences.

use std::{collections::BTreeSet, num::NonZeroUsize};

use candid::Principal;
use thiserror::Error;

pub mod membership;

/// Explicit processing and membership limits; no deployment defaults are chosen.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GatewayListLimits {
    /// Maximum input length, including duplicate principals.
    pub max_entries: NonZeroUsize,
    /// Maximum number of distinct principals retained after normalization.
    pub max_unique: NonZeroUsize,
}

/// A nonempty validated gateway list in first-occurrence order.
///
/// Anonymous and management principals are rejected. Other principal classes
/// are allowed, matching Canic's validation; this does not prove that a principal
/// is a deployed gateway. Limits apply before any replacement becomes visible.
/// No serialization or stable-state schema is implied by this transient value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GatewayList {
    principals: Vec<Principal>,
    limits: GatewayListLimits,
}

impl GatewayList {
    /// Validate a complete list, deduplicating without changing provider order.
    ///
    /// Raw length is checked before scanning or allocating normalization buffers.
    /// Duplicate detection uses an ordered set, avoiding a quadratic scan of the
    /// growing output. Decoded input size is still bounded by the caller's decoder.
    ///
    /// # Errors
    /// Rejects empty lists, excessive raw/distinct counts, and invalid principals.
    pub fn new(
        principals: &[Principal],
        limits: GatewayListLimits,
    ) -> Result<Self, GatewayListError> {
        if principals.len() > limits.max_entries.get() {
            return Err(GatewayListError::TooManyEntries {
                actual: principals.len(),
                maximum: limits.max_entries.get(),
            });
        }
        if principals.is_empty() {
            return Err(GatewayListError::Empty);
        }

        let mut seen = BTreeSet::new();
        let mut normalized = Vec::new();
        for (index, principal) in principals.iter().copied().enumerate() {
            if !is_gateway_candidate(principal) {
                return Err(GatewayListError::InvalidPrincipal { index, principal });
            }
            if seen.contains(&principal) {
                continue;
            }
            if normalized.len() == limits.max_unique.get() {
                return Err(GatewayListError::TooManyUnique {
                    maximum: limits.max_unique.get(),
                });
            }
            seen.insert(principal);
            normalized.push(principal);
        }

        Ok(Self {
            principals: normalized,
            limits,
        })
    }

    /// Replace the complete list using its existing limits.
    ///
    /// # Errors
    /// Returns the same validation errors as [`Self::new`]. Failure leaves the
    /// previous list and limits intact, including for an invalid final entry.
    pub fn replace(&mut self, principals: &[Principal]) -> Result<(), GatewayListError> {
        let replacement = Self::new(principals, self.limits)?;
        *self = replacement;
        Ok(())
    }

    /// Validated principals in first-occurrence order, without mutable access.
    #[must_use]
    pub fn principals(&self) -> &[Principal] {
        &self.principals
    }

    /// Bounds used for initial validation and every replacement.
    #[must_use]
    pub const fn limits(&self) -> GatewayListLimits {
        self.limits
    }

    /// Whether a principal occurs in this list, without granting it authority.
    #[must_use]
    pub fn contains(&self, principal: Principal) -> bool {
        self.principals.contains(&principal)
    }
}

fn is_gateway_candidate(principal: Principal) -> bool {
    principal != Principal::anonymous() && principal != Principal::management_canister()
}

/// Rejected gateway input. No partial normalized list is exposed on failure.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum GatewayListError {
    /// A synchronization response cannot revoke every gateway via an empty list.
    #[error("gateway list must not be empty")]
    Empty,
    /// The raw list exceeds its processing bound, including duplicates.
    #[error("gateway list has {actual} entries, maximum is {maximum}")]
    TooManyEntries {
        /// Observed input length.
        actual: usize,
        /// Configured raw-entry bound.
        maximum: usize,
    },
    /// A new distinct principal would exceed the membership bound.
    #[error("gateway list exceeds {maximum} distinct principals")]
    TooManyUnique {
        /// Configured distinct-principal bound.
        maximum: usize,
    },
    /// Anonymous and management principals are invalid gateway candidates.
    #[error("invalid gateway principal {principal} at index {index}")]
    InvalidPrincipal {
        /// Position in the original, unnormalized input.
        index: usize,
        /// Rejected principal.
        principal: Principal,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(id: u8) -> Principal {
        Principal::from_slice(&[id, 1])
    }

    fn limits(entries: usize, unique: usize) -> GatewayListLimits {
        GatewayListLimits {
            max_entries: NonZeroUsize::new(entries).expect("positive bound"),
            max_unique: NonZeroUsize::new(unique).expect("positive bound"),
        }
    }

    #[test]
    fn deduplication_preserves_first_occurrence_order_at_both_bounds() {
        let input = [p(3), p(1), p(3), p(2), p(1)];
        let list = GatewayList::new(&input, limits(5, 3)).expect("bounded list");
        assert_eq!(list.principals(), &[p(3), p(1), p(2)]);
        for principal in input {
            assert!(list.contains(principal));
        }
        assert!(!list.contains(p(4)));
        assert_eq!(list.limits(), limits(5, 3));
    }

    #[test]
    fn empty_and_invalid_principals_reject_even_after_valid_prefix() {
        assert_eq!(
            GatewayList::new(&[], limits(4, 4)),
            Err(GatewayListError::Empty)
        );
        for principal in [Principal::anonymous(), Principal::management_canister()] {
            for input in [vec![principal], vec![p(1), p(1), principal]] {
                assert_eq!(
                    GatewayList::new(&input, limits(4, 4)),
                    Err(GatewayListError::InvalidPrincipal {
                        index: input.len() - 1,
                        principal,
                    })
                );
            }
        }
    }

    #[test]
    fn duplicate_flood_and_unique_growth_have_independent_limits() {
        assert_eq!(
            GatewayList::new(&[p(1); 5], limits(4, 2)),
            Err(GatewayListError::TooManyEntries {
                actual: 5,
                maximum: 4,
            })
        );
        assert_eq!(
            GatewayList::new(&[p(1), p(1), p(2), p(3)], limits(4, 2)),
            Err(GatewayListError::TooManyUnique { maximum: 2 })
        );
        let repeated =
            GatewayList::new(&[p(1); 4], limits(4, 1)).expect("duplicates within input bound");
        assert_eq!(repeated.principals(), &[p(1)]);
        // The raw limit rejects before principal inspection.
        assert_eq!(
            GatewayList::new(&[Principal::anonymous(); 5], limits(4, 2)),
            Err(GatewayListError::TooManyEntries {
                actual: 5,
                maximum: 4,
            })
        );
    }

    #[test]
    fn failed_replacement_preserves_membership_order_and_limits_then_recovers() {
        let mut list = GatewayList::new(&[p(2), p(1)], limits(4, 2)).expect("initial list");
        let original = list.clone();
        for (input, error) in [
            (vec![], GatewayListError::Empty),
            (
                vec![p(3); 5],
                GatewayListError::TooManyEntries {
                    actual: 5,
                    maximum: 4,
                },
            ),
            (
                vec![p(3), p(4), p(5)],
                GatewayListError::TooManyUnique { maximum: 2 },
            ),
            (
                vec![p(3), p(4), Principal::anonymous()],
                GatewayListError::InvalidPrincipal {
                    index: 2,
                    principal: Principal::anonymous(),
                },
            ),
        ] {
            assert_eq!(list.replace(&input), Err(error));
            assert_eq!(list, original);
        }
        list.replace(&[p(3), p(1), p(3)])
            .expect("valid replacement");
        assert_eq!(list.principals(), &[p(3), p(1)]);
        assert!(!list.contains(p(2)));
        assert_eq!(list.limits(), original.limits());
        let replaced = list.clone();
        list.replace(&[p(3), p(1), p(3)])
            .expect("repeat replacement");
        assert_eq!(list, replaced);
    }
}
