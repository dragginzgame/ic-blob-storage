//! Bounded local verification and checkpoints, without retaining complete files.
use super::archive::ContentRecord;
use blob_test_protocol::journey::JourneyVerification;
use ic_blob_storage::model::identity::caffeine::manifest::verification::ordered::checkpoint::CaffeineVerificationCheckpointRecord;
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
    pub(crate) fn checkpoint(&self) -> Vec<u8> {
        match &self.state {
            State::Checking(verifier) => verifier.checkpoint().as_bytes().to_vec(),
            State::Verified | State::Rejected => Vec::new(),
        }
    }

    /// Reconstruct only a verification copy; callers must never infer restored
    /// operation authority from the archive or its unauthenticated checksum.
    pub(crate) fn from_record(
        manifest: CaffeineChunkManifest,
        digest: ContentDigest,
        record: &ContentRecord,
    ) -> Option<Self> {
        let (state, verdict) = match record.verdict {
            JourneyVerification::Pending => {
                let checkpoint =
                    CaffeineVerificationCheckpointRecord::try_from(record.checkpoint.as_slice())
                        .ok()?;
                let verifier = CaffeineOrderedChunkVerifier::from_checkpoint(
                    manifest.clone(),
                    digest,
                    &checkpoint,
                )
                .ok()?;
                // Full input transitions to a terminal verdict within append.
                if verifier.remaining_bytes() == 0 {
                    return None;
                }
                (State::Checking(Box::new(verifier)), ContentVerdict::Pending)
            }
            JourneyVerification::Verified | JourneyVerification::Rejected => {
                if !record.checkpoint.is_empty() {
                    return None;
                }
                match record.verdict {
                    JourneyVerification::Verified => (State::Verified, ContentVerdict::Verified),
                    JourneyVerification::Rejected => (State::Rejected, ContentVerdict::Rejected),
                    JourneyVerification::Pending => unreachable!("terminal verdict"),
                }
            }
        };
        let session = Self { manifest, state };
        (session.progress() == (record.next_chunk, record.verified_bytes, verdict))
            .then_some(session)
    }

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
