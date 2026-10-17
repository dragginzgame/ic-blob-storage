//! One exact tenant mutation, with retained intent and no redispatch or ID allocation.
use super::{
    Failure, agent,
    arguments::Options,
    references,
    signed_update::{PreparedUpdate, SMALL_REPLY_BYTES, UpdateInput},
};
use candid::Principal;
use ic_blob_storage_contracts::dto::reference::ReferenceAction;
use ic_blob_storage_contracts::dto::reference::ReferenceMutationResponse;
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_blob_storage_contracts::protocol::REFERENCE_APPLY_METHOD;
use ic_blob_storage_contracts::reference::reply;
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
    request_sha256: String,
}

fn outcome(request_id: &str, result: &Result<Option<ReferenceMutationResponse>, Failure>) -> Value {
    let state = super::signed_update::outcome(result, "recorded", |e| {
        matches!(e, Failure::ReferenceRefused(_))
    });
    let mut value = json!({"schema":1,"request_id":request_id,"outcome":state,
        "result":null,"replayed":null,"retry_authorized":false,
        "reference_liveness":"not_observed","provider_deletion":"not_established",
        "billing_cessation":"not_established","publication_authorized":false});
    match result {
        Ok(Some(response)) => {
            value["result"] = references::result_json(response.receipt.result);
            value["replayed"] = response.replayed.into();
        }
        Ok(None) => {}
        Err(failure) => {
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
            argument_file: "request.cbor",
            directory: &input.directory,
            reply_limit: SMALL_REPLY_BYTES,
        },
        || DispatchIntentRecord {
            schema: 1,
            runner_version: env!("CARGO_PKG_VERSION"),
            runner_source_sha256: crate::native::provenance::runner_source(),
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
            request_sha256: ContentDigest::compute(&argument).to_string(),
        },
    )?;
    let (_, report) = prepared.dispatch(&agent).await.finish(
        |bytes| {
            reply::decode_mutation(
                request,
                bytes,
                SMALL_REPLY_BYTES.try_into().expect("fixed reply bound"),
            )
            .map_err(references::failure)
        },
        |id, result| outcome(&id, result),
    )?;
    Ok(report)
}

#[cfg(test)]
mod tests;
