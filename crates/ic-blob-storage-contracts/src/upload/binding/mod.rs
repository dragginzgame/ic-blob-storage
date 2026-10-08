//! Immutable upload identities; construction grants no effects or tenant authority.
use crate::binding::ReferenceKey;
use crate::identity::ProviderRootHash;
use candid::Principal;
use std::num::NonZeroU128;
/// Upload operation ID scoped to a tenant in this service, across namespaces.
/// Allocation freshness and restore fencing are external requirements.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UploadRequestId(NonZeroU128);

impl UploadRequestId {
    /// Wrap a nonzero identity without granting authority or freshness.
    #[must_use]
    pub const fn new(value: NonZeroU128) -> Self {
        Self(value)
    }

    /// Original operation number for exact boundary encoding, not a fresh allocator.
    #[must_use]
    pub const fn get(self) -> NonZeroU128 {
        self.0
    }
}

/// Exact immutable local upload arguments; a root does not prove stored bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadRequest {
    /// Tenant-scoped operation identity; retries must preserve every argument.
    pub id: UploadRequestId,
    /// Expected root, length and first reference with the full object binding.
    pub object: UploadObject,
}

/// Expected object properties to reserve, without claiming upload completion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadObject {
    /// Expected provider root, reserved exclusively for this object lifetime.
    pub root: ProviderRootHash,
    /// Exact declared length to reserve before any upload authority escapes.
    pub bytes: u64,
    /// Initial reference and full service/tenant/namespace/object binding.
    pub first: ReferenceKey,
}

/// Authenticated execution context, supplied by the host rather than request data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadContext {
    /// Actual running service.
    pub service: Principal,
    /// Actual caller: operator for enrollment, project for admission/references,
    /// uploader for exposure. No role is inferred from controller status.
    pub actor: Principal,
}

/// Exact project-approved operation. No wildcard root, account or uploader exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadPermission {
    /// Full tenant/service/namespace/object/reference identity, root and declared size.
    /// A bounded manifest must be bound before local exposure.
    pub request: UploadRequest,
    /// Browser/session principal approved by the tenant project.
    pub uploader: Principal,
    /// Exclusive local certificate-issuance deadline in host nanoseconds.
    /// Expiry cannot revoke an escaped certificate or release uncertain capacity.
    pub expires_at_ns: u64,
}
