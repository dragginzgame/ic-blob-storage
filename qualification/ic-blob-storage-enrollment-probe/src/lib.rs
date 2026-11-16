//! Single-use owner-origin Cashier enrollment probe, not a Blob service or funding engine.
//! No uploads, certificates, arbitrary calls, attached cycles, timers or retries.

#[cfg(target_arch = "wasm32")]
mod host;
pub mod protocol;
#[cfg(any(target_arch = "wasm32", test))]
mod state;
