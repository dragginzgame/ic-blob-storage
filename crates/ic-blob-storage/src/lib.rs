//! Independent blob-storage service library for Internet Computer canisters.
//!
//! Content identities, pure policy, transient lifecycle models and bounded
//! Caffeine reply decoders, durable journals and explicit Cashier transport are
//! available. Shared funding preparation composes journal facts with host evidence;
//! production service qualification and complete adapters remain unfinished.
//!
//! The service owns tenant authorization, content references, quotas, provider
//! effects, billing, retention, and deletion. The standalone adapter delegates
//! to shared workflows; consumer frameworks own their wrappers externally.

pub mod dto;
pub mod model;
pub mod ops;
pub mod policy;
pub mod workflow;

/// Version compiled into this library, independent of the embedding host package.
/// This identifies the library contract, not the host Wasm or its deployment.
pub const LIBRARY_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Shared allocation governance and its exact stable-structures substrate.
///
/// The integrating host owns memory bootstrap, policy, bucket configuration and
/// allocation grants. Linking this crate does not register stores, initialize a
/// memory manager or export lifecycle hooks. Access stable collections through
/// `ic_memory::ic_stable_structures` to preserve the runtime's type identity.
pub use ic_memory;
