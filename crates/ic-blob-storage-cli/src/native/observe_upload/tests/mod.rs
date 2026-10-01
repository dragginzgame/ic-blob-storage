use super::*;
#[test]
fn observation_arguments_require_origin_budget_and_new_directory() {
    use super::super::arguments::{Command, Options};
    let p = Principal::self_authenticating([1]).to_text();
    let args: Vec<String> = [
        "observe-upload",
        "--network",
        "ic",
        "--url",
        "https://icp-api.io",
        "--identity",
        "key.pem",
        "--actor",
        &p,
        "--service",
        &p,
        "--namespace",
        "1",
        "--permission",
        "permission.candid",
        "--gateway",
        "https://gateway.example",
        "--max-bytes",
        "100",
        "--run-dir",
        "new-run",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    assert!(matches!(
        Options::parse(&args).unwrap().command,
        Command::ObserveUpload(_)
    ));
    for (flag, value) in [
        ("--gateway", "http://gateway.example"),
        ("--gateway", "https://user@gateway.example"),
        ("--gateway", "https://gateway.example/path"),
        ("--max-bytes", "0"),
        ("--max-bytes", "1073741825"),
    ] {
        let mut invalid = args.clone();
        let index = invalid.iter().position(|s| s == flag).unwrap();
        invalid[index + 1] = value.into();
        assert!(matches!(Options::parse(&invalid), Err(Failure::Arguments)));
    }
}
#[test]
fn observation_artifacts_are_no_clobber_and_partial_runs_stay_visible() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("run");
    let run = Run::create(&path).unwrap();
    run.bytes("statement.candid", b"original").unwrap();
    assert_eq!(
        run.bytes("statement.candid", b"changed"),
        Err(Failure::File)
    );
    assert!(matches!(Run::create(&path), Err(Failure::ExistingRun)));
    assert_eq!(
        std::fs::read(path.join("statement.candid")).unwrap(),
        b"original"
    );
    assert!(!path.join("summary.json").exists());
}
