//! Immutable installation binding, independent of endpoint and lifecycle ownership.
pub(crate) mod record;

/// Retained installation cannot be adopted by another service, release or layout.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum InstallationBindingError {
    /// The retained record has a different frozen layout identity.
    #[error("installation format mismatch")]
    Format,
    /// The running canister differs from the installed service.
    #[error("installation service mismatch")]
    Service,
    /// Cross-release restoration is unsupported; retirement precedes reinstall.
    #[error("installation release mismatch")]
    Release,
}
