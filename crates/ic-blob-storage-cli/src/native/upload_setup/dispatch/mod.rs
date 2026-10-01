//! Persist the exact signed local update before dispatch; never resubmit or poll it.
use super::{Failure, Input, Request, observation, permission_json};
use crate::native::{agent, arguments::Options, artifacts::Run};
use ic_agent::agent::CallResponse;
use ic_blob_storage::model::identity::ContentDigest;
use serde_json::{Value, json};
use std::time::Duration;

pub(super) async fn run(
    options: &Options,
    input: &Input,
    saved: &Request,
) -> Result<Value, Failure> {
    let agent = agent(options)?;
    let signed = agent
        .update(&input.service, input.kind.method())
        .with_arg(saved.argument.clone())
        .expire_after(Duration::from_secs(120))
        .sign()
        .map_err(|_| Failure::Identity)?;
    let directory = input.directory.as_ref().ok_or(Failure::Arguments)?;
    let run = Run::create(directory).map_err(|e| {
        if e == Failure::ExistingRun {
            Failure::SubmissionClaimed
        } else {
            e
        }
    })?;
    run.bytes("request.candid", &saved.argument)?;
    run.bytes(
        "permission.candid",
        &candid::encode_one(saved.permission).map_err(|_| Failure::Arguments)?,
    )?;
    run.bytes("signed-request.cbor", &signed.signed_update)?;
    run.json("intent.json", &json!({"schema":1,"runner_version":env!("CARGO_PKG_VERSION"),
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
                include_str!("../../artifacts/mod.rs"),include_str!("../../../../../../Cargo.lock")).as_bytes()).to_string(),
        "max_update_requests":1,"request_deadline_seconds":30,"provider_requests":0,
        "retry_authorized":false}))?;
    let result = tokio::time::timeout(
        Duration::from_secs(30),
        agent.update_signed(input.service, signed.signed_update),
    )
    .await
    .map_err(|_| Failure::Timeout)
    .and_then(|r| r.map_err(|_| Failure::Transport));
    let result = match result {
        Ok(CallResponse::Poll(_)) => Ok(None),
        Ok(CallResponse::Response(bytes)) => {
            if bytes.len() > 65536 {
                Err(Failure::ReplyLimit)
            } else {
                run.bytes("response.candid", &bytes)?;
                observation(input, saved, &bytes).map(Some)
            }
        }
        Err(e) => Err(e),
    };
    let mut report = json!({"schema":1,"operation":input.kind.method(),"request_id":signed.request_id.to_string(),
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
    run.json("outcome.json", &report)?;
    result?;
    Ok(report)
}
