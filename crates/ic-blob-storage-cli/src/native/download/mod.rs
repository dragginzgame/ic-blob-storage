//! Tenant-owned verified output: one replicated descriptor, then one bounded GET.
use super::{
    Failure, agent,
    arguments::Options,
    artifacts::{FailureRecord, Run},
    identity,
    provider_download::{self, ExpectedBody},
    read,
};
use candid::{Principal, de::DecoderConfig, decode_one_with_config};
use ic_blob_storage::{
    dto::download::{DownloadFailure as E, DownloadRequest, DownloadResponse},
    model::{
        identity::{ContentDigest, ProviderRootHash, caffeine::CaffeineHeader},
        service::read::download::CaffeineDownloadScope,
    },
    ops::{
        caffeine::download::request_target,
        service::reads::download::{
            DOWNLOAD_METHOD,
            reply::{self, DownloadReplyError, DownloadReplyLimits},
        },
    },
};
use serde_json::{Value, json};
use std::{num::NonZeroU64, path::PathBuf, time::Duration};
use url::Url;
#[cfg(test)]
mod tests;

pub(super) struct Input {
    pub service: Principal,
    pub namespace: u128,
    pub request: PathBuf,
    pub project: String,
    pub gateway: Url,
    pub directory: PathBuf,
    pub max_bytes: NonZeroU64,
}
fn open(
    input: &Input,
    actor: Principal,
) -> Result<(DownloadRequest, CaffeineDownloadScope), Failure> {
    let bytes = read(&input.request, 4096)?;
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(100_000)
        .set_skipping_quota(0)
        .set_max_type_len(64)
        .set_max_header_len(4096)
        .set_full_error_message(false);
    let request: DownloadRequest =
        decode_one_with_config(&bytes, &config).map_err(|_| Failure::Arguments)?;
    if request.service != input.service
        || request.namespace != input.namespace
        || request.tenant != actor
    {
        return Err(Failure::Binding);
    }
    if [
        request.namespace,
        request.object,
        request.incarnation,
        request.reference,
    ]
    .contains(&0)
    {
        return Err(Failure::Arguments);
    }
    let scope = CaffeineDownloadScope::new(
        input.service,
        input.namespace.try_into().map_err(|_| Failure::Arguments)?,
        &input.project,
    )
    .map_err(|_| Failure::Arguments)?;
    Ok((request, scope))
}
fn decode(
    input: &Input,
    request: DownloadRequest,
    scope: &CaffeineDownloadScope,
    bytes: &[u8],
) -> Result<DownloadResponse, Failure> {
    reply::decode(
        request,
        scope,
        bytes,
        DownloadReplyLimits {
            max_reply_bytes: 4096.try_into().unwrap(),
            max_content_bytes: input.max_bytes,
            max_headers: 16.try_into().unwrap(),
            max_header_bytes: 4096.try_into().unwrap(),
        },
    )
    .map_err(|error| match error {
        DownloadReplyError::Binding => Failure::Binding,
        DownloadReplyError::Limit => Failure::ReplyLimit,
        DownloadReplyError::Invalid => Failure::InvalidReply,
        DownloadReplyError::Remote(e) => Failure::DownloadRefused(e),
    })
}
pub(super) async fn run(options: &Options, input: &Input) -> Result<Value, Failure> {
    let (request, scope) = open(input, options.actor)?;
    identity(&options.identity, options.actor)?;
    let run = Run::create(&input.directory)?;
    let argument = candid::encode_one(request).map_err(|_| Failure::Arguments)?;
    run.bytes("request.candid", &argument)?;
    run.json("plan.json", &json!({"schema":1,"operation":"tenant_download",
        "network":options.network,"service_url":options.url.as_str(),"actor":options.actor.to_text(),
        "service":input.service.to_text(),"namespace":input.namespace.to_string(),
        "project":input.project,"gateway_origin":input.gateway.as_str(),
        "max_content_bytes":input.max_bytes.to_string(),"max_provider_requests":1,
        "descriptor_method":DOWNLOAD_METHOD,"request_deadline_seconds":30,
        "provider_charges":"unknown","retry_authorized":false}))?;
    let result = capture(options, input, request, &scope, argument, &run).await;
    if let Err(error) = result {
        run.json(
            "failure.json",
            &FailureRecord {
                error: error.code(),
            },
        )?;
    }
    result
}
async fn capture(
    options: &Options,
    input: &Input,
    request: DownloadRequest,
    scope: &CaffeineDownloadScope,
    argument: Vec<u8>,
    run: &Run,
) -> Result<Value, Failure> {
    let agent = agent(options)?;
    let bytes = tokio::time::timeout(
        Duration::from_secs(30),
        agent
            .update(&input.service, DOWNLOAD_METHOD)
            .with_arg(argument)
            .call_and_wait(),
    )
    .await
    .map_err(|_| Failure::Timeout)?
    .map_err(|_| Failure::Transport)?;
    if bytes.len() > 4096 {
        return Err(Failure::ReplyTooLarge);
    }
    run.bytes("service-response.candid", &bytes)?;
    let descriptor = decode(input, request, scope, &bytes)?;
    let root = ProviderRootHash::try_from(request.root.as_slice()).expect("fixed root");
    let url = input
        .gateway
        .join(&request_target(scope, root))
        .map_err(|_| Failure::Arguments)?;
    run.json("download-request.json", &json!({"method":"GET","url":url.as_str(),
        "service_reply_sha256":ContentDigest::compute(&bytes).to_string(),
        "owner":descriptor.owner.to_text(),"project":descriptor.project,"bytes":descriptor.bytes.to_string()}))?;
    let headers: Vec<_> = descriptor
        .headers
        .iter()
        .map(|h| CaffeineHeader {
            name: &h.name,
            value: &h.value,
        })
        .collect();
    let expected = ExpectedBody {
        root,
        bytes: descriptor.bytes,
        headers: &headers,
        maximum: input.max_bytes,
    };
    let mut body = run.open_body()?;
    let digest = provider_download::fetch(&url, options.network, &expected, run, &mut body).await?;
    body.sync_all().map_err(|_| Failure::File)?;
    drop(body);
    let output = run.publish_body()?;
    let summary = json!({"schema":1,"observation":"verified_tenant_download",
        "descriptor_authentication":"ic_update_certificate","actor":options.actor.to_text(),
        "request":{"service":request.service.to_text(),"tenant":request.tenant.to_text(),
            "namespace":request.namespace.to_string(),"object":request.object.to_string(),
            "incarnation":request.incarnation.to_string(),"reference":request.reference.to_string(),"root":root.to_string()},
        "owner":descriptor.owner.to_text(),"project":descriptor.project,"bytes":descriptor.bytes.to_string(),
        "headers":descriptor.headers.iter().map(|h|json!({"name":h.name,"value":h.value})).collect::<Vec<_>>(),
        "content_digest":digest.to_string(),"body":output,"verified":true,"provider_requests":1,
        "publication_authorized":false,"retry_authorized":false,"future_retention":"not_established",
        "billing_cessation":"not_established"});
    run.json("summary.json", &summary)?;
    Ok(summary)
}
pub(super) const fn refusal_code(error: E) -> &'static str {
    match error {
        E::Invalid => "download_invalid",
        E::Denied => "download_denied",
        E::Binding => "download_binding",
        E::Unavailable => "download_unavailable",
        E::Inactive => "download_inactive",
        E::Fenced => "download_fenced",
        E::Internal => "download_internal",
    }
}
