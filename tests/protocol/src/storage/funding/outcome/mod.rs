//! Retained local observations, never provider credit or retry authorization.
use super::{Intent, Phase};
use crate::funding::{FundingOutcome, FundingReconciliationView};
use candid::{CandidType, Deserialize};

/// Bounded operator query result from the durable journal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Outcome {
    /// Complete original identity.
    pub intent: Intent,
    /// Current retained phase.
    pub phase: Phase,
    /// Response classification, absent when no structured result was retained.
    pub response: Option<FundingOutcome>,
    /// Independently reported total/prepaid/promotional/ledger amounts, not credit.
    pub reported_balance: Option<[u128; 4]>,
    /// Conservative shared reconciliation diagnosis.
    pub reconciliation: FundingReconciliationView,
}
