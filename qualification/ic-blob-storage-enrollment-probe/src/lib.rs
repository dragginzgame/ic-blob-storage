//! Single-use owner-origin Cashier enrollment probe, not a Blob service or funding engine.
//! No uploads, certificates, arbitrary calls, attached cycles, timers or retries.

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
