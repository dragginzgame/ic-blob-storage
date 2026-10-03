//! Persist the exact signed local update before dispatch; never resubmit or poll it.
use super::{Failure, Input, Request, observation, permission_json};
use crate::native::{
    agent,
    arguments::Options,
    signed_update::{DEADLINE_SECONDS, MANIFEST_REPLY_BYTES, PreparedUpdate, UpdateInput},
};
use ic_blob_storage::model::identity::ContentDigest;
use serde_json::{Value, json};

pub(super) async fn run(
    options: &Options,
    input: &Input,
    saved: &Request,
) -> Result<Value, Failure> {
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
        |signed| {
            json!({"schema":1,"runner_version":env!("CARGO_PKG_VERSION"),
        "network":options.network,"service_url":options.url.as_str(),"actor":options.actor.to_text(),
        "service":input.service.to_text(),"namespace":input.namespace.to_string(),
        "permission":permission_json(saved.permission),"method":input.kind.method(),
        "request_id":signed.request_id.to_string(),"ingress_expiry_ns":signed.ingress_expiry.to_string(),
        "request_sha256":ContentDigest::compute(&saved.argument).to_string(),
        "signed_request_sha256":ContentDigest::compute(&signed.signed_update).to_string(),
            "root_key_sha256":ContentDigest::compute(&agent.read_root_key()).to_string(),
            "runner_source_sha256":ContentDigest::compute(concat!(include_str!("mod.rs"),
                include_str!("../mod.rs"),include_str!("../request/mod.rs"),
                include_str!("../../mod.rs"),include_str!("../../arguments/mod.rs"),
                include_str!("../../parsing/mod.rs"),include_str!("../../signed_update/mod.rs"),
                include_str!("../../artifacts/mod.rs"),include_str!("../../../../../../Cargo.lock")).as_bytes()).to_string(),
        "max_update_requests":1,"request_deadline_seconds":DEADLINE_SECONDS,"provider_requests":0,
        "retry_authorized":false})
        },
    )?;
    prepared.run().bytes(
        "permission.candid",
        &candid::encode_one(saved.permission).map_err(|_| Failure::Arguments)?,
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
            report["observation"] = observed.clone();
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
    result?;
    Ok(report)
}
