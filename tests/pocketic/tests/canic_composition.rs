//! Managed Canic installation, authority and same-release lifecycle evidence.
#![cfg(not(target_family = "wasm"))]
mod account_native_cli;
mod authenticated_cli;
mod canic_managed;
mod funding_assessment_cli;
mod gateway_native_cli;
mod observation_provider;
mod submission_proxy;
mod upload_setup_cli;
