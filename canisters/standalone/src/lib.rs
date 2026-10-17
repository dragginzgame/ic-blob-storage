//! Explicit standalone canister host for shared blob-storage workflows.
mod api;
mod ops;
mod workflow;

/// Export the external provider certificate Candid contract for deployment tooling.
#[cfg(any(not(target_arch = "wasm32"), feature = "provider-candid-export"))]
#[must_use]
pub fn provider_candid_interface() -> String {
    api::interface()
}
