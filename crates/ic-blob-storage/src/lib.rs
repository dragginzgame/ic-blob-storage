//! Blob Storage domain operations and local Caffeine content verification.
//!
//! The experimental local model composes grants, application references and
//! declared-byte accounting with the manifest check. Streaming verification
//! checks actual bytes against an independently trusted declaration.
//! These are local components, not an installed storage service. Hosts must
//! capture callers and supply memory/lifecycle composition. Durable state,
//! qualified provider identity and authority issuance are not yet built.

pub mod caffeine;
pub mod local;
pub mod references;
