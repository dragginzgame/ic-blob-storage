use super::{Cli, help_command};
use clap::{Parser, error::ErrorKind};

#[test]
fn help_navigates_groups_without_loading_inputs() {
    help_command().debug_assert();
    for path in [
        vec![],
        vec!["provider"],
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
