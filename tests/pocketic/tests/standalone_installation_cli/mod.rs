//! Offline CLI init bytes installed by the actual host, with independent platform binding.
use super::*;
use crate::authenticated_cli::run;
use ic_blob_storage_contracts::identity::ContentDigest;
use std::path::Path;

#[test]
fn deployment_candid_matches_the_exported_contract() {
    candid_parser::utils::service_equal(
        candid_parser::utils::CandidSource::Text(include_str!(
            "../../../../canisters/standalone/service.did"
        )),
        candid_parser::utils::CandidSource::Text(&ic_blob_storage_canister::candid_interface()),
    )
    .unwrap();
}

fn trial_configuration(f: &Fixture) -> ServiceConfigurationInput {
    let mut template =
        include_str!("../../../../canisters/standalone/trial/configuration.args.template")
            .to_owned();
    for (placeholder, principal) in [
        ("ACTUAL_SERVICE_PRINCIPAL", f.service),
        ("OPERATOR_PRINCIPAL", f.operator),
        ("ISOLATED_PAYER_PRINCIPAL", f.tenant),
        ("REVIEWED_CASHIER_PRINCIPAL", f.config.billing.cashier),
    ] {
        template = template.replace(placeholder, &principal.to_text());
    }
    let declared = include_str!("../../../../canisters/standalone/service.did");
    let (env, _) = candid_parser::utils::CandidSource::Text(declared)
        .load()
        .unwrap();
    let args = candid_parser::parse_idl_args(&template).unwrap();
    let bytes = args
        .to_bytes_with_types(
            &env,
            &[env.find_type("ServiceConfigurationInput").unwrap().clone()],
        )
        .unwrap();
    candid::decode_one(&bytes).unwrap()
}

fn record_trial(f: &Fixture, installation: &[u8], view: &HostConfigurationView) {
    let Some(path) = std::env::var_os("BLOB_TRIAL_INSTALLATION_REPORT") else {
        return;
    };
    let path = Path::new(&path);
    std::fs::create_dir(path).unwrap();
    for (name, bytes) in [
        (
            "configuration.candid",
            candid::encode_one(f.config).unwrap(),
        ),
        ("installation.candid", installation.to_vec()),
        ("readback.candid", candid::encode_one(view).unwrap()),
        (
            "summary.json",
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": 1, "evidence_class": "actual_local_trial_template_installation",
                "release": view.release, "service": f.service.to_text(),
                "declared_and_exported_did_equal": true,
                "exact_configuration_readback": true, "wrong_service_rejected": true,
                "controller_denied": true, "local_only_init": true,
                "live_provider_qualified": false, "deployment_authorized": false,
            }))
            .unwrap(),
        ),
    ] {
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path.join(name))
            .unwrap();
        file.write_all(&bytes).unwrap();
    }
}

fn carrier(f: &Fixture, directory: &Path, project: &str, verifier: Principal) -> Vec<u8> {
    let source = directory.join("input.candid");
    let output = directory.join("output");
    std::fs::write(&source, candid::encode_one(f.config).unwrap()).unwrap();
    let args = [
        "installation-check",
        "--configuration",
        &source.display().to_string(),
        "--service",
        &f.config.service.to_text(),
        "--project",
        project,
        "--verifier",
        &verifier.to_text(),
        "--trusted-uploader",
        &f.uploader.to_text(),
        "--release",
        &f.configuration(f.operator).unwrap().release,
        "--run-dir",
        &output.display().to_string(),
    ]
    .map(str::to_owned);
    let report = run(&args, 0);
    let bytes = std::fs::read(output.join("installation.candid")).unwrap();
    assert_eq!(report["host_init_encoded"], true);
    assert_eq!(report["platform_identity_checked"], false);
    assert_eq!(report["installation_dispatched"], false);
    assert_eq!(
        report["installation_sha256"],
        ContentDigest::compute(&bytes).to_string()
    );
    bytes
}

#[test]
fn standalone_installs_cli_carrier_and_independently_rejects_wrong_actual_service() {
    let mut f = Fixture::small(Harness::new(), Fake::principal(4));
    let before = f.configuration(f.operator).unwrap();
    assert_eq!(before.release, ic_blob_storage::LIBRARY_VERSION);
    // Deployment review supplies its independently selected release: retained
    // target/ artifacts may predate the release helper's version transaction.
    if let Ok(expected) = std::env::var("BLOB_EXPECTED_HOST_RELEASE") {
        assert_eq!(before.release, expected);
    }
    let stable = f.harness.pic.get_stable_memory(f.service);
    f.config = trial_configuration(&f);
    let directory = tempfile::tempdir().unwrap();
    let verifier = Fake::principal(92);
    f.uploader = Fake::principal(91);
    // The checker can only validate the proposed identity. The installed host
    // must compare that identity with the actual canister before allocation.
    f.config.service = Fake::principal(93);
    let wrong = carrier(&f, directory.path(), "standalone-trial", verifier);
    let error = f
        .harness
        .pic
        .reinstall_canister(f.service, wasm(), wrong, Some(f.controller))
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert_eq!(f.configuration(f.operator).unwrap(), before);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &stable);

    f.config.service = f.service;
    let correct = tempfile::tempdir().unwrap();
    let bytes = carrier(&f, correct.path(), "standalone-trial", verifier);
    // Only this empty local fixture is reinstalled: no provider objects, effects
    // or obligations exist. The exact generated bytes are passed without edits.
    f.harness
        .pic
        .reinstall_canister(f.service, wasm(), bytes.clone(), Some(f.controller))
        .unwrap();
    let installed = f.configuration(f.operator).unwrap();
    assert_eq!(installed.configuration, f.config);
    assert_eq!(installed.project, "standalone-trial");
    assert_eq!(installed.completion_verifier, verifier);
    assert_eq!(installed.trusted_uploader, f.uploader);
    assert_eq!(installed.release, before.release);
    assert!(!installed.fenced);
    assert_eq!(f.configuration(f.controller), Err(HostFailure::Denied));

    f.enroll(f.operator).unwrap();
    let manifest = f.small_manifest();
    f.harness
        .pic
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (manifest.permission,),
        )
        .unwrap()
        .unwrap();
    f.prepare(f.uploader, &manifest).unwrap();
    let assessment =
        standalone_certificate::inspect(&f, f.uploader, &standalone_certificate::root(&manifest))
            .unwrap();
    assert_eq!(assessment.permission, manifest.permission);
    assert_eq!(assessment.blockers, []);
    record_trial(&f, &bytes, &installed);
}
