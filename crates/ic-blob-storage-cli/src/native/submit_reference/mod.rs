//! One exact tenant mutation, with retained intent and no redispatch or ID allocation.
use super::{
    Failure, agent,
    arguments::Options,
    references,
    signed_update::{DEADLINE_SECONDS, PreparedUpdate, SMALL_REPLY_BYTES, UpdateInput},
};
use candid::Principal;
use ic_blob_storage::{
    dto::reference::{ReferenceAction, ReferenceMutationResponse},
    model::identity::ContentDigest,
    ops::service::references::{REFERENCE_APPLY_METHOD, reply},
};
use serde::Serialize;
use serde_json::{Value, json};
use std::path::PathBuf;

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
    let prepared = PreparedUpdate::claim(
        &agent,
        UpdateInput {
            service: input.service,
            method: REFERENCE_APPLY_METHOD,
            argument: &argument,
            argument_file: "request.candid",
            directory: &input.directory,
            reply_limit: SMALL_REPLY_BYTES,
        },
        |signed| DispatchIntentRecord {
            schema: 1,
            runner_version: env!("CARGO_PKG_VERSION"),
            runner_source_sha256: ContentDigest::compute(
                concat!(
                    include_str!("mod.rs"),
                    include_str!("../mod.rs"),
                    include_str!("../arguments/mod.rs"),
                    include_str!("../parsing/mod.rs"),
                    include_str!("../signed_update/mod.rs"),
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
            request_deadline_seconds: DEADLINE_SECONDS,
            retry_authorized: false,
        },
    )?;
    let dispatched = prepared.dispatch(&agent).await;
    let result = dispatched.response.and_then(|bytes| {
        bytes
            .map(|bytes| {
                reply::decode_mutation(
                    request,
                    &bytes,
                    SMALL_REPLY_BYTES.try_into().expect("fixed reply bound"),
                )
                .map_err(references::failure)
            })
            .transpose()
    });
    let report = outcome(&dispatched.request_id, &result);
    dispatched.run.json("outcome.json", &report)?;
    result?;
    Ok(report)
}

#[cfg(test)]
mod tests;
