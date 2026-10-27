//! Unpublished host tests for local IC fixtures, separate from the published library.
#![cfg(not(target_family = "wasm"))]

pub mod operator;

pub mod service_calls;

pub mod control_calls;

pub mod historical;

pub mod fixture_calls;
pub mod provider_calls;
