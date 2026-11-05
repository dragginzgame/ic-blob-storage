//! Stop at uncertainty; recovery observes the original operation without resubmission.
use super::{
    Failure, Input, Options, PreparationOutcome, PreparationState, Run, Value, journal, json,
    upload_setup,
};
use crate::native::{
    publish_check::observation::PreflightObservation, upload_inputs::PreparedInput,
};
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::upload::UploadState;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestInspection;
use std::path::Path;
use upload_setup::SetupObservation;

#[derive(Clone, Copy)]
enum SetupStep {
    Permission,
    Admission,
    Manifest,
    Preparation,
}

pub(super) struct Context<'a> {
    pub options: &'a Options,
    pub tenant: &'a ic_platform::agent::Agent,
    pub uploader: &'a ic_platform::agent::Agent,
    pub input: &'a Input,
    pub selected: &'a PreparedInput,
    pub origin: &'a Path,
    pub run: &'a Run,
}
fn preflight_allows(
    observation: &PreflightObservation,
    selected: ReferenceUpload,
    recovery: bool,
) -> Result<bool, Failure> {
    if recovery && let Some(original) = observation.recoverable_upload {
        if original != selected {
            return Err(Failure::Binding);
        }
        return Ok(observation.setup_allowed);
    }
    Ok(!recovery && observation.recoverable_upload.is_none() && observation.setup_allowed)
}
impl Context<'_> {
    fn setup(&self, step: SetupStep) -> Result<upload_setup::Request, Failure> {
        use upload_setup::Kind;
        let (kind, claim) = match step {
            SetupStep::Permission => (Kind::Permission, None),
            SetupStep::Admission => (Kind::Admit, Some(journal::Step::Admission)),
            SetupStep::Manifest => (Kind::Manifest, None),
            SetupStep::Preparation => (Kind::Prepare, Some(journal::Step::Preparation)),
        };
        let agent = if matches!(kind, Kind::Prepare) {
            self.uploader
        } else {
            self.tenant
        };
        upload_setup::Request::prepared(
            kind,
            self.selected,
            claim.map(|step| self.origin.join(journal::name(step))),
            self.input.service,
            self.input.namespace,
            agent.get_principal().map_err(|_| Failure::Identity)?.into(),
        )
    }
    fn report(
        &self,
        state: PreparationState,
        updates: u64,
        preflight: &Value,
    ) -> PreparationOutcome {
        PreparationOutcome {
            state,
            report: json!({"schema":1,"operation":"publish_prepare","state":state,"file_index":self.input.index,
            "permission":upload_setup::permission_json(self.selected.permission),"preflight":preflight,
            "original_run":self.origin,"service_updates_this_run":updates,"max_service_queries":super::MAX_QUERIES,
            "prepared":state==PreparationState::Prepared,"batch_complete":false,"certificate_issued":false,
            "provider_requests":0,"retry_authorized":false,"publication_authorized":false}),
        }
    }
    pub async fn advance(
        &self,
        preflight: PreflightObservation,
    ) -> Result<PreparationOutcome, Failure> {
        let recovery = self.input.source.is_some();
        if !preflight_allows(&preflight, self.selected.permission.upload, recovery)? {
            return Ok(self.report(PreparationState::Blocked, 0, &preflight.report));
        }
        let mut updates = 0;
        let permission = if recovery {
            let SetupObservation::Permission(permission) = upload_setup::inspect_recorded(
                self.tenant,
                &self.setup(SetupStep::Permission)?,
                self.run,
                "permission",
            )
            .await?
            else {
                return Err(Failure::InvalidReply);
            };
            permission
        } else {
            let result = upload_setup::execute(
                self.options,
                &self.setup(SetupStep::Admission)?,
                self.tenant,
            )
            .await?;
            updates += 1;
            match result.observation {
                Some(SetupObservation::Admission(admission)) => admission.admission,
                None => {
                    return Ok(self.report(
                        PreparationState::AdmissionUnobserved,
                        updates,
                        &preflight.report,
                    ));
                }
                Some(_) => return Err(Failure::InvalidReply),
            }
        };
        if permission.state != UploadState::Reserved || permission.revoked {
            return Ok(self.report(
                PreparationState::PermissionInactive,
                updates,
                &preflight.report,
            ));
        }
        let SetupObservation::Manifest {
            response: manifest, ..
        } = upload_setup::inspect_recorded(
            self.tenant,
            &self.setup(SetupStep::Manifest)?,
            self.run,
            "manifest",
        )
        .await?
        else {
            return Err(Failure::InvalidReply);
        };
        let state = match manifest.manifest {
            UploadManifestInspection::Prepared(declaration) => {
                if declaration != *self.selected.declaration() {
                    return Err(Failure::Binding);
                }
                PreparationState::Prepared
            }
            UploadManifestInspection::Unprepared => PreparationState::PreparationUnobserved,
        };
        // Fresh acknowledged preparation needs no historical claim inspection.
        // Every other surviving claim uses the same exact-envelope validation,
        // whether inspection found its manifest or its reply remains uncertain.
        let claimed = (recovery || state != PreparationState::Prepared)
            && journal::claimed(self.origin, journal::Step::Preparation)?;
        if claimed {
            journal::validate_attempt(
                &journal::RecoveryDirectory::open(self.origin)?,
                journal::Step::Preparation,
                self.options,
                self.selected,
                &self.tenant.read_root_key(),
            )?;
        }
        if state == PreparationState::Prepared || claimed {
            // Unprepared, missing/partial evidence or expiry never licenses another attempt.
            return Ok(self.report(state, updates, &preflight.report));
        }
        let result = upload_setup::execute(
            self.options,
            &self.setup(SetupStep::Preparation)?,
            self.uploader,
        )
        .await?;
        updates += 1;
        let state = match result.observation {
            Some(SetupObservation::Manifest { .. }) => PreparationState::Prepared,
            None => PreparationState::PreparationUnobserved,
            Some(_) => return Err(Failure::InvalidReply),
        };
        Ok(self.report(state, updates, &preflight.report))
    }
}

#[cfg(test)]
mod tests;
