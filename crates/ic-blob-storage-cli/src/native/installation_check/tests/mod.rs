use super::*;

fn fixture() -> Vec<u8> {
    // Independent didc encoding against the maintained standalone declaration.
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/installation/configuration.hex"
    ))
    .trim()
    .as_bytes()
    .as_chunks::<2>()
    .0
    .iter()
    .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
    .collect()
}
fn args(base: &Path, bytes: &[u8]) -> Vec<String> {
    std::fs::write(base.join("input.candid"), bytes).unwrap();
    [
        "installation-check",
        "--configuration",
        &base.join("input.candid").display().to_string(),
        "--service",
        "rrkah-fqaaa-aaaaa-aaaaq-cai",
        "--project",
        "isolated local fixture/β",
        "--verifier",
        "rdmx6-jaaaa-aaaaa-aaadq-cai",
        "--release",
        env!("CARGO_PKG_VERSION"),
        "--run-dir",
        &base.join("output").display().to_string(),
    ]
    .map(str::to_owned)
    .into()
}
fn replace(arguments: &mut [String], flag: &str, value: &str) {
    let position = arguments.iter().position(|arg| arg == flag).unwrap();
    arguments[position + 1] = value.into();
}

#[test]
fn complete_offline_check_preserves_proposal_without_allocating_or_replacing() {
    let base = tempfile::tempdir().unwrap();
    let original = fixture();
    let arguments = args(base.path(), &original);
    let report = crate::native::execute(&arguments).unwrap();
    let output = base.path().join("output");
    assert_eq!(
        std::fs::read(output.join("configuration.candid")).unwrap(),
        original
    );
    assert_eq!(
        report["configuration_sha256"],
        ContentDigest::compute(&original).to_string()
    );
    assert_eq!(report["namespace"], u128::MAX.to_string());
    assert_eq!(report["service"], "rrkah-fqaaa-aaaaa-aaaaq-cai");
    assert_eq!(report["payment_account"], "ryjl3-tyaaa-aaaaa-aaaba-cai");
    assert_eq!(report["project"], "isolated local fixture/β");
    assert_eq!(report["expected_host_release"], env!("CARGO_PKG_VERSION"));
    assert_eq!(report["configuration_validated"], true);
    for field in [
        "authenticated",
        "platform_identity_checked",
        "compiled_release_checked",
        "stable_memory_allocated",
        "host_init_encoded",
        "installation_dispatched",
        "provider_dispatched",
        "funding_dispatched",
        "namespace_provisioned",
        "provider_qualified",
        "operational_recovery_qualified",
        "retry_authorized",
    ] {
        assert_eq!(report[field], false);
    }
    let summary = std::fs::read(output.join("summary.json")).unwrap();
    std::fs::write(
        base.path().join("input.candid"),
        b"edited after preparation",
    )
    .unwrap();
    assert_eq!(
        std::fs::read(output.join("configuration.candid")).unwrap(),
        original
    );
    // Restore only the source input; repeat must preserve the original artifacts.
    std::fs::write(base.path().join("input.candid"), &original).unwrap();
    assert_eq!(
        crate::native::execute(&arguments),
        Err(Failure::ExistingRun)
    );
    assert_eq!(std::fs::read(output.join("summary.json")).unwrap(), summary);
    assert_eq!(
        std::fs::read(output.join("configuration.candid")).unwrap(),
        original
    );
    let partial = tempfile::tempdir().unwrap();
    let arguments = args(partial.path(), &original);
    std::fs::create_dir(partial.path().join("output")).unwrap();
    std::fs::write(
        partial.path().join("output/configuration.candid"),
        b"partial",
    )
    .unwrap();
    assert_eq!(
        crate::native::execute(&arguments),
        Err(Failure::ExistingRun)
    );
    assert_eq!(
        std::fs::read(partial.path().join("output/configuration.candid")).unwrap(),
        b"partial"
    );
    assert!(!partial.path().join("output/summary.json").exists());
}

#[test]
fn complete_validator_refuses_inconsistent_limits_bindings_and_authorities_before_output() {
    let original = decode(&fixture()).unwrap();
    let setters: [fn(&mut ServiceConfigurationInput); 6] = [
        |input| input.namespace = 0,
        |input| input.operator = Principal::anonymous(),
        |input| input.payment_account = Principal::management_canister(),
        |input| input.resources.max_receipts_per_object = 2,
        |input| input.funding.reserve = input.funding.allocated + 1,
        |input| input.reads.tenant_sessions = input.reads.sessions + 1,
    ];
    for change in setters {
        let base = tempfile::tempdir().unwrap();
        let mut input = original;
        change(&mut input);
        let arguments = args(base.path(), &candid::encode_one(input).unwrap());
        assert_eq!(crate::native::execute(&arguments), Err(Failure::Arguments));
        assert!(!base.path().join("output").exists());
    }
    for (flag, value) in [
        ("--service", "ryjl3-tyaaa-aaaaa-aaaba-cai"),
        ("--service", "2vxsx-fae"),
        ("--service", "invalid"),
        ("--verifier", "2vxsx-fae"),
        ("--verifier", "aaaaa-aa"),
        ("--project", ""),
        ("--project", " surrounding space "),
        ("--project", "line\nbreak"),
        ("--release", ""),
        ("--release", " trailing "),
        ("--release", "line\nbreak"),
    ] {
        let base = tempfile::tempdir().unwrap();
        let mut arguments = args(base.path(), &fixture());
        replace(&mut arguments, flag, value);
        assert_eq!(crate::native::execute(&arguments), Err(Failure::Arguments));
        assert!(!base.path().join("output").exists());
    }
}

#[test]
fn bounded_exact_input_refuses_malformed_extra_or_ambiguous_arguments() {
    let input = decode(&fixture()).unwrap();
    let mut trailing = fixture();
    trailing.push(0);
    for (bytes, expected) in [
        (vec![], Failure::File),
        (b"not candid".to_vec(), Failure::Arguments),
        (vec![0; CONFIGURATION_BYTES + 1], Failure::File),
        (
            candid::encode_args((input, ())).unwrap(),
            Failure::Arguments,
        ),
        (trailing, Failure::Arguments),
    ] {
        let base = tempfile::tempdir().unwrap();
        let arguments = args(base.path(), &bytes);
        assert_eq!(crate::native::execute(&arguments), Err(expected));
        assert!(!base.path().join("output").exists());
    }
    for mode in ["missing", "duplicate", "unknown", "unpaired"] {
        let base = tempfile::tempdir().unwrap();
        let mut arguments = args(base.path(), &fixture());
        match mode {
            "missing" => {
                arguments.drain(9..11);
            }
            "duplicate" => arguments.extend(["--release".into(), "duplicate".into()]),
            "unknown" => arguments.extend(["--network".into(), "ic".into()]),
            "unpaired" => arguments.push("--network".into()),
            _ => unreachable!(),
        }
        assert_eq!(crate::native::execute(&arguments), Err(Failure::Arguments));
        assert!(!base.path().join("output").exists());
    }
}
