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

fn publication() -> Vec<String> {
    [
        "blob-storage",
        "publish-check",
        "--network",
        "ic",
        "--url",
        "https://icp-api.io",
        "--identity",
        "unused.pem",
        "--service",
        "rrkah-fqaaa-aaaaa-aaaaq-cai",
        "--namespace",
        "1",
        "--actor",
        "rrkah-fqaaa-aaaaa-aaaaq-cai",
        "--inputs",
        "missing",
        "--run-dir",
        "out",
        "--max-bytes",
        "1073741824",
        "--max-total-bytes",
        "1099511627776",
        "--max-queries",
        "4097",
        "--timeout-seconds",
        "3600",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

#[test]
fn typed_values_keep_canonical_spelling_and_command_specific_ceilings() {
    let original = publication();
    let matches = cli().try_get_matches_from(&original).unwrap();
    let (_, flags) = matches.subcommand().unwrap();
    assert_eq!(*flags.get_one::<u128>("--namespace").unwrap(), 1);
    assert_eq!(
        flags.get_one::<NonZeroU64>("--max-bytes").unwrap().get(),
        1_073_741_824
    );
    for (flag, invalid) in [
        ("--namespace", "0"),
        ("--namespace", "01"),
        ("--namespace", "+1"),
        ("--namespace", "340282366920938463463374607431768211456"),
        ("--max-bytes", "1073741825"),
        ("--max-total-bytes", "1099511627777"),
        ("--max-queries", "4098"),
        ("--timeout-seconds", "3601"),
        ("--actor", "aaaaa-aa"),
        ("--service", "2vxsx-fae"),
    ] {
        let mut args = original.clone();
        let index = args.iter().position(|s| s == flag).unwrap();
        args[index + 1] = invalid.into();
        assert!(cli().try_get_matches_from(args).is_err(), "{flag}");
    }
    let mut args = original;
    args[1] = "publish-file-status".into();
    args.extend(
        [
            "--file-index",
            "0",
            "--gateway",
            "https://example.test",
            "--operator-identity",
            "unused.pem",
        ]
        .map(str::to_owned),
    );
    let index = args.iter().position(|s| s == "--max-queries").unwrap();
    assert!(cli().try_get_matches_from(&args).is_err());
    args[index + 1] = "4".into();
    assert!(cli().try_get_matches_from(&args).is_ok());
    let index = args.iter().position(|s| s == "--file-index").unwrap();
    for invalid in ["00", "4096"] {
        args[index + 1] = invalid.into();
        assert!(cli().try_get_matches_from(&args).is_err());
    }
}

#[cfg(unix)]
#[test]
fn production_invocation_keeps_non_utf8_paths_native() {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};
    let path = OsString::from_vec(b"identity-\xff.pem".to_vec());
    let mut args: Vec<OsString> = [
        "metrics",
        "--network",
        "ic",
        "--url",
        "https://icp-api.io",
        "--identity",
        "unused",
        "--service",
        "rrkah-fqaaa-aaaaa-aaaaq-cai",
        "--namespace",
        "1",
        "--operator",
        "rrkah-fqaaa-aaaaa-aaaaq-cai",
    ]
    .into_iter()
    .map(OsString::from)
    .collect();
    args[6] = path.clone();
    let matches = crate::native::arguments::invocation(&args)
        .unwrap()
        .unwrap();
    let (name, flags) = matches.subcommand().unwrap();
    let (_, options) = crate::native::arguments::Options::from_matches(name, flags).unwrap();
    assert_eq!(options.identity.as_os_str(), path);
}
