//! Pure provider contracts.
pub mod download;
#[cfg(feature = "caffeine-candid")]
pub mod gc;
#[cfg(feature = "caffeine-candid")]
pub mod onboarding;
#[cfg(feature = "provider-json")]
pub mod preparation;
#[cfg(feature = "provider-json")]
pub mod upload;
#[cfg(feature = "caffeine-candid")]
mod wire;
