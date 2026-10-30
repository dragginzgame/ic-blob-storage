//! Offline complete candidate validation and init encoding; no platform or provider effect.

use super::arguments::value;
use super::{Failure, artifacts::Run, exact_wire, read};
use ic_blob_storage_contracts::configuration::ServiceInstallationCandidate;
use ic_blob_storage_contracts::configuration::ValidatedInstallationInput;
use ic_blob_storage_contracts::dto::configuration::ServiceInstallationInput;
use ic_blob_storage_contracts::identity::ContentDigest;
use serde_json::{Value, json};
use std::path::PathBuf;

const CONFIGURATION_BYTES: usize = exact_wire::INSTALLATION_BYTES;

pub(super) fn run(flags: &clap::ArgMatches) -> Result<Value, Failure> {
    let path = value::<PathBuf>(flags, "--configuration");
    let service = value(flags, "--service");
    let project = value::<String>(flags, "--project");
    let release = value::<String>(flags, "--release");
    let directory = value::<PathBuf>(flags, "--run-dir");
    let bytes = read(&path, CONFIGURATION_BYTES as u64)?;
    let configuration = serde_json::from_slice(&bytes).map_err(|_| Failure::Arguments)?;
    let candidate = ServiceInstallationCandidate {
        configuration,
        project: &project,
        release: &release,
        platform_installation_version: 0,
    };
    // This is a proposed platform identity, not an authenticated actual host.
    ValidatedInstallationInput::new(service, candidate).map_err(|_| Failure::Arguments)?;
    let input = ServiceInstallationInput {
        configuration,
        project: project.clone(),
    };
    let configuration_bytes =
        ic_blob_storage_contracts::document::encode(configuration).map_err(|_| Failure::File)?;
    let installation = ic_blob_storage_contracts::wire::encode_request("blob_install", input)
        .map_err(|_| Failure::File)?;
    let report = json!({"schema":1,"observation":"local_installation_check",
        "configuration_validated":true,"configuration_file":"configuration.wire",
        "proposal_file":"configuration.json","proposal_sha256":ContentDigest::compute(&bytes).to_string(),"configuration_sha256":ContentDigest::compute(&configuration_bytes).to_string(),
        "installation_file":"installation.wire",
        "installation_sha256":ContentDigest::compute(&installation).to_string(),
        "service":service.to_text(),"operator":configuration.operator.to_text(),
        "payment_account":configuration.payment_account.to_text(),
        "cashier":configuration.billing.cashier.to_text(),
        "namespace":configuration.namespace.to_string(),"project":project,
        "expected_host_release":release,
        "authenticated":false,"platform_identity_checked":false,
        "compiled_release_checked":false,"stable_memory_allocated":false,"host_init_encoded":true,
        "installation_dispatched":false,"provider_dispatched":false,"funding_dispatched":false,
        "namespace_provisioned":false,"provider_qualified":false,
        "operational_recovery_qualified":false,"retry_authorized":false});
    let output = Run::create(&directory)?;
    output.bytes("configuration.json", &bytes)?;
    output.bytes("configuration.wire", &configuration_bytes)?;
    output.bytes("installation.wire", &installation)?;
    output.json("summary.json", &report)?;
    Ok(report)
}

#[cfg(test)]
mod tests;
