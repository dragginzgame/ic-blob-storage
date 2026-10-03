//! Retained control history selects sources; only phase owners authorize effects.
use super::*;
use crate::native::{artifacts::Run, publish_check::tests as fixture, upload_inputs::digest};
use ic_blob_storage::dto::configuration::{
    HostConfigurationView, HostFailure, ServiceInstallationInput,
};
use serde_json::json;

fn intent(batch: &PreparedBatch) -> SessionIntentRecord {
    let installation: ServiceInstallationInput =
        candidate_candid::decode(&batch.installation).unwrap();
    let upload = batch.files[0].input.permission.upload;
    serde_json::from_value(json!({"format":"ic-blob-storage/publication-session:retained-browser-handoffs",
        "source_session":null,"browser":null,"operation":"publish_session",
        "network":"ic","service_url":"https://icp-api.io/",
        "service":upload.service.to_text(),"namespace":upload.namespace.to_string(),
        "tenant":upload.tenant.to_text(),"uploader":installation.trusted_uploader.to_text(),
        "operator":installation.configuration.operator.to_text(),
        "verifier":installation.completion_verifier.to_text(),"gateway":"https://gateway.example/",
        "inventory_sha256":digest(&batch.inventory),"installation_sha256":digest(&batch.installation),
        "root_key_sha256":digest(b"selected-root"),"files":1,"max_steps":9,
        "max_service_updates":3,"max_service_queries":50,"timeout_seconds":30,
        "max_provider_requests":1,"automatic_retries":0,
        "input_verification":"one_complete_startup_pass_and_selected_body_before_setup"})).unwrap()
}

fn history(path: &Path, batch: &PreparedBatch, expected: &SessionIntentRecord) -> Run {
    let run = Run::create(path).unwrap();
    run.json("intent.json", expected).unwrap();
    run.bytes("inventory.json", &batch.inventory).unwrap();
    run.bytes("installation.candid", &batch.installation)
        .unwrap();
    let installation: ServiceInstallationInput =
        candidate_candid::decode(&batch.installation).unwrap();
    let host = HostConfigurationView {
        configuration: installation.configuration,
        project: installation.project,
        completion_verifier: installation.completion_verifier,
        trusted_uploader: installation.trusted_uploader,
        release: ic_blob_storage::LIBRARY_VERSION.into(),
        fenced: false,
    };
    let configuration = Run::create(&path.join("configuration")).unwrap();
    configuration
        .bytes(
            "query-0000-reply.candid",
            &candid::encode_one(Ok::<_, HostFailure>(host)).unwrap(),
        )
        .unwrap();
    run
}
fn request(path: &Path, step: usize, frame: &Frame) -> PathBuf {
    let path = path.join(format!("step-{step:04}"));
    Run::create(&path)
        .unwrap()
        .json("request.json", frame)
        .unwrap();
    path
}

#[test]
fn reopened_sources_keep_original_setup_and_observation_without_following_report_json() {
    let directory = fixture::frozen();
    let batch = fixture::batch(&directory);
    let mut expected = intent(&batch);
    let source = directory.path().join("original");
    history(&source, &batch, &expected);
    let setup = request(
        &source,
        0,
        &Frame::Prepare {
            index: 0,
            source_run: None,
        },
    )
    .join("setup");
    std::fs::create_dir(&setup).unwrap();
    request(
        &source,
        1,
        &Frame::Prepare {
            index: 0,
            source_run: Some(setup.clone()),
        },
    );
    request(
        &source,
        2,
        &Frame::Verify {
            index: 0,
            source_observation: None,
        },
    );
    let observation = source.join("verification-0000/observation");
    std::fs::create_dir_all(&observation).unwrap();
    // Presentation values confer no advancement or authority.
    std::fs::write(
        source.join("summary.json"),
        b"{\"all_references_live\":true}",
    )
    .unwrap();
    expected.max_steps = 1;
    expected.max_service_queries = 10;
    expected.timeout_seconds = 60;
    let mut sources = Sources::load(Some(&source), &expected, &batch).unwrap();
    let status_only = directory.path().join("status-only");
    expected.source_session = Some(source.clone());
    let resumed = history(&status_only, &batch, &expected);
    sources.record(&resumed, &source).unwrap();
    request(&status_only, 0, &Frame::Status { index: 0 });
    let carried = Sources::load(Some(&status_only), &expected, &batch).unwrap();
    assert_eq!(carried.setup, sources.setup);
    assert_eq!(carried.observations, sources.observations);
    let provenance = std::fs::read(status_only.join("source-session.json")).unwrap();
    std::fs::remove_file(status_only.join("source-session.json")).unwrap();
    assert!(matches!(
        Sources::load(Some(&status_only), &expected, &batch),
        Err(Failure::Binding)
    ));
    std::fs::write(status_only.join("source-session.json"), provenance).unwrap();
    assert!(
        matches!(sources.resolve(Frame::Prepare { index:0, source_run:None }).unwrap(),
        Frame::Prepare { source_run:Some(path), .. } if path == setup.canonicalize().unwrap())
    );
    assert!(
        matches!(sources.resolve(Frame::Verify { index:0, source_observation:None }).unwrap(),
        Frame::Verify { source_observation:Some(path), .. } if path == observation.canonicalize().unwrap())
    );
    let foreign = directory.path().join("foreign");
    std::fs::create_dir(&foreign).unwrap();
    for frame in [
        Frame::Prepare {
            index: 0,
            source_run: Some(foreign.clone()),
        },
        Frame::Verify {
            index: 0,
            source_observation: Some(foreign),
        },
    ] {
        assert!(matches!(sources.resolve(frame), Err(Failure::Binding)));
    }
}

#[test]
fn source_history_refuses_changed_scope_content_roles_gateway_root_and_release() {
    let directory = fixture::frozen();
    let batch = fixture::batch(&directory);
    let expected = intent(&batch);
    let source = directory.path().join("original");
    history(&source, &batch, &expected);
    let mut missing = serde_json::to_value(&expected).unwrap();
    missing.as_object_mut().unwrap().remove("source_session");
    assert!(serde_json::from_value::<SessionIntentRecord>(missing).is_err());
    let mut missing = serde_json::to_value(&expected).unwrap();
    missing.as_object_mut().unwrap().remove("browser");
    assert!(serde_json::from_value::<SessionIntentRecord>(missing).is_err());
    for key in [
        "tenant",
        "service",
        "namespace",
        "operator",
        "uploader",
        "verifier",
        "gateway",
        "service_url",
        "network",
        "root_key_sha256",
        "inventory_sha256",
        "installation_sha256",
    ] {
        let mut changed = serde_json::to_value(&expected).unwrap();
        changed[key] = json!("changed");
        let changed = serde_json::from_value(changed).unwrap();
        assert!(matches!(
            Sources::load(Some(&source), &changed, &batch),
            Err(Failure::Binding)
        ));
    }
    let reply = source.join("configuration/query-0000-reply.candid");
    let original = std::fs::read(&reply).unwrap();
    let mut host: Result<HostConfigurationView, HostFailure> =
        candid::decode_one(&original).unwrap();
    host.as_mut().unwrap().release = "different-release".into();
    std::fs::write(&reply, candid::encode_one(host).unwrap()).unwrap();
    assert!(matches!(
        Sources::load(Some(&source), &expected, &batch),
        Err(Failure::Binding)
    ));
    std::fs::write(&reply, original).unwrap();
    let mut inventory = batch.inventory.clone();
    inventory[0] ^= 1;
    std::fs::write(source.join("inventory.json"), inventory).unwrap();
    assert!(matches!(
        Sources::load(Some(&source), &expected, &batch),
        Err(Failure::Binding)
    ));
}

#[test]
fn source_history_preserves_exact_browser_selection_and_refuses_switch_to_unbound() {
    let directory = fixture::frozen();
    let batch = fixture::batch(&directory);
    let mut expected = intent(&batch);
    let source = directory.path().join("original-browser");
    expected.browser = Some(serde_json::from_value(json!({
        "format":"ic-blob-storage/browser-selection", "session":source,
        "profile":directory.path().join("profile"), "project":"fixture-project", "bucket":"fixture-bucket",
        "asset_port":45000,"signer_sha256":"1".repeat(64),"host_sha256":"2".repeat(64),
        "worker_sha256":"3".repeat(64),"database":"original","max_slots":1,
    })).unwrap());
    history(&source, &batch, &expected);
    assert!(Sources::load(Some(&source), &expected, &batch).is_ok());
    for (key, value) in [
        ("profile", json!(directory.path().join("other"))),
        ("asset_port", json!(45001)),
        ("signer_sha256", json!("4".repeat(64))),
        ("database", json!("other")),
        ("session", json!(directory.path().join("other-session"))),
    ] {
        let mut changed = serde_json::to_value(&expected).unwrap();
        changed["browser"][key] = value;
        let changed = serde_json::from_value(changed).unwrap();
        assert!(matches!(
            Sources::load(Some(&source), &changed, &batch),
            Err(Failure::Binding)
        ));
    }
    expected.browser = None;
    assert!(matches!(
        Sources::load(Some(&source), &expected, &batch),
        Err(Failure::Binding)
    ));
}

#[test]
fn partial_or_discontinuous_control_history_never_selects_fresh_phase_sources() {
    let directory = fixture::frozen();
    let batch = fixture::batch(&directory);
    let expected = intent(&batch);
    for (label, step, frame) in [
        (
            "missing-setup",
            0,
            Some(Frame::Prepare {
                index: 0,
                source_run: None,
            }),
        ),
        (
            "missing-observation",
            0,
            Some(Frame::Verify {
                index: 0,
                source_observation: None,
            }),
        ),
        ("gap", 1, Some(Frame::Status { index: 0 })),
        (
            "wrong-index",
            0,
            Some(Frame::Prepare {
                index: 1,
                source_run: None,
            }),
        ),
        ("partial-request", 0, None),
    ] {
        let source = directory.path().join(label);
        history(&source, &batch, &expected);
        if let Some(frame) = frame {
            request(&source, step, &frame);
        } else {
            std::fs::create_dir(source.join("step-0000")).unwrap();
        }
        assert!(matches!(
            Sources::load(Some(&source), &expected, &batch),
            Err(Failure::Binding)
        ));
    }
    let mut partial = Sources {
        setup: vec![None],
        transfers: vec![None],
        observations: vec![None],
    };
    assert!(matches!(
        partial.resolve(Frame::Prepare {
            index: 1,
            source_run: None
        }),
        Err(Failure::Arguments)
    ));
}

#[test]
fn only_exact_unattempted_facts_distinguish_ordering_refusals_from_missing_claims() {
    let directory = fixture::frozen();
    let batch = fixture::batch(&directory);
    let expected = intent(&batch);
    let source = directory.path().join("blocked");
    history(&source, &batch, &expected);
    for (step, frame) in [
        Frame::Prepare {
            index: 0,
            source_run: None,
        },
        Frame::Verify {
            index: 0,
            source_observation: None,
        },
    ]
    .into_iter()
    .enumerate()
    {
        let action = request(&source, step, &frame);
        std::fs::write(
            action.join("unattempted.json"),
            serde_json::to_vec(&UnattemptedPhaseRecord {
                format: UnattemptedPhaseRecord::FORMAT.into(),
                index: 0,
                next_index: 1,
            })
            .unwrap(),
        )
        .unwrap();
    }
    let sources = Sources::load(Some(&source), &expected, &batch).unwrap();
    assert!(sources.setup[0].is_none());
    assert!(sources.observations[0].is_none());
    for invalid in [
        json!({"format":UnattemptedPhaseRecord::FORMAT,"index":0,"next_index":0}),
        json!({"format":UnattemptedPhaseRecord::FORMAT,"index":1,"next_index":0}),
        json!({"format":"different-contract","index":0,"next_index":1}),
    ] {
        std::fs::write(
            source.join("step-0000/unattempted.json"),
            serde_json::to_vec(&invalid).unwrap(),
        )
        .unwrap();
        assert!(matches!(
            Sources::load(Some(&source), &expected, &batch),
            Err(Failure::Binding)
        ));
    }
}
