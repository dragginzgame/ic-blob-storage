use super::{Cli, help_command};
use clap::{Parser, error::ErrorKind};

#[test]
fn help_navigates_groups_without_loading_inputs() {
    help_command().debug_assert();
    for path in [
        vec![],
        vec!["provider"],
        vec!["app"],
        vec!["app", "catalog"],
        vec!["app", "draft"],
        vec!["app", "prepare-installation"],
        vec!["app", "check-targets"],
        vec!["app", "prepare-install"],
        vec!["app", "inspect-install"],
        vec!["app", "dispatch-install"],
        vec!["app", "observe-install"],
        vec!["certificate"],
        vec!["transfer"],
        vec!["maintenance"],
        vec!["certificate", "dispatch"],
        vec!["maintenance", "dispatch-upgrade"],
    ] {
        for flag in ["-h", "--help"] {
            let args = ["blob"]
                .into_iter()
                .chain(path.iter().copied())
                .chain([flag]);
            let error = help_command().try_get_matches_from(args).err().unwrap();
            assert_eq!(error.kind(), ErrorKind::DisplayHelp);
            assert!(error.to_string().starts_with(concat!(
                "Blob ",
                env!("CARGO_PKG_VERSION"),
                " : "
            )));
        }
    }
    let help = help_command().render_help().to_string();
    assert!(help.contains("overview"));
    assert!(help.contains("maintenance"));
    assert!(!help.contains("dispatch-upgrade"));
}

#[test]
fn groups_and_incomplete_dispatch_never_select_an_operation() {
    for path in [
        vec!["certificate"],
        vec!["maintenance"],
        vec!["app"],
        vec!["app", "draft"],
        vec!["app", "dispatch-install"],
        vec!["certificate", "dispatch"],
        vec!["maintenance", "dispatch-drain"],
    ] {
        let error = Cli::try_parse_from(["blob"].into_iter().chain(path))
            .err()
            .unwrap();
        assert!(matches!(
            error.kind(),
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
                | ErrorKind::MissingRequiredArgument
        ));
    }
}

#[test]
fn application_inspection_requires_a_signer_and_nonzero_draft_id() {
    let target = [
        "--url",
        "https://icp-api.io",
        "--canister",
        "rrkah-fqaaa-aaaaa-aaaaq-cai",
    ];
    let missing = Cli::try_parse_from(["blob", "app", "catalog"].into_iter().chain(target))
        .err()
        .unwrap();
    assert_eq!(missing.kind(), ErrorKind::MissingRequiredArgument);
    let invalid = Cli::try_parse_from(["blob", "app", "draft"].into_iter().chain(target).chain([
        "--identity",
        "not-opened.pem",
        "--id",
        "0",
    ]))
    .err()
    .unwrap();
    assert_eq!(invalid.kind(), ErrorKind::ValueValidation);
}
