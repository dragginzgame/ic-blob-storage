//! One durable signed update claim, consumed by one dispatch without polling or retry.
use super::{Failure, artifacts::Run};
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_platform::Principal;
use ic_platform::agent::{Agent, CallResponse, SignedUpdate};
use serde::Serialize;
pub(super) mod record;
use record::{DispatchIntentRecord, SignedDispatchRecord};
use std::{path::Path, time::Duration};

#[cfg(test)]
mod tests;

pub(super) const INGRESS_SECONDS: u64 = 120;
pub(super) const DEADLINE_SECONDS: u8 = 30;
pub(super) const SMALL_REPLY_BYTES: usize = 4096;
pub(super) const MANIFEST_REPLY_BYTES: usize = 65536;

/// Classify an observation only; the operation owns its authenticated refusal predicate.
pub(super) fn outcome<T>(
    result: &Result<Option<T>, Failure>,
    acknowledged: &'static str,
    refused: impl FnOnce(&Failure) -> bool,
) -> &'static str {
    match result {
        Ok(Some(_)) => acknowledged,
        Ok(None) => "pending",
        Err(error) => {
            if refused(error) {
                "refused"
            } else {
                "uncertain"
            }
        }
    }
}

/// The operation owns its authority, request name, intent fields and reply decoder.
#[derive(Clone, Copy)]
pub(super) struct UpdateInput<'a> {
    pub service: Principal,
    pub method: &'a str,
    pub argument: &'a [u8],
    pub argument_file: &'a str,
    pub directory: &'a Path,
    pub reply_limit: usize,
}

/// Only constructed after the exact packets and caller's intent have been synced.
pub(super) struct PreparedUpdate {
    run: Run,
    signed: SignedUpdate,
    service: Principal,
    reply_limit: usize,
}

/// A response is saved before operation-specific decoding; absence remains pending.
pub(super) struct DispatchedUpdate {
    run: Run,
    request_id: String,
    response: Result<Option<Vec<u8>>, Failure>,
}

impl DispatchedUpdate {
    /// Decode only a present reply, then retain the operation's report before
    /// returning its result. A failed report write takes precedence; no dispatch,
    /// polling or retry occurs here. Report fields and refusal policy stay local.
    pub fn finish<T, R: Serialize>(
        self,
        decode: impl FnOnce(&[u8]) -> Result<T, Failure>,
        report: impl FnOnce(String, &Result<Option<T>, Failure>) -> R,
    ) -> Result<(Option<T>, R), Failure> {
        let result = self
            .response
            .and_then(|bytes| bytes.map(|bytes| decode(&bytes)).transpose());
        let report = report(self.request_id, &result);
        self.run.json("outcome.json", &report)?;
        Ok((result?, report))
    }
}

impl PreparedUpdate {
    pub fn claim<R: Serialize>(
        agent: &Agent,
        input: UpdateInput<'_>,
        intent: impl FnOnce() -> R,
    ) -> Result<Self, Failure> {
        let signed = agent
            .update(&input.service.into(), input.method)
            .with_arg(input.argument.to_vec())
            .expire_after(Duration::from_secs(INGRESS_SECONDS))
            .sign()
            .map_err(|_| Failure::Identity)?;
        // Even an empty/partial directory remains a permanent claim after interruption.
        let run = Run::create(input.directory).map_err(|error| {
            if error == Failure::ExistingRun {
                Failure::SubmissionClaimed
            } else {
                error
            }
        })?;
        run.bytes(input.argument_file, input.argument)?;
        run.bytes("signed-request.cbor", &signed.signed_update)?;
        run.json(
            "intent.json",
            &DispatchIntentRecord {
                signed: SignedDispatchRecord {
                    request_id: signed.request_id.to_string(),
                    ingress_expiry_ns: signed.ingress_expiry.to_string(),
                    signed_request_sha256: ContentDigest::compute(&signed.signed_update)
                        .to_string(),
                    root_key_sha256: ContentDigest::compute(&agent.read_root_key()).to_string(),
                    max_update_requests: 1,
                    request_deadline_seconds: DEADLINE_SECONDS,
                    retry_authorized: false,
                },
                operation: intent(),
            },
        )?;
        Ok(Self {
            run,
            signed,
            service: input.service,
            reply_limit: input.reply_limit,
        })
    }

    pub fn run(&self) -> &Run {
        &self.run
    }

    pub async fn dispatch(self, agent: &Agent) -> DispatchedUpdate {
        let request_id = self.signed.request_id.to_string();
        let response = tokio::time::timeout(
            Duration::from_secs(u64::from(DEADLINE_SECONDS)),
            agent.update_signed(
                ic_platform::principal::ExternalPrincipal::from(self.service),
                self.signed.signed_update,
            ),
        )
        .await
        .map_err(|_| Failure::Timeout)
        .and_then(|reply| reply.map_err(|_| Failure::Transport));
        let response = response.and_then(|reply| match reply {
            CallResponse::Poll(_) => Ok(None),
            CallResponse::Response(bytes) => {
                if bytes.len() > self.reply_limit {
                    return Err(Failure::ReplyLimit);
                }
                self.run.bytes("response.wire", &bytes)?;
                Ok(Some(bytes))
            }
        });
        DispatchedUpdate {
            run: self.run,
            request_id,
            response,
        }
    }
}
