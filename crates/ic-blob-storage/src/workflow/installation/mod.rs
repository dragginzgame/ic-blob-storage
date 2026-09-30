//! Shared passive installation readback; hosts authenticate actual caller and service.
use crate::{
    dto::configuration::{HostConfigurationView, HostFailure},
    model::service::upload::UploadContext,
    ops::service::installation::ServiceInstallation,
    policy::installation::may_inspect,
};
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
