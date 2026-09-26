//! Parse the completion indicator without confusing it with durable storage.

use std::num::NonZeroUsize;

use serde::Deserialize;
use thiserror::Error;

/// What one gateway chunk reply reports, not verified upload completion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChunkUploadStatus {
    /// The status is exactly `blob_complete`; authoritative evidence is still needed.
    CompletionReported,
    /// Another status was returned; it cannot authorize completing the upload.
    CompletionNotReported,
}

#[derive(Deserialize)]
struct ChunkReply {
    status: String,
}

/// Decode a bounded JSON chunk reply from an independently checked HTTP response.
///
/// Transfer progress, a returned root and successful HTTP transport are not inputs
/// to this decision. Only the exact provider indicator reports completion, and
/// even that observation does not construct a confirmed lifecycle or prove storage.
/// The transport must bound response buffering separately before calling this.
/// # Errors
/// Rejects oversized, malformed, missing, duplicate or wrongly typed status data.
pub fn decode_chunk_status(
    body: &[u8],
    max_bytes: NonZeroUsize,
) -> Result<ChunkUploadStatus, UploadReplyError> {
    if body.len() > max_bytes.get() {
        return Err(UploadReplyError::ReplyTooLarge);
    }
    let reply: ChunkReply =
        serde_json::from_slice(body).map_err(|_| UploadReplyError::InvalidReply)?;
    Ok(if reply.status == "blob_complete" {
        ChunkUploadStatus::CompletionReported
    } else {
        ChunkUploadStatus::CompletionNotReported
    })
}

/// An unusable observation; neither error permits retrying a paid upload.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum UploadReplyError {
    /// Reply exceeded the configured byte bound before parsing.
    #[error("upload reply exceeds byte limit")]
    ReplyTooLarge,
    /// JSON did not contain one valid string status.
    #[error("invalid upload reply")]
    InvalidReply,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bound(value: usize) -> NonZeroUsize {
        NonZeroUsize::new(value).expect("positive reply bound")
    }

    #[test]
    fn progress_and_root_do_not_replace_the_completion_indicator() {
        for body in [
            br#"{"status":"fixture_not_complete","progress":100,"hash":"sha256:any"}"#.as_slice(),
            br#"{"status":"BLOB_COMPLETE"}"#,
            br#"{"status":"blob_complete "}"#,
            br#"{"status":""}"#,
        ] {
            assert_eq!(
                decode_chunk_status(body, bound(body.len())),
                Ok(ChunkUploadStatus::CompletionNotReported)
            );
        }
        let body = br#"{"status":"blob_complete"}"#;
        assert_eq!(
            decode_chunk_status(body, bound(body.len())),
            Ok(ChunkUploadStatus::CompletionReported)
        );
        assert_eq!(
            decode_chunk_status(body, bound(body.len() - 1)),
            Err(UploadReplyError::ReplyTooLarge)
        );
    }

    #[test]
    fn malformed_or_ambiguous_replies_never_report_completion() {
        for body in [
            b"".as_slice(),
            b"{}",
            br#"{"status":true}"#,
            br#"{"status":null}"#,
            br#"{"progress":100,"hash":"sha256:any"}"#,
            br#"{"status":"blob_complete","status":"other"}"#,
            br#"{"status":"other","status":"blob_complete"}"#,
            br#"{"status":"blob_complete"} {}"#,
        ] {
            assert_eq!(
                decode_chunk_status(body, bound(256)),
                Err(UploadReplyError::InvalidReply)
            );
        }
    }
}
