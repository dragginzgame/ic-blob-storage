//! Query-only funding preview. No route to the raw transfer experiment exists here.
mod model;
mod ops;
use super::CommandResult;
use serde_json::json;

const USAGE: &str = "blob-fixture-funding-preview dry-run --server LOOPBACK_IP:PORT --instance ID --canister PRINCIPAL --caller PRINCIPAL --kind funding --peer PRINCIPAL --id DECIMAL --amount DECIMAL --revision DECIMAL\nQueries preview_funding only. No transfer, reservation or retry. Caller is simulated.\nExit: 0 unblocked assessment (not effect authority); 2 invalid arguments; 3 transport/query/reply/authority/binding failure; 4 valid blocked assessment.\n";

/// Query a bound local funding journal for passive admission blockers.
/// Missing spendability is never supplied by a CLI argument or gross cycle balance.
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
                "method":"preview_funding", "mode":"query",
                "target":super::ops::target_json(&selection.target), "preview":value,
            })
        },
    )
}
