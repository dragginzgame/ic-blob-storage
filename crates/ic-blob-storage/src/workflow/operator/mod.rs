//! Scope/authentication and passive conversion of four synchronous local owners.
use crate::model::billing::journal::FundingJournalScope;
use crate::model::gateway::registry::GatewayScope;
use ic_blob_storage_contracts::dto::operator::LocalGatewayStatus;
use ic_blob_storage_contracts::dto::operator::LocalReadStatus;
use ic_blob_storage_contracts::dto::operator::LocalServiceStatus;
use ic_blob_storage_contracts::dto::operator::LocalStatusFailure;
use ic_blob_storage_contracts::dto::operator::LocalUploadStatus;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::Memory;

use crate::ops::service::operator::{OperatorStores, funding_status};
/// Inspect one explicit service/provider/account scope as its configured operator.
/// The host supplies actual caller/service and holds all owner borrows synchronously.
/// Reads maintained counters and one bounded gateway row, not lifetime history.
/// Provider balances, credit, reconciliation and readiness are not inferred.
/// # Errors
/// Rejects scope/authority, mismatched owner configurations or inconsistent state.
pub fn inspect<M: Memory>(
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
