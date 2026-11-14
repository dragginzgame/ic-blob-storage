//! Blob Storage domain operations and local Caffeine content verification.
//!
//! The experimental local model composes grants, application references and
//! declared-byte accounting with the manifest check. Streaming verification
//! checks actual bytes against an independently trusted declaration.
//! The bounded durable service reuses that model with a host-granted Storage
//! lease. Hosts capture callers and compose memory/lifecycle hooks. The standalone
//! host exports experimental Wire methods, without provider authority or transfers.

pub mod caffeine;
pub mod local;
pub mod protocol;
pub mod references;
