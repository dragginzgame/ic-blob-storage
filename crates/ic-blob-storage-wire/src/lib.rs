//! Shared Blob transport contracts for standalone and embedded hosts.

#![forbid(unsafe_code)]

mod contract;
pub mod frame;
pub use contract::{Contract, Management, Query, Update};
#[cfg(feature = "inspection")]
pub mod inspection;
#[cfg(feature = "inspection")]
mod projection;
#[cfg(feature = "inspection")]
mod schemas;

/// Admission shared by the host and its client, including the Wire header.
pub const RPC_LIMITS: ic_wire::Limits = ic_wire::Limits {
    bytes: 65_536,
    allocation: 8 * 1024 * 1024,
    nodes: 65_536,
    depth: 32,
};

/// Shared routing ID for Blob mutations.
pub const UPDATE: u64 = Update::ID;
/// Shared routing ID for Blob inspection.
pub const QUERY: u64 = Query::ID;
/// Shared routing ID for replicated management inspection.
pub const MANAGEMENT: u64 = Management::ID;
