//! Independent blob-storage service library for Internet Computer canisters.
//!
//! This crate is a repository scaffold. It currently exports no storage API,
//! provider implementation, canister endpoints, or lifecycle hooks.
//!
//! The intended service owns tenant authorization, content references, quotas,
//! provider effects, billing, retention, and deletion. Standalone and managed
//! canister adapters will delegate to the same implementation after the service
//! contract is frozen.
