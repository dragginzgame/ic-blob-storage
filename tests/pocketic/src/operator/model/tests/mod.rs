use super::*;
use ic_testkit::Fake;

fn args() -> Vec<String> {
    [
        "status",
        "--server",
        "127.0.0.1:1",
        "--instance",
        "0",
        "--canister",
        &Fake::principal(1).to_text(),
        "--caller",
        "2vxsx-fae",
        "--kind",
        "authority",
        "--namespace",
        "1",
    ]
    .map(str::to_owned)
    .to_vec()
}

#[test]
fn explicit_targets_reject_ambiguous_or_remote_inputs() {
    assert!(Command::parse(&args()).is_ok());
    for (index, value) in [
        (2, "192.0.2.1:1"),
        (2, "localhost:1"),
        (2, "127.0.0.1:0"),
        (4, "01"),
        (4, "+1"),
        (6, "aaaaa-aa"),
        (10, "provider"),
        (12, "-1"),
        (12, "01"),
        (12, "340282366920938463463374607431768211456"),
    ] {
        let mut input = args();
        input[index] = value.into();
        assert!(matches!(Command::parse(&input), Err(Failure::Arguments)));
    }
    for tail in [
        vec!["--caller", "2vxsx-fae"],
        vec!["--peer", "2vxsx-fae"],
        vec!["--method", "fund"],
        vec!["--update"],
        vec!["--retry", "1"],
    ] {
        let mut input = args();
        input.extend(tail.into_iter().map(str::to_owned));
        assert!(matches!(Command::parse(&input), Err(Failure::Arguments)));
    }
    for len in 0..args().len() {
        assert!(matches!(
            Command::parse(&args()[..len]),
            Err(Failure::Arguments)
        ));
    }
}
