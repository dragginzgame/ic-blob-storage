//! One durable signed update claim, consumed by one dispatch without polling or retry.
#[cfg(test)]
mod tests;
use super::{Failure, artifacts::Run};
use candid::Principal;
use ic_agent::{
    Agent,
    agent::{CallResponse, signed::SignedUpdate},
};
use serde::Serialize;
use std::{path::Path, time::Duration};

pub(super) const INGRESS_SECONDS: u64 = 120;
pub(super) const DEADLINE_SECONDS: u8 = 30;
pub(super) const SMALL_REPLY_BYTES: usize = 4096;
pub(super) const MANIFEST_REPLY_BYTES: usize = 65536;

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
    pub run: Run,
    pub request_id: String,
    pub response: Result<Option<Vec<u8>>, Failure>,
}

impl PreparedUpdate {
    pub fn claim<R: Serialize>(
        agent: &Agent,
        input: UpdateInput<'_>,
        intent: impl FnOnce(&SignedUpdate) -> R,
    ) -> Result<Self, Failure> {
        let signed = agent
            .update(&input.service, input.method)
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
        run.json("intent.json", &intent(&signed))?;
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
            agent.update_signed(self.service, self.signed.signed_update),
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
                self.run.bytes("response.candid", &bytes)?;
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
