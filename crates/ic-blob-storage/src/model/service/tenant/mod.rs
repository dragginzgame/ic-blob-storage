//! Shared enrollment transitions and bounded records for heap and stable stores.

use candid::Principal;
use ic_blob_storage_contracts::tenant::TenantEnrollmentView;
use ic_blob_storage_contracts::tenant::TenantError;
use ic_blob_storage_contracts::tenant::TenantUpdate;
use ic_blob_storage_contracts::tenant::next_enrollment;
use std::{
    collections::BTreeMap,
    num::{NonZeroU64, NonZeroUsize},
};

pub(crate) mod record;

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
