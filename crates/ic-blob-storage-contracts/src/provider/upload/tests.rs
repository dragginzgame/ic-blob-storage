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
