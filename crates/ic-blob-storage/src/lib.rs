//! Independent blob-storage service library for Internet Computer canisters.
//!
//! Service policy, lifecycle models, durable journals and explicit Cashier transport
//! are available. Runtime-free wire contracts and content verification live in
//! `ic_blob_storage_contracts`. Shared funding preparation composes journal facts
//! with host evidence. The standalone adapter mounts shared service workflows;
//! deployed provider qualification remains separate from local implementation.
//!
//! The service owns tenant authorization, content references, quotas, provider
//! effects, billing, retention, and deletion. The standalone adapter delegates
//! to shared workflows; consumer frameworks own their wrappers externally.

//!
//! Hosts construct and restore the complete owner through
//! [`ops::service::installation::ServiceInstallation`]. Component constructors
//! and evidence-taking certificate issuance are internal. The non-default
//! `test-support` feature exists only for isolated probes and reference-model
//! comparisons; production integrations must leave it disabled. Heap catalog,
//! lifecycle and receipt owners and their read-policy adapters are available only
//! with that feature (or in this library's unit tests). Shared domain types, pure
//! transitions and the durable service owner remain available without it.
//!
//! Component stores cannot be installed independently through the production API:
//! ```compile_fail
//! use ic_memory::{Memory, RuntimeMemory};
//! use ic_blob_storage::ops::service::stores::{
//!     ServiceMemories, ServiceStoreConfiguration, ServiceStoreError, ServiceStores,
//! };
//! fn construct<M: Memory>(memory: ServiceMemories<RuntimeMemory<M>>, config: ServiceStoreConfiguration)
//!     -> Result<ServiceStores<M>, ServiceStoreError>
//! {
//!     ServiceStores::install(memory, config)
//! }
//! ```
//! Certificate issuance derives evidence from the installed owner:
//! ```compile_fail
//! use ic_blob_storage::workflow::uploads::certificate::issue;
//! ```
//! Direct evidence-taking exposure commit is internal as well:
//! ```compile_fail
//! use ic_blob_storage::workflow::uploads::exposure::commit;
//! ```

pub mod model;
pub mod ops;
pub mod policy;
pub mod workflow;

/// Version compiled into this library, independent of the embedding host package.
/// This identifies the library contract, not the host Wasm or its deployment.
pub const LIBRARY_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Shared allocation governance for host-owned memory.
///
/// The integrating host owns memory bootstrap, policy, bucket configuration and
/// allocation grants. Linking this crate does not register stores, initialize a
/// memory manager or export lifecycle hooks. IC Storage owns stable collections;
/// constructors require this runtime's exclusive allocation leases. Store and
/// workflow type parameters name the backing memory, not `RuntimeMemory<Backing>`.
/// Hosts should use this re-export or align a direct Memory dependency with
/// this crate's selected release line.
pub use ic_memory;

/// Probe-only reference models and fault-injection access.
/// Never enable this feature in a production host; it deliberately bypasses the
/// complete installation boundary for isolated component and recovery fixtures.
#[cfg(any(test, feature = "test-support"))]
pub mod test_support;
