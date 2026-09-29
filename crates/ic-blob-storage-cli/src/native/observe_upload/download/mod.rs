//! Provider body verification; original service metadata always owns the root.
use super::{Failure, record};
use ic_blob_storage::{
    dto::upload::completion::UploadVerificationPlan,
    model::identity::{
        ContentDigest, ProviderRootHash,
        caffeine::{CaffeineHashLimits, CaffeineHeader, verification::CaffeineRootVerifier},
    },
};
use std::{num::NonZeroU64, time::Duration};
use url::Url;
#[cfg(test)]
mod tests;

pub(super) async fn fetch(
    url: &Url,
    network: &str,
    plan: &UploadVerificationPlan,
    maximum: NonZeroU64,
    run: &record::Run,
) -> Result<ContentDigest, Failure> {
    let mut received = 0_u64;
    let result = tokio::time::timeout(
        Duration::from_secs(30),
        body(url, network, plan, maximum, run, &mut received),
    )
    .await
    .map_err(|_| Failure::Timeout)
    .and_then(std::convert::identity);
    run.json(
        "download-outcome.json",
        &record::DownloadOutcomeRecord {
            received_bytes: received.to_string(),
            outcome: if result.is_ok() { "verified" } else { "failed" },
            error: result.as_ref().err().map(|e| e.code()),
        },
    )?;
    result
}
async fn body(
    url: &Url,
    network: &str,
    plan: &UploadVerificationPlan,
    maximum: NonZeroU64,
    run: &record::Run,
    received: &mut u64,
) -> Result<ContentDigest, Failure> {
    let headers: Vec<_> = plan
        .declaration
        .headers
        .iter()
        .map(|h| CaffeineHeader {
            name: &h.name,
            value: &h.value,
        })
        .collect();
    let mut verifier = CaffeineRootVerifier::new(
        ProviderRootHash::try_from(plan.permission.upload.root.as_slice()).expect("fixed root"),
        plan.permission.upload.bytes,
        &headers,
        CaffeineHashLimits {
            max_content_bytes: maximum,
            max_append_bytes: (64 * 1024).try_into().unwrap(),
            max_headers: 16.try_into().unwrap(),
            max_header_bytes: 4096.try_into().unwrap(),
        },
    )
    .map_err(|_| Failure::InvalidReply)?;
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .timeout(Duration::from_secs(30))
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .no_zstd();
    let client = if network == "local" {
        client.no_proxy()
    } else {
        client
    };
    let mut response = client
        .build()
        .map_err(|_| Failure::Transport)?
        .get(url.clone())
        .header(reqwest::header::ACCEPT_ENCODING, "identity")
        .send()
        .await
        .map_err(|_| Failure::Transport)?;
    run.json(
        "http-response.json",
        &record::HttpResponseRecord {
            status: response.status().as_u16(),
        },
    )?;
    if response.status() != reqwest::StatusCode::OK
        || response
            .headers()
            .contains_key(reqwest::header::CONTENT_RANGE)
        || response
            .headers()
            .get_all(reqwest::header::CONTENT_ENCODING)
            .iter()
            .any(|h| h != "identity")
    {
        return Err(Failure::ProviderResponse);
    }
    if response
        .content_length()
        .is_some_and(|size| size != plan.permission.upload.bytes)
    {
        return Err(Failure::Content);
    }
    while let Some(chunk) = response.chunk().await.map_err(|_| Failure::Transport)? {
        *received = received
            .checked_add(chunk.len().try_into().map_err(|_| Failure::Content)?)
            .ok_or(Failure::Content)?;
        if *received > plan.permission.upload.bytes {
            return Err(Failure::Content);
        }
        for frame in chunk.chunks(64 * 1024) {
            verifier
                .append(verifier.received_bytes(), frame)
                .map_err(|_| Failure::Content)?;
        }
    }
    verifier
        .finish()
        .map(|h| h.content_digest)
        .map_err(|_| Failure::Content)
}
