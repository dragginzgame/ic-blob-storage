//! Complete local journal status, never provider-account clearance.
use super::Allocation;
use crate::status::FundingActivityView;
use candid::{CandidType, Deserialize};
use ic_blob_storage_contracts::dto::operator::OperatorScope;

/// Fixed-size operator observation without scanning or paging intent history.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Summary {
    /// Full installed journal scope.
    pub scope: OperatorScope,
    /// Maintained lifetime attachment totals and independent restore fence.
    pub allocation: Allocation,
    /// All retained intent rows, including no-transfer outcomes.
    pub retained_intents: u64,
    /// Installed lifetime capacity.
    pub intent_capacity: u64,
    /// Highest retained ID, without freshness authority.
    pub last_operation: Option<u128>,
    /// Shared policy's local diagnosis, not complete external account activity.
    pub local_activity: FundingActivityView,
}
