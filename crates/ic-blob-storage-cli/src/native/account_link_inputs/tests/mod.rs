use super::*;

fn args(base: &Path) -> Vec<String> {
    [
        "account-link-inputs",
        "--cashier",
        "72ch2-fiaaa-aaaar-qbsvq-cai",
        "--caller",
        "r7inp-6aaaa-aaaaa-aaabq-cai",
        "--owner",
        "rrkah-fqaaa-aaaaa-aaaaq-cai",
        "--payer",
        "ryjl3-tyaaa-aaaaa-aaaba-cai",
        "--daily-limit",
        &u128::MAX.to_string(),
        "--expiry",
        &u64::MAX.to_string(),
        "--run-dir",
        &base.join("output").display().to_string(),
    ]
    .map(str::to_owned)
    .into()
}
#[test]
fn offline_account_link_preserves_exact_request_and_rejects_repeat_or_partial_output() {
    let base = tempfile::tempdir().unwrap();
    let arguments = args(base.path());
    let result = crate::native::execute(&arguments).unwrap();
    let directory = base.path().join("output");
    let request = std::fs::read(directory.join("account-link.candid")).unwrap();
    assert_eq!(result["daily_limit"], u128::MAX.to_string());
    assert_eq!(result["expiration_timestamp"], u64::MAX.to_string());
    assert_eq!(
        result["request_sha256"],
        ContentDigest::compute(&request).to_string()
    );
    assert_eq!(result["paid_canister"], "rrkah-fqaaa-aaaaa-aaaaq-cai");
    assert_eq!(result["payment_account"], "ryjl3-tyaaa-aaaaa-aaaba-cai");
    for field in [
        "authenticated",
        "identities_allocated",
        "provider_dispatched",
        "funding_dispatched",
        "retry_authorized",
        "units_interpreted",
    ] {
        assert_eq!(result[field], false);
    }
    let summary = std::fs::read(directory.join("summary.json")).unwrap();
    assert_eq!(
        crate::native::execute(&arguments),
        Err(Failure::ExistingRun)
    );
    assert_eq!(
        std::fs::read(directory.join("account-link.candid")).unwrap(),
        request
    );
    assert_eq!(
        std::fs::read(directory.join("summary.json")).unwrap(),
        summary
    );
    let partial = tempfile::tempdir().unwrap();
    std::fs::create_dir(partial.path().join("output")).unwrap();
    std::fs::write(
        partial.path().join("output/account-link.candid"),
        b"partial",
    )
    .unwrap();
    assert_eq!(
        crate::native::execute(&args(partial.path())),
        Err(Failure::ExistingRun)
    );
    assert_eq!(
        std::fs::read(partial.path().join("output/account-link.candid")).unwrap(),
        b"partial"
    );
    assert!(!partial.path().join("output/summary.json").exists());
}
#[test]
fn sentinel_missing_ambiguous_or_unusable_inputs_refuse_before_output() {
    for (key, values) in [
        (
            "--daily-limit",
            vec![
                "0",
                "-1",
                "+1",
                "01",
                "340282366920938463463374607431768211456",
            ],
        ),
        ("--expiry", vec!["0", "-1", "01", "18446744073709551616"]),
        ("--cashier", vec!["2vxsx-fae", "aaaaa-aa", "invalid"]),
        ("--caller", vec!["2vxsx-fae", "aaaaa-aa"]),
        ("--owner", vec!["2vxsx-fae", "aaaaa-aa"]),
        ("--payer", vec!["2vxsx-fae", "aaaaa-aa"]),
    ] {
        for value in values {
            let base = tempfile::tempdir().unwrap();
            let mut arguments = args(base.path());
            let position = arguments.iter().position(|arg| arg == key).unwrap();
            arguments[position + 1] = value.into();
            assert_eq!(crate::native::execute(&arguments), Err(Failure::Arguments));
            assert!(!base.path().join("output").exists());
        }
    }
    for mode in ["missing", "duplicate", "unknown", "unpaired"] {
        let base = tempfile::tempdir().unwrap();
        let mut arguments = args(base.path());
        match mode {
            "missing" => {
                arguments.drain(9..11);
            }
            "duplicate" => {
                arguments.extend(["--payer".into(), "ryjl3-tyaaa-aaaaa-aaaba-cai".into()]);
            }
            "unknown" => arguments.extend(["--network".into(), "ic".into()]),
            "unpaired" => arguments.push("--network".into()),
            _ => unreachable!(),
        }
        assert_eq!(crate::native::execute(&arguments), Err(Failure::Arguments));
        assert!(!base.path().join("output").exists());
    }
}
