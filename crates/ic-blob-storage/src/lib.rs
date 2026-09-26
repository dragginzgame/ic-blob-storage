//! Independent blob-storage service library for Internet Computer canisters.
//!
//! Content identities, pure policy, transient lifecycle models and bounded
//! Caffeine reply decoders are available. Storage workflows,
//! provider effects, canister endpoints, and lifecycle hooks are not implemented.
//!
//! The intended service owns tenant authorization, content references, quotas,
//! provider effects, billing, retention, and deletion. Standalone and managed
//! canister adapters will delegate to the same implementation after the service
//! contract is frozen.

pub mod model;
pub mod ops;
pub mod policy;
