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
                "method":"preview_funding", "mode":"query", "target":super::ops::target_json(&selection.target), "preview":report.value,
            }),
        ),
        Err(error) => (
            if error == super::Failure::Arguments {
                2
            } else {
                3
            },
            json!({"scope":"pocketic_fixture", "error":error.code(), "target":selection.as_ref().ok().map(|s| super::ops::target_json(&s.target))}),
        ),
    };
    CommandResult {
        exit_code,
        output: value.to_string(),
    }
}
