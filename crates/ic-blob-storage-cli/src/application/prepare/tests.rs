use super::*;
use crate::{Cli, Command as RootCommand, application::Command};
use clap::Parser;
use std::fs;

fn arguments(output: &std::path::Path) -> Vec<String> {
    let mut args = [
        "blob",
        "app",
        "prepare-installation",
        "--max-content-bytes",
        "100",
        "--max-chunks",
        "1",
        "--max-headers",
        "1",
        "--max-header-bytes",
        "100",
        "--uploads",
        "64",
        "--references",
        "64",
        "--logical-bytes",
        "100",
        "--exposure-bytes",
        "100",
    ]
    .map(str::to_owned)
    .to_vec();
    for (flag, id) in [
        ("--canister", 1),
        ("--blob-canister", 2),
        ("--manager", 3),
        ("--user", 4),
    ] {
        args.extend([flag.to_owned(), Principal::from_slice(&[id, 1]).to_text()]);
    }
    args.extend(["--output".into(), output.to_str().unwrap().into()]);

    args
}

fn prepare(args: Vec<String>) -> Result<String, String> {
    let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;
    let RootCommand::Application(Command::PrepareInstallation(input)) = cli.command else {
        panic!("expected installation preparation")
    };

    run(input)
}

fn fixture() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/installation-input-tests")
        .join(format!(
            "{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&path).unwrap();

    path.canonicalize().unwrap()
}

#[test]
fn invalid_roles_users_and_limits_leave_no_output() {
    let root = fixture();
    let output = root.join("inputs");
    for (flag, value) in [
        ("--canister", Principal::anonymous().to_text()),
        (
            "--blob-canister",
            Principal::management_canister().to_text(),
        ),
        ("--manager", Principal::from_slice(&[1, 1]).to_text()),
        ("--user", Principal::from_slice(&[3, 1]).to_text()),
        ("--blob-canister", Principal::from_slice(&[1, 1]).to_text()),
        ("--uploads", "65".into()),
        ("--references", "0".into()),
        ("--logical-bytes", "0".into()),
        ("--max-content-bytes", "0".into()),
        ("--max-header-bytes", "4294967296".into()),
    ] {
        let mut args = arguments(&output);
        let index = args.iter().position(|arg| arg == flag).unwrap();
        args[index + 1] = value;
        assert!(prepare(args).is_err());
        assert!(!output.exists());
    }
    for ids in [vec![4], (5..13).collect()] {
        let mut args = arguments(&output);
        for id in ids {
            args.extend(["--user".into(), Principal::from_slice(&[id, 1]).to_text()]);
        }
        assert!(prepare(args).is_err());
        assert!(!output.exists());
    }
}

#[test]
fn completed_and_partial_outputs_are_never_replaced() {
    let root = fixture();
    let output = root.join("inputs");
    let report = prepare(arguments(&output)).unwrap();
    assert_eq!(
        fs::read_to_string(output.join("inputs.txt")).unwrap(),
        report
    );
    let blob = fs::read(output.join("blob-init.wire")).unwrap();
    let app = fs::read(output.join("app-init.wire")).unwrap();
    assert!(prepare(arguments(&output)).is_err());
    assert_eq!(fs::read(output.join("blob-init.wire")).unwrap(), blob);
    assert_eq!(fs::read(output.join("app-init.wire")).unwrap(), app);
    let partial = root.join("partial");
    fs::create_dir(&partial).unwrap();
    fs::write(partial.join("blob-init.wire"), &blob).unwrap();
    assert!(prepare(arguments(&partial)).is_err());
    assert_eq!(fs::read(partial.join("blob-init.wire")).unwrap(), blob);
    assert!(!partial.join("app-init.wire").exists());
}
