//! Explicit trust in one uploader; never proof of provider economics or replay safety.
use candid::Principal;
use std::num::NonZeroU128;

/// Immutable certificate trust for a single service namespace and uploader.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadIssuerAuthority {
    service: Principal,
    namespace: NonZeroU128,
    uploader: Principal,
}
impl UploadIssuerAuthority {
    /// Validate explicit identities without granting tenant or controller authority.
    /// # Errors
    /// Rejects anonymous or management identities.
    pub fn new(
        service: Principal,
        namespace: NonZeroU128,
        uploader: Principal,
    ) -> Result<Self, InvalidUploadIssuerAuthority> {
        if [service, uploader]
            .iter()
            .any(|p| *p == Principal::anonymous() || *p == Principal::management_canister())
        {
            return Err(InvalidUploadIssuerAuthority);
        }
        Ok(Self {
            service,
            namespace,
            uploader,
        })
    }
    /// Installed trusted uploader; a tenant's permission must still match exactly.
    #[must_use]
    pub const fn uploader(self) -> Principal {
        self.uploader
    }
    /// Check the complete permission's local service/namespace and uploader trust.
    #[must_use]
    pub fn permits(self, permission: super::UploadPermission) -> bool {
        let object = permission.request.object.first.object();
        object.service() == self.service
            && object.identity().namespace == self.namespace
            && permission.uploader == self.uploader
    }
}
/// Invalid explicitly installed certificate authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("invalid upload issuer authority")]
pub struct InvalidUploadIssuerAuthority;
