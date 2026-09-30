use super::*;
use crate::native::arguments::Command;

fn arguments() -> Vec<String> {
    let p = Principal::self_authenticating([1]).to_text();
    [
        "submit-attestation",
        "--network",
        "ic",
        "--url",
        "https://icp-api.io",
        "--identity",
        "verifier.pem",
        "--actor",
        &p,
        "--service",
        &p,
        "--namespace",
        "340282366920938463463374607431768211455",
        "--run-dir",
        "observation",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
#[test]
fn submission_requires_explicit_actor_scope_and_observation_directory() {
    let args = arguments();
    let options = Options::parse(&args).unwrap();
    let Command::SubmitAttestation(input) = options.command else {
        panic!("submit command")
    };
    assert_eq!(input.namespace, u128::MAX);
    for flag in ["--run-dir", "--actor", "--service", "--namespace"] {
        let mut args = arguments();
        let i = args.iter().position(|a| a == flag).unwrap();
        args.drain(i..i + 2);
        assert!(matches!(Options::parse(&args), Err(Failure::Arguments)));
    }
    for extra in ["--statement", "--gateway", "--retry"] {
        let mut args = arguments();
        args.extend([extra.into(), "x".into()]);
        assert!(matches!(Options::parse(&args), Err(Failure::Arguments)));
    }
}
#[test]
fn statement_alone_or_failure_marker_never_authorizes_submission() {
    let dir = tempfile::tempdir().unwrap();
    let options = Options::parse(&arguments()).unwrap();
    let Command::SubmitAttestation(mut input) = Options::parse(&arguments()).unwrap().command
    else {
        panic!("submit command")
    };
    input.directory = dir.path().into();
    std::fs::write(dir.path().join("statement.candid"), b"not an observation").unwrap();
    assert!(matches!(
        observation::Observation::open(&options, &input),
        Err(Failure::Observation)
    ));
    std::fs::write(dir.path().join("failure.json"), b"{}").unwrap();
    assert!(matches!(
        observation::Observation::open(&options, &input),
        Err(Failure::Observation)
    ));
    assert!(!dir.path().join("attestation").exists());
}
