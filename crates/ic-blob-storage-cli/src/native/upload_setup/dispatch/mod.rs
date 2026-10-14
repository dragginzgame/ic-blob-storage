//! Persist the exact signed local update before dispatch; never resubmit or poll it.
use super::{Failure, Input, Request, SetupOutcome, observation, permission_json};
use crate::native::{
    agent,
    arguments::Options,
    signed_update::{MANIFEST_REPLY_BYTES, PreparedUpdate, UpdateInput},
};
use ic_blob_storage_contracts::identity::ContentDigest;
use serde_json::json;

pub(super) async fn run(
    options: &Options,
    input: &Input,
    saved: &Request,
) -> Result<SetupOutcome, Failure> {
    let agent = agent(options)?;
    let directory = input.directory.as_ref().ok_or(Failure::Arguments)?;
    let prepared = PreparedUpdate::claim(
        &agent,
        UpdateInput {
            service: input.service,
            method: input.kind.method(),
            argument: &saved.argument,
            argument_file: "request.candid",
            directory,
            reply_limit: MANIFEST_REPLY_BYTES,
        },
        || {
            json!({"schema":1,"runner_version":env!("CARGO_PKG_VERSION"),
        "network":options.network,"service_url":options.url.as_str(),"actor":options.actor.to_text(),
        "service":input.service.to_text(),"namespace":input.namespace.to_string(),
        "permission":permission_json(saved.payload.permission()),"method":input.kind.method(),
        "request_sha256":ContentDigest::compute(&saved.argument).to_string(),
            "runner_source_sha256":crate::native::provenance::runner_source(concat!(
include_str!("mod.rs"),
include_str!("../mod.rs"),
include_str!("../request/mod.rs"),
)),
        "provider_requests":0})
        },
    )?;
    prepared.run().bytes(
        "permission.candid",
        &candid::encode_one(saved.payload.permission()).map_err(|_| Failure::Arguments)?,
    )?;
    let dispatched = prepared.dispatch(&agent).await;
    let result = dispatched.response.and_then(|bytes| {
        bytes
            .map(|bytes| observation(input, saved, &bytes))
            .transpose()
    });
    let mut report = json!({"schema":1,"operation":input.kind.method(),"request_id":dispatched.request_id,
        "outcome":"pending","observation":null,"authentication":null,
        "retry_authorized":false,"certificate_issued":false,"provider_requests":0,"publication_authorized":false});
    match &result {
        Ok(Some(observed)) => {
            report["authentication"] = "ic_update_certificate".into();
            report["outcome"] = "acknowledged".into();
            report["observation"] = observed.json();
        }
        Ok(None) => {}
        Err(e) => {
            report["outcome"] = if matches!(
                e,
                Failure::UploadAdmissionRefused(_) | Failure::UploadManifestRefused(_)
            ) {
                "refused"
            } else {
                "uncertain"
            }
            .into();
            report["error"] = e.code().into();
            if matches!(
                e,
                Failure::UploadAdmissionRefused(_) | Failure::UploadManifestRefused(_)
            ) {
                report["authentication"] = "ic_update_certificate".into();
            }
        }
    }
    dispatched.run.json("outcome.json", &report)?;
    Ok(SetupOutcome {
        observation: result?,
        report,
    })
}
