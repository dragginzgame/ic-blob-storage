//! Scope/authentication and passive conversion of four synchronous local owners.
use crate::{
    dto::operator::{
        LocalFundingStatus, LocalGatewayStatus, LocalReadStatus, LocalServiceStatus,
        LocalStatusFailure, LocalUploadStatus, OperatorScope,
    },
    model::{
        billing::journal::FundingJournalScope, gateway::registry::GatewayScope,
        service::upload::UploadContext,
    },
    ops::service::{
        funding::StableFundingJournal, gateways::StableGatewayRegistry, reads::StableReadSessions,
        stores::ServiceStores, uploads::StableUploads,
    },
};
use ic_memory::ic_stable_structures::Memory;

/// Canonical passive local query; linking exports no endpoint.
pub const LOCAL_STATUS_METHOD: &str = "blob_local_status";

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
pub(crate) fn inspect<M: Memory>(
    stores: OperatorStores<'_, M>,
    context: UploadContext,
    scope: OperatorScope,
) -> Result<LocalServiceStatus, LocalStatusFailure> {
    let config = stores.uploads.callback_configuration();
    let bindings = config.bindings();
    let expected = OperatorScope {
        service: bindings.service,
        namespace: bindings.namespace.get(),
        cashier: config.billing().cashier(),
        payment_account: bindings.payment_account,
    };
    if context.service != bindings.service || scope != expected {
        return Err(LocalStatusFailure::Binding);
    }
    if context.actor != bindings.operator {
        return Err(LocalStatusFailure::Denied);
    }
    if stores.funding.inspection_configuration() != config
        || stores.gateways.inspection_configuration() != config
        || stores.reads.inspection_configuration() != config
    {
        return Err(LocalStatusFailure::Binding);
    }
    // Each owner still applies its own authority/scope and retained-state checks.
    // No incomplete observation is replaced by zero, absence or an empty list.
    let uploads = stores
        .uploads
        .usage()
        .map_err(|_| LocalStatusFailure::Internal)?;
    let funding = stores
        .funding
        .summary(
            context,
            FundingJournalScope {
                service: scope.service,
                namespace: bindings.namespace,
                cashier: scope.cashier,
                account: scope.payment_account,
            },
        )
        .map_err(|_| LocalStatusFailure::Internal)?;
    let gateway_scope = GatewayScope::new(scope.service, bindings.namespace, scope.cashier)
        .map_err(|_| LocalStatusFailure::Binding)?;
    let gateways = stores
        .gateways
        .inspect(context, gateway_scope)
        .map_err(|_| LocalStatusFailure::Internal)?;
    let reads = stores
        .reads
        .inspect(context)
        .map_err(|_| LocalStatusFailure::Internal)?;
    Ok(LocalServiceStatus {
        scope,
        uploads: LocalUploadStatus {
            operations: u64::try_from(uploads.operations)
                .map_err(|_| LocalStatusFailure::Internal)?,
            active_reservations: u64::try_from(uploads.active_reservations)
                .map_err(|_| LocalStatusFailure::Internal)?,
            reserved_bytes: uploads.reserved_bytes,
            logical_bytes: uploads.logical_bytes,
            physical_bytes: uploads.physical_bytes,
            liability_bytes: uploads.liability_bytes,
            fenced: stores.uploads.is_fenced(),
        },
        funding: funding_status(&funding),
        gateways: LocalGatewayStatus {
            members: gateways.principals,
            last_sequence: gateways.sync.last_sequence,
            pending_sequence: gateways.sync.pending_sequence,
            fenced: gateways.fenced,
        },
        reads: LocalReadStatus {
            last_sequence: reads.last_sequence,
            sessions: reads.usage.sessions,
            reserved_bytes: reads.usage.reserved_bytes,
            fenced: reads.fenced,
        },
    })
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
        refunded: funding.allocation.refunded(),
        not_enqueued: funding.allocation.not_enqueued(),
        reserved_or_uncertain: funding.allocation.reserved_or_uncertain(),
        retained_intents: funding.retained_intents,
        intent_capacity: funding.intent_capacity,
        last_operation: funding.last_operation.map(std::num::NonZeroU128::get),
        fenced: funding.fenced,
    }
}
