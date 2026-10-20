use super::*;
use ic_host_fs::testing::FixtureDirectory;
use ic_platform::agent::{BasicIdentity, Identity};
use serde_json::json;

fn agent() -> (Agent, Principal) {
    // Fixed test-only seed [42; 32], never a deployment identity.
    let pem = b"-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEICoqKioqKioqKioqKioqKioqKioqKioqKioqKioqKioq\n-----END PRIVATE KEY-----\n";
    let identity = BasicIdentity::from_pem(pem.as_slice()).unwrap();
    let actor = identity.sender().unwrap();
    (
        Agent::builder()
            .with_url("http://127.0.0.1:1")
            .with_identity(identity)
            .build()
            .unwrap(),
        actor.into(),
    )
}

#[test]
fn exact_signed_packet_is_durable_and_partial_or_complete_claims_refuse_reuse() {
    let (agent, actor) = agent();
    let base = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let directory = base.path().join("attempt");
    let service = Principal::self_authenticating([7]);
    let argument = ic_blob_storage_contracts::document::encode(42u64).unwrap();
    let input = || UpdateInput {
        service,
        method: "test_update",
        argument: &argument,
        argument_file: "request.wire",
        directory: &directory,
        reply_limit: SMALL_REPLY_BYTES,
    };
    let prepared = PreparedUpdate::claim(&agent, input(), || json!({"operation":"test"})).unwrap();
    let packet = std::fs::read(directory.join("signed-request.cbor")).unwrap();
    assert_eq!(
        std::fs::read(directory.join("request.wire")).unwrap(),
        argument
    );
    ic_platform::agent::signed_update_inspect(
        actor.into(),
        service.into(),
        "test_update",
        &argument,
        prepared.signed.ingress_expiry,
        packet.clone(),
    )
    .unwrap();
    let intent: serde_json::Value =
        serde_json::from_slice(&std::fs::read(directory.join("intent.json")).unwrap()).unwrap();
    assert_eq!(
        intent,
        json!({
            "operation":"test",
            "request_id":prepared.signed.request_id.to_string(),
            "ingress_expiry_ns":prepared.signed.ingress_expiry.to_string(),
            "signed_request_sha256":ContentDigest::compute(&packet).to_string(),
            "root_key_sha256":ContentDigest::compute(&agent.read_root_key()).to_string(),
            "max_update_requests":1,
            "request_deadline_seconds":30,
            "retry_authorized":false,
        })
    );
    assert_eq!(
        record::inspect::<serde_json::Value>(
            &serde_json::to_vec(&intent).unwrap(),
            packet.clone(),
            actor,
            service,
            "test_update",
            &argument,
            &agent.read_root_key(),
        )
        .unwrap(),
        json!({"operation":"test"})
    );
    assert!(matches!(
        PreparedUpdate::claim(&agent, input(), || json!({})),
        Err(Failure::SubmissionClaimed)
    ));
    assert_eq!(
        std::fs::read(directory.join("signed-request.cbor")).unwrap(),
        packet
    );
    let partial = base.path().join("partial");
    std::fs::create_dir(&partial).unwrap();
    assert!(matches!(
        PreparedUpdate::claim(
            &agent,
            UpdateInput {
                directory: &partial,
                ..input()
            },
            || json!({})
        ),
        Err(Failure::SubmissionClaimed)
    ));
    assert!(std::fs::read_dir(partial).unwrap().next().is_none());
}

#[test]
fn historical_reader_accepts_expired_correspondence_and_refuses_corrupt_claims() {
    let (agent, actor) = agent();
    let service = Principal::self_authenticating([7]);
    let argument = ic_blob_storage_contracts::document::encode(42u64).unwrap();
    let signed = agent
        .update(&service.into(), "test_update")
        .with_arg(argument.clone())
        .expire_at(std::time::UNIX_EPOCH + Duration::from_secs(1))
        .sign()
        .unwrap();
    let packet = signed.signed_update;
    let root = agent.read_root_key();
    let intent = json!({
        "request_id":signed.request_id.to_string(), "ingress_expiry_ns":signed.ingress_expiry.to_string(),
        "signed_request_sha256":ContentDigest::compute(&packet).to_string(),
        "root_key_sha256":ContentDigest::compute(&root).to_string(),
        "max_update_requests":1, "request_deadline_seconds":30, "retry_authorized":false,
        "operation":"historical"
    });
    let inspect = |intent: &[u8], packet: Vec<u8>, method| {
        record::inspect::<serde_json::Value>(
            intent, packet, actor, service, method, &argument, &root,
        )
    };
    let original = serde_json::to_vec(&intent).unwrap();
    assert_eq!(
        inspect(&original, packet.clone(), "test_update").unwrap(),
        json!({"operation":"historical"})
    );
    for (field, invalid) in [
        ("max_update_requests", json!(2)),
        ("request_deadline_seconds", json!(31)),
        ("retry_authorized", json!(true)),
        ("ingress_expiry_ns", json!("2")),
        ("signed_request_sha256", json!("changed")),
        ("root_key_sha256", json!("changed")),
    ] {
        let mut changed = intent.clone();
        changed[field] = invalid;
        assert!(matches!(
            inspect(
                &serde_json::to_vec(&changed).unwrap(),
                packet.clone(),
                "test_update"
            ),
            Err(Failure::Binding)
        ));
    }
    assert!(matches!(
        inspect(&original, packet.clone(), "different_method"),
        Err(Failure::Binding)
    ));
    assert!(matches!(
        inspect(&original, b"changed".to_vec(), "test_update"),
        Err(Failure::Binding)
    ));
    assert!(matches!(
        inspect(br#"{"operation":"partial"}"#, packet.clone(), "test_update"),
        Err(Failure::Binding)
    ));
    assert!(matches!(
        inspect(b"{", packet, "test_update"),
        Err(Failure::Arguments)
    ));
}

struct FailedIntent;
impl Serialize for FailedIntent {
    fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
        Err(serde::ser::Error::custom("deliberate intent failure"))
    }
}

#[test]
fn failed_intent_retains_packets_and_permanent_claim_without_dispatch() {
    let (agent, _) = agent();
    let base = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let directory = base.path().join("attempt");
    let input = || UpdateInput {
        service: Principal::self_authenticating([7]),
        method: "test_update",
        argument: b"request",
        argument_file: "request.wire",
        directory: &directory,
        reply_limit: SMALL_REPLY_BYTES,
    };
    assert!(matches!(
        PreparedUpdate::claim(&agent, input(), || FailedIntent),
        Err(Failure::File)
    ));
    assert_eq!(
        std::fs::read(directory.join("request.wire")).unwrap(),
        b"request"
    );
    assert!(directory.join("signed-request.cbor").is_file());
    assert!(!directory.join("intent.json").exists());
    assert!(matches!(
        PreparedUpdate::claim(&agent, input(), || json!({})),
        Err(Failure::SubmissionClaimed)
    ));
}

#[test]
fn completion_retains_present_pending_and_failed_observations_before_returning() {
    let base = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    for (name, response, expected, failure) in [
        ("present", Ok(Some(vec![7])), Some(7), None),
        ("pending", Ok(None), None, None),
        (
            "transport",
            Err(Failure::Transport),
            None,
            Some(Failure::Transport),
        ),
        (
            "decode",
            Ok(Some(Vec::new())),
            None,
            Some(Failure::InvalidReply),
        ),
    ] {
        let path = base.path().join(name);
        let present = response.as_ref().is_ok_and(Option::is_some);
        let decoded = std::cell::Cell::new(false);
        let result = DispatchedUpdate {
            run: Run::create(&path).unwrap(),
            request_id: name.into(),
            response,
        }
        .finish(
            |bytes| {
                decoded.set(true);
                bytes.first().copied().ok_or(Failure::InvalidReply)
            },
            |id, result| {
                json!({"request_id": id,
                "outcome": outcome(result, "accepted", |_| false),
                "error": result.as_ref().err().map(|error| error.code()), "retry_authorized": false})
            },
        );
        assert_eq!(decoded.get(), present);
        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(path.join("outcome.json")).unwrap()).unwrap();
        assert_eq!(saved["request_id"], name);
        assert_eq!(saved["retry_authorized"], false);
        if let Some(error) = failure {
            assert_eq!(saved["error"], error.code());
            assert_eq!(result, Err(error));
        } else {
            let (observed, report) = result.unwrap();
            assert_eq!(observed, expected);
            assert_eq!(saved, report);
        }
    }
}

#[test]
fn completion_write_failure_preserves_existing_evidence_and_precedes_remote_failure() {
    let base = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let path = base.path().join("occupied-outcome");
    let run = Run::create(&path).unwrap();
    run.bytes("outcome.json", b"original evidence").unwrap();
    let completed = DispatchedUpdate {
        run,
        request_id: "original".into(),
        response: Err(Failure::Transport),
    }
    .finish(
        |_| -> Result<(), Failure> { panic!("absent response must not decode") },
        |_, _| json!({"replacement":true}),
    );
    assert_eq!(completed, Err(Failure::File));
    assert_eq!(
        std::fs::read(path.join("outcome.json")).unwrap(),
        b"original evidence"
    );
}
