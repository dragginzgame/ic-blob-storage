//! Current-instance recovery orchestration; arbitrary backup activation stays fenced.
use crate::{
    model::service::upload::UploadContext,
    ops::service::recovery::{
        CurrentInstanceRecoveryError, RecoveryInstallationAccess, prove_current_instance,
    },
};

/// Authenticate the installed operator, obtain current IC history once, recheck
/// the exact installation after the await and consume the sealed proof locally.
/// All journals, reservations and unresolved effects remain intact. This does not
/// dispatch provider work, settle bills, repair snapshots or authorize any retry.
/// # Errors
/// Denies foreign callers/scopes and every incomplete or noncontinuous history.
pub async fn resume_current_instance(
    host: &impl RecoveryInstallationAccess,
    context: UploadContext,
) -> Result<(), CurrentInstanceRecoveryError> {
    use CurrentInstanceRecoveryError as Error;
    let anchor = host.with_installation(|installation| {
        let input = installation.configuration();
        if input.service != context.service {
            return Err(Error::Binding);
        }
        if input.operator != context.actor {
            return Err(Error::Denied);
        }
        Ok(installation.platform_installation_version())
    })?;
    let proof = prove_current_instance(anchor).await?;
    host.with_installation(|installation| {
        let input = installation.configuration();
        if input.service != context.service
            || installation.platform_installation_version() != anchor
        {
            return Err(Error::Binding);
        }
        if input.operator != context.actor {
            return Err(Error::Denied);
        }
        installation.resume_current_instance(proof)
    })
}
