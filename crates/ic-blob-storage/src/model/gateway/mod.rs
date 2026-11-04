//! Bounded gateway-list validation, independent of storage and provider calls.
//!
//! Membership in this value is not callback authority. A workflow must establish
//! the source's authority, service/provider namespace and freshness before using
//! the list, and separately enforce revocation and recovery fences.

use ic_blob_storage_contracts::configuration::limits::GatewayListLimits;
use std::collections::BTreeSet;

use ic_platform::Principal;
use thiserror::Error;

pub mod membership;
pub mod registry;

/// A nonempty validated gateway list in first-occurrence order.
///
/// Anonymous and management principals are rejected. Other principal classes
/// are allowed; this does not prove that a principal
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
mod tests;
