//! Scope/authentication and passive conversion of four synchronous local owners.
use crate::ops::service::funding::StableFundingJournal;
use crate::ops::service::gateways::StableGatewayRegistry;
use crate::ops::service::reads::StableReadSessions;
use crate::ops::service::stores::ServiceStores;
use crate::ops::service::uploads::StableUploads;
use ic_blob_storage_contracts::dto::operator::LocalFundingStatus;
use ic_memory::Memory;

/// Borrowed owners from one installed service, held only for synchronous inspection.
/// Full configurations are checked before any accounting is returned.
pub struct OperatorStores<'a, M: Memory> {
    /// Upload, reference and continuing-obligation owner.
    pub uploads: &'a StableUploads<M>,
    /// Local funding allocation and exact intent journal.
    pub funding: &'a StableFundingJournal<M>,
    /// Gateway membership and pending sync.
    pub gateways: &'a StableGatewayRegistry<M>,
    /// Read session occupancy.
    pub reads: &'a StableReadSessions<M>,
}
// Copy only the synchronous borrows, never the owners or their authority.
impl<M: Memory> Copy for OperatorStores<'_, M> {}
impl<M: Memory> Clone for OperatorStores<'_, M> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, M: Memory> From<&'a ServiceStores<M>> for OperatorStores<'a, M> {
    fn from(stores: &'a ServiceStores<M>) -> Self {
        Self {
            uploads: &stores.uploads,
            funding: &stores.funding,
            gateways: &stores.gateways,
            reads: &stores.reads,
        }
    }
}

pub(crate) fn funding_status(
    funding: &crate::ops::service::funding::summary::FundingJournalSummary,
) -> LocalFundingStatus {
    LocalFundingStatus {
        cumulative_allocation: funding.allocation.allocated(),
        renewal_ceiling: funding.allocation.renewal_ceiling(),
        available_allocation: funding.allocation.available(),
        attachment_allowance: funding.allocation.transferable(),
        transport_accepted: funding.allocation.accepted(),
        uncredited_accepted: funding.allocation.uncredited(),
        provider_acknowledged: funding.allocation.acknowledged(),
        refunded: funding.allocation.refunded(),
        not_enqueued: funding.allocation.not_enqueued(),
        reserved_or_uncertain: funding.allocation.reserved_or_uncertain(),
        retained_intents: funding.retained_intents,
        intent_capacity: funding.intent_capacity,
        last_operation: funding.last_operation.map(std::num::NonZeroU128::get),
        fenced: funding.fenced,
    }
}
