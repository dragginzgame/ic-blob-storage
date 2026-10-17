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
    super::query_command(
        args,
        USAGE,
        model::Selection::parse,
        |selection| &selection.target,
        ops::query,
        |selection, value| {
            json!({
                "scope":"pocketic_fixture", "caller_identity":"simulated", "service_qualification":"not_assessed",
                "lookup_scope":"retained_local_journal", "method":"lookup_funding", "mode":"query",
                "target":super::ops::target_json(&selection.target), "lookup":value,
            })
        },
    )
}
