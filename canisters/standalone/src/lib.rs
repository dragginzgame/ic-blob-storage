//! Explicit standalone canister host for shared blob-storage workflows.
mod api;
mod ops;
mod workflow;

/// Export the exact standalone Candid contract for deployment tooling.
#[must_use]
pub fn candid_interface() -> String {
    api::interface()
}
