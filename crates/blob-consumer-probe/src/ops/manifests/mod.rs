//! One shared client call using separately saved uploader intent.
use blob_test_protocol::consumer::Failure;
pub(crate) fn fault(
    selected: blob_test_protocol::consumer::manifests::ManifestFault,
    at: blob_test_protocol::consumer::manifests::ManifestFault,
) {
    if selected == at {
        ic_cdk::trap("fixture manifest interruption");
    }
}
use ic_blob_storage::{
    dto::upload::{
        admission::UploadAdmissionRequest,
        manifest::{UploadManifestFailure, UploadManifestRequest, UploadManifestResponse},
    },
    ops::service::uploads::manifests::{
        client::{ReplicatedUploadManifestClient, UploadManifestClientError},
        reply::{UploadManifestReplyError, UploadManifestReplyLimits},
    },
};
fn client(permission: UploadAdmissionRequest) -> Result<ReplicatedUploadManifestClient, Failure> {
    ReplicatedUploadManifestClient::new(
        ic_cdk::api::canister_self(),
        permission.upload.tenant,
        permission.upload.service,
        30.try_into().unwrap(),
    )
    .map_err(|_| Failure::Invalid)
}
fn limits(max: u32) -> Result<UploadManifestReplyLimits, Failure> {
    Ok(UploadManifestReplyLimits {
        max_reply_bytes: (max as usize).try_into().map_err(|_| Failure::Invalid)?,
        declaration: crate::model::manifests::limits(),
    })
}
fn outcome(
    result: Result<UploadManifestResponse, UploadManifestClientError>,
) -> Result<Result<UploadManifestResponse, UploadManifestFailure>, Failure> {
    match result {
        Ok(response) => Ok(Ok(response)),
        Err(UploadManifestClientError::Reply(UploadManifestReplyError::Remote(error))) => {
            Ok(Err(error))
        }
        Err(_) => Err(Failure::Transport),
    }
}
pub(crate) async fn prepare(
    input: &UploadManifestRequest,
    max: u32,
) -> Result<Result<UploadManifestResponse, UploadManifestFailure>, Failure> {
    outcome(
        client(input.permission)?
            .prepare(input, limits(max)?)
            .await
            .map(|r| r.observation),
    )
}
pub(crate) async fn inspect(
    input: UploadAdmissionRequest,
) -> Result<UploadManifestResponse, Failure> {
    // Inspection refusal/absence does not establish the uncertain mutation's result.
    client(input)?
        .inspect(input, limits(4096)?)
        .await
        .map_err(|_| Failure::Transport)
}
