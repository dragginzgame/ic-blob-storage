//! Passive descriptor recovery from the single tenant-owned admission journal.

use super::content::{ContentLookup, TenantContentView};
use super::{UploadAdmissionError, UploadAdmissions, UploadContext, key};
use crate::model::{identity::ProviderRootHash, lifecycle::binding::ReferenceKey};

/// Original validated hash metadata; owning a copy grants no authority.
///
/// The admission owner retains at most `max_headers` entries and
/// `max_header_bytes` framed UTF-8 bytes per lifetime object slot. It exposes only
/// immutable borrows. Header names/case/values are not rewritten for HTTP use.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentHeader {
    /// Exact name from the first successful manifest preparation.
    pub name: String,
    /// Exact value from the first successful manifest preparation.
    pub value: String,
}

/// Borrowed declaration and current lifecycle, not a download authorization.
///
/// Available for every prepared state, including uncertain/cancelled/retired
/// history. The tenant must separately hold the exact retained reference and
/// establish completion before publication. A descriptor neither reserves a
/// reference nor proves that the provider still holds the object.
///
/// The request binds service, tenant, local namespace, object, root and length.
/// The local namespace is not a provider project ID, bucket or public URL. A
/// production host must authenticate/certify delivery to the consumer and bind
/// the separately configured provider locator and serving policy. Ordinary query
/// output does not itself establish those properties.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContentDescriptorView<'a> {
    /// Original immutable operation with the same live/history observation as discovery.
    pub content: TenantContentView,
    /// Original hash metadata. No body bytes or credentials are disclosed.
    pub headers: &'a [ContentHeader],
}

/// Descriptor observed with confirmed completion and one exact live reference.
///
/// This passive view is neither a retain receipt nor a publication capability.
/// Its borrow prevents mutation of this owner while inspecting it, but a copied
/// or serialized observation can become stale immediately. The consumer must
/// coordinate publication and release through its own durable workflow/outbox.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetainedContentDescriptorView<'a> {
    /// Descriptor from the same owner observation as the reference check.
    pub descriptor: ContentDescriptorView<'a>,
    /// Exact live reference checked, which need not be the initial reference.
    pub reference: ReferenceKey,
}

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
