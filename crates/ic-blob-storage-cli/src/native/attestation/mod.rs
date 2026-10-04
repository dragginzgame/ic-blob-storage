//! Compare saved intent to immutable service history without resending it.
use super::{Failure, read, references::upload_json};
use candid::Principal;
use ic_blob_storage::{
    dto::upload::completion::{UploadAttestationLookup, UploadAttestationRequest},
    model::{identity::ContentDigest, service::upload::completion::CompletionAuthority},
    ops::service::uploads::completion::reply::{self, UploadAttestationReplyError},
};
use serde_json::{Value, json};
use std::path::Path;

pub(super) struct Recovery {
    pub(super) authority: CompletionAuthority,
    pub(super) statement: UploadAttestationRequest,
    argument: Vec<u8>,
}

/// Typed receipt correlation; report JSON does not decide recovery authority.
pub(super) struct RecoveryInspection {
    pub matched: bool,
    pub report: Value,
}

pub(super) fn failure(error: UploadAttestationReplyError) -> Failure {
    match error {
        UploadAttestationReplyError::Limit => Failure::ReplyLimit,
        UploadAttestationReplyError::Invalid => Failure::InvalidReply,
        UploadAttestationReplyError::Binding => Failure::Binding,
        UploadAttestationReplyError::Remote(_) => Failure::AttestationRefused,
    }
}

impl Recovery {
    pub fn query(&self) -> (Principal, &'static str, Vec<u8>) {
        (
            self.authority.service(),
            ic_blob_storage::ops::service::uploads::completion::UPLOAD_ATTESTATION_METHOD,
            self.argument.clone(),
        )
    }

    pub fn open(
        service: Principal,
        namespace: u128,
        verifier: Principal,
        actor: Principal,
        path: &Path,
    ) -> Result<Self, Failure> {
        let bytes = read(path, 4096)?;
        Self::decode(service, namespace, verifier, actor, &bytes)
    }

    pub(super) fn decode(
        service: Principal,
        namespace: u128,
        verifier: Principal,
        actor: Principal,
        bytes: &[u8],
    ) -> Result<Self, Failure> {
        let statement: UploadAttestationRequest =
            crate::native::exact_candid::decode(bytes, 4096, 64, 100_000)?;
        let authority = CompletionAuthority::new(
            service,
            namespace.try_into().map_err(|_| Failure::Arguments)?,
            verifier,
        )
        .map_err(|_| Failure::Arguments)?;
        let argument = reply::inspection_request(authority, statement.permission).map_err(
            |error| match error {
                UploadAttestationReplyError::Invalid => Failure::Arguments,
                _ => failure(error),
            },
        )?;
        if ![
            verifier,
            statement.permission.upload.tenant,
            statement.permission.uploader,
        ]
        .contains(&actor)
        {
            return Err(Failure::Denied);
        }
        Ok(Self {
            authority,
            statement,
            argument,
        })
    }

    pub fn output(
        &self,
        bytes: &[u8],
        actor: Principal,
        network: &str,
        url: &str,
    ) -> Result<Value, Failure> {
        Ok(self.inspect(bytes, actor, network, url)?.report)
    }

    pub fn inspect(
        &self,
        bytes: &[u8],
        actor: Principal,
        network: &str,
        url: &str,
    ) -> Result<RecoveryInspection, Failure> {
        let response = reply::inspection(
            self.authority,
            self.statement.permission,
            bytes,
            4096.try_into().expect("positive bound"),
        )
        .map_err(failure)?;
        let matched = matches!(response.attestation,
            UploadAttestationLookup::Found(receipt) if receipt.request == self.statement);
        let (outcome, receipt) = match response.attestation {
            UploadAttestationLookup::Absent => ("absent", Value::Null),
            UploadAttestationLookup::Found(receipt) => (
                if matched { "matched" } else { "conflict" },
                json!({
                    "verifier":receipt.verifier.to_text(),
                    "content_digest":digest(receipt.request.content_digest),
                    "observed_at_ns":receipt.request.observed_at_ns.to_string(),
                    "accepted_at_ns":receipt.accepted_at_ns.to_string(),
                }),
            ),
        };
        let p = self.statement.permission;
        Ok(RecoveryInspection {
            matched,
            report: json!({
                "schema":1,"observation":"upload_attestation","actor":actor.to_text(),
                "network":network,"url":url,"authentication":"query_signatures",
                "expected_verifier":self.authority.verifier().to_text(),
                "upload":upload_json(p.upload),
                "uploader":p.uploader.to_text(),"expires_at_ns":p.expires_at_ns.to_string(),
                "expected_statement":{"content_digest":digest(self.statement.content_digest),
                    "observed_at_ns":self.statement.observed_at_ns.to_string()},
                "outcome":outcome,"receipt":receipt,"fenced":response.fenced,
                "retry_authorized":false,"current_availability":"not_observed",
                "reference_liveness":"not_observed","billing_cessation":"not_established",
            }),
        })
    }
}
fn digest(bytes: [u8; 32]) -> String {
    ContentDigest::try_from(bytes.as_slice())
        .expect("fixed digest")
        .to_string()
}

#[cfg(test)]
mod tests;
