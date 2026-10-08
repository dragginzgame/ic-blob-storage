//! Pure tenant contracts.
use crate::dto::tenant::TenantFailure;
use crate::dto::tenant::TenantUpdateRequest;
use candid::Principal;
use std::num::NonZeroU64;
use thiserror::Error;

pub mod reply;

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

/// Predict the exact enrollment transition without owning state or caller authority.
/// # Errors
/// Rejects malformed tenant, changed precondition, exhausted generation or capacity.
// Both heap and stable stores use this transition. Storage mechanics must never
// change suspension, compare-and-set or activation-generation semantics.
pub fn next_enrollment(
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

/// Convert passive enrollment fields; the calling boundary verifies scope and authority.
/// # Errors
/// Rejects a zero activation generation in the expected enrollment.
pub fn parse(input: TenantUpdateRequest) -> Result<TenantUpdate, TenantFailure> {
    Ok(TenantUpdate {
        tenant: input.scope.tenant,
        expected: input
            .expected
            .map(|v| {
                Ok(TenantEnrollmentView {
                    generation: NonZeroU64::new(v.generation).ok_or(TenantFailure::Invalid)?,
                    active: v.active,
                })
            })
            .transpose()?,
        active: input.active,
    })
}
