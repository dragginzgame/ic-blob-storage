//! Mismatched or malformed installation candidates produce no usable upload files.
use super::*;
use ic_host_fs::testing::FixtureDirectory;

#[test]
fn one_installation_prepares_exact_permissions_for_two_distinct_uploaders() {
    let mut retained_installation = None;
    for uploader in [
        Principal::self_authenticating([3]),
        Principal::self_authenticating([9]),
    ] {
        let base = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
        let mut input = binding();
        input["uploader"] = json!(uploader.to_text());
        let args = arguments(base.path(), &input, &manifest());
        crate::native::execute(&args).unwrap();
        let output = base.path().join("output");
        let permission: UploadAdmissionRequest = ic_blob_storage_contracts::document::decode(
            &std::fs::read(output.join("permission.wire")).unwrap(),
            ic_blob_storage_contracts::wire::MAX_MESSAGE_BYTES,
        )
        .unwrap();
        assert_eq!(permission.uploader, uploader);
        let installation = std::fs::read(output.join("installation.wire")).unwrap();
        if let Some(original) = &retained_installation {
            assert_eq!(&installation, original);
        } else {
            retained_installation = Some(installation);
        }
    }
}

#[test]
fn different_installation_bindings_and_invalid_models_refuse_before_output() {
    let setters: [fn(&mut ServiceInstallationInput); 5] = [
        |input| input.configuration.service = Principal::self_authenticating([9]),
        |input| input.configuration.namespace = 1,
        |input| input.project = "another-project".into(),
        |input| input.configuration.operator = Principal::anonymous(),
        |input| input.configuration.funding.reserve = input.configuration.funding.allocated + 1,
    ];
    for change in setters {
        let base = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
        let input = binding();
        let args = arguments(base.path(), &input, &manifest());
        let mut candidate = installation(&input);
        change(&mut candidate);
        std::fs::write(
            base.path().join("installation.wire"),
            ic_blob_storage_contracts::wire::encode_request("blob_install", candidate).unwrap(),
        )
        .unwrap();
        assert_eq!(crate::native::execute(&args), Err(Failure::Arguments));
        assert!(!base.path().join("output").exists());
    }
}

#[test]
fn exact_candidate_decoding_and_file_bounds_refuse_before_output() {
    let candidate = installation(&binding());
    let mut trailing =
        ic_blob_storage_contracts::wire::encode_request("blob_install", &candidate).unwrap();
    trailing.push(0);
    let extended =
        ic_blob_storage_contracts::wire::encode_request("blob_install", (candidate.clone(), true))
            .unwrap();
    for (bytes, expected) in [
        (b"not candid".to_vec(), Failure::Arguments),
        (vec![], Failure::File),
        (
            vec![0; crate::native::exact_wire::INSTALLATION_BYTES + 1],
            Failure::File,
        ),
        (
            ic_blob_storage_contracts::wire::encode_request("blob_install", (&candidate, false))
                .unwrap(),
            Failure::Arguments,
        ),
        (extended, Failure::Arguments),
        (trailing, Failure::Arguments),
    ] {
        let base = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
        let args = arguments(base.path(), &binding(), &manifest());
        std::fs::write(base.path().join("installation.wire"), bytes).unwrap();
        assert_eq!(crate::native::execute(&args), Err(expected));
        assert!(!base.path().join("output").exists());
    }
    let base = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
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
        let base = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
        let args = arguments(base.path(), &binding(), &manifest());
        let mut candidate = installation(&binding());
        change(&mut candidate);
        std::fs::write(
            base.path().join("installation.wire"),
            ic_blob_storage_contracts::wire::encode_request("blob_install", candidate).unwrap(),
        )
        .unwrap();
        assert_eq!(
            crate::native::execute(&args),
            Err(Failure::PreparedManifest)
        );
        assert!(!base.path().join("output").exists());
    }
}
