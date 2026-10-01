//! One exact tenant mutation, with retained intent and no redispatch or ID allocation.
use super::{Failure, agent, arguments::Options, artifacts::Run, references};
use candid::Principal;
use ic_agent::agent::CallResponse;
use ic_blob_storage::{
    dto::reference::{ReferenceAction, ReferenceMutationResponse},
    model::identity::ContentDigest,
    ops::service::references::{REFERENCE_APPLY_METHOD, reply},
};
use serde::Serialize;
use serde_json::{Value, json};
use std::{path::PathBuf, time::Duration};

pub(super) struct Input {
    pub service: Principal,
    pub namespace: u128,
    pub request: PathBuf,
    pub directory: PathBuf,
}

#[derive(Serialize)]
struct DispatchIntentRecord<'a> {
    schema: u8,
    runner_version: &'a str,
    runner_source_sha256: String,
    network: &'a str,
    service_url: &'a str,
    service: String,
    namespace: String,
    tenant: String,
    upload: Value,
    reference: String,
    operation: String,
    action: &'a str,
    method: &'a str,
    request_id: String,
    ingress_expiry_ns: String,
    signed_request_sha256: String,
    request_sha256: String,
    root_key_sha256: String,
    max_update_requests: u8,
    request_deadline_seconds: u8,
    retry_authorized: bool,
}

fn outcome(request_id: &str, result: &Result<Option<ReferenceMutationResponse>, Failure>) -> Value {
    let mut value = json!({"schema":1,"request_id":request_id,"outcome":"pending",
        "result":null,"replayed":null,"retry_authorized":false,
        "reference_liveness":"not_observed","provider_deletion":"not_established",
        "billing_cessation":"not_established","publication_authorized":false});
    match result {
        Ok(Some(response)) => {
            value["outcome"] = "recorded".into();
            value["result"] = references::result_json(response.receipt.result);
            value["replayed"] = response.replayed.into();
        }
        Ok(None) => {}
        Err(failure) => {
            value["outcome"] = if matches!(failure, Failure::ReferenceRefused(_)) {
                "refused"
            } else {
                "uncertain"
            }
            .into();
            value["error"] = failure.code().into();
        }
    }
    value
}

pub(super) async fn run(options: &Options, input: &Input) -> Result<Value, Failure> {
    let (request, argument) = references::command(
        input.service,
        input.namespace,
        &input.request,
        options.actor,
    )?;
    let agent = agent(options)?;
    let signed = agent
        .update(&input.service, REFERENCE_APPLY_METHOD)
        .with_arg(argument.clone())
        .expire_after(Duration::from_secs(120))
        .sign()
        .map_err(|_| Failure::Identity)?;
    // The claim survives interruption even before the first file. A caller must
    // reconcile the exact receipt; neither a partial claim nor absence authorizes retry.
    let run = Run::create(&input.directory).map_err(|error| {
        if error == Failure::ExistingRun {
            Failure::SubmissionClaimed
        } else {
            error
        }
    })?;
    run.bytes("request.candid", &argument)?;
    run.bytes("signed-request.cbor", &signed.signed_update)?;
    run.json(
        "intent.json",
        &DispatchIntentRecord {
            schema: 1,
            runner_version: env!("CARGO_PKG_VERSION"),
            runner_source_sha256: ContentDigest::compute(
                concat!(
                    include_str!("mod.rs"),
                    include_str!("../mod.rs"),
                    include_str!("../arguments/mod.rs"),
                    include_str!("../references/mod.rs"),
                    include_str!("../observe_upload/record/mod.rs"),
                    include_str!("../../../../../Cargo.lock")
                )
                .as_bytes(),
            )
            .to_string(),
            network: options.network,
            service_url: options.url.as_str(),
            service: input.service.to_text(),
            namespace: input.namespace.to_string(),
            tenant: options.actor.to_text(),
            upload: references::upload_json(request.upload),
            reference: request.reference.to_string(),
            operation: request.operation.to_string(),
            action: match request.action {
                ReferenceAction::Retain => "retain",
                ReferenceAction::Release => "release",
            },
            method: REFERENCE_APPLY_METHOD,
            request_id: signed.request_id.to_string(),
            ingress_expiry_ns: signed.ingress_expiry.to_string(),
            signed_request_sha256: ContentDigest::compute(&signed.signed_update).to_string(),
            request_sha256: ContentDigest::compute(&argument).to_string(),
            root_key_sha256: ContentDigest::compute(&agent.read_root_key()).to_string(),
            max_update_requests: 1,
            request_deadline_seconds: 30,
            retry_authorized: false,
        },
    )?;
    let result = tokio::time::timeout(
        Duration::from_secs(30),
        agent.update_signed(input.service, signed.signed_update),
    )
    .await
    .map_err(|_| Failure::Timeout)
    .and_then(|result| result.map_err(|_| Failure::Transport));
    let result = match result {
        Ok(CallResponse::Response(bytes)) => {
            if bytes.len() > 4096 {
                Err(Failure::ReplyLimit)
            } else {
                run.bytes("response.candid", &bytes)?;
                reply::decode_mutation(request, &bytes, 4096.try_into().unwrap())
                    .map(Some)
                    .map_err(references::failure)
            }
        }
        Ok(CallResponse::Poll(_)) => Ok(None),
        Err(failure) => Err(failure),
    };
    let report = outcome(&signed.request_id.to_string(), &result);
    run.json("outcome.json", &report)?;
    result?;
    Ok(report)
}

#[cfg(test)]
mod tests;
