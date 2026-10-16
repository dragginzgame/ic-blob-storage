//! Verify the same open file buffers that a snapshot saves; no source reopen.
mod source;
use super::Failure;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestDeclaration;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::identity::caffeine::CaffeineContentHashes;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHashLimits;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHeader;
use ic_blob_storage_contracts::identity::caffeine::verification::CaffeineRootVerifier;
pub(super) use source::BodySource;
use std::{
    fs::File,
    io::{Read, Write},
    num::NonZeroU64,
    path::Path,
};

pub(super) const FRAME: usize = 64 * 1024;

pub(super) struct LocalBody {
    file: File,
    bytes: u64,
}
impl LocalBody {
    pub fn open(path: &Path, bytes: u64) -> Result<Self, Failure> {
        let file = ic_host_fs::read::open_regular_file(path).map_err(|_| Failure::File)?;
        Self::from_file(file, bytes)
    }

    pub(super) fn from_file(file: File, bytes: u64) -> Result<Self, Failure> {
        let metadata = file.metadata().map_err(|_| Failure::File)?;
        if metadata.len() != bytes {
            return Err(Failure::Content);
        }
        Ok(Self { file, bytes })
    }

    /// EOF and the original root must both match before callers publish the sink.
    pub fn verify(
        mut self,
        root: ProviderRootHash,
        declaration: &UploadManifestDeclaration,
        maximum: NonZeroU64,
        sink: &mut impl Write,
    ) -> Result<CaffeineContentHashes, Failure> {
        let headers = ic_blob_storage_contracts::upload::manifests::headers(declaration);
        let mut verifier = verifier(root, self.bytes, &headers, maximum)?;
        let mut frame = vec![0; FRAME];
        loop {
            let count = match self.file.read(&mut frame) {
                Ok(0) => break,
                Ok(count) => count,
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_) => return Err(Failure::File),
            };
            verifier
                .append(verifier.received_bytes(), &frame[..count])
                .map_err(|_| Failure::Content)?;
            sink.write_all(&frame[..count]).map_err(|_| Failure::File)?;
        }
        verifier.finish().map_err(|_| Failure::Content)
    }
}

/// The CLI's bounded verifier is shared by local snapshots and provider reads.
pub(super) fn verifier(
    root: ProviderRootHash,
    bytes: u64,
    headers: &[CaffeineHeader<'_>],
    maximum: NonZeroU64,
) -> Result<CaffeineRootVerifier, Failure> {
    CaffeineRootVerifier::new(
        root,
        bytes,
        headers,
        CaffeineHashLimits {
            max_content_bytes: maximum,
            max_append_bytes: FRAME.try_into().expect("positive frame bound"),
            max_headers: 16.try_into().expect("positive header bound"),
            max_header_bytes: 4096.try_into().expect("positive metadata bound"),
        },
    )
    .map_err(|_| Failure::InvalidReply)
}

#[cfg(test)]
mod tests;
