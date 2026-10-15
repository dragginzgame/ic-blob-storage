//! Explicitly selected probe access, outside the supported production integration API.
//! Production hosts use `ServiceInstallation` and installed-owner certificate workflows.
pub mod admission;
pub mod certificate;
pub mod exposure;
pub mod stores;
