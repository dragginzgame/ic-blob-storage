//! Offline complete candidate validation; no host carrier, platform or provider effect.
use super::{Failure, artifacts::Run, read};
use candid::{DecoderConfig, Principal, de::IDLDeserialize};
use ic_blob_storage::{
    dto::configuration::ServiceConfigurationInput,
    model::identity::ContentDigest,
    ops::service::installation::{ServiceInstallationCandidate, ValidatedServiceInstallation},
};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};

const CONFIGURATION_BYTES: usize = 16 * 1024;

fn principal(value: &str) -> Result<Principal, Failure> {
    let result = Principal::from_text(value).map_err(|_| Failure::Arguments)?;
    if result.to_text() != value {
        return Err(Failure::Arguments);
    }
    Ok(result)
}

fn decode(bytes: &[u8]) -> Result<ServiceConfigurationInput, Failure> {
    let mut limits = DecoderConfig::new();
    limits
        .set_decoding_quota(100_000)
        .set_skipping_quota(0)
        .set_max_type_len(64)
        .set_max_header_len(CONFIGURATION_BYTES)
        .set_full_error_message(false);
    let mut decoder =
        IDLDeserialize::new_with_config(bytes, &limits).map_err(|_| Failure::Arguments)?;
    let input = decoder.get_value().map_err(|_| Failure::Arguments)?;
    if !decoder.is_done() {
        return Err(Failure::Arguments);
    }
    decoder.done().map_err(|_| Failure::Arguments)?;
    Ok(input)
}

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
        release,
    };
    // This is a proposed platform identity, not an authenticated actual host.
    ValidatedServiceInstallation::new(service, candidate).map_err(|_| Failure::Arguments)?;
    let report = json!({"schema":1,"observation":"local_installation_check",
        "configuration_validated":true,"configuration_file":"configuration.candid",
        "configuration_sha256":ContentDigest::compute(&bytes).to_string(),
        "service":service.to_text(),"operator":configuration.operator.to_text(),
        "payment_account":configuration.payment_account.to_text(),
        "cashier":configuration.billing.cashier.to_text(),
        "namespace":configuration.namespace.to_string(),"project":project,
        "completion_verifier":verifier.to_text(),"expected_host_release":release,
        "authenticated":false,"platform_identity_checked":false,
        "compiled_release_checked":false,"stable_memory_allocated":false,"host_init_encoded":false,
        "installation_dispatched":false,"provider_dispatched":false,"funding_dispatched":false,
        "namespace_provisioned":false,"provider_qualified":false,
        "operational_recovery_qualified":false,"retry_authorized":false});
    let output = Run::create(directory)?;
    output.bytes("configuration.candid", &bytes)?;
    output.json("summary.json", &report)?;
    Ok(report)
}

#[cfg(test)]
mod tests;
