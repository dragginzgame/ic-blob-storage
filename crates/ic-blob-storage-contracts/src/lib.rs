//! Runtime-free Blob wire contracts, identity validation and content verification.
//! No storage, CDK, lifecycle, provider dispatch or implicit authority is owned here.
pub mod binding;
pub mod configuration;
pub mod decoding;
/// Passive offline document encoding, separate from endpoint frames.
pub mod document;
pub mod download;
pub mod dto;
pub mod endpoints;
pub mod funding;
pub mod identity;
pub mod protocol;
pub mod provider;
pub mod reference;
/// Optional standalone host contracts, separate from the shared mounted service.
pub mod standalone;
pub mod tenant;
pub mod upload;
pub mod wire;

/// Compiled client/service contract identity; not a host deployment identity.
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
