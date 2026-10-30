//! Offline CLI init bytes installed by the actual host, with independent platform binding.
use super::*;
use crate::authenticated_cli::run;
use ic_blob_storage_contracts::endpoints::{MethodCodec, MethodMode, SERVICE_METHODS};
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
use ic_host_fs::testing::FixtureDirectory;
use std::path::Path;

#[test]
fn deployment_surface_matches_catalog_and_provider_contract() {
    use ic_host_artifacts::wasm::{InspectionLimits, inspect};
    use ic_host_tools::interface;
    use ic_platform::candid_parser::{CandidSource, FuncMode, instantiate_candid};
    use std::collections::BTreeSet;

    let declared = include_str!("../../../../canisters/standalone/service.did");
    ic_platform::candid_parser::service_equal(
        ic_platform::candid_parser::CandidSource::Text(declared),
        ic_platform::candid_parser::CandidSource::Text(
            &ic_blob_storage_canister::provider_candid_interface(),
        ),
    )
    .unwrap();

    // Native registration equality cannot prove the compiled IC dispatch surface.
    // Platform checks this fixed, repository-owned DID; no untrusted inventory is loaded.
    let (_, (environment, actor)) = instantiate_candid(CandidSource::Text(declared)).unwrap();
    let methods = environment.as_service(&actor).unwrap();
    let provider = methods
        .iter()
        .map(|(name, ty)| interface::ExportedMethod {
            name,
            mode: match environment.as_func(ty).unwrap().modes.first() {
                None | Some(FuncMode::Oneway) => interface::MethodMode::Update,
                Some(FuncMode::Query) => interface::MethodMode::Query,
                Some(FuncMode::CompositeQuery) => interface::MethodMode::CompositeQuery,
            },
        })
        .collect::<BTreeSet<_>>();
    let expected = SERVICE_METHODS
        .iter()
        .chain(std::iter::once(
            &ic_blob_storage_contracts::standalone::METRICS_METHOD,
        ))
        .map(|method| interface::ExportedMethod {
            name: method.name,
            mode: match method.mode {
                MethodMode::Query => interface::MethodMode::Query,
                MethodMode::Update => interface::MethodMode::Update,
            },
        })
        .collect::<BTreeSet<_>>();
    let declared_provider = SERVICE_METHODS
        .iter()
        .filter(|m| m.codec == MethodCodec::Candid)
        .map(|m| m.name)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        provider.iter().map(|m| m.name).collect::<BTreeSet<_>>(),
        declared_provider
    );
    assert!(provider.is_subset(&expected), "provider names and modes");
    let bytes = wasm();
    let facts = inspect(
        &bytes,
        InspectionLimits {
            module_bytes: 64 * 1024 * 1024,
            sections: 1024,
            exports: 1024,
            custom_sections: 1024,
        },
    )
    .expect("inspect compiled standalone Wasm");
    assert!(
        !facts.exports.contains_key("get_candid_pointer"),
        "deployment Wasm must omit Candid extraction machinery"
    );
    let mut gzip = Vec::new();
    ic_host_artifacts::artifact::encode_gzip(&bytes, &mut gzip, 9, 64 * 1024 * 1024).unwrap();
    let code = ic_host_artifacts::wasm::code_statistics(&bytes, 64 * 1024 * 1024).unwrap();
    println!(
        "BLOB_ARTIFACT {}",
        serde_json::json!({
            "sha256": ContentDigest::compute(&bytes).to_string(),
            "raw_bytes":facts.raw_bytes, "code_bytes":facts.code_section_bytes,
            "data_bytes":facts.data_section_bytes, "functions":facts.defined_functions,
            "indirect_calls":code.call_indirect_count, "gzip9_bytes":gzip.len(),
            "gzip9_sha256":ContentDigest::compute(&gzip).to_string(),
        })
    );
    let exported = interface::exported_methods(&facts)
        // This namespace is reserved by IC CDK and is not an application API.
        .filter(|method| !method.name.starts_with("<ic-cdk internal> "))
        .collect::<BTreeSet<_>>();
    assert_eq!(exported, expected, "complete callable standalone surface");
}

fn trial_configuration(f: &Fixture) -> ServiceConfigurationInput {
    let template =
        include_str!("../../../../canisters/standalone/trial/configuration.json.template");
    let mut template = template.to_owned();
    for (placeholder, principal) in [
        ("ACTUAL_SERVICE_PRINCIPAL", f.service),
        ("OPERATOR_PRINCIPAL", f.operator),
        ("ISOLATED_PAYER_PRINCIPAL", f.tenant),
        (
            "REVIEWED_CASHIER_PRINCIPAL",
            f.config.billing.cashier.into(),
        ),
    ] {
        template = template.replace(placeholder, &principal.to_text());
    }
    serde_json::from_str(&template).unwrap()
}

fn record_trial(f: &Fixture, installation: &[u8], view: &HostConfigurationView) {
    let Some(path) = std::env::var_os("BLOB_TRIAL_INSTALLATION_REPORT") else {
        return;
    };
    let path = Path::new(&path);
    std::fs::create_dir(path).unwrap();
    for (name, bytes) in [
        (
            "configuration.wire",
            ic_blob_storage_contracts::document::encode(f.config).unwrap(),
        ),
        ("installation.wire", installation.to_vec()),
        (
            "readback.wire",
            ic_blob_storage_contracts::document::encode(view).unwrap(),
        ),
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

fn carrier(f: &Fixture, directory: &Path, project: &str) -> Vec<u8> {
    let source = directory.join("input.json");
    let output = directory.join("output");
    std::fs::write(&source, serde_json::to_vec(&f.config).unwrap()).unwrap();
    let args = [
        "installation-check",
        "--configuration",
        &source.display().to_string(),
        "--service",
        &f.config.service.to_text(),
        "--project",
        project,
        "--release",
        &f.configuration(f.operator).unwrap().release,
        "--run-dir",
        &output.display().to_string(),
    ]
    .map(str::to_owned);
    let report = run(&args, 0);
    let bytes = std::fs::read(output.join("installation.wire")).unwrap();
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
    let mut f = Fixture::small(Harness::new(), fake_principal(4));
    let before = f.configuration(f.operator).unwrap();
    assert_eq!(before.release, ic_blob_storage::LIBRARY_VERSION);
    // Deployment review supplies its independently selected release: retained
    // target/ artifacts may predate the release helper's version transaction.
    if let Ok(expected) = std::env::var("BLOB_EXPECTED_HOST_RELEASE") {
        assert_eq!(before.release, expected);
    }
    let stable = f.harness.pic.get_stable_memory(f.service);
    f.config = trial_configuration(&f);
    let directory = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    f.uploader = fake_principal(91);
    // The checker can only validate the proposed identity. The installed host
    // must compare that identity with the actual canister before allocation.
    f.config.service = fake_principal(93).into();
    let wrong = carrier(&f, directory.path(), "standalone-trial");
    // Install-code instruction debt is platform-owned; qualify the identity
    // refusal after its cooldown rather than assuming immediate reinstall.
    install_code::settle_install_code_debt(&f.harness.pic);
    let error = f
        .harness
        .pic
        .reinstall_canister(f.service, wasm(), wrong, Some(f.controller))
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError, "{error:?}");
    assert_eq!(f.configuration(f.operator).unwrap(), before);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &stable);

    f.config.service = f.service.into();
    let correct = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let bytes = carrier(&f, correct.path(), "standalone-trial");
    install_code::settle_install_code_debt(&f.harness.pic);
    // Only this empty local fixture is reinstalled: no provider objects, effects
    // or obligations exist. The exact generated bytes are passed without edits.
    f.harness
        .pic
        .reinstall_canister(f.service, wasm(), bytes.clone(), Some(f.controller))
        .unwrap();
    let installed = f.configuration(f.operator).unwrap();
    assert_eq!(installed.configuration, f.config);
    assert_eq!(installed.project, "standalone-trial");
    assert_eq!(installed.release, before.release);
    assert!(!installed.fenced);
    assert_eq!(f.configuration(f.controller), Err(HostFailure::Denied));

    f.enroll(f.operator).unwrap();
    let manifest = f.small_manifest();
    f.harness
        .pic
        .update_blob_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            manifest.permission,
            REPLY_BYTES,
        )
        .unwrap()
        .unwrap();
    f.prepare(f.uploader, &manifest).unwrap();
    let assessment =
        standalone_certificate::inspect(&f, f.uploader, &standalone_certificate::root(&manifest))
            .unwrap();
    assert_eq!(assessment.permission, manifest.permission);
    record_trial(&f, &bytes, &installed);
}
