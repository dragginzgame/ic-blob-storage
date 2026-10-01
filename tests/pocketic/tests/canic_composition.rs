//! Managed Canic installation, authority and same-release lifecycle evidence.
#![cfg(not(target_family = "wasm"))]
mod account_native_cli;
mod authenticated_cli;
mod canic_managed;
mod observation_provider;
mod submission_proxy;
