//! Shared descriptor boundary and an explicitly authorized local consumer probe.
use crate::ops::read::download;
use blob_test_protocol::storage::read::{DownloadClientInput, DownloadProbeFailure};
use ic_blob_storage::{
    dto::download::{DownloadFailure, DownloadRequest, DownloadResponse},
    model::service::{read::download::CaffeineDownloadScope, upload::UploadContext},
    ops::service::reads::download::{
        client::{DownloadClientError, ReplicatedDownloadClient},
        reply::{DownloadReplyError, DownloadReplyLimits},
    },
};
pub(crate) fn describe(
    context: UploadContext,
    input: DownloadRequest,
) -> Result<DownloadResponse, DownloadFailure> {
    let scope = download::scope(context);
    download::with_uploads(|uploads| {
        ic_blob_storage::workflow::reads::download::handle(uploads, context, &scope, input)
    })
}
pub(crate) async fn fetch(
    context: UploadContext,
    input: DownloadClientInput,
) -> Result<DownloadResponse, DownloadProbeFailure> {
    if !download::operator(context) {
        return Err(DownloadProbeFailure::Denied);
    }
    // This operator-controlled consumer can target a different storage installation;
    // its own fixture namespace is not the target's serving namespace.
    let namespace = input
        .request
        .namespace
        .try_into()
        .map_err(|_| DownloadProbeFailure::Invalid)?;
    let scope = CaffeineDownloadScope::new(input.request.service, namespace, &input.project)
        .map_err(|_| DownloadProbeFailure::Invalid)?;
    let client =
        ReplicatedDownloadClient::new(input.tenant, input.request.service, 30.try_into().unwrap())
            .map_err(failure)?;
    let limits = DownloadReplyLimits {
        max_reply_bytes: (input.max_reply_bytes as usize)
            .try_into()
            .map_err(|_| DownloadProbeFailure::Invalid)?,
        max_content_bytes: 10.try_into().unwrap(),
        max_headers: 8.try_into().unwrap(),
        max_header_bytes: 1024.try_into().unwrap(),
    };
    client
        .fetch(input.request, &scope, limits)
        .await
        .map_err(failure)
}
fn failure(error: DownloadClientError) -> DownloadProbeFailure {
    match error {
        DownloadClientError::Binding | DownloadClientError::Reply(DownloadReplyError::Binding) => {
            DownloadProbeFailure::Binding
        }
        DownloadClientError::Execution => DownloadProbeFailure::Execution,
        DownloadClientError::Rejected(code) => DownloadProbeFailure::Rejected(code),
        DownloadClientError::NotEnqueued => DownloadProbeFailure::NotEnqueued,
        DownloadClientError::Reply(DownloadReplyError::Limit) => DownloadProbeFailure::Limit,
        DownloadClientError::Reply(DownloadReplyError::Remote(error)) => {
            DownloadProbeFailure::Remote(error)
        }
        _ => DownloadProbeFailure::Invalid,
    }
}
