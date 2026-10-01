//! Synchronous local operator inspection, without transport or readiness inference.
use crate::{
    dto::operator::{LocalServiceStatus, LocalStatusFailure, OperatorScope},
    model::service::upload::UploadContext,
    ops::service::operator::{self, OperatorStores},
};
use ic_memory::ic_stable_structures::Memory;

/// Inspect host backing capacity only after service/operator scope authentication.
/// The host supplies its sole runtime's compact summary synchronously. This is a
/// passive observation available while fenced; it grants no mutation authority.
/// # Errors
/// Rejects incorrect authority/bindings or failed runtime measurement.
pub fn memory_status<M: Memory>(
    stores: OperatorStores<'_, M>,
    context: UploadContext,
    scope: OperatorScope,
    measure: impl FnOnce() -> Result<
        ic_memory::MemoryAllocationSummary,
        ic_memory::RuntimeDiagnosticError,
    >,
) -> Result<crate::dto::operator::memory::HostMemoryStatus, LocalStatusFailure> {
    operator::memory::inspect(stores, context, scope, measure)
}

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
