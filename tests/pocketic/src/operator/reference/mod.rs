//! Exact local intent copies and query-only receipt recovery, without dispatch.
mod observation;
mod record;

use super::{
    CommandResult, Failure,
    model::{canister, decimal},
    ops::{QueryTarget, query_target},
};
use ic_blob_storage::ops::service::references::REFERENCE_RECEIPT_METHOD;
use serde_json::json;
use std::{collections::BTreeMap, net::SocketAddr, path::Path};

const USAGE: &str = "blob-fixture-reference save --intent INPUT.json --journal EXISTING_DIRECTORY\nblob-fixture-reference inspect --intent INTENT.json --server LOOPBACK_IP:PORT --instance ID --canister PRINCIPAL --caller PRINCIPAL\nSave locks a bounded journal and syncs the exact identity-keyed file and directory before acknowledgment. Exact retries recover the same file; changed arguments conflict. Use an existing durable caller-controlled local directory. Never remove its .writer.lock. IDs and restore authority are not allocated. Inspect queries blob_reference_receipt only. No mutation, upload or funding is sent.\nExit: 0 saved or historical successful receipt; 2 arguments; 3 input/storage/busy/capacity/transport/reply/binding/conflict; 4 absent or recorded failure (service refusals exit 3). Success is not current reference liveness or dispatch authority.\n";

/// Save an explicit local fixture intent or inspect its exact historical receipt.
/// Neither command authorizes mutation, publication or recovery after restoring state.
#[must_use]
pub fn run(args: &[String]) -> CommandResult {
    if args == ["--help"] {
        return CommandResult {
            exit_code: 0,
            output: USAGE.into(),
        };
    }
    match execute(args) {
        Ok((exit_code, value)) => CommandResult {
            exit_code,
            output: value.to_string(),
        },
        Err(error) => CommandResult {
            exit_code: if error == Failure::Arguments { 2 } else { 3 },
            output: json!({
                "scope":"pocketic_fixture","error":error.code(),
            })
            .to_string(),
        },
    }
}

fn execute(args: &[String]) -> Result<(u8, serde_json::Value), Failure> {
    let mode = args.first().ok_or(Failure::Arguments)?;
    if !matches!(mode.as_str(), "save" | "inspect") {
        return Err(Failure::Arguments);
    }
    let mut flags = BTreeMap::new();
    for pair in args[1..].chunks(2) {
        if pair.len() != 2 || flags.insert(pair[0].as_str(), pair[1].as_str()).is_some() {
            return Err(Failure::Arguments);
        }
    }
    let input = flags.remove("--intent").ok_or(Failure::Arguments)?;
    if mode == "save" {
        let directory = flags.remove("--journal").ok_or(Failure::Arguments)?;
        if !flags.is_empty() {
            return Err(Failure::Arguments);
        }
        let saved = record::save(Path::new(input), Path::new(directory))?;
        return Ok((
            0,
            json!({"scope":"pocketic_fixture","saved":saved.path,"intent":saved.record,
                "existing":saved.existing,"persistence":"file_and_directory_synced",
                "restore_authority":"not_established","dispatch":"not_performed"}),
        ));
    }
    let mut take = |name| flags.remove(name).ok_or(Failure::Arguments);
    let server: SocketAddr = take("--server")?.parse().map_err(|_| Failure::Arguments)?;
    if !server.ip().is_loopback() || server.port() == 0 {
        return Err(Failure::Arguments);
    }
    let instance = decimal(take("--instance")?)?;
    let service = canister(take("--canister")?)?;
    let caller = canister(take("--caller")?)?;
    if !flags.is_empty() {
        return Err(Failure::Arguments);
    }
    let record = record::load(Path::new(input))?;
    let request = record.request()?;
    if request.upload.service != service || request.upload.tenant != caller {
        return Err(Failure::Binding);
    }
    let bytes = query_target(
        &QueryTarget {
            server,
            instance,
            canister: service,
            caller,
        },
        REFERENCE_RECEIPT_METHOD,
        candid::encode_one(request).expect("fixed reference input"),
    )?;
    let (exit, observation) = observation::decode(request, &bytes)?;
    Ok((
        exit,
        json!({"scope":"pocketic_fixture","caller_identity":"simulated","mode":"query",
        "intent":record,"observation":observation,"current_liveness":"not_assessed","dispatch":"not_performed"}),
    ))
}
