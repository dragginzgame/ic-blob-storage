//! Passive descriptor recovery from the single tenant-owned admission journal.

use super::content::TenantContentView;
use ic_blob_storage_contracts::binding::ReferenceKey;

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
