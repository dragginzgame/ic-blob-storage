//! Shared passive installation readback; hosts authenticate actual caller and service.
pub mod recovery;
use crate::ops::service::installation::ServiceInstallation;
use crate::policy::installation::may_inspect;
use ic_blob_storage_contracts::dto::configuration::HostConfigurationView;
use ic_blob_storage_contracts::dto::configuration::HostFailure;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::ic_stable_structures::Memory;

/// Inspect exact retained configuration as the installed operator, including after restore.
/// This neither calls a provider nor clears any owner fence.
/// # Errors
/// Refuses another service or any caller other than the installed operator.
pub fn inspect<M: Memory>(
    installation: &ServiceInstallation<M>,
    context: UploadContext,
) -> Result<HostConfigurationView, HostFailure> {
    let configuration = installation.configuration();
    if !may_inspect(
        configuration.service,
        configuration.operator,
        context.service,
        context.actor,
    ) {
        return Err(HostFailure::Denied);
    }
    Ok(installation.configuration_view())
}
