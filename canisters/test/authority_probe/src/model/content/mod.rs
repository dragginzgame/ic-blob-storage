//! Bounded local verification across messages, without retaining content bytes.
use ic_blob_storage::model::identity::{
    ContentDigest,
    caffeine::manifest::{
        CaffeineChunkManifest, verification::ordered::CaffeineOrderedChunkVerifier,
    },
};

pub(crate) struct ContentSession {
    manifest: CaffeineChunkManifest,
    state: State,
}

enum State {
    Checking(Box<CaffeineOrderedChunkVerifier>),
    Verified,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ContentVerdict {
    Pending,
    Verified,
    Rejected,
}

pub(crate) enum ContentError {
    OutOfOrder,
    Mismatch,
}

impl ContentSession {
    pub(crate) fn new(manifest: CaffeineChunkManifest, digest: ContentDigest) -> Self {
        let verifier = CaffeineOrderedChunkVerifier::new(manifest.clone(), digest);
        Self {
            manifest,
            state: State::Checking(Box::new(verifier)),
        }
    }

    pub(crate) fn manifest(&self) -> &CaffeineChunkManifest {
        &self.manifest
    }

    pub(crate) fn progress(&self) -> (u64, u64, ContentVerdict) {
        match &self.state {
            State::Checking(verifier) => (
                verifier.next_chunk(),
                verifier.verified_bytes(),
                ContentVerdict::Pending,
            ),
            State::Verified => (
                u64::try_from(self.manifest.chunk_count()).expect("bounded chunk count"),
                self.manifest.content_bytes(),
                ContentVerdict::Verified,
            ),
            State::Rejected => (
                u64::try_from(self.manifest.chunk_count()).expect("bounded chunk count"),
                self.manifest.content_bytes(),
                ContentVerdict::Rejected,
            ),
        }
    }

    /// Exact old chunks are checked but never fed into the raw hash twice.
    /// Final digest failure is terminal; no retry may replace that declaration.
    pub(crate) fn append(&mut self, index: u64, bytes: &[u8]) -> Result<(), ContentError> {
        let next = match &self.state {
            State::Checking(verifier) => verifier.next_chunk(),
            State::Verified => {
                u64::try_from(self.manifest.chunk_count()).expect("bounded chunk count")
            }
            State::Rejected => return Err(ContentError::Mismatch),
        };
        if index > next {
            return Err(ContentError::OutOfOrder);
        }
        if index < next || matches!(self.state, State::Verified) {
            return self
                .manifest
                .verify_chunk(index, bytes)
                .map_err(|_| ContentError::Mismatch);
        }
        let State::Checking(verifier) = &mut self.state else {
            unreachable!("receiving state")
        };
        verifier
            .append_chunk(index, bytes)
            .map_err(|_| ContentError::Mismatch)?;
        if verifier.remaining_bytes() != 0 {
            return Ok(());
        }
        let State::Checking(verifier) = std::mem::replace(&mut self.state, State::Rejected) else {
            unreachable!("completed receiving state")
        };
        verifier.finish().map_err(|_| ContentError::Mismatch)?;
        self.state = State::Verified;
        Ok(())
    }
}
