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
        .args(arguments(required, true))
        .args(arguments(optional, false))
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
        .after_long_help(super::super::SAFETY_HELP);
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
