//! Local observation command; action and subsequent diagnosis have separate outcomes.
mod model;
mod ops;

use super::CommandResult;
use model::{Action, Selection};
use serde_json::json;

fn usage(action: Action) -> String {
    let (binary, command, account) = match action {
        Action::Balance => ("blob-fixture-refresh", "refresh", " --account PRINCIPAL"),
        Action::Gateway => ("blob-fixture-sync", "sync", ""),
    };
    format!(
        "{binary} dry-run|{command} --server LOOPBACK_IP:PORT --instance ID --canister PRINCIPAL --caller PRINCIPAL --kind authority --namespace DECIMAL --source PRINCIPAL{account} --revision DECIMAL --sequence DECIMAL\nLocal PocketIC fixture only. Dry-run queries admission; explicit action updates once then queries operator_status separately. Never funds or selects a production provider.\nSequence is the expected next lifetime attempt, starting at 1. A consumed sequence cannot dispatch again. No automatic retry. Gateway revision starts at 0; balance revision starts at 1.\nExit: 0 preview eligible or action completed; 2 arguments; 5 typed operation failure; 6 unknown outcome/read failure; 7 completed action with failed post-status.\n"
    )
}

/// Explicitly preview or execute one local fixture balance observation.
/// Transport failure is an unknown outcome, never permission to repeat an action.
#[must_use]
pub fn run_refresh(args: &[String]) -> CommandResult {
    run(args, Action::Balance)
}

/// Explicitly preview or execute one local fixture gateway synchronization.
/// A successful preview grants no future admission or permission to retry.
#[must_use]
pub fn run_sync(args: &[String]) -> CommandResult {
    run(args, Action::Gateway)
}

fn run(args: &[String], action: Action) -> CommandResult {
    if args == ["--help"] {
        return CommandResult {
            exit_code: 0,
            output: usage(action),
        };
    }
    let selection = match Selection::parse(args, action) {
        Ok(selection) => selection,
        Err(error) => {
            return CommandResult {
                exit_code: 2,
                output: json!({"scope":"pocketic_fixture", "error":error.code()}).to_string(),
            };
        }
    };
    let outcome = ops::execute(&selection);
    // This is a separate, passive diagnosis. Its failure never feeds back into
    // execute, and cannot erase the action's acknowledged or uncertain outcome.
    let status = (!selection.dry_run).then(|| {
        super::ops::query(&selection.target)
            .and_then(|bytes| super::ops::report(&selection.target, &bytes))
    });
    let exit_code = match (&outcome, &status) {
        (Ok(Ok(())), Some(Err(_))) => 7,
        (Ok(Ok(())), _) => 0,
        (Ok(Err(_)), _) => 5,
        (Err(_), _) => 6,
    };
    let action = match outcome {
        Ok(Ok(())) => json!({"outcome": if selection.dry_run {"eligible"} else {"completed"}}),
        Ok(Err(error)) => json!({"outcome":"failed", "failure":error}),
        Err(error) => {
            json!({"outcome": if selection.dry_run {"unknown"} else {"uncertain"}, "error":error.code()})
        }
    };
    let post_status = status.map(|status| match status {
        Ok(report) => json!({"status":report.value}),
        Err(error) => json!({"error":error.code()}),
    });
    CommandResult {
        exit_code,
        output: json!({
            "scope":"pocketic_fixture", "caller_identity":"simulated",
            "service_qualification":"not_assessed",
            "method": ops::method(&selection),
            "mode": if selection.dry_run {"query"} else {"update"},
            "target":super::ops::target_json(&selection.target),
            "request": ops::request_json(&selection),
            "action":action, "post_status":post_status,
        })
        .to_string(),
    }
}
