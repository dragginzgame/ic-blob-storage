//! Explicit replicated Blob calls without the service storage implementation.
//!
//! Contracts owns request/reply codecs. Callers retain durable intent and recovery
//! policy; construction installs no endpoint, lifecycle hook, memory or timer.

pub mod admission;
pub mod download;
pub mod funding;
pub mod manifests;
pub mod references;
pub mod status;
pub mod tenants;
mod transport;
