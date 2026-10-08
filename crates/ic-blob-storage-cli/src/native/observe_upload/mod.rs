//! One bounded provider observation, retained before any future attestation dispatch.
pub(super) mod record;
use super::Failure;
use super::arguments::Options;
use super::artifacts::FailureRecord;
use super::artifacts::Run;
use super::identity;
use super::provider_download;
use super::provider_download::ExpectedBody;
use super::query;
use super::read;
use super::upload_setup::manifest_reply_limits;
use candid::Principal;
use ic_blob_storage_contracts::download::scope::CaffeineDownloadScope;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationRequest;
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::protocol::UPLOAD_VERIFICATION_PLAN_METHOD;
use ic_blob_storage_contracts::provider::download::request_target;
use ic_blob_storage_contracts::upload::completion::CompletionAuthority;
use ic_blob_storage_contracts::upload::completion::reply::UploadAttestationReplyError;
use ic_blob_storage_contracts::upload::completion::reply::inspection_request;
use ic_blob_storage_contracts::upload::completion::verification;
use serde_json::Value;
use std::{
    num::NonZeroU64,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use url::Url;

#[cfg(test)]
mod tests;

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
    let permission: UploadAdmissionRequest =
        crate::native::exact_candid::decode(&bytes, 4096, 32, 100_000)?;
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
    let run = Run::create(&input.directory)?;
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
                    include_str!("../provider_download/mod.rs"),
                    include_str!("../upload_setup/mod.rs"),
                    include_str!("../artifacts/mod.rs"),
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
            &FailureRecord {
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
    run: &Run,
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
        manifest_reply_limits(input.max_bytes),
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
    let digest = download_plan(&target, options.network, &plan, input.max_bytes, run).await?;
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

async fn download_plan(
    target: &Url,
    network: &str,
    plan: &ic_blob_storage_contracts::dto::upload::completion::UploadVerificationPlan,
    maximum: NonZeroU64,
    run: &Run,
) -> Result<ContentDigest, Failure> {
    let headers: Vec<_> = plan
        .declaration
        .headers
        .iter()
        .map(
            |h| ic_blob_storage_contracts::identity::caffeine::CaffeineHeader {
                name: &h.name,
                value: &h.value,
            },
        )
        .collect();
    let expected = ExpectedBody {
        root: ProviderRootHash::try_from(plan.permission.upload.root.as_slice())
            .expect("fixed root"),
        bytes: plan.permission.upload.bytes,
        headers: &headers,
        maximum,
    };
    provider_download::fetch(target, network, &expected, run, &mut std::io::sink()).await
}
