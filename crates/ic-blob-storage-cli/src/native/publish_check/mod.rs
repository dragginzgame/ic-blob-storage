//! Bounded authenticated batch observations, never allocation or dispatch authority.
pub(super) mod observation;
#[cfg(test)]
pub(super) mod tests;
use super::{Failure, arguments::Options, artifacts::Run, publish_inputs};
use candid::Principal;
use serde_json::{Value, json};
use std::{num::NonZeroU64, path::PathBuf, time::Duration};

pub(super) struct Input {
    pub service: Principal,
    pub namespace: u128,
    pub inputs: PathBuf,
    pub directory: PathBuf,
    pub max_bytes: NonZeroU64,
    pub max_total_bytes: NonZeroU64,
    pub max_queries: u64,
    pub timeout_seconds: u64,
}

pub(super) async fn run(options: &Options, input: &Input) -> Result<Value, Failure> {
    let batch = publish_inputs::PreparedBatch::open_frozen(
        &input.inputs,
        input.max_bytes,
        input.max_total_bytes,
    )?;
    let scope = observation::scope(&batch.files, input.service, input.namespace, options.actor)?;
    if input.max_queries < batch.files.len() as u64 + 1 {
        return Err(Failure::ReplyLimit);
    }
    // Authentication/root checks precede claiming a run or recording query intent.
    let agent = super::agent(options)?;
    let run = Run::create(&input.directory)?;
    run.bytes("inventory.json", &batch.inventory)?;
    run.bytes("installation.candid", &batch.installation)?;
    run.json(
        "intent.json",
        &json!({"schema":1,"operation":"publish_check",
        "network":options.network,"url":options.url.as_str(),"actor":options.actor.to_text(),
        "service":scope.service.to_text(),"namespace":scope.namespace.to_string(),
        "inventory_sha256":super::upload_inputs::digest(&batch.inventory),
        "max_queries":input.max_queries,"timeout_seconds":input.timeout_seconds,
        "query_reply_bytes":4096,"service_updates":0,"provider_requests":0,
        "identities_allocated":false,"automatic_retries":0}),
    )?;
    let result = tokio::time::timeout(Duration::from_secs(input.timeout_seconds), async {
        observation::inspect(scope, &batch.files, &run, |method, args| {
            let agent = &agent;
            async move {
                tokio::time::timeout(
                    Duration::from_secs(30),
                    agent.query(&scope.service, method).with_arg(args).call(),
                )
                .await
                .map_err(|_| Failure::Timeout)?
                .map_err(|_| Failure::Transport)
            }
        })
        .await
    })
    .await
    .map_err(|_| Failure::Timeout)
    .and_then(|v| v);
    let mut report = match result {
        Ok(observation) => observation.report,
        Err(error) => {
            run.json(
                "failure.json",
                &json!({"error":error.code(),"complete":false,
                "redispatch_authorized":false}),
            )?;
            return Err(error);
        }
    };
    report["inventory_sha256"] = json!(super::upload_inputs::digest(&batch.inventory));
    report["installation_sha256"] = json!(super::upload_inputs::digest(&batch.installation));
    report["network"] = json!(options.network);
    report["url"] = json!(options.url.as_str());
    run.json("summary.json", &report)?;
    Ok(report)
}
