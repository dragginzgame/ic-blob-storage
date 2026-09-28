//! One service-owned descriptor boundary, bounded decoding and explicit IC client.
pub mod client;
pub mod reply;
use crate::{
    dto::download::{DownloadFailure, DownloadHeader, DownloadRequest, DownloadResponse},
    model::{
        identity::ProviderRootHash,
        lifecycle::{
            ReferenceId,
            binding::{ObjectBinding, ObjectIdentity, ReferenceKey},
        },
        service::upload::{UploadAdmissionError, UploadContext},
    },
    ops::service::uploads::{
        UploadStoreError,
        read::download::{CaffeineDownloadDescriptorView, DownloadDescriptorError},
    },
};
use std::num::NonZeroU128;
/// Canonical service update method. No endpoint is exported by this constant.
pub const DOWNLOAD_METHOD: &str = "blob_download_descriptor";

pub(crate) fn parse(
    context: UploadContext,
    request: DownloadRequest,
) -> Result<(ProviderRootHash, ReferenceKey), DownloadFailure> {
    if request.service != context.service {
        return Err(DownloadFailure::Binding);
    }
    if request.tenant != context.actor {
        return Err(DownloadFailure::Denied);
    }
    let positive = |n| NonZeroU128::new(n).ok_or(DownloadFailure::Invalid);
    let object = ObjectBinding::new(
        request.service,
        request.tenant,
        ObjectIdentity {
            namespace: positive(request.namespace)?,
            object: positive(request.object)?,
            incarnation: positive(request.incarnation)?,
        },
    )
    .map_err(|_| DownloadFailure::Invalid)?;
    Ok((
        ProviderRootHash::try_from(request.root.as_slice()).expect("fixed root"),
        ReferenceKey::new(object, ReferenceId::new(positive(request.reference)?)),
    ))
}
pub(crate) fn present(
    request: DownloadRequest,
    view: CaffeineDownloadDescriptorView,
) -> DownloadResponse {
    DownloadResponse {
        request,
        owner: view.scope.owner(),
        project: view.scope.project().to_owned(),
        bytes: view.content.descriptor.content.request.object.bytes,
        headers: view
            .content
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
                crate::model::service::tenant::TenantError::NotEnrolled
                | crate::model::service::tenant::TenantError::Suspended,
            ),
        )) => DownloadFailure::Inactive,
        DownloadDescriptorError::Store(_) => DownloadFailure::Internal,
    }
}
