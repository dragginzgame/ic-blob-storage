use super::*;
use ic_agent::{Identity, identity::BasicIdentity};
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
        actor,
    )
}

#[test]
fn exact_signed_packet_is_durable_and_partial_or_complete_claims_refuse_reuse() {
    let (agent, actor) = agent();
    let base = tempfile::tempdir().unwrap();
    let directory = base.path().join("attempt");
    let service = Principal::self_authenticating([7]);
    let argument = candid::encode_one(42u64).unwrap();
    let input = || UpdateInput {
        service,
        method: "test_update",
        argument: &argument,
        argument_file: "request.candid",
        directory: &directory,
        reply_limit: SMALL_REPLY_BYTES,
    };
    let prepared = PreparedUpdate::claim(&agent, input(), || json!({"operation":"test"})).unwrap();
    let packet = std::fs::read(directory.join("signed-request.cbor")).unwrap();
    assert_eq!(
        std::fs::read(directory.join("request.candid")).unwrap(),
        argument
    );
    ic_agent::agent::signed_update_inspect(
        actor,
        service,
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

struct FailedIntent;
impl Serialize for FailedIntent {
    fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
        Err(serde::ser::Error::custom("deliberate intent failure"))
    }
}

#[test]
fn failed_intent_retains_packets_and_permanent_claim_without_dispatch() {
    let (agent, _) = agent();
    let base = tempfile::tempdir().unwrap();
    let directory = base.path().join("attempt");
    let input = || UpdateInput {
        service: Principal::self_authenticating([7]),
        method: "test_update",
        argument: b"request",
        argument_file: "request.candid",
        directory: &directory,
        reply_limit: SMALL_REPLY_BYTES,
    };
    assert!(matches!(
        PreparedUpdate::claim(&agent, input(), || FailedIntent),
        Err(Failure::File)
    ));
    assert_eq!(
        std::fs::read(directory.join("request.candid")).unwrap(),
        b"request"
    );
    assert!(directory.join("signed-request.cbor").is_file());
    assert!(!directory.join("intent.json").exists());
    assert!(matches!(
        PreparedUpdate::claim(&agent, input(), || json!({})),
        Err(Failure::SubmissionClaimed)
    ));
}
