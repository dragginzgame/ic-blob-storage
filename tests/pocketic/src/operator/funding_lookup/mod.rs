//! Query retained exact intent evidence without repeating a transfer.
mod model;
mod ops;
use super::CommandResult;
use serde_json::json;

const USAGE: &str = "blob-fixture-funding-lookup lookup --server LOOPBACK_IP:PORT --instance ID --canister PRINCIPAL --caller PRINCIPAL --kind funding --peer PRINCIPAL --id DECIMAL --amount DECIMAL --accept DECIMAL --reply MODE --trap-callback true|false\nMODE is the exact fixture reply variant, e.g. Success or InternalError. Queries lookup_funding only; never funds, retries or clears a fence. Absent means missing from this journal, not proof of no effect.\nExit: 0 retained observation (not provider credit); 2 invalid arguments; 3 transport/query/reply/authority/binding/conflict failure; 4 absent or pending evidence. Caller is simulated.\n";

/// Query a local funding operation by its complete original request.
/// A missing or retained result never authorizes retry or independent settlement.
#[must_use]
pub fn run(args: &[String]) -> CommandResult {
    if args == ["--help"] {
        return CommandResult {
            exit_code: 0,
            output: USAGE.into(),
        };
    }
    let selection = model::Selection::parse(args);
    let result = selection
        .as_ref()
        .map_err(|e| *e)
        .and_then(|s| ops::query(s).map(|r| (s, r)));
    let (exit_code, value) = match result {
        Ok((selection, report)) => (
            if report.blocked { 4 } else { 0 },
            json!({
                "scope":"pocketic_fixture", "caller_identity":"simulated", "service_qualification":"not_assessed",
                "lookup_scope":"retained_local_journal", "method":"lookup_funding", "mode":"query",
                "target":super::ops::target_json(&selection.target), "lookup":report.value,
            }),
        ),
        Err(error) => (
            if error == super::Failure::Arguments {
                2
            } else {
                3
            },
            json!({
                "scope":"pocketic_fixture", "error":error.code(), "target":selection.as_ref().ok().map(|s| super::ops::target_json(&s.target)),
            }),
        ),
    };
    CommandResult {
        exit_code,
        output: value.to_string(),
    }
}
