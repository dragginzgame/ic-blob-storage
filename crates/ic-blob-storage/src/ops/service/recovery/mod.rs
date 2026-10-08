//! Replicated IC history acquisition and sealed same-execution continuation proofs.
use crate::{
    model::service::recovery::{InstanceChange, InstanceChangeKind, InstanceHistory},
    policy::recovery::{InstanceContinuityError, assess_instance_continuity},
};
use candid::{DecoderConfig, Principal};
use ic_cdk::call::Call;
use ic_management_canister_types::{
    CanisterInfoArgs, CanisterInfoResult, ChangeDetails, CodeDeploymentMode,
};

/// Synchronous host access to the same restored installation. The host releases
/// its borrow before returning; no saved handle crosses the platform await.
pub trait RecoveryInstallationAccess {
    /// Host-granted memory, from its sole runtime.
    type Memory: ic_memory::ic_stable_structures::Memory;
    /// Borrow the same installation for an authenticated synchronous operation.
    fn with_installation<R>(
        &self,
        operation: impl FnOnce(&mut super::installation::ServiceInstallation<Self::Memory>) -> R,
    ) -> R;
}

/// Convert domain/platform refusals to passive endpoint data.
#[must_use]
pub fn failure(
    error: CurrentInstanceRecoveryError,
) -> ic_blob_storage_contracts::dto::recovery::CurrentInstanceRecoveryFailure {
    use CurrentInstanceRecoveryError as Input;
    use ic_blob_storage_contracts::dto::recovery::CurrentInstanceRecoveryFailure as Output;
    match error {
        Input::Denied => Output::Denied,
        Input::Binding => Output::Binding,
        Input::Execution => Output::Execution,
        Input::Platform => Output::Platform,
        Input::Reply => Output::Reply,
        Input::Continuity(error) => match error {
            InstanceContinuityError::MissingAnchor => Output::MissingAnchor,
            InstanceContinuityError::IncompleteHistory => Output::IncompleteHistory,
            InstanceContinuityError::InvalidHistory => Output::InvalidHistory,
            InstanceContinuityError::SnapshotRestored => Output::SnapshotRestored,
            InstanceContinuityError::Replaced => Output::Replaced,
            InstanceContinuityError::UnqualifiedChange => Output::UnqualifiedChange,
        },
    }
}

/// Recovery refusal never clears an existing fence or discards an obligation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum CurrentInstanceRecoveryError {
    /// The actual caller is not the installed operator.
    #[error("recovery operator denied")]
    Denied,
    /// Service, immutable anchor or executing platform version changed.
    #[error("recovery binding changed")]
    Binding,
    /// Only a replicated execution can obtain or consume a platform proof.
    #[error("replicated recovery execution required")]
    Execution,
    /// The bounded platform call failed; there is no automatic retry.
    #[error("platform history request failed")]
    Platform,
    /// Reply size, shape or module identity could not be validated.
    #[error("invalid platform history reply")]
    Reply,
    /// Independently obtained history does not prove the current instance continuous.
    #[error(transparent)]
    Continuity(#[from] InstanceContinuityError),
}

/// Single-use current-platform proof. It cannot be constructed from ingress,
/// serialized, cloned or supplied by an old backup. Consumption in a later
/// execution is refused, including after an intervening await.
pub struct CurrentInstanceProof {
    service: Principal,
    installation_version: u64,
    execution_version: u64,
}
impl CurrentInstanceProof {
    pub(super) fn check(
        self,
        service: Principal,
        installation_version: u64,
    ) -> Result<(), CurrentInstanceRecoveryError> {
        if !ic_cdk::api::in_replicated_execution() {
            return Err(CurrentInstanceRecoveryError::Execution);
        }
        if self.service != service
            || self.service != ic_cdk::api::canister_self()
            || self.installation_version != installation_version
            || self.execution_version != ic_cdk::api::canister_version()
        {
            return Err(CurrentInstanceRecoveryError::Binding);
        }
        Ok(())
    }
}

/// Fetch the actual canister's IC-maintained history once with a thirty-second
/// bounded wait, no attached cycles and no retry. The installing host supplies
/// its immutable retained anchor; no ingress field may override it. Scope and
/// operator checks belong to the host before this call and after its await.
///
/// The proof covers ordinary current-instance upgrades only. History must reach
/// the installation and contain no later snapshot load, replacement or unknown
/// change. Expired platform history cannot be replaced by local counters. The
/// CDK buffers its platform-bounded reply before the 64 KiB application check.
/// # Errors
/// Rejects nonreplicated execution, missing anchors, failed/invalid platform replies
/// and unqualified continuity. Does not clear a fence or mutate any journal.
pub async fn prove_current_instance(
    installation_version: u64,
) -> Result<CurrentInstanceProof, CurrentInstanceRecoveryError> {
    use CurrentInstanceRecoveryError as Error;
    if !ic_cdk::api::in_replicated_execution() {
        return Err(Error::Execution);
    }
    if installation_version == 0 {
        return Err(InstanceContinuityError::MissingAnchor.into());
    }
    let service = ic_cdk::api::canister_self();
    let request_version = ic_cdk::api::canister_version();
    let reply = Call::bounded_wait(Principal::management_canister(), "canister_info")
        .change_timeout(30)
        .with_arg(CanisterInfoArgs {
            canister_id: service,
            num_requested_changes: Some(20),
        })
        .await
        .map_err(|_| Error::Platform)?;
    let bytes = reply.into_bytes();
    if bytes.len() > 65_536 || service != ic_cdk::api::canister_self() {
        return Err(Error::Reply);
    }
    let mut limits = DecoderConfig::new();
    limits
        .set_decoding_quota(2_000_000)
        .set_skipping_quota(1024)
        .set_max_type_len(128)
        .set_max_header_len(4096)
        .set_full_error_message(false);
    let info: CanisterInfoResult =
        candid::decode_one_with_config(&bytes, &limits).map_err(|_| Error::Reply)?;
    if info
        .module_hash
        .as_ref()
        .is_none_or(|hash| hash.len() != 32)
    {
        return Err(Error::Reply);
    }
    let history = InstanceHistory {
        total_changes: info.total_num_changes,
        changes: info
            .recent_changes
            .iter()
            .map(|change| InstanceChange {
                version: change.canister_version,
                kind: match &change.details {
                    Some(ChangeDetails::CodeDeployment(deployment))
                        if matches!(deployment.mode, CodeDeploymentMode::Upgrade) =>
                    {
                        InstanceChangeKind::Upgrade
                    }
                    Some(ChangeDetails::ControllersChange(_)) => InstanceChangeKind::Controllers,
                    Some(ChangeDetails::LoadSnapshot(_)) => InstanceChangeKind::Snapshot,
                    Some(
                        ChangeDetails::Creation(_)
                        | ChangeDetails::CodeUninstall
                        | ChangeDetails::CodeDeployment(_),
                    ) => InstanceChangeKind::Replacement,
                    Some(ChangeDetails::RenameCanister(_)) | None => {
                        InstanceChangeKind::Unqualified
                    }
                },
            })
            .collect(),
    };
    let execution_version = ic_cdk::api::canister_version();
    // The request commits once before this callback. Any additional replicated
    // execution or management change makes the returned history potentially
    // stale, even if it reached installation when the IC answered. Reacquire
    // through a new explicit read-only call; never reuse or rotate this proof.
    if request_version.checked_add(1) != Some(execution_version) {
        return Err(Error::Binding);
    }
    assess_instance_continuity(installation_version, execution_version, &history)?;
    Ok(CurrentInstanceProof {
        service,
        installation_version,
        execution_version,
    })
}
