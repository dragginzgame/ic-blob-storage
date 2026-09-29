//! Synchronous local operator inspection, without transport or readiness inference.
use crate::{
    dto::operator::{LocalServiceStatus, LocalStatusFailure, OperatorScope},
    model::service::upload::UploadContext,
    ops::service::operator::{self, OperatorStores},
};
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
