//! Pure provider contracts.
pub mod download;
pub mod gc;
pub mod onboarding;
#[cfg(feature = "provider-json")]
pub mod preparation;
#[cfg(feature = "provider-json")]
pub mod upload;
mod wire;
