//! Independent blob-storage service library for Internet Computer canisters.
//!
//! Service policy, lifecycle models, durable journals and explicit Cashier transport
//! are available. Runtime-free wire contracts and content verification live in
//! `ic_blob_storage_contracts`. Shared funding preparation composes journal facts with host evidence;
//! production service qualification and complete adapters remain unfinished.
//!
//! The service owns tenant authorization, content references, quotas, provider
//! effects, billing, retention, and deletion. The standalone adapter delegates
//! to shared workflows; consumer frameworks own their wrappers externally.

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
/// Hosts with a direct Memory dependency must select the same 0.31 release line.
pub use ic_memory;
