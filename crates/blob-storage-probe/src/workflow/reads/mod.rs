//! Durable verified chunks across an IC await using the labelled local source.
pub(crate) mod download;
use crate::ops::{
    gateways::callbacks,
    read::{authority, sessions, transport::LocalChunk},
};
use blob_test_protocol::{
    journey::readback::JourneyReadChunk,
    storage::{Failure, gateways::ReadSessionInput},
};
use ic_blob_storage::{
    model::service::{read::session::ReadChunkTarget, upload::UploadContext},
    ops::service::uploads::read::verification::ReadVerificationError,
    workflow::reads::{
        ReadAuthorityError,
        chunk::{ReadChunkError, read_chunk},
        sessions::ReadSessionWorkflowError,
    },
};
fn authority_failure(error: ReadAuthorityError) -> Failure {
    match error {
        ReadAuthorityError::Registry(e) => callbacks::registry_failure(e),
        ReadAuthorityError::Uploads(e) => callbacks::upload_failure(e),
        ReadAuthorityError::Gateway(e) => callbacks::access_failure(e),
        ReadAuthorityError::Unavailable => Failure::Unknown,
        ReadAuthorityError::Stale => Failure::Conflict,
        ReadAuthorityError::Exhausted => Failure::Capacity,
    }
}
fn failure(error: ReadChunkError<Failure>) -> Failure {
    match error {
        ReadChunkError::Session(ReadSessionWorkflowError::Session(e)) => sessions::failure(e),
        ReadChunkError::Session(ReadSessionWorkflowError::Authority(e)) => authority_failure(e),
        ReadChunkError::Transport(e) => e,
        ReadChunkError::Binding => Failure::Binding,
        ReadChunkError::ReplyTooLarge => Failure::Capacity,
        ReadChunkError::Verification(ReadVerificationError::Store(e)) => {
            callbacks::upload_failure(e)
        }
        ReadChunkError::Verification(ReadVerificationError::Content(_)) => Failure::ContentMismatch,
    }
}
pub(crate) async fn run(
    context: UploadContext,
    input: ReadSessionInput,
) -> Result<JourneyReadChunk, Failure> {
    let (scope, target) = authority::parse(context, input)?;
    let host = sessions::Host {
        admit_fault: input.admit_fault,
        callback_fault: input.callback_fault,
        admitted: false.into(),
    };
    let chunk = read_chunk(
        &host,
        &LocalChunk,
        context,
        scope,
        ReadChunkTarget {
            target,
            index: input.index,
        },
    )
    .await
    .map_err(failure)?;
    Ok(JourneyReadChunk {
        index: chunk.index,
        offset: chunk.offset,
        bytes: chunk.bytes,
    })
}
