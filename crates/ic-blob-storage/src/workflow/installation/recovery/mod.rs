//! Current-instance recovery orchestration; arbitrary backup activation stays fenced.
use crate::ops::service::recovery::ContinuityInstallationAccess;
use crate::ops::service::recovery::CurrentInstanceRecoveryError;
use crate::ops::service::recovery::RecoveryInstallationAccess;
use crate::ops::service::recovery::prove_observed_current_instance;
use ic_blob_storage_contracts::upload::binding::UploadContext;

/// Check a previously active owner's continuity once before an update. The host
/// releases its installation borrow before history acquisition. Any refusal
/// retains the fence; the endpoint then returns its own typed fenced outcome.
/// This never automatically resumes an already-fenced restoration or retries work.
pub async fn before_update(host: &impl ContinuityInstallationAccess) {
    if let Some(request) = host.prepare_active_continuity() {
        if let Ok(proof) = prove_observed_instance(host, request.installation_version).await {
            let _ =
                host.with_installation(|installation| installation.resume_current_instance(proof));
        }
        host.observe_version();
    }
}

/// Explicit host recovery with checkpoint observation after either success or
/// refusal, including a failed history await. Keeps authenticated recovery and
/// its post-await observation together without claiming lifecycle ownership.
/// # Errors
/// Returns the existing passive recovery refusal; no refusal clears the fence.
pub async fn resume_host_instance(
    host: &impl ContinuityInstallationAccess,
    context: UploadContext,
) -> Result<(), ic_blob_storage_contracts::dto::recovery::CurrentInstanceRecoveryFailure> {
    let outcome = resume_current_instance(host, context)
        .await
        .map_err(crate::ops::service::recovery::failure);
    host.observe_version();
    outcome
}

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
    let proof = prove_observed_instance(host, anchor).await?;
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

async fn prove_observed_instance(
    host: &impl RecoveryInstallationAccess,
    anchor: u64,
) -> Result<crate::ops::service::recovery::CurrentInstanceProof, CurrentInstanceRecoveryError> {
    let observation = host.with_installation(|installation| {
        installation.begin_history_observation(ic_platform::ic0::canister_version())
    });
    prove_observed_current_instance(anchor, |request, execution| {
        host.with_installation(|installation| {
            installation.check_history_observation(observation, request, execution)
        })
    })
    .await
}
