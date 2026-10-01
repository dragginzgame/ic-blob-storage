//! Explicit one-shot submission of a complete, retained verifier observation.
mod observation;
#[cfg(test)]
mod tests;
use super::{Failure, agent, arguments::Options, artifacts::Run, attestation};
use candid::Principal;
use ic_agent::agent::CallResponse;
use ic_blob_storage::{
    dto::upload::completion::UploadAttestationMutation,
    model::identity::ContentDigest,
    ops::service::uploads::completion::{UPLOAD_ATTEST_METHOD, reply},
};
use serde::Serialize;
use serde_json::Value;
use std::{collections::BTreeMap, path::PathBuf, time::Duration};

pub(super) struct Input {
    pub service: Principal,
    pub namespace: u128,
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
    verifier: String,
    method: &'a str,
    request_id: String,
    ingress_expiry_ns: String,
    signed_request_sha256: String,
    statement_sha256: String,
    observation_sha256: BTreeMap<&'static str, String>,
    root_key_sha256: String,
    max_update_requests: u8,
    request_deadline_seconds: u8,
    retry_authorized: bool,
}

#[derive(Serialize)]
struct DispatchOutcomeRecord {
    schema: u8,
    request_id: String,
    outcome: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    changed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accepted_at_ns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<&'static str>,
    retry_authorized: bool,
    billing_cessation: &'static str,
    future_retention: &'static str,
}

impl DispatchOutcomeRecord {
    fn new(
        request_id: String,
        result: &Result<Option<UploadAttestationMutation>, Failure>,
    ) -> Self {
        let mut record = Self {
            schema: 1,
            request_id,
            outcome: "pending",
            changed: None,
            accepted_at_ns: None,
            error: None,
            retry_authorized: false,
            billing_cessation: "not_established",
            future_retention: "not_established",
        };
        match result {
            Ok(Some(ack)) => {
                record.outcome = "accepted";
                record.changed = Some(ack.changed);
                record.accepted_at_ns = Some(ack.receipt.accepted_at_ns.to_string());
            }
            Ok(None) => {}
            Err(failure) => {
                record.outcome = if *failure == Failure::AttestationRefused {
                    "refused"
                } else {
                    "uncertain"
                };
                record.error = Some(failure.code());
            }
        }
        record
    }
}

pub(super) async fn run(options: &Options, input: &Input) -> Result<Value, Failure> {
    let observed = observation::Observation::open(options, input)?;
    let agent = agent(options)?;
    let signed = agent
        .update(&input.service, UPLOAD_ATTEST_METHOD)
        .with_arg(observed.statement.clone())
        .expire_after(Duration::from_secs(120))
        .sign()
        .map_err(|_| Failure::Identity)?;
    // Atomic directory creation claims this run even if the process dies before
    // its first file. An empty/partial claim never grants redispatch authority.
    let run = Run::create(&input.directory.join("attestation")).map_err(|e| {
        if e == Failure::ExistingRun {
            Failure::SubmissionClaimed
        } else {
            e
        }
    })?;
    run.bytes("statement.candid", &observed.statement)?;
    run.bytes("signed-request.cbor", &signed.signed_update)?;
    run.json(
        "intent.json",
        &DispatchIntentRecord {
            schema: 1,
            runner_version: env!("CARGO_PKG_VERSION"),
            runner_source_sha256: ContentDigest::compute(
                concat!(
                    include_str!("mod.rs"),
                    include_str!("observation/mod.rs"),
                    include_str!("../mod.rs"),
                    include_str!("../arguments/mod.rs"),
                    include_str!("../attestation/mod.rs"),
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
            verifier: options.actor.to_text(),
            method: UPLOAD_ATTEST_METHOD,
            request_id: signed.request_id.to_string(),
            ingress_expiry_ns: signed.ingress_expiry.to_string(),
            signed_request_sha256: ContentDigest::compute(&signed.signed_update).to_string(),
            statement_sha256: ContentDigest::compute(&observed.statement).to_string(),
            observation_sha256: observed.hashes,
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
    .and_then(|r| r.map_err(|_| Failure::Transport));
    let result = match result {
        Ok(CallResponse::Response(bytes)) => {
            if bytes.len() > 4096 {
                Err(Failure::ReplyLimit)
            } else {
                run.bytes("response.candid", &bytes)?;
                reply::mutation(
                    observed.recovery.authority,
                    &observed.recovery.statement,
                    &bytes,
                    4096.try_into().unwrap(),
                )
                .map(Some)
                .map_err(attestation::failure)
            }
        }
        Ok(CallResponse::Poll(_)) => Ok(None),
        Err(failure) => Err(failure),
    };
    let report = DispatchOutcomeRecord::new(signed.request_id.to_string(), &result);
    run.json("outcome.json", &report)?;
    result?;
    serde_json::to_value(report).map_err(|_| Failure::File)
}
