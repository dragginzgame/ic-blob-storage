//! Local-fixture tools with separate passive diagnosis and explicit observation commands.

mod action;
mod funding_lookup;
mod funding_preview;
pub use funding_lookup::run as run_funding_lookup;
mod model;
mod ops;
pub use funding_preview::run as run_funding_preview;

pub use action::{run_refresh, run_sync};

use model::Command;
use serde_json::json;

/// Explicit selection is mandatory; caller identity is simulated by `PocketIC`.
pub const USAGE: &str = "blob-fixture-status status|check --server LOOPBACK_IP:PORT --instance ID --canister PRINCIPAL --caller PRINCIPAL --kind authority --namespace DECIMAL\nblob-fixture-status status|check --server LOOPBACK_IP:PORT --instance ID --canister PRINCIPAL --caller PRINCIPAL --kind funding --peer PRINCIPAL\nLocal PocketIC fixtures only. Queries operator_status; never updates or retries a failed diagnosis.\nstatus: 0 for a valid report. check: 4 when reported blockers exist (not service qualification).\nErrors: 2 invalid arguments; 3 transport, query, permission, reply or binding failure.\n";

/// Machine-readable command result; output is one JSON value except for help.
pub struct CommandResult {
    /// Process exit code documented by [`USAGE`].
    pub exit_code: u8,
    /// JSON report/error, or usage text for the sole `--help` argument.
    pub output: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Failure {
    Arguments,
    Transport,
    QueryRejected,
    Denied,
    ReplyTooLarge,
    InvalidReply,
    Binding,
    Conflict,
    InvalidRequest,
}

impl Failure {
    const fn code(self) -> &'static str {
        match self {
            Self::Arguments => "invalid_arguments",
            Self::Transport => "transport_failure",
            Self::QueryRejected => "query_rejected",
            Self::Denied => "denied",
            Self::ReplyTooLarge => "reply_too_large",
            Self::InvalidReply => "invalid_reply",
            Self::Binding => "binding_mismatch",
            Self::Conflict => "request_conflict",
            Self::InvalidRequest => "invalid_request",
        }
    }
}

/// Run an explicitly bound, query-only diagnosis against an existing local instance.
/// This does not create, advance, upgrade or delete a `PocketIC` instance.
#[must_use]
pub fn run(args: &[String]) -> CommandResult {
    if args == ["--help"] {
        return CommandResult {
            exit_code: 0,
            output: USAGE.into(),
        };
    }
    let selection = Command::parse(args);
    let result = selection.as_ref().map_err(|e| *e).and_then(|command| {
        let bytes = ops::query(&command.target)?;
        ops::report(&command.target, &bytes).map(|report| (command, report))
    });
    let (exit_code, value) = match result {
        Ok((command, report)) => (
            if command.check && report.blocked {
                4
            } else {
                0
            },
            json!({
                "scope":"pocketic_fixture", "caller_identity":"simulated",
                "service_qualification":"not_assessed", "method":"operator_status",
                "mode":"query", "check_scope":"reported_blockers_only",
                "target": ops::target_json(&command.target), "status":report.value,
            }),
        ),
        Err(failure) => (
            if failure == Failure::Arguments { 2 } else { 3 },
            json!({
                "scope":"pocketic_fixture", "error":failure.code(),
                "target":selection.as_ref().ok().map(|c| ops::target_json(&c.target)),
            }),
        ),
    };
    CommandResult {
        exit_code,
        output: value.to_string(),
    }
}
