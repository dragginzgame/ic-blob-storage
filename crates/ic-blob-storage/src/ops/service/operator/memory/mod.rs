//! Authenticate before measuring the host's compact, read-only allocation summary.
use super::{OperatorStores, authorize};
use crate::{
    dto::operator::{LocalStatusFailure, OperatorScope, memory::HostMemoryStatus},
    model::service::upload::UploadContext,
};
use ic_memory::{MemoryAllocationSummary, RuntimeDiagnosticError, ic_stable_structures::Memory};

/// Canonical host-capacity query; linking exports no endpoint.
pub const HOST_MEMORY_STATUS_METHOD: &str = "blob_host_memory_status";

pub(crate) fn inspect<M: Memory>(
    stores: OperatorStores<'_, M>,
    context: UploadContext,
    scope: OperatorScope,
    measure: impl FnOnce() -> Result<MemoryAllocationSummary, RuntimeDiagnosticError>,
) -> Result<HostMemoryStatus, LocalStatusFailure> {
    authorize(stores, context, scope)?;
    let summary = measure().map_err(|_| LocalStatusFailure::Internal)?;
    Ok(HostMemoryStatus {
        scope,
        bucket_size_pages: summary.bucket_size_pages,
        physical_extent_bytes: summary.physical_extent.bytes,
        virtual_extent_bytes: summary.virtual_extent.bytes,
        allocated_bucket_bytes: summary.allocated_bucket_bytes,
        bucket_slack_bytes: summary.bucket_slack_bytes,
        remaining_buckets: summary.remaining_buckets,
        current_binding_bytes: summary.current_binding.allocated_bytes,
        ledger_binding_bytes: summary.ledger_binding.allocated_bytes,
        unknown_binding_bytes: summary.unknown_binding.allocated_bytes,
        unmanaged_bytes: summary.unmanaged_bytes,
        manager_metadata_bytes: summary.manager_metadata_bytes,
    })
}
