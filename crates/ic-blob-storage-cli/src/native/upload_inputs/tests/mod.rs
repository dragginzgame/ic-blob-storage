use super::*;
mod installation;
use ic_blob_storage::dto::configuration::{ServiceConfigurationInput, ServiceInstallationInput};
use ic_blob_storage::dto::upload::manifest::UploadManifestRequest;

// Independent Caffeine 1.1.2 abc/text vector, also used by the core preparation tests.
const ROOT: &str = "sha256:0e9afaf413b048e40834d5b0e737d80fbf304af2045c7564d96ad8aebaf74dfd";
const LEAF: &str = "sha256:b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7";

pub(in crate::native) fn binding() -> Value {
    json!({"schema":1, "preparation":{"content_type":"text/plain"}, "project":"fixture-project", "bucket":"fixture-bucket",
        "service": Principal::self_authenticating([1]).to_text(),
        "namespace": u128::MAX.to_string(),
        "tenant": Principal::self_authenticating([2]).to_text(),
        "uploader": Principal::self_authenticating([3]).to_text(),
        "upload": u128::MAX.to_string(), "object": "2", "incarnation": "3",
        "first_reference": "4", "root": ROOT, "bytes": "3",
        "expires_at_ns": u64::MAX.to_string()})
}
pub(in crate::native) fn manifest() -> Value {
    json!({"tree_type":"DSBMTWH", "chunk_hashes":[LEAF],
        "tree":{"hash":ROOT}, "headers":["Content-Length: 3", "Content-Type: text/plain"]})
}
pub(in crate::native) fn installation(binding: &Value) -> ServiceInstallationInput {
    let fixture = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/installation/configuration.hex"
    ));
    let bytes = fixture
        .trim()
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect::<Vec<_>>();
    let mut configuration: ServiceConfigurationInput = candid::decode_one(&bytes).unwrap();
    configuration.service = principal(binding["service"].as_str().unwrap()).unwrap();
    configuration.namespace = u128::MAX;
    ServiceInstallationInput {
        configuration,
        project: binding["project"].as_str().unwrap().into(),
        trusted_uploader: Principal::self_authenticating([3]),
        completion_verifier: Principal::self_authenticating([4]),
    }
}
fn arguments(base: &Path, binding: &Value, manifest: &Value) -> Vec<String> {
    std::fs::write(
        base.join("installation.candid"),
        candid::encode_one(installation(binding)).unwrap(),
    )
    .unwrap();
    std::fs::write(
        base.join("binding.json"),
        serde_json::to_vec(binding).unwrap(),
    )
    .unwrap();
    std::fs::write(
        base.join("manifest.json"),
        serde_json::to_vec(manifest).unwrap(),
    )
    .unwrap();
    std::fs::write(base.join("body.bin"), b"abc").unwrap();
    vec![
        "upload-inputs".into(),
        "--binding".into(),
        base.join("binding.json").display().to_string(),
        "--manifest".into(),
        base.join("manifest.json").display().to_string(),
        "--max-bytes".into(),
        "10".into(),
        "--run-dir".into(),
        base.join("output").display().to_string(),
        "--body".into(),
        base.join("body.bin").display().to_string(),
        "--installation".into(),
        base.join("installation.candid").display().to_string(),
    ]
}

#[test]
fn offline_command_emits_exact_full_width_service_requests_without_network_configuration() {
    let base = tempfile::tempdir().unwrap();
    let args = arguments(base.path(), &binding(), &manifest());
    // Exercise the actual CLI dispatch without a signer, URL or runtime.
    let report = crate::native::execute(&args).unwrap();
    let output = base.path().join("output");
    let permission_bytes = std::fs::read(output.join("permission.candid")).unwrap();
    let preparation_bytes = std::fs::read(output.join("manifest.candid")).unwrap();
    let permission: UploadAdmissionRequest = candid::decode_one(&permission_bytes).unwrap();
    let preparation: UploadManifestRequest = candid::decode_one(&preparation_bytes).unwrap();
    assert_eq!(permission, preparation.permission);
    assert_eq!(permission.upload.namespace, u128::MAX);
    assert_eq!(permission.upload.upload, u128::MAX);
    assert_eq!(permission.expires_at_ns, u64::MAX);
    assert_ne!(permission.upload.tenant, permission.uploader);
    let status: ic_blob_storage::dto::reference::status::ReferenceStatusRequest =
        candid::decode_one(&std::fs::read(output.join("reference-status.candid")).unwrap())
            .unwrap();
    assert_eq!(status.upload, permission.upload);
    assert_eq!(status.reference, permission.upload.first_reference);
    let download: ic_blob_storage::dto::download::DownloadRequest =
        candid::decode_one(&std::fs::read(output.join("download.candid")).unwrap()).unwrap();
    assert_eq!(download.reference, permission.upload.first_reference);
    assert_eq!(download.object, permission.upload.object);
    assert_eq!(download.root, permission.upload.root);
    assert_eq!(preparation.declaration.headers[1].value, "text/plain");
    assert_eq!(report["permission_sha256"], digest(&permission_bytes));
    assert_eq!(report["preparation_sha256"], digest(&preparation_bytes));
    let browser_bytes = std::fs::read(output.join("certificate-binding.json")).unwrap();
    let browser: Value = serde_json::from_slice(&browser_bytes).unwrap();
    assert_eq!(browser["permission"], json!(permission_bytes));
    assert_eq!(browser["service"], permission.upload.service.to_text());
    assert_eq!(browser["tenant"], permission.upload.tenant.to_text());
    assert_eq!(browser["uploader"], permission.uploader.to_text());
    assert_eq!(browser["operation"], permission.upload.upload.to_string());
    assert_eq!(browser["root"], ROOT);
    assert_eq!(browser["project"], "fixture-project");
    assert_eq!(browser["bucket"], "fixture-bucket");
    assert_eq!(
        browser["key"],
        format!(
            "{}:{}:{}",
            permission.upload.service.to_text(),
            permission.upload.tenant.to_text(),
            permission.upload.upload
        )
    );
    assert_eq!(report["certificate_binding_sha256"], digest(&browser_bytes));
    assert_eq!(report["body_verified"], true);
    let init = std::fs::read(output.join("installation.candid")).unwrap();
    assert_eq!(
        init,
        std::fs::read(base.path().join("installation.candid")).unwrap()
    );
    assert_eq!(report["installation_sha256"], digest(&init));
    assert_eq!(report["installation_binding_checked"], true);
    assert_eq!(report["installed_state_observed"], false);
    assert_eq!(report["namespace_provisioned"], false);
    assert_eq!(std::fs::read(output.join("body.bin")).unwrap(), b"abc");
    assert_eq!(report["identities_allocated"], false);
    assert_eq!(report["provider_dispatched"], false);
    assert_eq!(
        std::fs::read(output.join("manifest.json")).unwrap(),
        std::fs::read(base.path().join("manifest.json")).unwrap()
    );
    assert!(matches!(
        crate::native::execute(&args),
        Err(Failure::ExistingRun)
    ));
    assert_eq!(
        std::fs::read(output.join("permission.candid")).unwrap(),
        permission_bytes
    );
    assert_eq!(
        std::fs::read(output.join("manifest.candid")).unwrap(),
        preparation_bytes
    );
}

#[test]
fn changed_root_length_leaves_or_metadata_refuse_before_output_claim() {
    let cases = [
        (
            "tree",
            json!({"hash": format!("sha256:{}", "00".repeat(32))}),
        ),
        (
            "chunk_hashes",
            json!([format!("sha256:{}", "00".repeat(32))]),
        ),
        (
            "headers",
            json!(["Content-Length: 4", "Content-Type: text/plain"]),
        ),
        ("tree_type", json!("other")),
    ];
    for (field, value) in cases {
        let base = tempfile::tempdir().unwrap();
        let mut changed = manifest();
        changed[field] = value;
        let args = arguments(base.path(), &binding(), &changed);
        assert!(matches!(
            crate::native::execute(&args),
            Err(Failure::PreparedManifest)
        ));
        assert!(!base.path().join("output").exists());
    }
    let base = tempfile::tempdir().unwrap();
    let mut args = arguments(base.path(), &binding(), &manifest());
    args[6] = "2".into();
    assert!(matches!(
        crate::native::execute(&args),
        Err(Failure::PreparedManifest)
    ));
    assert!(!base.path().join("output").exists());
}

#[test]
fn binding_requires_canonical_strings_current_schema_and_valid_authority_principals() {
    for (field, value) in [
        ("upload", json!(1)),
        ("upload", json!("01")),
        ("upload", json!("0")),
        ("expires_at_ns", json!("18446744073709551616")),
        ("tenant", json!("2vxsx-fae")),
        ("service", json!("aaaaa-aa")),
        ("uploader", json!("2vxsx-fae")),
        ("schema", json!(2)),
        ("unexpected", json!(true)),
        ("project", json!("")),
        ("project", json!(" surrounding ")),
        ("project", json!("line\nbreak")),
        ("project", json!("β")),
        ("project", json!("\u{0085}")),
        ("project", json!("a".repeat(257))),
        ("bucket", json!("")),
        ("bucket", json!(" padded ")),
        ("bucket", json!("\u{0085}")),
        ("bucket", json!("a".repeat(257))),
        ("bucket", json!("\u{feff}bucket")),
    ] {
        let base = tempfile::tempdir().unwrap();
        let mut changed = binding();
        changed[field] = value;
        let args = arguments(base.path(), &changed, &manifest());
        assert!(matches!(
            crate::native::execute(&args),
            Err(Failure::Arguments)
        ));
        assert!(!base.path().join("output").exists());
    }
}

#[test]
fn preparation_hints_preserve_omission_empty_values_and_utf8_byte_bounds() {
    for preparation in [
        json!({}),
        json!({"content_type":"text/plain"}),
        json!({"content_type":"", "filename":""}),
        json!({"filename":"β".repeat(2048)}),
    ] {
        let base = tempfile::tempdir().unwrap();
        let mut binding = binding();
        binding["preparation"] = preparation.clone();
        let args = arguments(base.path(), &binding, &manifest());
        let output = crate::native::execute(&args).unwrap();
        let input = PreparedInput::load(
            InputFiles {
                binding: std::fs::read(base.path().join("output/binding.json")).unwrap(),
                manifest: std::fs::read(base.path().join("output/manifest.json")).unwrap(),
                installation: std::fs::read(base.path().join("output/installation.candid"))
                    .unwrap(),
                body: base.path().join("output/body.bin"),
            },
            NonZeroU64::new(10).unwrap(),
        )
        .unwrap();
        let transfer = input
            .transfer_input(
                output["content_digest"]
                    .as_str()
                    .unwrap()
                    .trim_start_matches("sha256:"),
            )
            .unwrap();
        assert_eq!(transfer["preparation"], preparation);
    }
    for preparation in [
        json!(null),
        json!({"filename":null}),
        json!({"filename":12}),
        json!({"filename":"β".repeat(2049)}),
        json!({"content_type":"x".repeat(4097)}),
        json!({"inferred_type":"text/plain"}),
    ] {
        let base = tempfile::tempdir().unwrap();
        let mut binding = binding();
        binding["preparation"] = preparation;
        assert_eq!(
            crate::native::execute(&arguments(base.path(), &binding, &manifest())),
            Err(Failure::Arguments)
        );
        assert!(!base.path().join("output").exists());
    }
}

#[test]
fn namespace_byte_bounds_preserve_header_compatible_project_and_utf8_bucket() {
    let base = tempfile::tempdir().unwrap();
    let mut input = binding();
    input["project"] = json!("é".repeat(128));
    input["bucket"] = json!("β".repeat(128));
    let args = arguments(base.path(), &input, &manifest());
    crate::native::execute(&args).unwrap();
    let browser: Value = serde_json::from_slice(
        &std::fs::read(base.path().join("output/certificate-binding.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(browser["project"], input["project"]);
    assert_eq!(browser["bucket"], input["bucket"]);
    for field in ["project", "bucket"] {
        let rejected = tempfile::tempdir().unwrap();
        let mut oversized = input.clone();
        oversized[field] = json!("é".repeat(129));
        let args = arguments(rejected.path(), &oversized, &manifest());
        assert_eq!(crate::native::execute(&args), Err(Failure::Arguments));
        assert!(!rejected.path().join("output").exists());
    }
}

#[test]
fn bounded_regular_inputs_and_exact_options_refuse_without_creating_a_run() {
    let base = tempfile::tempdir().unwrap();
    let args = arguments(base.path(), &binding(), &manifest());
    for extra in [
        vec!["--unknown".into(), "value".into()],
        vec!["--max-bytes".into(), "10".into()],
        vec!["--incomplete".into()],
    ] {
        let mut changed = args.clone();
        changed.extend(extra);
        assert!(matches!(
            crate::native::execute(&changed),
            Err(Failure::Arguments)
        ));
    }
    std::fs::write(
        base.path().join("manifest.json"),
        vec![b' '; usize::try_from(MANIFEST_BYTES).unwrap() + 1],
    )
    .unwrap();
    assert!(matches!(crate::native::execute(&args), Err(Failure::File)));
    assert!(!base.path().join("output").exists());
    std::fs::remove_file(base.path().join("manifest.json")).unwrap();
    std::fs::create_dir(base.path().join("manifest.json")).unwrap();
    assert!(matches!(crate::native::execute(&args), Err(Failure::File)));
    assert!(!base.path().join("output").exists());
}

#[test]
fn corrupt_source_leaves_only_private_partial_evidence_and_no_request_files() {
    let base = tempfile::tempdir().unwrap();
    let args = arguments(base.path(), &binding(), &manifest());
    std::fs::write(base.path().join("body.bin"), b"abd").unwrap();
    assert!(matches!(
        crate::native::execute(&args),
        Err(Failure::Content)
    ));
    let output = base.path().join("output");
    assert_eq!(std::fs::read(output.join("body.part")).unwrap(), b"abd");
    let failure: Value =
        serde_json::from_slice(&std::fs::read(output.join("failure.json")).unwrap()).unwrap();
    assert_eq!(failure["error"], "content_mismatch");
    for name in [
        "body.bin",
        "summary.json",
        "permission.candid",
        "manifest.candid",
        "download.candid",
        "reference-status.candid",
        "certificate-binding.json",
    ] {
        assert!(!output.join(name).exists());
    }
    std::fs::write(base.path().join("body.bin"), b"abc").unwrap();
    assert!(matches!(
        crate::native::execute(&args),
        Err(Failure::ExistingRun)
    ));
    assert_eq!(std::fs::read(output.join("body.part")).unwrap(), b"abd");
}
