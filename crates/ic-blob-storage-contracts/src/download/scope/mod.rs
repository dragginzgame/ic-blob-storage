//! Explicit Caffeine serving identity; never inferred from the payer or tenant.
use candid::Principal;
use std::num::NonZeroU128;
use thiserror::Error;
#[cfg(test)]
mod tests;

/// Host-provisioned mapping from a local namespace to a Caffeine owner/project.
/// Construction checks representation only, not provider assignment or ownership.
/// This carries no origin, credentials, bucket inference or default project.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaffeineDownloadScope {
    owner: Principal,
    namespace: NonZeroU128,
    project: String,
}
/// Invalid serving identity, rejected before copying project text.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum DownloadScopeError {
    /// Anonymous and management principals cannot own service content.
    #[error("invalid download owner")]
    Owner,
    /// Empty, surrounding whitespace, controls or more than 256 UTF-8 bytes.
    #[error("invalid download project")]
    Project,
}
impl CaffeineDownloadScope {
    /// Validate an explicit mapping. The project limit is a local representation
    /// bound, not a claim about Caffeine's supported project names.
    /// # Errors
    /// Rejects unusable owner identities and invalid or over-budget project text.
    pub fn new(
        owner: Principal,
        namespace: NonZeroU128,
        project: &str,
    ) -> Result<Self, DownloadScopeError> {
        if owner == Principal::anonymous() || owner == Principal::management_canister() {
            return Err(DownloadScopeError::Owner);
        }
        if project.is_empty()
            || project.len() > 256
            || project.trim() != project
            || project.chars().any(char::is_control)
        {
            return Err(DownloadScopeError::Project);
        }
        Ok(Self {
            owner,
            namespace,
            project: project.to_owned(),
        })
    }
    /// Storage owner/certificate issuer, independent of tenant and payer.
    #[must_use]
    pub const fn owner(&self) -> Principal {
        self.owner
    }
    /// Local namespace whose independently provisioned mapping the host supplied.
    #[must_use]
    pub const fn namespace(&self) -> NonZeroU128 {
        self.namespace
    }
    /// Original provider project text, not the decimal local namespace.
    #[must_use]
    pub fn project(&self) -> &str {
        &self.project
    }
}
