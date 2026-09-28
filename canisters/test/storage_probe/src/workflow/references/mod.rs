//! Shared receipt boundary and explicit local tenant proxy.
use crate::ops::read::download;
use blob_test_protocol::storage::reference::{ReferenceClientInput, ReferenceProbeFailure};
use ic_blob_storage::{
    dto::reference::{ReferenceCommand, ReferenceFailure, ReferenceReceiptLookup},
    model::service::upload::UploadContext,
    ops::service::references::{
        client::{ReferenceClientError, ReplicatedReferenceClient},
        reply::ReferenceReplyError,
    },
};
pub(crate) fn receipt(
    context: UploadContext,
    input: ReferenceCommand,
) -> Result<ReferenceReceiptLookup, ReferenceFailure> {
    download::with_uploads(|uploads| {
        ic_blob_storage::workflow::references::receipt(uploads, context, input)
    })
}
pub(crate) async fn fetch(
    context: UploadContext,
    input: &ReferenceClientInput,
) -> Result<ReferenceReceiptLookup, ReferenceProbeFailure> {
    if !download::operator(context) {
        return Err(ReferenceProbeFailure::Denied);
    }
    let client = ReplicatedReferenceClient::new(
        input.tenant,
        input.request.upload.service,
        30.try_into().unwrap(),
    )
    .map_err(failure)?;
    let max = (input.max_reply_bytes as usize)
        .try_into()
        .map_err(|_| ReferenceProbeFailure::Invalid)?;
    client.receipt(input.request, max).await.map_err(failure)
}
fn failure(error: ReferenceClientError) -> ReferenceProbeFailure {
    match error {
        ReferenceClientError::Binding
        | ReferenceClientError::Reply(ReferenceReplyError::Binding) => {
            ReferenceProbeFailure::Binding
        }
        ReferenceClientError::Execution => ReferenceProbeFailure::Execution,
        ReferenceClientError::Rejected(code) => ReferenceProbeFailure::Rejected(code),
        ReferenceClientError::NotEnqueued => ReferenceProbeFailure::NotEnqueued,
        ReferenceClientError::Reply(ReferenceReplyError::Limit) => ReferenceProbeFailure::Limit,
        ReferenceClientError::Reply(ReferenceReplyError::Remote(error)) => {
            ReferenceProbeFailure::Remote(error)
        }
        _ => ReferenceProbeFailure::Invalid,
    }
}

pub(crate) fn apply(
    context: UploadContext,
    input: ReferenceCommand,
    fault: Option<blob_test_protocol::storage::WriteFault>,
) -> Result<ic_blob_storage::dto::reference::ReferenceMutationResponse, ReferenceFailure> {
    crate::ops::references::with_uploads_mut(fault, |uploads| {
        ic_blob_storage::workflow::references::apply(uploads, context, input)
    })
}

pub(crate) async fn mutate(
    context: UploadContext,
    input: &ReferenceClientInput,
) -> Result<ic_blob_storage::dto::reference::ReferenceMutationResponse, ReferenceProbeFailure> {
    if !download::operator(context) {
        return Err(ReferenceProbeFailure::Denied);
    }
    let client = ReplicatedReferenceClient::new(
        input.tenant,
        input.request.upload.service,
        30.try_into().unwrap(),
    )
    .map_err(failure)?;
    let max = (input.max_reply_bytes as usize)
        .try_into()
        .map_err(|_| ReferenceProbeFailure::Invalid)?;
    client.apply(input.request, max).await.map_err(failure)
}
