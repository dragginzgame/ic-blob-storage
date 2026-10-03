//! Stop at uncertainty; recovery observes the original operation without resubmission.
use super::{
    Failure, Input, Options, PreparationOutcome, PreparationState, Run, Value, journal, json,
    upload_setup,
};
use crate::native::{
    publish_check::observation::PreflightObservation, upload_inputs::PreparedInput,
};
use ic_blob_storage::dto::reference::ReferenceUpload;
use ic_blob_storage::dto::upload::{UploadState, manifest::UploadManifestInspection};
use std::path::Path;
use upload_setup::SetupObservation;

pub(super) struct Context<'a> {
    pub options: &'a Options,
    pub uploader: &'a Options,
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
    fn setup(&self, kind: upload_setup::Kind) -> upload_setup::Input {
        upload_setup::Input {
            kind,
            service: self.input.service,
            namespace: self.input.namespace,
            request: self.origin.join(if kind == upload_setup::Kind::Prepare {
                "manifest.candid"
            } else {
                "permission.candid"
            }),
            directory: kind
                .mutation()
                .then(|| self.origin.join(journal::name(kind))),
        }
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
            "original_run":self.origin,"service_updates_this_run":updates,"max_service_queries":4,
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
                self.options,
                &self.setup(upload_setup::Kind::Permission),
                self.run,
                "permission",
            )
            .await?
            else {
                return Err(Failure::InvalidReply);
            };
            permission
        } else {
            let result =
                upload_setup::run(self.options, &self.setup(upload_setup::Kind::Admit)).await?;
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
            self.options,
            &self.setup(upload_setup::Kind::Manifest),
            self.run,
            "manifest",
        )
        .await?
        else {
            return Err(Failure::InvalidReply);
        };
        if let UploadManifestInspection::Prepared(declaration) = manifest.manifest {
            if declaration != *self.selected.declaration() {
                return Err(Failure::Binding);
            }
            if recovery && journal::claimed(self.origin, upload_setup::Kind::Prepare)? {
                journal::validate_attempt(
                    self.origin,
                    upload_setup::Kind::Prepare,
                    self.options,
                    self.selected,
                )?;
            }
            return Ok(self.report(PreparationState::Prepared, updates, &preflight.report));
        }
        if journal::claimed(self.origin, upload_setup::Kind::Prepare)? {
            // Unprepared, missing/partial evidence or expiry never licenses another attempt.
            journal::validate_attempt(
                self.origin,
                upload_setup::Kind::Prepare,
                self.options,
                self.selected,
            )?;
            return Ok(self.report(
                PreparationState::PreparationUnobserved,
                updates,
                &preflight.report,
            ));
        }
        let result =
            upload_setup::run(self.uploader, &self.setup(upload_setup::Kind::Prepare)).await?;
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
mod tests {
    use super::*;
    use crate::native::publish_check::tests::{batch, frozen};

    #[test]
    fn recovery_requires_original_identity_even_when_a_fence_blocks_setup() {
        let directory = frozen();
        let original = batch(&directory).files[0].input.permission.upload;
        let mut observation = PreflightObservation {
            recoverable_upload: Some(original),
            setup_allowed: false,
            report: Value::Null,
        };
        assert_eq!(preflight_allows(&observation, original, true), Ok(false));
        let mut changed = original;
        changed.upload ^= 1;
        assert_eq!(
            preflight_allows(&observation, changed, true),
            Err(Failure::Binding)
        );
        observation.setup_allowed = true;
        assert_eq!(preflight_allows(&observation, original, true), Ok(true));
        assert_eq!(preflight_allows(&observation, original, false), Ok(false));
    }
}
