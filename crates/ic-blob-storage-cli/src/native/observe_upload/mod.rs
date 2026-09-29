//! One bounded provider observation, retained before any future attestation dispatch.
mod download;
mod record;
#[cfg(test)]
mod tests;
use super::{Failure, arguments::Options, identity, query, read};
use candid::{Principal, de::DecoderConfig, decode_one_with_config};
use ic_blob_storage::{
    dto::upload::{admission::UploadAdmissionRequest, completion::UploadAttestationRequest},
    model::{
        identity::{ContentDigest, ProviderRootHash, caffeine::manifest::CaffeineManifestLimits},
        service::{read::download::CaffeineDownloadScope, upload::completion::CompletionAuthority},
    },
    ops::{
        caffeine::download::request_target,
        service::uploads::{
            completion::{
                reply::{UploadAttestationReplyError, inspection_request},
                verification::{self, UPLOAD_VERIFICATION_PLAN_METHOD},
            },
            manifests::reply::UploadManifestReplyLimits,
        },
    },
};
use serde_json::Value;
use std::{
    num::NonZeroU64,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use url::Url;

pub(super) struct Input {
    pub service: Principal,
    pub namespace: u128,
    pub permission: PathBuf,
    pub gateway: Url,
    pub directory: PathBuf,
    pub max_bytes: NonZeroU64,
}
fn now() -> Result<u64, Failure> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Failure::Clock)?
        .as_nanos()
        .try_into()
        .map_err(|_| Failure::Clock)
}
fn permission(input: &Input) -> Result<UploadAdmissionRequest, Failure> {
    let bytes = read(&input.permission, 4096)?;
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(100_000)
        .set_skipping_quota(0)
        .set_max_type_len(32)
        .set_max_header_len(4096)
        .set_full_error_message(false);
    let permission: UploadAdmissionRequest =
        decode_one_with_config(&bytes, &config).map_err(|_| Failure::Arguments)?;
    if permission.upload.bytes > input.max_bytes.get() {
        return Err(Failure::Arguments);
    }
    Ok(permission)
}
fn error(e: UploadAttestationReplyError) -> Failure {
    match e {
        UploadAttestationReplyError::Limit => Failure::ReplyLimit,
        UploadAttestationReplyError::Binding => Failure::Binding,
        UploadAttestationReplyError::Invalid => Failure::InvalidReply,
        UploadAttestationReplyError::Remote(_) => Failure::VerificationRefused,
    }
}
pub(super) async fn run(options: &Options, input: &Input) -> Result<Value, Failure> {
    let permission = permission(input)?;
    let authority = CompletionAuthority::new(
        input.service,
        input.namespace.try_into().map_err(|_| Failure::Arguments)?,
        options.actor,
    )
    .map_err(|_| Failure::Arguments)?;
    let argument = inspection_request(authority, permission).map_err(|e| match e {
        UploadAttestationReplyError::Invalid => Failure::Arguments,
        other => error(other),
    })?;
    identity(&options.identity, options.actor)?;
    let run = record::Run::create(&input.directory)?;
    run.bytes("permission.candid", &argument)?;
    run.json(
        "plan.json",
        &record::ObservationPlanRecord {
            schema: 1,
            runner_version: env!("CARGO_PKG_VERSION"),
            runner_source_sha256: ContentDigest::compute(
                concat!(
                    include_str!("mod.rs"),
                    include_str!("record/mod.rs"),
                    include_str!("download/mod.rs"),
                    include_str!("../mod.rs"),
                    include_str!("../arguments/mod.rs"),
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
            gateway_origin: input.gateway.as_str(),
            max_content_bytes: input.max_bytes.to_string(),
            max_provider_requests: 1,
            request_deadline_seconds: 30,
            // A loopback origin can forward to a real provider; network mode is not billing evidence.
            provider_charges: "unknown",
            started_at_ns: now()?.to_string(),
        },
    )?;
    let result = capture(options, input, authority, permission, argument, &run).await;
    if let Err(failure) = result {
        run.json(
            "failure.json",
            &record::FailureRecord {
                error: failure.code(),
            },
        )?;
    }
    result
}

async fn capture(
    options: &Options,
    input: &Input,
    authority: CompletionAuthority,
    permission: UploadAdmissionRequest,
    argument: Vec<u8>,
    run: &record::Run,
) -> Result<Value, Failure> {
    let bytes = query(
        options,
        input.service,
        UPLOAD_VERIFICATION_PLAN_METHOD,
        argument,
    )
    .await?;
    if bytes.len() > 64 * 1024 {
        return Err(Failure::ReplyLimit);
    }
    run.bytes("service-response.candid", &bytes)?;
    let plan = verification::reply::decode(
        authority,
        permission,
        &bytes,
        UploadManifestReplyLimits {
            max_reply_bytes: (64 * 1024).try_into().unwrap(),
            declaration: CaffeineManifestLimits {
                max_content_bytes: input.max_bytes,
                max_chunks: 1024.try_into().unwrap(),
                max_headers: 16.try_into().unwrap(),
                max_header_bytes: 4096.try_into().unwrap(),
            },
        },
    )
    .map_err(error)?;
    let scope = CaffeineDownloadScope::new(plan.owner, authority.namespace(), &plan.project)
        .map_err(|_| Failure::InvalidReply)?;
    let root = ProviderRootHash::try_from(permission.upload.root.as_slice()).expect("fixed root");
    let target = input
        .gateway
        .join(&request_target(&scope, root))
        .map_err(|_| Failure::Arguments)?;
    run.json(
        "download-request.json",
        &record::DownloadRequestRecord {
            method: "GET",
            url: target.as_str(),
            service_reply_sha256: ContentDigest::compute(&bytes).to_string(),
            owner: plan.owner.to_text(),
            project: &plan.project,
            bytes: permission.upload.bytes.to_string(),
            started_at_ns: now()?.to_string(),
        },
    )?;
    let digest = download::fetch(&target, options.network, &plan, input.max_bytes, run).await?;
    let observed_at_ns = now()?;
    if observed_at_ns < plan.admitted_at_ns {
        return Err(Failure::Clock);
    }
    let statement = UploadAttestationRequest {
        permission,
        content_digest: *digest.as_bytes(),
        observed_at_ns,
    };
    let encoded = candid::encode_one(statement).map_err(|_| Failure::File)?;
    run.bytes("statement.candid", &encoded)?;
    let report = record::ObservationSummaryRecord {
        schema: 1,
        observation: "provider_content",
        network: options.network,
        service: input.service.to_text(),
        namespace: input.namespace.to_string(),
        verifier: options.actor.to_text(),
        owner: plan.owner.to_text(),
        project: &plan.project,
        gateway_origin: input.gateway.as_str(),
        root: root.to_string(),
        bytes: permission.upload.bytes.to_string(),
        content_digest: digest.to_string(),
        observed_at_ns: observed_at_ns.to_string(),
        statement_sha256: ContentDigest::compute(&encoded).to_string(),
        authentication: "query_signatures",
        attestation_dispatched: false,
        retry_authorized: false,
        future_retention: "not_established",
        billing_cessation: "not_established",
    };
    run.json("summary.json", &report)?;
    serde_json::to_value(report).map_err(|_| Failure::File)
}
