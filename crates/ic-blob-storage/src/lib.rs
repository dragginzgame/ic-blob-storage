//! Provider-independent Blob Storage domain operations.
//!
//! The first slice implements bounded references for one registered object.
//! It is an in-memory domain component, not an installed storage service.
//! The service library will own authorization rules, persistent state and unique
//! provider-object resolution. Hosts capture callers, supply memory allocations
//! and compose endpoints and lifecycle hooks. Those integrations are not yet built.

pub mod references;
