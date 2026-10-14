//! Checked fixture projection; it neither allocates identities nor grants authority.

use super::Request;
use ic_blob_storage_contracts::{
    binding::{ObjectBinding, ObjectIdentity, ReferenceId, ReferenceKey},
    identity::ProviderRootHash,
    upload::binding::{UploadObject, UploadRequest, UploadRequestId},
};
use std::num::NonZeroU128;

/// Project the fixture's exact upload identity, with incarnation/reference one.
///
/// Returns `None` for a zero operation/namespace or invalid object binding.
/// Callers retain their failure category and check actual service/tenant authority.
#[must_use]
pub fn upload_request(input: Request) -> Option<UploadRequest> {
    let id = NonZeroU128::new(input.id)?;
    let namespace = NonZeroU128::new(input.namespace)?;
    let binding = ObjectBinding::new(
        input.service,
        input.tenant,
        ObjectIdentity {
            namespace,
            object: id,
            incarnation: NonZeroU128::MIN,
        },
    )
    .ok()?;
    Some(UploadRequest {
        id: UploadRequestId::new(id),
        object: UploadObject {
            root: ProviderRootHash::try_from(input.root.as_slice()).ok()?,
            bytes: input.bytes,
            first: ReferenceKey::new(binding, ReferenceId::new(NonZeroU128::MIN)),
        },
    })
}
