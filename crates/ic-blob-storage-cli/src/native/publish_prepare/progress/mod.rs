//! Stop at uncertainty; recovery observes the original operation without resubmission.
use super::{Failure, Input, Options, Run, Value, journal, json, upload_setup};
use crate::native::{references, upload_inputs::PreparedInput};
use std::path::Path;

pub(super) struct Context<'a> {
    pub options: &'a Options,
    pub uploader: &'a Options,
    pub input: &'a Input,
    pub selected: &'a PreparedInput,
    pub origin: &'a Path,
    pub run: &'a Run,
}
fn preflight_allows(
    report: &Value,
    selected: &PreparedInput,
    recovery: bool,
) -> Result<bool, Failure> {
    let blockers = report["blockers"].as_array().ok_or(Failure::InvalidReply)?;
    let entry = &report["files"][0];
    if recovery && entry["status"] == "recover_existing_operation" {
        if entry["original_upload"] != references::upload_json(selected.permission.upload) {
            return Err(Failure::Binding);
        }
        return Ok(blockers.iter().all(|b| b == "recover_existing_operation"));
    }
    Ok(!recovery && blockers.is_empty() && entry["status"] == "not_visible")
}
fn reserved(observation: &Value) -> bool {
    observation["state"] == "reserved" && observation["revoked"] == false
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
    fn report(&self, state: &str, updates: u64, preflight: &Value) -> Value {
        json!({"schema":1,"operation":"publish_prepare","state":state,"file_index":self.input.index,
            "permission":upload_setup::permission_json(self.selected.permission),"preflight":preflight,
            "original_run":self.origin,"service_updates_this_run":updates,"max_service_queries":4,
            "prepared":state=="prepared","batch_complete":false,"certificate_issued":false,
            "provider_requests":0,"retry_authorized":false,"publication_authorized":false})
    }
    pub async fn advance(&self, preflight: Value) -> Result<Value, Failure> {
        let recovery = self.input.source.is_some();
        if !preflight_allows(&preflight, self.selected, recovery)? {
            return Ok(self.report("blocked", 0, &preflight));
        }
        let mut updates = 0;
        let permission = if recovery {
            upload_setup::inspect_recorded(
                self.options,
                &self.setup(upload_setup::Kind::Permission),
                self.run,
                "permission",
            )
            .await?
        } else {
            let result =
                upload_setup::run(self.options, &self.setup(upload_setup::Kind::Admit)).await?;
            updates += 1;
            if result["outcome"] != "acknowledged" {
                return Ok(self.report("admission_unobserved", updates, &preflight));
            }
            result["observation"]["admission"].clone()
        };
        if !reserved(&permission) {
            return Ok(self.report("permission_inactive", updates, &preflight));
        }
        let manifest = upload_setup::inspect_recorded(
            self.options,
            &self.setup(upload_setup::Kind::Manifest),
            self.run,
            "manifest",
        )
        .await?;
        let expected = upload_setup::declaration_json(self.selected.declaration());
        if !manifest["manifest"].is_null() {
            if manifest["manifest"] != expected {
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
            return Ok(self.report("prepared", updates, &preflight));
        }
        if journal::claimed(self.origin, upload_setup::Kind::Prepare)? {
            // Unprepared, missing/partial evidence or expiry never licenses another attempt.
            journal::validate_attempt(
                self.origin,
                upload_setup::Kind::Prepare,
                self.options,
                self.selected,
            )?;
            return Ok(self.report("preparation_unobserved", updates, &preflight));
        }
        let result =
            upload_setup::run(self.uploader, &self.setup(upload_setup::Kind::Prepare)).await?;
        updates += 1;
        let state = if result["outcome"] == "acknowledged" {
            "prepared"
        } else {
            "preparation_unobserved"
        };
        Ok(self.report(state, updates, &preflight))
    }
}
