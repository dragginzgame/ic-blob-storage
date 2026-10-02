//! Offline complete candidate validation and init encoding; no platform or provider effect.
use super::{Failure, artifacts::Run, candidate_candid, read};
use candid::Principal;
use ic_blob_storage::{
    dto::configuration::ServiceInstallationInput,
    model::identity::ContentDigest,
    ops::service::installation::{ServiceInstallationCandidate, ValidatedServiceInstallation},
};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};

const CONFIGURATION_BYTES: usize = candidate_candid::MAX_BYTES;

fn principal(value: &str) -> Result<Principal, Failure> {
    let result = Principal::from_text(value).map_err(|_| Failure::Arguments)?;
    if result.to_text() != value {
        return Err(Failure::Arguments);
    }
    Ok(result)
}

use candidate_candid::decode;

pub(super) fn run(args: &[String]) -> Result<Value, Failure> {
    let mut flags = BTreeMap::new();
    for pair in args[1..].chunks(2) {
        if pair.len() != 2 || flags.insert(pair[0].as_str(), pair[1].as_str()).is_some() {
            return Err(Failure::Arguments);
        }
    }
    let mut take = |key| flags.remove(key).ok_or(Failure::Arguments);
    let path = Path::new(take("--configuration")?);
    let service = principal(take("--service")?)?;
    let project = take("--project")?;
    let verifier = principal(take("--verifier")?)?;
    let trusted_uploader = principal(take("--trusted-uploader")?)?;
    let release = take("--release")?;
    let directory = Path::new(take("--run-dir")?);
    if !flags.is_empty() {
        return Err(Failure::Arguments);
    }
    let bytes = read(path, CONFIGURATION_BYTES as u64)?;
    let configuration = decode(&bytes)?;
    let candidate = ServiceInstallationCandidate {
        configuration,
        project,
        completion_verifier: verifier,
        trusted_uploader,
        release,
    };
    // This is a proposed platform identity, not an authenticated actual host.
    ValidatedServiceInstallation::new(service, candidate).map_err(|_| Failure::Arguments)?;
    let input = ServiceInstallationInput {
        configuration,
        project: project.into(),
        completion_verifier: verifier,
        trusted_uploader,
    };
    let installation = candid::encode_one(input).map_err(|_| Failure::File)?;
    let report = json!({"schema":1,"observation":"local_installation_check",
        "configuration_validated":true,"configuration_file":"configuration.candid",
        "configuration_sha256":ContentDigest::compute(&bytes).to_string(),
        "installation_file":"installation.candid",
        "installation_sha256":ContentDigest::compute(&installation).to_string(),
        "service":service.to_text(),"operator":configuration.operator.to_text(),
        "payment_account":configuration.payment_account.to_text(),
        "cashier":configuration.billing.cashier.to_text(),
        "namespace":configuration.namespace.to_string(),"project":project,
        "completion_verifier":verifier.to_text(),"trusted_uploader":trusted_uploader.to_text(),"expected_host_release":release,
        "authenticated":false,"platform_identity_checked":false,
        "compiled_release_checked":false,"stable_memory_allocated":false,"host_init_encoded":true,
        "installation_dispatched":false,"provider_dispatched":false,"funding_dispatched":false,
        "namespace_provisioned":false,"provider_qualified":false,
        "operational_recovery_qualified":false,"retry_authorized":false});
    let output = Run::create(directory)?;
    output.bytes("configuration.candid", &bytes)?;
    output.bytes("installation.candid", &installation)?;
    output.json("summary.json", &report)?;
    Ok(report)
}

#[cfg(test)]
mod tests;
