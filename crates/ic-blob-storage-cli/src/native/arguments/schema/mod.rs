//! CLI grammar. Clap owns required/unknown/duplicate flags; handlers own domain policy.
use clap::{Arg, ArgAction, Command};

fn arguments(names: &'static str, required: bool) -> impl Iterator<Item = Arg> {
    names.split_ascii_whitespace().map(move |name| {
        Arg::new(name)
            .long(&name[2..])
            .value_name("VALUE")
            .required(required)
            .action(ArgAction::Set)
    })
}

fn command(name: &'static str, required: &'static str, optional: &'static str) -> Command {
    Command::new(name)
        .after_long_help(help(name))
        .args(arguments(required, true))
        .args(arguments(optional, false))
}

fn help(name: &str) -> &'static str {
    match name {
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
            "Inspect missing prerequisites without issuing a certificate. Even an unblocked assessment is not retry authority."
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
            "--network --url --identity --service --namespace",
            true,
        ))
        .args(arguments(
            if operator { "--operator" } else { "--actor" },
            true,
        ))
        .args(arguments("--root-key", false))
        .mut_arg("--network", |arg| arg.value_parser(["ic", "local"]))
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
    for name in [
        "admit-upload",
        "prepare-upload",
        "revoke-upload",
        "submit-reference",
    ] {
        cli = cli.subcommand(online(name, false, "--request --run-dir", ""));
    }
    for name in [
        "upload-permission",
        "upload-manifest",
        "reference-receipt",
        "reference-status",
    ] {
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
        cli = cli.subcommand(
            online(name, true, required, optional).args(arguments("--cashier --payer", true)),
        );
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
            selected = selected.args(arguments("--gateway --operator-identity", true));
        }
        if matches!(
            name,
            "publish-prepare" | "publish-prepare-resume" | "publish-file-status"
        ) {
            selected = selected.args(arguments("--file-index", true));
        }
        if name == "publish-prepare-resume" {
            selected = selected.args(arguments("--source-run", true));
        }
        if name == "publish-session" {
            selected = selected
                .args(arguments("--max-steps", true))
                .args(arguments("--source-session --browser-selection", false));
        }
        cli = cli.subcommand(selected);
    }
    cli
}

#[cfg(test)]
mod tests;
