//! One service-owned descriptor boundary, bounded decoding and explicit IC client.
pub mod client;
use crate::model::service::upload::UploadAdmissionError;
use crate::ops::service::uploads::UploadStoreError;
use crate::ops::service::uploads::read::RetainedUploadDescriptorView;
use crate::ops::service::uploads::read::download::DownloadDescriptorError;
use ic_blob_storage_contracts::download::scope::CaffeineDownloadScope;
use ic_blob_storage_contracts::dto::download::DownloadFailure;
use ic_blob_storage_contracts::dto::download::DownloadHeader;
use ic_blob_storage_contracts::dto::download::DownloadRequest;
use ic_blob_storage_contracts::dto::download::DownloadResponse;

pub(crate) fn present(
    request: DownloadRequest,
    scope: &CaffeineDownloadScope,
    view: RetainedUploadDescriptorView,
) -> DownloadResponse {
    DownloadResponse {
        request,
        owner: scope.owner(),
        project: scope.project().to_owned(),
        bytes: view.descriptor.content.request.object.bytes,
        headers: view
            .descriptor
            .headers
            .into_iter()
            .map(|h| DownloadHeader {
                name: h.name,
                value: h.value,
            })
            .collect(),
    }
}
pub(crate) fn failure(error: DownloadDescriptorError) -> DownloadFailure {
    match error {
        DownloadDescriptorError::Unavailable => DownloadFailure::Unavailable,
        DownloadDescriptorError::Store(UploadStoreError::Fenced) => DownloadFailure::Fenced,
        DownloadDescriptorError::Store(UploadStoreError::Admission(
            UploadAdmissionError::NotProject,
        )) => DownloadFailure::Denied,
        DownloadDescriptorError::Store(UploadStoreError::Admission(
            UploadAdmissionError::WrongService | UploadAdmissionError::WrongNamespace,
        ))
        | DownloadDescriptorError::Binding => DownloadFailure::Binding,
        DownloadDescriptorError::Store(UploadStoreError::Admission(
            UploadAdmissionError::Tenant(
                ic_blob_storage_contracts::tenant::TenantError::NotEnrolled
                | ic_blob_storage_contracts::tenant::TenantError::Suspended,
            ),
        )) => DownloadFailure::Inactive,
        DownloadDescriptorError::Store(_) => DownloadFailure::Internal,
    }
}
