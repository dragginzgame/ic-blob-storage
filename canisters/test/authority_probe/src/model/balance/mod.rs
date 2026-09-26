//! Bounded local observation history; no payment or recovery authority.
pub(crate) mod billing;

use blob_test_protocol::balance::{BalanceFailure as Failure, BalanceUsability};
use candid::{CandidType, Principal};
use serde::Deserialize;

pub(crate) const MAX_AGE_NS: u64 = 30_000_000_000;
const MAX_ATTEMPTS: usize = 16;

#[derive(Clone, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct BalanceJournalRecord {
    pub configured: Option<ScopeRecord>,
    pub revision: u64,
    pub billing: billing::BillingRecord,
    pub attempts: Vec<AttemptRecord>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct ScopeRecord {
    pub service: Principal,
    pub namespace: u128,
    pub source: Principal,
    pub account: Principal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct AttemptRecord {
    pub revision: u64,
    pub scope: ScopeRecord,
    pub started_at: u64,
    pub outcome: Option<OutcomeRecord>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) enum OutcomeRecord {
    Reported {
        amounts: [u128; 4],
        received_at: u64,
    },
    Failed(Failure),
}

impl ScopeRecord {
    pub fn new(
        service: Principal,
        namespace: u128,
        source: Principal,
        account: Principal,
    ) -> Result<Self, Failure> {
        let value = Self {
            service,
            namespace,
            source,
            account,
        };
        value
            .valid(service, namespace)
            .then_some(value)
            .ok_or(Failure::Binding)
    }

    fn valid(self, service: Principal, namespace: u128) -> bool {
        let ordinary = |p| p != Principal::anonymous() && p != Principal::management_canister();
        self.service == service
            && self.namespace == namespace
            && namespace != 0
            && ordinary(service)
            && ordinary(self.source)
            && ordinary(self.account)
            && self.source != service
    }
}

impl BalanceJournalRecord {
    pub fn new() -> Self {
        Self {
            configured: None,
            revision: 0,
            billing: billing::BillingRecord { configured: None },
            attempts: vec![],
        }
    }

    pub fn configure(&mut self, scope: ScopeRecord) -> Result<(), Failure> {
        self.revision = self.revision.checked_add(1).ok_or(Failure::Limit)?;
        self.configured = Some(scope);
        Ok(())
    }

    pub fn busy(&self) -> bool {
        self.attempts.last().is_some_and(|a| a.outcome.is_none())
    }

    pub fn begin(&mut self, now: u64) -> Result<(usize, ScopeRecord), Failure> {
        let scope = self.configured.ok_or(Failure::NotConfigured)?;
        if self.busy() {
            return Err(Failure::Busy);
        }
        if self.attempts.len() == MAX_ATTEMPTS {
            return Err(Failure::Limit);
        }
        let id = self.attempts.len();
        self.attempts.push(AttemptRecord {
            revision: self.revision,
            scope,
            started_at: now,
            outcome: None,
        });
        Ok((id, scope))
    }

    pub fn complete(
        &mut self,
        id: usize,
        scope: ScopeRecord,
        result: Result<[u128; 4], Failure>,
        now: u64,
    ) -> Result<(), Failure> {
        let attempt = self.attempts.get_mut(id).ok_or(Failure::Stale)?;
        if attempt.outcome.is_some() || attempt.scope != scope {
            return Err(Failure::Stale);
        }
        let result = if attempt.revision == self.revision && self.configured == Some(attempt.scope)
        {
            result
        } else {
            Err(Failure::Stale)
        };
        attempt.outcome = Some(match result {
            Ok(amounts) => OutcomeRecord::Reported {
                amounts,
                received_at: now,
            },
            Err(error) => OutcomeRecord::Failed(error),
        });
        result.map(|_| ())
    }

    pub fn usability(&self, fenced: bool, now: u64) -> BalanceUsability {
        if fenced {
            return BalanceUsability::Fenced;
        }
        let Some(attempt) = self.attempts.last() else {
            return BalanceUsability::Unobserved;
        };
        if attempt.revision != self.revision {
            return BalanceUsability::Invalidated;
        }
        match attempt.outcome {
            None => BalanceUsability::Pending,
            Some(OutcomeRecord::Failed(_)) => BalanceUsability::Failed,
            Some(OutcomeRecord::Reported { received_at, .. }) => {
                // Age starts before dispatch, so a slow reply cannot freshen an old read.
                if now >= received_at
                    && now
                        .checked_sub(attempt.started_at)
                        .is_some_and(|age| age <= MAX_AGE_NS)
                {
                    BalanceUsability::Observed
                } else {
                    BalanceUsability::Expired
                }
            }
        }
    }

    pub fn total(&self, fenced: bool, now: u64) -> Option<u128> {
        if self.usability(fenced, now) != BalanceUsability::Observed {
            return None;
        }
        match self.attempts.last()?.outcome? {
            OutcomeRecord::Reported { amounts, .. } => Some(amounts[0]),
            OutcomeRecord::Failed(_) => None,
        }
    }

    pub fn valid(&self, service: Principal, namespace: u128) -> bool {
        let Some(scope) = self.configured else {
            return self.revision == 0
                && self.attempts.is_empty()
                && self.billing.configured.is_none();
        };
        self.revision > 0
            && self.billing_valid()
            && scope.valid(service, namespace)
            && self.attempts.len() <= MAX_ATTEMPTS
            && self.attempts.iter().enumerate().all(|(id, attempt)| {
                attempt.scope.valid(service, namespace)
                    && attempt.revision > 0
                    && attempt.revision <= self.revision
                    && (attempt.revision != self.revision || attempt.scope == scope)
                    && match attempt.outcome {
                        None => id + 1 == self.attempts.len(),
                        Some(OutcomeRecord::Reported { received_at, .. }) => {
                            received_at >= attempt.started_at
                        }
                        Some(OutcomeRecord::Failed(Failure::Stale)) => {
                            attempt.revision < self.revision
                        }
                        Some(OutcomeRecord::Failed(error)) => matches!(
                            error,
                            Failure::Transport
                                | Failure::Oversized
                                | Failure::Malformed
                                | Failure::AccountMismatch
                                | Failure::AccountNotFound
                                | Failure::ProviderInternal
                        ),
                    }
            })
            && self.attempts.windows(2).all(|pair| {
                pair[0].revision < pair[1].revision
                    || (pair[0].revision == pair[1].revision && pair[0].scope == pair[1].scope)
            })
    }
}

#[cfg(test)]
mod tests;
