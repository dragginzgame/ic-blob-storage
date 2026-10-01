//! One explicit gateway decision, durably claimed before dispatch and never retried.
use super::{Failure, agent, arguments::Options, observe_upload::record::Run};
use candid::{Principal, de::DecoderConfig, decode_one_with_config};
use ic_agent::{Agent, agent::CallResponse};
use ic_blob_storage::{
    dto::{
        gateway::{
            GatewayRevocationFailure as RevokeError, GatewayRevocationRequest,
            GatewayRevocationResponse,
            sync::{GatewaySyncCancellation, GatewaySyncFailure as SyncError, GatewaySyncResponse},
        },
        operator::OperatorScope,
    },
    model::identity::ContentDigest,
    ops::service::gateways::{
        revocation::GATEWAY_REVOCATION_METHOD,
        sync::{GATEWAY_SYNC_CANCEL_METHOD, GATEWAY_SYNC_METHOD},
    },
};
use serde::Serialize;
use serde_json::{Value, json};
use std::{path::PathBuf, time::Duration};

pub(super) struct Input {
    pub scope: OperatorScope,
    pub action: Action,
    pub directory: PathBuf,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Action {
    Sync,
    Cancel(u64),
    Revoke(Principal),
}
impl Input {
    fn method(&self) -> &'static str {
        match self.action {
            Action::Sync => GATEWAY_SYNC_METHOD,
            Action::Cancel(_) => GATEWAY_SYNC_CANCEL_METHOD,
            Action::Revoke(_) => GATEWAY_REVOCATION_METHOD,
        }
    }
    fn argument(&self) -> Result<Vec<u8>, Failure> {
        match self.action {
            Action::Sync => candid::encode_one(self.scope),
            Action::Cancel(sequence) => candid::encode_one(GatewaySyncCancellation {
                scope: self.scope,
                sequence,
            }),
            Action::Revoke(gateway) => candid::encode_one(GatewayRevocationRequest {
                scope: self.scope,
                gateway,
            }),
        }
        .map_err(|_| Failure::Arguments)
    }
    fn decision(&self) -> Value {
        match self.action {
            Action::Sync => json!({"kind":"sync"}),
            Action::Cancel(sequence) => json!({"kind":"cancel","sequence":sequence.to_string()}),
            Action::Revoke(gateway) => json!({"kind":"revoke","gateway":gateway.to_text()}),
        }
    }
}

#[derive(Serialize)]
struct DispatchIntentRecord<'a> {
    schema: u8,
    runner_version: &'a str,
    runner_source_sha256: String,
    network: &'a str,
    service_url: &'a str,
    scope: Value,
    operator: String,
    decision: Value,
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
fn scope(scope: OperatorScope) -> Value {
    json!({"service":scope.service.to_text(),"namespace":scope.namespace.to_string(),
        "cashier":scope.cashier.to_text(),"payment_account":scope.payment_account.to_text()})
}
fn prepare(
    options: &Options,
    input: &Input,
    agent: &Agent,
) -> Result<(Run, ic_agent::agent::signed::SignedUpdate), Failure> {
    let argument = input.argument()?;
    let signed = agent
        .update(&input.scope.service, input.method())
        .with_arg(argument.clone())
        .expire_after(Duration::from_secs(120))
        .sign()
        .map_err(|_| Failure::Identity)?;
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
                    include_str!("../observe_upload/record/mod.rs"),
                    include_str!("../../../../../Cargo.lock")
                )
                .as_bytes(),
            )
            .to_string(),
            network: options.network,
            service_url: options.url.as_str(),
            scope: scope(input.scope),
            operator: options.actor.to_text(),
            decision: input.decision(),
            method: input.method(),
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
    Ok((run, signed))
}
fn decode<T: candid::CandidType + for<'de> serde::Deserialize<'de>>(
    bytes: &[u8],
) -> Result<T, Failure> {
    if bytes.len() > 4096 {
        return Err(Failure::ReplyLimit);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(100_000)
        .set_skipping_quota(0)
        .set_max_type_len(128)
        .set_max_header_len(4096)
        .set_full_error_message(false);
    decode_one_with_config(bytes, &config).map_err(|_| Failure::InvalidReply)
}
fn acknowledgment(input: &Input, bytes: &[u8]) -> Result<Value, Failure> {
    match input.action {
        Action::Sync => {
            let reply: Result<GatewaySyncResponse, SyncError> = decode(bytes)?;
            let reply = reply.map_err(Failure::GatewaySyncRefused)?;
            if reply.scope != input.scope {
                return Err(Failure::Binding);
            }
            if reply.sequence == 0 {
                return Err(Failure::InvalidReply);
            }
            Ok(json!({"sequence":reply.sequence.to_string()}))
        }
        Action::Cancel(sequence) => {
            let reply: Result<(), SyncError> = decode(bytes)?;
            reply.map_err(Failure::GatewaySyncRefused)?;
            Ok(json!({"sequence":sequence.to_string()}))
        }
        Action::Revoke(gateway) => {
            let reply: Result<GatewayRevocationResponse, RevokeError> = decode(bytes)?;
            let reply = reply.map_err(Failure::GatewayRevocationRefused)?;
            if reply.request
                != (GatewayRevocationRequest {
                    scope: input.scope,
                    gateway,
                })
            {
                return Err(Failure::Binding);
            }
            Ok(json!({"removed":reply.removed}))
        }
    }
}
fn outcome(input: &Input, id: &str, result: &Result<Option<Value>, Failure>) -> Value {
    let mut report = json!({"schema":1,"request_id":id,"outcome":"pending",
        "scope":scope(input.scope),"decision":input.decision(),"acknowledgment":null,
        "retry_authorized":false,"provider_deletion":"not_established",
        "billing_cessation":"not_established","membership":"not_observed",
        "historical_receipt":"not_available"});
    match result {
        Ok(Some(ack)) => {
            report["outcome"] = "acknowledged".into();
            report["acknowledgment"] = ack.clone();
        }
        Ok(None) => {}
        Err(error) => {
            report["outcome"] = if matches!(
                error,
                Failure::GatewaySyncRefused(_) | Failure::GatewayRevocationRefused(_)
            ) {
                "refused"
            } else {
                "uncertain"
            }
            .into();
            report["error"] = error.code().into();
        }
    }
    report
}
pub(super) async fn run(options: &Options, input: &Input) -> Result<Value, Failure> {
    let agent = agent(options)?;
    let (run, signed) = prepare(options, input, &agent)?;
    let result = tokio::time::timeout(
        Duration::from_secs(30),
        agent.update_signed(input.scope.service, signed.signed_update),
    )
    .await
    .map_err(|_| Failure::Timeout)
    .and_then(|reply| reply.map_err(|_| Failure::Transport));
    let result = match result {
        Ok(CallResponse::Response(bytes)) => {
            if bytes.len() > 4096 {
                Err(Failure::ReplyLimit)
            } else {
                run.bytes("response.candid", &bytes)?;
                acknowledgment(input, &bytes).map(Some)
            }
        }
        Ok(CallResponse::Poll(_)) => Ok(None),
        Err(error) => Err(error),
    };
    let report = outcome(input, &signed.request_id.to_string(), &result);
    run.json("outcome.json", &report)?;
    result?;
    Ok(report)
}
pub(super) const fn sync_code(error: SyncError) -> &'static str {
    match error {
        SyncError::Invalid => "gateway_invalid",
        SyncError::Binding => "gateway_binding",
        SyncError::Denied => "gateway_denied",
        SyncError::Fenced => "gateway_fenced",
        SyncError::Busy => "gateway_busy",
        SyncError::Exhausted => "gateway_exhausted",
        SyncError::Conflict => "gateway_conflict",
        SyncError::ReplyTooLarge => "gateway_reply_too_large",
        SyncError::InvalidReply => "gateway_invalid_reply",
        SyncError::NotEnqueued => "gateway_not_enqueued",
        SyncError::Rejected(_) => "gateway_rejected",
        SyncError::Internal => "gateway_internal",
    }
}
pub(super) const fn revocation_code(error: RevokeError) -> &'static str {
    match error {
        RevokeError::Invalid => "gateway_invalid",
        RevokeError::Binding => "gateway_binding",
        RevokeError::Denied => "gateway_denied",
        RevokeError::Fenced => "gateway_fenced",
        RevokeError::Internal => "gateway_internal",
    }
}

#[cfg(test)]
mod tests;
