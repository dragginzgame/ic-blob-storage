//! Synchronous local operator inspection, without transport or readiness inference.
use crate::ops::service::operator;
use crate::ops::service::operator::OperatorStores;
use ic_blob_storage_contracts::dto::operator::LocalServiceStatus;
use ic_blob_storage_contracts::dto::operator::LocalStatusFailure;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;

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
    operator::inspect(stores, context, scope)
}
