//! Standalone reference asset catalog over a separate experimental Blob host.
//! Publication is the owning user's decision, not an independent read-verification proof.
//! Durable drafts precede calls; cancellation is irreversible and cleanup uses original IDs.

// Platform Runtime exposes IC system calls and transport only on Wasm.
#[cfg(target_arch = "wasm32")]
mod host;
pub mod protocol;
#[cfg_attr(
    not(target_arch = "wasm32"),
    expect(
        dead_code,
        reason = "Native consumers use the protocol; installed IC handlers drive the private state."
    )
)]
mod state;
