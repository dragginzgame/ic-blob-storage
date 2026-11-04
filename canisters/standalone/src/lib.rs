//! Explicit standalone canister host for shared blob-storage workflows.
mod api;
mod ops;

/// Export the external provider certificate Candid contract for deployment tooling.
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn provider_candid_interface() -> String {
    api::interface()
}
