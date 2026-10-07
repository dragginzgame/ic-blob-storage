//! Provider body verification; original service metadata always owns the root.
mod headers;
use super::{
    Failure,
    artifacts::{DownloadOutcomeRecord, HttpResponseRecord, Run},
};
use ic_blob_storage::model::identity::{
    ContentDigest, ProviderRootHash,
    caffeine::{CaffeineHashLimits, CaffeineHeader, verification::CaffeineRootVerifier},
};
use std::{io::Write, num::NonZeroU64, time::Duration};
use url::Url;
#[cfg(test)]
mod tests;

pub(super) struct ExpectedBody<'a> {
    pub root: ProviderRootHash,
    pub bytes: u64,
    pub headers: &'a [CaffeineHeader<'a>],
    pub maximum: NonZeroU64,
}

pub(super) async fn fetch(
    url: &Url,
    network: &str,
    expected: &ExpectedBody<'_>,
    run: &Run,
    sink: &mut impl Write,
) -> Result<ContentDigest, Failure> {
    let mut received = 0_u64;
    let result = tokio::time::timeout(
        Duration::from_secs(30),
        body(url, network, expected, run, sink, &mut received),
    )
    .await
    .map_err(|_| Failure::Timeout)
    .and_then(std::convert::identity);
    run.json(
        "download-outcome.json",
        &DownloadOutcomeRecord {
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
    expected: &ExpectedBody<'_>,
    run: &Run,
    sink: &mut impl Write,
    received: &mut u64,
) -> Result<ContentDigest, Failure> {
    let mut verifier = CaffeineRootVerifier::new(
        expected.root,
        expected.bytes,
        expected.headers,
        CaffeineHashLimits {
            max_content_bytes: expected.maximum,
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
        &HttpResponseRecord {
            status: response.status().as_u16(),
        },
    )?;
    // Native HTTP observations are evidence, never replacement hash metadata or
    // browser-serving acceptance. Keep the existing attestation input unchanged.
    run.json(
        "http-response-headers.json",
        &headers::record(response.headers()),
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
        .is_some_and(|size| size != expected.bytes)
    {
        return Err(Failure::Content);
    }
    while let Some(chunk) = response.chunk().await.map_err(|_| Failure::Transport)? {
        *received = received
            .checked_add(chunk.len().try_into().map_err(|_| Failure::Content)?)
            .ok_or(Failure::Content)?;
        if *received > expected.bytes {
            return Err(Failure::Content);
        }
        for frame in chunk.chunks(64 * 1024) {
            verifier
                .append(verifier.received_bytes(), frame)
                .map_err(|_| Failure::Content)?;
            sink.write_all(frame).map_err(|_| Failure::File)?;
        }
    }
    sink.flush().map_err(|_| Failure::File)?;
    verifier
        .finish()
        .map(|h| h.content_digest)
        .map_err(|_| Failure::Content)
}
