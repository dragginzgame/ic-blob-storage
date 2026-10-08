//! Persisted limits retain their exact observation configuration binding.
use super::{BalanceJournalRecord, ScopeRecord};
use blob_test_protocol::balance::BalanceFailure;
use candid::CandidType;
use ic_blob_storage_contracts::configuration::funding::FundingLimits;
use serde::Deserialize;

// Required wrapper: a missing record cannot default an older schema to no limits.
#[derive(Clone, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct BillingRecord {
    pub configured: Option<LimitsRecord>,
}

#[derive(Clone, Copy, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct LimitsRecord {
    pub scope: ScopeRecord,
    pub revision: u64,
    pub reserve: u128,
    pub minimum: u128,
    pub target: u128,
}

impl BalanceJournalRecord {
    pub fn configure_limits(
        &mut self,
        scope: ScopeRecord,
        revision: u64,
        limits: FundingLimits,
    ) -> Result<(), BalanceFailure> {
        if self.configured != Some(scope) || self.revision != revision {
            return Err(BalanceFailure::Binding);
        }
        self.billing.configured = Some(LimitsRecord {
            scope,
            revision,
            reserve: limits.reserve(),
            minimum: limits.minimum_balance(),
            target: limits.target_balance(),
        });
        Ok(())
    }

    pub fn current_limits(&self) -> Option<FundingLimits> {
        let value = self.billing.configured?;
        if self.configured != Some(value.scope) || self.revision != value.revision {
            return None;
        }
        FundingLimits::new(value.reserve, value.minimum, value.target).ok()
    }

    pub(super) fn billing_valid(&self) -> bool {
        self.billing.configured.is_none_or(|value| {
            let Some(current) = self.configured else {
                return false;
            };
            let binding = value.scope.valid(current.service, current.namespace)
                && value.revision > 0
                && value.revision <= self.revision;
            let coherent = (value.revision != self.revision || value.scope == current)
                && self
                    .attempts
                    .iter()
                    .all(|a| a.revision != value.revision || a.scope == value.scope);
            binding
                && coherent
                && FundingLimits::new(value.reserve, value.minimum, value.target).is_ok()
        })
    }
}
