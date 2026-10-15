//! Reference-model descriptor observations.

use crate::model::service::upload::UploadAdmissionError;
use crate::model::service::upload::content::ContentLookup;
use ic_blob_storage_contracts::binding::ReferenceKey;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::upload::binding::UploadContext;

use super::{UploadAdmissions, key};
use crate::model::service::upload::download::{
    ContentDescriptorView, RetainedContentDescriptorView,
};

impl UploadAdmissions {
    /// Observe a descriptor only for the consumer's exact confirmed live reference.
    ///
    /// Service, namespace and tenant authority are checked before content access.
    /// Unknown/foreign/unconfirmed roots, mismatched object incarnations and
    /// unknown/released references return `None`. Another live reference cannot
    /// substitute for the supplied key. Suspended tenants retain read access.
    /// This allocates no reference or receipt and performs no provider effect.
    /// Publication still needs trusted delivery, provider locator/serving policy
    /// and coordination preventing release while a consumer relies on the object.
    /// # Errors
    /// Rejects wrong service, namespace or authenticated tenant.
    /// # Panics
    /// Panics only if private confirmed state disagrees with its descriptor.
    pub fn retained_content_descriptor(
        &self,
        context: UploadContext,
        root: ProviderRootHash,
        reference: ReferenceKey,
    ) -> Result<Option<RetainedContentDescriptorView<'_>>, UploadAdmissionError> {
        let object = reference.object();
        self.check_object(context, object)?;
        let Some(descriptor) = self.content_descriptor(
            context,
            ContentLookup {
                tenant: object.tenant(),
                namespace: object.identity().namespace,
                root,
            },
        )?
        else {
            return Ok(None);
        };
        if descriptor.content.request.object.first.object() != object {
            return Ok(None);
        }
        let Some(confirmed) = self.catalog.confirmed().get(root) else {
            return Ok(None);
        };
        if !confirmed
            .lifecycle()
            .reference_is_live(reference)
            .expect("descriptor and confirmed object binding agree")
        {
            return Ok(None);
        }
        Ok(Some(RetainedContentDescriptorView {
            descriptor,
            reference,
        }))
    }

    /// Recover a bounded descriptor under exactly the content-discovery authority.
    ///
    /// Unknown/foreign/unprepared roots return `None`. Suspended tenants can still
    /// inspect their own history. Borrowing performs no copy or allocation and
    /// changes no accounting, lifecycle or receipt. Metadata survives cancellation,
    /// provider deletion and settlement because lifetime permission slots remain.
    /// # Errors
    /// Rejects wrong service, namespace or tenant actor before reading metadata.
    /// # Panics
    /// Panics only if the private permission index disagrees with discovery.
    pub fn content_descriptor(
        &self,
        context: UploadContext,
        input: ContentLookup,
    ) -> Result<Option<ContentDescriptorView<'_>>, UploadAdmissionError> {
        let Some(observation) = self.lookup_content(context, input)? else {
            return Ok(None);
        };
        let permission = self
            .permissions
            .get(&key(observation.request))
            .expect("indexed permission");
        Ok(permission
            .manifest
            .as_ref()
            .map(|manifest| ContentDescriptorView {
                content: observation,
                headers: &manifest.headers,
            }))
    }
}
