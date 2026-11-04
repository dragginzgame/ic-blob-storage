//! Passive service boundary data; linking exports no endpoints or lifecycle hooks.
#[cfg(feature = "caffeine-candid")]
pub mod account;
pub mod configuration;
pub mod download;
pub mod funding;
pub mod gateway;
pub mod operator;
pub mod recovery;
pub mod reference;
pub mod tenant;
pub mod upload;
