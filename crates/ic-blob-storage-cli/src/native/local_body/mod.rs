//! Verify the same open file buffers that a snapshot saves; no source reopen.
use super::Failure;
use ic_blob_storage::{
    dto::upload::manifest::UploadManifestDeclaration,
    model::identity::{
        ProviderRootHash,
        caffeine::{
            CaffeineContentHashes, CaffeineHashLimits, CaffeineHeader,
            verification::CaffeineRootVerifier,
        },
    },
};
use std::{
    fs::File,
    io::{Read, Write},
    num::NonZeroU64,
    path::Path,
};

const FRAME: usize = 64 * 1024;

pub(super) struct LocalBody {
    file: File,
    bytes: u64,
}
impl LocalBody {
    pub fn open(path: &Path, bytes: u64) -> Result<Self, Failure> {
        let file = super::open_regular(path)?;
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
        let headers: Vec<_> = declaration
            .headers
            .iter()
            .map(|h| CaffeineHeader {
                name: &h.name,
                value: &h.value,
            })
            .collect();
        let mut verifier = CaffeineRootVerifier::new(
            root,
            self.bytes,
            &headers,
            CaffeineHashLimits {
                max_content_bytes: maximum,
                max_append_bytes: FRAME.try_into().expect("positive frame bound"),
                max_headers: 16.try_into().expect("positive header bound"),
                max_header_bytes: 4096.try_into().expect("positive metadata bound"),
            },
        )
        .map_err(|_| Failure::InvalidReply)?;
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

#[cfg(test)]
mod tests;
