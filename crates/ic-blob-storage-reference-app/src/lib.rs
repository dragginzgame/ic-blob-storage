//! Standalone reference asset catalog over a separate experimental Blob host.
//! Publication is the owning user's decision, not an independent read-verification proof.
//! Durable drafts precede calls; cancellation is irreversible and cleanup uses original IDs.

#[cfg(target_arch = "wasm32")]
mod host;
pub mod protocol;
#[cfg(target_arch = "wasm32")]
mod state;
