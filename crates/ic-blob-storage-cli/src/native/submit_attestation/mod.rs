//! Explicit one-shot submission of a complete, retained verifier observation.
mod observation;
#[cfg(test)]
mod tests;
use super::{
    Failure, agent,
    arguments::Options,
    attestation,
    signed_update::{DEADLINE_SECONDS, PreparedUpdate, SMALL_REPLY_BYTES, UpdateInput},
};
use candid::Principal;
use ic_blob_storage::{
    dto::upload::{admission::UploadAdmissionRequest, completion::UploadAttestationMutation},
    model::identity::ContentDigest,
    ops::service::uploads::completion::{UPLOAD_ATTEST_METHOD, reply},
};
use serde::Serialize;
use serde_json::Value;
use std::{collections::BTreeMap, path::PathBuf};

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
    dispatch(options, input, observed).await
}

/// A publication session pins both the exact original permission and raw body
/// digest before allowing the existing one-shot signed attestation claim.
pub(super) async fn run_for_upload(
    options: &Options,
    input: &Input,
    permission: UploadAdmissionRequest,
    digest: ContentDigest,
    gateway: &url::Url,
) -> Result<Value, Failure> {
    let observed = bound_observation(options, input, permission, digest, gateway)?;
    dispatch(options, input, observed).await
}

fn bound_observation(
    options: &Options,
    input: &Input,
    permission: UploadAdmissionRequest,
    digest: ContentDigest,
    gateway: &url::Url,
) -> Result<observation::Observation, Failure> {
    let observed = observation::Observation::open(options, input)?;
    if observed.recovery.statement.permission != permission
        || observed.recovery.statement.content_digest != *digest.as_bytes()
        || &observed.gateway != gateway
    {
        return Err(Failure::Binding);
    }
    Ok(observed)
}

/// Reconcile the exact original observation against immutable history. No
/// provider GET, regenerated statement or signed submission occurs here.
pub(super) async fn recover_for_upload(
    options: &Options,
    input: &Input,
    permission: UploadAdmissionRequest,
    digest: ContentDigest,
    gateway: &url::Url,
    run: &crate::native::artifacts::Run,
) -> Result<Value, Failure> {
    let observed = bound_observation(options, input, permission, digest, gateway)?;
    let (service, method, argument) = observed.recovery.query();
    run.bytes("attestation-query.candid", &argument)?;
    let bytes = crate::native::query(options, service, method, argument).await?;
    if bytes.len() > 4096 {
        return Err(Failure::ReplyLimit);
    }
    run.bytes("attestation-reply.candid", &bytes)?;
    let inspection =
        observed
            .recovery
            .inspect(&bytes, options.actor, options.network, options.url.as_str())?;
    run.json("attestation-inspection.json", &inspection.report)?;
    if !inspection.matched {
        return Err(Failure::VerificationRefused);
    }
    Ok(inspection.report)
}

async fn dispatch(
    options: &Options,
    input: &Input,
    observed: observation::Observation,
) -> Result<Value, Failure> {
    let agent = agent(options)?;
    let prepared = PreparedUpdate::claim(
        &agent,
        UpdateInput {
            service: input.service,
            method: UPLOAD_ATTEST_METHOD,
            argument: &observed.statement,
            argument_file: "statement.candid",
            directory: &input.directory.join("attestation"),
            reply_limit: SMALL_REPLY_BYTES,
        },
        |signed| DispatchIntentRecord {
            schema: 1,
            runner_version: env!("CARGO_PKG_VERSION"),
            runner_source_sha256: ContentDigest::compute(
                concat!(
                    include_str!("mod.rs"),
                    include_str!("observation/mod.rs"),
                    include_str!("../mod.rs"),
                    include_str!("../arguments/mod.rs"),
                    include_str!("../parsing/mod.rs"),
                    include_str!("../signed_update/mod.rs"),
                    include_str!("../attestation/mod.rs"),
                    include_str!("../upload_setup/mod.rs"),
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
            verifier: options.actor.to_text(),
            method: UPLOAD_ATTEST_METHOD,
            request_id: signed.request_id.to_string(),
            ingress_expiry_ns: signed.ingress_expiry.to_string(),
            signed_request_sha256: ContentDigest::compute(&signed.signed_update).to_string(),
            statement_sha256: ContentDigest::compute(&observed.statement).to_string(),
            observation_sha256: observed.hashes,
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
                reply::mutation(
                    observed.recovery.authority,
                    &observed.recovery.statement,
                    &bytes,
                    SMALL_REPLY_BYTES.try_into().expect("fixed reply bound"),
                )
                .map_err(attestation::failure)
            })
            .transpose()
    });
    let report = DispatchOutcomeRecord::new(dispatched.request_id, &result);
    dispatched.run.json("outcome.json", &report)?;
    result?;
    serde_json::to_value(report).map_err(|_| Failure::File)
}
