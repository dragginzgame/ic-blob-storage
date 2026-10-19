//! Unpublished host tests for local IC fixtures, separate from the published library.

#[cfg(not(target_family = "wasm"))]
pub mod operator;

#[cfg(not(target_family = "wasm"))]
pub mod service_calls;

#[cfg(not(target_family = "wasm"))]
pub mod control_calls;
