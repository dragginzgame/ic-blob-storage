//! Provider body verification; original service metadata always owns the root.
mod headers;
use super::{
    Failure,
    artifacts::{DownloadOutcomeRecord, HttpResponseRecord, Run},
};
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHeader;
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
    let verification = ic_blob_storage_client::native::Verification::new(
        expected.root,
        expected.bytes,
        expected.headers,
        None,
        ic_blob_storage_contracts::identity::caffeine::CaffeineHashLimits {
            max_content_bytes: expected.maximum,
            max_append_bytes: super::local_body::FRAME.try_into().expect("positive frame"),
            max_headers: 16.try_into().expect("positive header count"),
            max_header_bytes: 4096.try_into().expect("positive header bytes"),
        },
    )
    .map_err(|_| Failure::InvalidReply)?;
    let client = super::http::client_builder(network)
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .no_zstd();
    let response = client
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
    // browser-serving acceptance. Keep the original expected body metadata unchanged.
    run.json(
        "http-response-headers.json",
        &headers::record(response.headers()),
    )?;
    verification
        .read_response(response, sink, received)
        .await
        .map(|hashes| hashes.content_digest)
        .map_err(|error| match error {
            ic_blob_storage_client::native::DownloadError::Response => Failure::ProviderResponse,
            ic_blob_storage_client::native::DownloadError::Transport(_) => Failure::Transport,
            ic_blob_storage_client::native::DownloadError::Io(_)
            | ic_blob_storage_client::native::DownloadError::Directory(_)
            | ic_blob_storage_client::native::DownloadError::Publication(_) => Failure::File,
            ic_blob_storage_client::native::DownloadError::Timeout => Failure::Timeout,
            ic_blob_storage_client::native::DownloadError::Input => Failure::Arguments,
            ic_blob_storage_client::native::DownloadError::Length
            | ic_blob_storage_client::native::DownloadError::Checksum
            | ic_blob_storage_client::native::DownloadError::Integrity(_) => Failure::Content,
        })
}
