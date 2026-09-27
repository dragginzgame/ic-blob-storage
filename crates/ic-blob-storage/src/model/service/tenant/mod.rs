//! Shared enrollment transitions and bounded records for heap and stable stores.

use std::{
    collections::BTreeMap,
    num::{NonZeroU64, NonZeroUsize},
};

use candid::Principal;
use thiserror::Error;

pub(crate) mod record;

/// Passive enrollment observation, also used as the exact update precondition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TenantEnrollmentView {
    /// Local activation generation, never a restore-safe identity allocator.
    pub generation: NonZeroU64,
    /// Whether fresh uploads, exposure and reference retains are permitted.
    pub active: bool,
}

/// Operator instruction with a compare-and-set precondition, not a blind retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TenantUpdate {
    /// Project principal to enroll, suspend or reactivate.
    pub tenant: Principal,
    /// Exact observed state; `None` requires a never-enrolled tenant.
    pub expected: Option<TenantEnrollmentView>,
    /// Desired admission state. Suspension never deletes history or obligations.
    pub active: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct TenantEnrollments {
    limit: NonZeroUsize,
    entries: BTreeMap<Principal, TenantEnrollmentView>,
}

impl TenantEnrollments {
    pub(super) fn new(limit: NonZeroUsize) -> Self {
        Self {
            limit,
            entries: BTreeMap::new(),
        }
    }

    pub(super) fn get(&self, tenant: Principal) -> Option<TenantEnrollmentView> {
        self.entries.get(&tenant).copied()
    }

    pub(super) fn active_generation(&self, tenant: Principal) -> Result<NonZeroU64, TenantError> {
        let entry = self.get(tenant).ok_or(TenantError::NotEnrolled)?;
        if !entry.active {
            return Err(TenantError::Suspended);
        }
        Ok(entry.generation)
    }

    pub(super) fn update(
        &mut self,
        update: TenantUpdate,
    ) -> Result<TenantEnrollmentView, TenantError> {
        let next = next_enrollment(
            update,
            self.get(update.tenant),
            self.entries.len() < self.limit.get(),
        )?;
        self.entries.insert(update.tenant, next);
        Ok(next)
    }
}

// Both heap and stable stores use this transition. Storage mechanics must never
// change suspension, compare-and-set or activation-generation semantics.
pub(crate) fn next_enrollment(
    update: TenantUpdate,
    current: Option<TenantEnrollmentView>,
    has_new_slot: bool,
) -> Result<TenantEnrollmentView, TenantError> {
    if update.tenant == Principal::anonymous() || update.tenant == Principal::management_canister()
    {
        return Err(TenantError::InvalidTenant);
    }
    if current != update.expected {
        return Err(TenantError::Conflict);
    }
    let next = match current {
        None => {
            if !has_new_slot {
                return Err(TenantError::Capacity);
            }
            TenantEnrollmentView {
                generation: NonZeroU64::MIN,
                active: update.active,
            }
        }
        Some(current) => {
            // Suspension must work even at the largest generation. Increment
            // only on reactivation, so old upload permissions stay invalid.
            let generation = if !current.active && update.active {
                current
                    .generation
                    .checked_add(1)
                    .ok_or(TenantError::GenerationExhausted)?
            } else {
                current.generation
            };
            TenantEnrollmentView {
                generation,
                active: update.active,
            }
        }
    };
    Ok(next)
}

/// Enrollment rejection; it never releases a reservation, reference or liability.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum TenantError {
    /// Anonymous and management principals cannot be tenants.
    #[error("invalid tenant principal")]
    InvalidTenant,
    /// Lifetime enrollment capacity is full, including suspended tenants.
    #[error("tenant enrollment capacity exhausted")]
    Capacity,
    /// Update precondition differs from current enrollment. Inspect before retrying.
    #[error("tenant enrollment precondition conflict")]
    Conflict,
    /// Tenant has never been enrolled.
    #[error("tenant is not enrolled")]
    NotEnrolled,
    /// New authority is disabled; inspection, cleanup and reconciliation remain possible.
    #[error("tenant is suspended")]
    Suspended,
    /// A permission predates the current activation.
    #[error("upload permission belongs to an earlier tenant activation")]
    StalePermission,
    /// Reactivation cannot allocate a fresh generation. Suspension remains possible.
    #[error("tenant activation generation exhausted")]
    GenerationExhausted,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_exhaustion_preserves_suspension_and_never_wraps() {
        let tenant = Principal::from_slice(&[4, 1]);
        let mut enrollments = TenantEnrollments::new(NonZeroUsize::MIN);
        let active = TenantEnrollmentView {
            generation: NonZeroU64::MAX,
            active: true,
        };
        // Native boundary setup, not a production restore or reset path.
        enrollments.entries.insert(tenant, active);
        let suspended = enrollments
            .update(TenantUpdate {
                tenant,
                expected: Some(active),
                active: false,
            })
            .unwrap();
        assert_eq!(
            enrollments.active_generation(tenant),
            Err(TenantError::Suspended)
        );
        assert_eq!(
            enrollments.update(TenantUpdate {
                tenant,
                expected: Some(suspended),
                active: true
            }),
            Err(TenantError::GenerationExhausted)
        );
        assert_eq!(enrollments.get(tenant), Some(suspended));
        assert_eq!(
            enrollments.update(TenantUpdate {
                tenant,
                expected: Some(suspended),
                active: false
            }),
            Ok(suspended)
        );
    }
}
