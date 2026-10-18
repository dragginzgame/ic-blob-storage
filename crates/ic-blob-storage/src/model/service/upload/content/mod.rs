//! Tenant-scoped discovery of original upload identities and current lifecycle.

use std::num::NonZeroU128;

use ic_platform::Principal;

use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::upload::binding::UploadRequest;
use ic_blob_storage_contracts::upload::history::UploadRootState;

/// Explicit lookup scope; knowing a content root supplies no tenant authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContentLookup {
    /// Exact publishing tenant, which must also be the authenticated actor.
    pub tenant: Principal,
    /// Configured provider namespace.
    pub namespace: NonZeroU128,
    /// Provider identity to discover within this scope.
    pub root: ProviderRootHash,
}

/// Passive current observation, not a retain receipt or authorization to upload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TenantContentView {
    /// Original operation and complete object/initial-reference binding.
    /// Recover this identity rather than allocating a replacement after a lost reply.
    pub request: UploadRequest,
    /// Current reservation or confirmed lifecycle, including retired history.
    /// Even a live observation can become stale before a retain is applied.
    pub state: UploadRootState,
}
