//! Optional native verified downloads. No tenant identity, attestation or publication policy.
//!
//! Expectations must come from trusted application state, not the HTTP response.
//! Success describes one observed body, never future availability or a catalog commit.
use ic_blob_storage_contracts::identity::{
    ContentDigest, ProviderRootHash,
    caffeine::{
        CaffeineContentHashes, CaffeineHashError, CaffeineHashLimits, CaffeineHeader,
        verification::CaffeineRootVerifier,
    },
};
use reqwest::{Response, Url};
use std::{
    io::{self, Write},
    path::PathBuf,
};
#[cfg(unix)]
use std::{path::Path, time::Duration};

/// A single-use, bounded verification plan. Construction performs no I/O.
pub struct Verification {
    verifier: CaffeineRootVerifier,
    bytes: u64,
    checksum: Option<ContentDigest>,
    frame_bytes: usize,
}
impl Verification {
    /// Bind trusted original metadata and an optional application checksum.
    /// The checksum is raw SHA-256 (for Cargo, the `.crate` archive checksum),
    /// not the metadata-bound provider root. Limits are explicitly caller-owned.
    /// # Errors
    /// Rejects invalid or over-budget original content metadata.
    pub fn new(
        root: ProviderRootHash,
        bytes: u64,
        headers: &[CaffeineHeader<'_>],
        checksum: Option<ContentDigest>,
        limits: CaffeineHashLimits,
    ) -> Result<Self, CaffeineHashError> {
        Ok(Self {
            verifier: CaffeineRootVerifier::new(root, bytes, headers, limits)?,
            bytes,
            checksum,
            frame_bytes: limits.max_append_bytes.get(),
        })
    }

    /// Consume an already requested response, hashing the same buffers written to `sink`.
    /// Requires 200, identity encoding, no ranges, successful transport EOF, exact
    /// length/root and the supplied checksum. Response headers never replace metadata.
    /// Callers own request/redirect/retry/decompression policy and the deadline;
    /// disable decompression before sending. Keep `sink` private until success.
    /// `received` resets when first polled and retains yielded-byte progress on
    /// failure/cancellation; it is not verified progress or retry authority.
    /// Request transport may allocate its own chunks; this bounds accepted content
    /// and hashing frames, not the HTTP implementation's allocation granularity.
    /// # Errors
    /// Preserves HTTP admission, transport, integrity and sink failures. Partial
    /// sink bytes remain unverified; never publish them after an error or cancellation.
    pub async fn read_response(
        mut self,
        mut response: Response,
        sink: &mut impl Write,
        received: &mut u64,
    ) -> Result<CaffeineContentHashes, DownloadError> {
        *received = 0;
        if response.status() != reqwest::StatusCode::OK
            || response
                .headers()
                .contains_key(reqwest::header::CONTENT_RANGE)
            || response
                .headers()
                .get_all(reqwest::header::CONTENT_ENCODING)
                .iter()
                .any(|v| v != "identity")
        {
            return Err(DownloadError::Response);
        }
        if response.content_length().is_some_and(|n| n != self.bytes) {
            return Err(DownloadError::Length);
        }
        while let Some(chunk) = response.chunk().await? {
            *received = received
                .checked_add(u64::try_from(chunk.len()).map_err(|_| DownloadError::Length)?)
                .ok_or(DownloadError::Length)?;
            if *received > self.bytes {
                return Err(DownloadError::Length);
            }
            for frame in chunk.chunks(self.frame_bytes) {
                self.verifier
                    .append(self.verifier.received_bytes(), frame)?;
                sink.write_all(frame)?;
            }
        }
        sink.flush()?;
        let hashes = self.verifier.finish()?;
        if self
            .checksum
            .is_some_and(|expected| expected != hashes.content_digest)
        {
            return Err(DownloadError::Checksum);
        }
        Ok(hashes)
    }
}

/// Native observation failures, not provider-effect or retry classifications.
#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    /// URL/deadline is invalid; no directory or request was created.
    #[error("invalid download URL or deadline")]
    Input,
    /// Status, range or encoding does not describe a complete identity body.
    #[error("unusable HTTP response")]
    Response,
    /// Declared or streamed byte length differs from the original expectation.
    #[error("download length mismatch")]
    Length,
    /// The checked body differs from the application's expected raw SHA-256.
    #[error("download checksum mismatch")]
    Checksum,
    /// The request or body transport failed; no retry is performed.
    #[error("download transport failed")]
    Transport(#[from] reqwest::Error),
    /// Original root, length or hashing-budget verification failed.
    #[error(transparent)]
    Integrity(#[from] CaffeineHashError),
    /// Filesystem/staging/publication failure. Reconcile the retained directory;
    /// publication sync can fail after the complete `body.bin` becomes visible.
    #[error("download filesystem operation failed")]
    Io(#[from] io::Error),
    /// Directory creation failed; preserve whether creation already occurred.
    #[error(transparent)]
    Directory(#[from] ic_host_fs::durable::DirectoryCreationError),
    /// Preserve Host's before/after-publication and cleanup evidence.
    #[error(transparent)]
    Publication(#[from] ic_host_fs::durable::NamedWriteError<std::convert::Infallible>),
    /// The async deadline expired. Any retained partial body remains unverified.
    #[error("download deadline expired")]
    Timeout,
}

/// One successful read observation. Paths are not durable integrity capabilities:
/// the owner must protect the private directory against later mutation/replacement.
#[derive(Debug)]
pub struct VerifiedDownload {
    /// Create-only published verified output inside the supplied private directory.
    pub path: PathBuf,
    /// Both digests computed from exactly the returned file's downloaded bytes.
    pub hashes: CaffeineContentHashes,
    /// Original and verified body length.
    pub bytes: u64,
}

/// Download once into a fresh private directory; publish only fully checked bytes.
///
/// The caller approves the URL and authenticates the verification expectations.
/// HTTPS is required except explicitly enabled loopback HTTP. Credentials and
/// fragments refuse. Redirects, automatic retries and decompression are disabled;
/// loopback requests bypass proxies. No IC call or tenant impersonation is used.
///
/// `directory` is a permanent create-new claim, including failed/cancelled runs.
/// Its parent must exist and remain caller-controlled. Partial `body.part` remains
/// on failure; success syncs and publishes `body.bin` without replacement via Host FS.
/// Do not retry using a replacement directory merely because this operation failed.
/// Retain application intent/evidence separately; this creates no receipt protocol.
///
/// The deadline covers the async send/read/verification future. Synchronous filesystem and hashing
/// work is not preemptible. Dropping the future leaves no verified result.
/// # Errors
/// Preserves transport/verification/I/O failures; an existing directory refuses
/// before any request. I/O failure after publication needs inspection, not resend.
#[cfg(unix)]
pub async fn download(
    url: &Url,
    directory: &Path,
    verification: Verification,
    timeout: Duration,
    allow_loopback_http: bool,
) -> Result<VerifiedDownload, DownloadError> {
    use std::{ffi::OsStr, os::fd::AsFd};
    let loopback = matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"));
    if !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
        || !(url.scheme() == "https" || (url.scheme() == "http" && loopback && allow_loopback_http))
        || timeout.is_zero()
        || tokio::time::Instant::now().checked_add(timeout).is_none()
    {
        return Err(DownloadError::Input);
    }
    let mut builder = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .no_zstd();
    if loopback {
        builder = builder.no_proxy();
    }
    let client = builder.build()?;
    let held = ic_host_fs::durable::create_private_directory(directory)?;
    let mut body =
        ic_host_fs::durable::create_retained_at(held.as_fd(), OsStr::new("body.part"), 0o600)?;
    let bytes = verification.bytes;
    let hashes = tokio::time::timeout(timeout, async {
        let response = client
            .get(url.clone())
            .header(reqwest::header::ACCEPT_ENCODING, "identity")
            .send()
            .await?;
        verification
            .read_response(response, &mut body, &mut 0)
            .await
    })
    .await
    .map_err(|_| DownloadError::Timeout)??;
    ic_host_fs::durable::admit_directory(directory, &held)?;
    ic_host_fs::durable::publish_retained_at(
        held.as_fd(),
        OsStr::new("body.part"),
        OsStr::new("body.bin"),
        body,
    )?;
    Ok(VerifiedDownload {
        path: directory.join("body.bin"),
        hashes,
        bytes,
    })
}

#[cfg(all(test, unix))]
mod tests;
