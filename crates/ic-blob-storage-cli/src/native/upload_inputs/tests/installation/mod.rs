//! Mismatched or malformed installation candidates produce no usable upload files.
use super::*;

#[test]
fn different_installation_bindings_and_invalid_models_refuse_before_output() {
    let setters: [fn(&mut ServiceInstallationInput); 8] = [
        |input| input.configuration.service = Principal::self_authenticating([9]),
        |input| input.configuration.namespace = 1,
        |input| input.project = "another-project".into(),
        |input| input.trusted_uploader = Principal::self_authenticating([9]),
        |input| input.completion_verifier = Principal::anonymous(),
        |input| input.configuration.operator = Principal::anonymous(),
        |input| input.configuration.reads.tenant_sessions = 2,
        |input| input.configuration.funding.reserve = input.configuration.funding.allocated + 1,
    ];
    for change in setters {
        let base = tempfile::tempdir().unwrap();
        let input = binding();
        let args = arguments(base.path(), &input, &manifest());
        let mut candidate = installation(&input);
        change(&mut candidate);
        std::fs::write(
            base.path().join("installation.candid"),
            candid::encode_one(candidate).unwrap(),
        )
        .unwrap();
        assert_eq!(crate::native::execute(&args), Err(Failure::Arguments));
        assert!(!base.path().join("output").exists());
    }
}

#[test]
fn exact_candidate_decoding_and_file_bounds_refuse_before_output() {
    let candidate = installation(&binding());
    let mut trailing = candid::encode_one(&candidate).unwrap();
    trailing.push(0);
    let extended = candid::encode_one((candidate.clone(), true)).unwrap();
    for (bytes, expected) in [
        (b"not candid".to_vec(), Failure::Arguments),
        (vec![], Failure::File),
        (
            vec![0; crate::native::candidate_candid::MAX_BYTES + 1],
            Failure::File,
        ),
        (
            candid::encode_args((&candidate, ())).unwrap(),
            Failure::Arguments,
        ),
        (extended, Failure::Arguments),
        (trailing, Failure::Arguments),
    ] {
        let base = tempfile::tempdir().unwrap();
        let args = arguments(base.path(), &binding(), &manifest());
        std::fs::write(base.path().join("installation.candid"), bytes).unwrap();
        assert_eq!(crate::native::execute(&args), Err(expected));
        assert!(!base.path().join("output").exists());
    }
    let base = tempfile::tempdir().unwrap();
    let mut args = arguments(base.path(), &binding(), &manifest());
    let index = args
        .iter()
        .position(|flag| flag == "--installation")
        .unwrap();
    args.drain(index..index + 2);
    assert_eq!(crate::native::execute(&args), Err(Failure::Arguments));
    assert!(!base.path().join("output").exists());
}

#[test]
fn valid_installed_object_and_header_bounds_constrain_preparation() {
    let setters: [fn(&mut ServiceInstallationInput); 3] = [
        |input| input.configuration.resources.max_object_bytes = 2,
        |input| input.configuration.resources.max_headers = 1,
        |input| input.configuration.resources.max_header_bytes = 32,
    ];
    for change in setters {
        let base = tempfile::tempdir().unwrap();
        let args = arguments(base.path(), &binding(), &manifest());
        let mut candidate = installation(&binding());
        change(&mut candidate);
        std::fs::write(
            base.path().join("installation.candid"),
            candid::encode_one(candidate).unwrap(),
        )
        .unwrap();
        assert_eq!(
            crate::native::execute(&args),
            Err(Failure::PreparedManifest)
        );
        assert!(!base.path().join("output").exists());
    }
}
