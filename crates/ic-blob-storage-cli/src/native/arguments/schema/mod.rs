//! CLI grammar and typed admission; handlers retain request and trust policy.
use clap::builder::{PossibleValuesParser, StringValueParser, TypedValueParser};
use clap::{Arg, ArgAction, Command};
use ic_platform::Principal;
use std::{
    fmt::Display,
    num::{NonZeroU64, NonZeroU128},
    path::PathBuf,
    str::FromStr,
};
use url::Url;

// Canonical spelling is product policy; Clap owns invocation, storage and errors.
fn positive<T>(maximum: T) -> impl TypedValueParser<Value = T>
where
    T: FromStr + Display + Ord + Clone + Send + Sync + 'static,
{
    StringValueParser::new().try_map(move |text| {
        let value = crate::native::parsing::positive::<T>(&text)
            .map_err(|_| "invalid canonical positive integer")?;
        if value > maximum {
            Err("value exceeds command limit")
        } else {
            Ok(value)
        }
    })
}
fn principal(text: &str) -> Result<Principal, &'static str> {
    crate::native::parsing::principal(text).map_err(|_| "invalid canonical principal")
}
fn ordinary_principal(text: &str) -> Result<Principal, &'static str> {
    let value = principal(text)?;
    if value == Principal::anonymous() || value == Principal::management_canister() {
        Err("ordinary principal required")
    } else {
        Ok(value)
    }
}
fn file_index(text: &str) -> Result<usize, &'static str> {
    let index = text.parse::<usize>().map_err(|_| "invalid file index")?;
    if index.to_string() != text || index >= crate::native::publish_inputs::MAX_FILES {
        Err("invalid file index")
    } else {
        Ok(index)
    }
}
fn argument(command: &str, name: &'static str, required: bool) -> Arg {
    use crate::native::publish_inputs::{
        MAX_FILE_BYTES, MAX_FILES, MAX_TIMEOUT_SECONDS, MAX_TOTAL_BYTES,
    };
    let arg = Arg::new(name)
        .long(&name[2..])
        .required(required)
        .action(ArgAction::Set);
    match name {
        "--identity"
        | "--root-key"
        | "--run-dir"
        | "--inputs"
        | "--request"
        | "--permission"
        | "--body"
        | "--cursor"
        | "--configuration"
        | "--inventory"
        | "--root"
        | "--installation"
        | "--binding"
        | "--manifest"
        | "--uploader-identity"
        | "--operator-identity"
        | "--source-run"
        | "--source-session"
        | "--browser-selection" => arg.value_parser(clap::value_parser!(PathBuf)),
        "--network" => arg.value_parser(
            PossibleValuesParser::new(["ic", "local"])
                .map(|value| if value == "ic" { "ic" } else { "local" }),
        ),
        "--url" => arg.value_parser(clap::value_parser!(Url)),
        "--gateway" if command != "revoke-gateway" => arg.value_parser(clap::value_parser!(Url)),
        "--service" | "--operator" | "--actor" | "--cashier" | "--payer" | "--caller"
        | "--owner" | "--gateway" => {
            if matches!(command, "account-link-inputs" | "installation-check") {
                arg.value_parser(principal)
            } else {
                arg.value_parser(ordinary_principal)
            }
        }
        "--namespace" | "--operation" | "--offered" | "--reference" | "--target-balance" => {
            arg.value_parser(positive(u128::MAX))
        }
        "--daily-limit" => arg.value_parser(positive(NonZeroU128::MAX)),
        "--expiry" => arg.value_parser(positive(NonZeroU64::MAX)),
        "--sequence" => arg.value_parser(positive(u64::MAX)),
        "--max-bytes" => arg.value_parser(positive(
            NonZeroU64::new(MAX_FILE_BYTES).expect("positive ceiling"),
        )),
        "--max-total-bytes" => arg.value_parser(positive(
            NonZeroU64::new(MAX_TOTAL_BYTES).expect("positive ceiling"),
        )),
        "--timeout-seconds" => arg.value_parser(positive(MAX_TIMEOUT_SECONDS)),
        "--file-index" => arg.value_parser(file_index),
        "--max-queries" => arg.value_parser(positive(match command {
            "publish-file-status" => 4,
            "publish-map" => 2 * MAX_FILES as u64 + 2,
            _ => MAX_FILES as u64 + 1,
        })),
        "--max-steps" => arg.value_parser(positive(
            crate::native::publish_session::budget::step_limit(MAX_FILES)
                .expect("bounded file ceiling"),
        )),
        "--kind" => arg.value_parser(|text: &str| {
            crate::native::account::kind(text).map_err(|_| "invalid account observation kind")
        }),
        "--filter" => arg.value_parser(|text: &str| {
            crate::native::upload_history::filter(text).map_err(|_| "invalid upload filter")
        }),
        "--action" => arg.value_parser(PossibleValuesParser::new(["retain", "release"]).map(
            |value| {
                if value == "retain" {
                    ic_blob_storage_contracts::dto::reference::ReferenceAction::Retain
                } else {
                    ic_blob_storage_contracts::dto::reference::ReferenceAction::Release
                }
            },
        )),
        _ => arg.value_name("VALUE"),
    }
}
fn arguments(
    command: &'static str,
    names: &'static str,
    required: bool,
) -> impl Iterator<Item = Arg> {
    names
        .split_ascii_whitespace()
        .map(move |name| argument(command, name, required))
}

fn command(name: &'static str, required: &'static str, optional: &'static str) -> Command {
    Command::new(name)
        .after_long_help(help(name))
        .args(arguments(name, required, true))
        .args(arguments(name, optional, false))
}

fn help(name: &str) -> &'static str {
    match name {
        "metrics" => {
            "Standalone heap telemetry only. Queries do not record; traps can roll observations back. Compare only matching lifecycle windows with no intervening snapshot restore. Counts prove neither stored bytes nor provider billing."
        }
        "account-link-inputs" => {
            "Offline provider terms only; no signing, linking, funding or financial-cap proof."
        }
        "installation-check" => {
            "Offline validation and retained configuration packets; no deployment, memory grant or provider qualification."
        }
        "upload-inputs" | "publish-inputs" => {
            "Freeze verified bodies and caller-supplied identities offline. No allocation, installed-state proof or provider effect. Partial output is retained and cannot be overwritten."
        }
        "reference-inputs" => {
            "Encode exact reference/download requests from the saved permission. No identity allocation, mutation, liveness proof or retry authority."
        }
        "admit-upload" | "revoke-upload" => {
            "Tenant-authorized update. Persist exact signed intent before one dispatch; no polling or retry. Preserve partial and uncertain journals."
        }
        "prepare-upload" => {
            "Uploader-authorized manifest preparation. Persist exact signed intent before one dispatch; no certificate, provider upload or automatic retry."
        }
        "upload-permission" | "upload-manifest" => {
            "Inspect the exact saved permission. Historical state does not authorize resubmission or establish stored bytes."
        }
        "submit-reference" => {
            "Submit the exact retained reference command once. Lost replies require receipt inspection; never allocate a replacement operation to retry."
        }
        "reference-receipt" | "reference-status" => {
            "Read the exact reference identity without mutation. Observation is not a retention lease or retry permit."
        }
        "download" => {
            "One replicated descriptor and one bounded provider GET. Only root-verified bytes become body.bin; partial output remains body.part. No automatic retry."
        }
        "verify-upload" => {
            "Compare a local body with the saved declaration. This does not establish provider retention or issue a certificate."
        }
        "certificate-assessment" => {
            "Inspect current issuance eligibility without issuing a certificate. A successful assessment is not retry authority."
        }
        "upload-history" | "funding-history" => {
            "One bounded page. Retain a non-null cursor for explicit continuation, including empty filtered pages; no automatic pagination or provider calls."
        }
        "publish-check" => {
            "Retain bounded capacity/content observations. Existing roots require recovery, retain or retirement handling; no reservation, mutation or publication authority."
        }
        "publish-map" | "publish-file-status" => {
            "Authenticate current scope and exact reference liveness. Sequential observations are not a publication lease or provider availability proof; incomplete batches emit no map."
        }
        "publish-prepare" | "publish-prepare-resume" => {
            "At most two original setup updates and four queries per file. Resume binds --source-run and never resubmits a claimed update. Preserve original journals; no certificate or provider transfer."
        }
        "publish-prepare-batch" => {
            "Serial per-file preparation stops at the first blocker or uncertainty. Never resume the whole batch; use publish-prepare-resume with the original file journal."
        }
        "publish-session" => {
            "Bounded JSON-lines prepare/transfer/status/map session. Retain browser handoff journals for lost replies. Registration is not provider availability; no automatic retry or publication lease."
        }
        "sync-gateways" | "cancel-gateway-sync" | "revoke-gateway" => {
            "Operator-authorized control with retained intent. Uncertain results require original-state inspection, never automatic redispatch."
        }
        _ => {
            "Authenticated observation only. Local accounting and provider acknowledgments do not establish independent account credit or authorize a paid effect."
        }
    }
}

fn online(
    name: &'static str,
    operator: bool,
    required: &'static str,
    optional: &'static str,
) -> Command {
    command(name, required, optional)
        .args(arguments(
            name,
            "--network --url --identity --service --namespace",
            true,
        ))
        .args(arguments(
            name,
            if operator { "--operator" } else { "--actor" },
            true,
        ))
        .args(arguments(name, "--root-key", false))
}

pub(super) fn cli() -> Command {
    let cli = Command::new("blob-storage")
        .args_conflicts_with_subcommands(true)
        .arg(
            Arg::new("version")
                .long("version")
                .action(ArgAction::SetTrue)
                .exclusive(true),
        )
        .arg_required_else_help(true)
        .after_long_help("Use COMMAND --help for operation constraints. New run directories refuse existing or partial output and CR/LF paths. Preserve original journals after uncertainty; never infer retry authority from a missing reply. See docs/operator-guide.md for recovery procedures.");
    publishing(operator(tenant(offline(cli))))
}

fn offline(mut cli: Command) -> Command {
    for (name, required) in [
        (
            "account-link-inputs",
            "--cashier --caller --owner --payer --daily-limit --expiry --run-dir",
        ),
        (
            "installation-check",
            "--configuration --service --project --release --run-dir",
        ),
        (
            "upload-inputs",
            "--installation --binding --manifest --body --max-bytes --run-dir",
        ),
        (
            "publish-inputs",
            "--inventory --root --installation --max-bytes --max-total-bytes --run-dir",
        ),
        (
            "reference-inputs",
            "--permission --action --reference --operation --run-dir",
        ),
    ] {
        cli = cli.subcommand(command(name, required, ""));
    }
    cli
}

fn tenant(mut cli: Command) -> Command {
    for (name, kind) in crate::native::upload_setup::Kind::COMMANDS {
        let required = if kind.mutation() {
            "--request --run-dir"
        } else {
            "--request"
        };
        cli = cli.subcommand(online(name, false, required, ""));
    }
    cli = cli.subcommand(online("submit-reference", false, "--request --run-dir", ""));
    for (name, _) in crate::native::references::Kind::COMMANDS {
        cli = cli.subcommand(online(name, false, "--request", ""));
    }
    cli.subcommand(online(
        "download",
        false,
        "--request --project --gateway --max-bytes --run-dir",
        "",
    ))
    .subcommand(online(
        "verify-upload",
        false,
        "--permission --body --max-bytes",
        "",
    ))
    .subcommand(online("certificate-assessment", false, "--permission", ""))
    .subcommand(online("upload-history", true, "--filter", "--cursor"))
}

fn operator(mut cli: Command) -> Command {
    cli = cli.subcommand(online("metrics", true, "", ""));
    for (name, required, optional) in [
        ("status", "", ""),
        ("funding-history", "", "--cursor"),
        ("inspect-account", "--kind", ""),
        (
            "funding-assessment",
            "--operation --offered",
            "--target-balance",
        ),
        (
            "funding-outcome",
            "--operation --offered",
            "--target-balance",
        ),
        ("sync-gateways", "--run-dir", ""),
        ("cancel-gateway-sync", "--run-dir --sequence", ""),
        ("revoke-gateway", "--run-dir --gateway", ""),
    ] {
        cli = cli.subcommand(online(name, true, required, optional).args(arguments(
            name,
            "--cashier --payer",
            true,
        )));
    }
    cli
}

fn publishing(mut cli: Command) -> Command {
    for name in [
        "publish-check",
        "publish-map",
        "publish-file-status",
        "publish-prepare",
        "publish-prepare-resume",
        "publish-prepare-batch",
        "publish-session",
    ] {
        let mut selected = online(
            name,
            false,
            "--inputs --max-bytes --max-total-bytes --timeout-seconds --run-dir",
            "",
        );
        selected = selected.args(arguments(
            name,
            if matches!(
                name,
                "publish-check" | "publish-map" | "publish-file-status"
            ) {
                "--max-queries"
            } else {
                "--uploader-identity"
            },
            true,
        ));
        if matches!(
            name,
            "publish-map" | "publish-file-status" | "publish-session"
        ) {
            selected = selected.args(arguments(name, "--gateway --operator-identity", true));
        }
        if matches!(
            name,
            "publish-prepare" | "publish-prepare-resume" | "publish-file-status"
        ) {
            selected = selected.args(arguments(name, "--file-index", true));
        }
        if name == "publish-prepare-resume" {
            selected = selected.args(arguments(name, "--source-run", true));
        }
        if name == "publish-session" {
            selected = selected
                .args(arguments(name, "--max-steps", true))
                .args(arguments(
                    name,
                    "--source-session --browser-selection",
                    false,
                ));
        }
        cli = cli.subcommand(selected);
    }
    cli
}

#[cfg(test)]
mod tests;
