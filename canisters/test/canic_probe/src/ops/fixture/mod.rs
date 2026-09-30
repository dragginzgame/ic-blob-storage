//! Controlled exposure-state setup, with no certificate or provider qualification facts.
use super::mutate;
use crate::dto::ProbeExposureFailure;
use ic_blob_storage::{
    dto::upload::admission::UploadAdmissionRequest,
    model::{
        catalog::admission::{UploadObject, UploadRequest, UploadRequestId},
        identity::ProviderRootHash,
        lifecycle::{
            ReferenceId,
            binding::{ObjectBinding, ObjectIdentity, ReferenceKey},
        },
        service::upload::UploadContext,
    },
};
use std::num::NonZeroU128;

// This deliberately selects the recorded uploader as a local test actor after
// authenticating the operator. It must never be copied into a deployment adapter.
// StableUploads::expose enforces local prerequisites, without claiming the
// additional provider/recovery evidence required by production exposure workflow.
pub(crate) fn expose(
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<(), ProbeExposureFailure> {
    mutate(|owner| {
        let configuration = owner.configuration();
        if context.actor != configuration.operator {
            return Err(ProbeExposureFailure::Denied);
        }
        let upload = input.upload;
        if upload.service != context.service || upload.namespace != configuration.namespace {
            return Err(ProbeExposureFailure::Binding);
        }
        let positive = |n| NonZeroU128::new(n).ok_or(ProbeExposureFailure::Binding);
        let object = ObjectBinding::new(
            context.service,
            upload.tenant,
            ObjectIdentity {
                namespace: positive(upload.namespace)?,
                object: positive(upload.object)?,
                incarnation: positive(upload.incarnation)?,
            },
        )
        .map_err(|_| ProbeExposureFailure::Binding)?;
        let request = UploadRequest {
            id: UploadRequestId::new(positive(upload.upload)?),
            object: UploadObject {
                root: ProviderRootHash::try_from(upload.root.as_slice()).expect("fixed root"),
                bytes: upload.bytes,
                first: ReferenceKey::new(
                    object,
                    ReferenceId::new(positive(upload.first_reference)?),
                ),
            },
        };
        let simulated = UploadContext {
            service: context.service,
            actor: input.uploader,
        };
        let uploads = &mut owner.stores_mut().uploads;
        let retained = uploads
            .lookup(simulated, request)
            .map_err(|_| ProbeExposureFailure::State)?;
        if retained.permission.uploader != input.uploader
            || retained.permission.expires_at_ns != input.expires_at_ns
        {
            return Err(ProbeExposureFailure::Conflict);
        }
        uploads
            .expose(simulated, request, ic_cdk::api::time())
            .map_err(|_| ProbeExposureFailure::State)
    })
}
