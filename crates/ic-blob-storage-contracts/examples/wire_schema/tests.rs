use super::*;
#[test]
fn modes_help_and_extra_arguments_are_owned_by_clap() {
    cli().debug_assert();
    for mode in [
        "metadata",
        "client",
        "standalone-metadata",
        "standalone-client",
        "adapter",
        "runtime",
    ] {
        assert!(cli().try_get_matches_from(["wire_schema", mode]).is_ok());
        assert!(
            cli()
                .try_get_matches_from(["wire_schema", mode, "extra"])
                .is_err()
        );
    }
    assert!(
        cli()
            .try_get_matches_from(["wire_schema", "unknown"])
            .is_err()
    );
    assert_eq!(
        cli()
            .try_get_matches_from(["wire_schema", "--help"])
            .unwrap_err()
            .kind(),
        clap::error::ErrorKind::DisplayHelp
    );
}
