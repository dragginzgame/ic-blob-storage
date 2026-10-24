use super::*;

#[test]
fn grammar_requires_complete_unique_scoped_flags_and_supports_help() {
    cli().debug_assert();
    let valid = [
        "blob-storage",
        "installation-check",
        "--configuration",
        "c.json",
        "--service",
        "aaaaa-aa",
        "--project",
        "p",
        "--release",
        "r",
        "--run-dir",
        "out",
    ];
    assert!(cli().try_get_matches_from(valid).is_ok());
    for suffix in [
        &["--service", "aaaaa-aa"][..],
        &["--unknown", "x"],
        &["--actor", "aaaaa-aa"],
    ] {
        assert!(
            cli()
                .try_get_matches_from(valid.into_iter().chain(suffix.iter().copied()))
                .is_err()
        );
    }
    assert!(
        cli()
            .try_get_matches_from(&valid[..valid.len() - 1])
            .is_err()
    );
    for args in [
        vec!["blob-storage", "--help"],
        vec!["blob-storage", "status", "--help"],
    ] {
        assert_eq!(
            cli().try_get_matches_from(args).unwrap_err().kind(),
            clap::error::ErrorKind::DisplayHelp
        );
    }
    assert!(
        cli()
            .try_get_matches_from(["blob-storage", "--version"])
            .is_ok()
    );
    assert!(
        cli()
            .try_get_matches_from(["blob-storage", "--version", "status"])
            .is_err()
    );
}
