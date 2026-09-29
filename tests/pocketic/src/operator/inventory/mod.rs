//! Local publisher inspection; no updates, funding or provider effects.

mod input;
mod observation;
mod selection;

use super::{CommandResult, Failure};
use serde_json::json;

const USAGE: &str = "blob-fixture-inventory inspect --server LOOPBACK_IP:PORT --instance ID --canister PRINCIPAL --caller PRINCIPAL --tenant PRINCIPAL --namespace DECIMAL --inventory PREPARED_INVENTORY.json\nQueries blob_upload_capacity, lookup_content and blob_reference_capacity sequentially. Caller is simulated and must equal tenant. Input is the report from prepare_upload --inventory (or a snapshot's inventory.json), not its source declaration. No file bodies are read.\nExit: 0 complete observation with no observed blocker (not upload permission); 2 invalid arguments; 3 input/query/reply/binding failure; 4 observed capacity, enrollment, pending or retired-content blocker.\n";

/// Inspect a validated prepared inventory using only bounded local fixture queries.
/// Observations can become stale and never reserve capacity or certify admission.
#[must_use]
pub fn run(args: &[String]) -> CommandResult {
    if args == ["--help"] {
        return CommandResult {
            exit_code: 0,
            output: USAGE.into(),
        };
    }
    let result = selection::Selection::parse(args).and_then(|selection| {
        let inventory = input::load(&selection.path)?;
        observation::query(&selection, &inventory)
    });
    let (exit_code, value) = match result {
        Ok(report) => (if report.blocked { 4 } else { 0 }, report.value),
        Err(error) => (
            if error == Failure::Arguments { 2 } else { 3 },
            json!({
                "scope":"pocketic_fixture", "error": error.code(),
            }),
        ),
    };
    CommandResult {
        exit_code,
        output: value.to_string(),
    }
}
