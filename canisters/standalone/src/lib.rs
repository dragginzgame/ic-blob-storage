//! Explicit standalone canister host. Provider effects and operational recovery remain disabled.
mod api;
pub mod dto;
mod model;
mod ops;
mod workflow;

/// Export the exact standalone Candid contract for deployment tooling.
#[must_use]
pub fn candid_interface() -> String {
    api::interface()
}
