//! Compare saved intent to immutable service history without resending it.
use super::{Failure, read};
use candid::{Principal, de::DecoderConfig, decode_one_with_config};
use ic_blob_storage::{
    dto::upload::completion::{UploadAttestationLookup, UploadAttestationRequest},
    model::{
        identity::{ContentDigest, ProviderRootHash},
        service::upload::completion::CompletionAuthority,
    },
    ops::service::uploads::completion::reply::{self, UploadAttestationReplyError},
};
use serde_json::{Value, json};
use std::path::Path;

pub(super) struct Recovery {
    pub(super) authority: CompletionAuthority,
    pub(super) statement: UploadAttestationRequest,
    argument: Vec<u8>,
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
        if bytes.len() > 4096 {
            return Err(Failure::Arguments);
        }
        let mut config = DecoderConfig::new();
        config
            .set_decoding_quota(100_000)
            .set_skipping_quota(0)
            .set_max_type_len(64)
            .set_max_header_len(4096)
            .set_full_error_message(false);
        let statement: UploadAttestationRequest =
            decode_one_with_config(bytes, &config).map_err(|_| Failure::Arguments)?;
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
        let response = reply::inspection(
            self.authority,
            self.statement.permission,
            bytes,
            4096.try_into().expect("positive bound"),
        )
        .map_err(failure)?;
        let (outcome, receipt) = match response.attestation {
            UploadAttestationLookup::Absent => ("absent", Value::Null),
            UploadAttestationLookup::Found(receipt) => (
                if receipt.request == self.statement {
                    "matched"
                } else {
                    "conflict"
                },
                json!({
                    "verifier":receipt.verifier.to_text(),
                    "content_digest":digest(receipt.request.content_digest),
                    "observed_at_ns":receipt.request.observed_at_ns.to_string(),
                    "accepted_at_ns":receipt.accepted_at_ns.to_string(),
                }),
            ),
        };
        let p = self.statement.permission;
        let upload = p.upload;
        Ok(json!({
            "schema":1,"observation":"upload_attestation","actor":actor.to_text(),
            "network":network,"url":url,"authentication":"query_signatures",
            "expected_verifier":self.authority.verifier().to_text(),
            "upload":{"service":upload.service.to_text(),"namespace":upload.namespace.to_string(),
                "tenant":upload.tenant.to_text(),"upload":upload.upload.to_string(),
                "object":upload.object.to_string(),"incarnation":upload.incarnation.to_string(),
                "first_reference":upload.first_reference.to_string(),
                "root":ProviderRootHash::try_from(upload.root.as_slice()).expect("fixed root").to_string(),
                "bytes":upload.bytes.to_string()},
            "uploader":p.uploader.to_text(),"expires_at_ns":p.expires_at_ns.to_string(),
            "expected_statement":{"content_digest":digest(self.statement.content_digest),
                "observed_at_ns":self.statement.observed_at_ns.to_string()},
            "outcome":outcome,"receipt":receipt,"fenced":response.fenced,
            "retry_authorized":false,"current_availability":"not_observed",
            "reference_liveness":"not_observed","billing_cessation":"not_established",
        }))
    }
}
fn digest(bytes: [u8; 32]) -> String {
    ContentDigest::try_from(bytes.as_slice())
        .expect("fixed digest")
        .to_string()
}

#[cfg(test)]
mod tests;
