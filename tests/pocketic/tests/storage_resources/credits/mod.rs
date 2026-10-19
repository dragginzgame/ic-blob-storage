//! Opt-in fully credited local funding histories; receipts are synthetic host facts.
use super::*;
use crate::support::REPLY_LIMITS;
use blob_test_protocol::storage::{
    funding::{CreditCommand, Intent, Phase as FundingPhase},
    resources::{FundingCreditResources, FundingPopulationIntent, HistoryPopulationBatch},
};
use ic_platform::candid::decoder;
use ic_testkit::pic::update_candid_args;

fn digest(operation: u128) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    Sha256::digest(operation.to_le_bytes()).into()
}

fn intent(f: &Fixture, operation: u128) -> Intent {
    Intent {
        service: f.service,
        cashier: f.operator,
        account: f.service,
        namespace: 1,
        operation,
        offered: 1,
        target_balance: None,
    }
}

fn measured(
    f: &Fixture,
    directory: &Path,
    name: &str,
    actor: Principal,
    input: CreditCommand,
) -> FundingCreditResources {
    let packet = ic_platform::candid::contract::encode_one(input).unwrap();
    retain_bytes(directory, &format!("{name}-request.candid"), &packet);
    let reply =
        f.harness
            .pic
            .update_call(f.service, actor, "fixture_measure_funding_credit", packet);
    retain(
        directory,
        &format!("{name}-result.json"),
        &serde_json::json!({"result":format!("{reply:?}")}),
    );
    let reply = reply.unwrap();
    retain_bytes(directory, &format!("{name}-reply.cbor"), &reply);
    let observation: FundingCreditResources =
        decoder::decode_one_with_config(&reply, &decoder::DecoderConfig::new()).unwrap();
    retain(
        directory,
        &format!("{name}-metrics.json"),
        &serde_json::json!({
            "instructions":observation.instructions,"heap_bytes":observation.heap_bytes,
            "stable_bytes":observation.stable_bytes,
            "reads":memory_reads(&observation.reads)
        }),
    );
    observation
}

#[test]
fn credit_measurements_have_independent_exact_read_windows() {
    let f = Fixture::new();
    let directory = tempfile::tempdir().unwrap();
    let batch = HistoryPopulationBatch {
        funding_start: 0,
        read_start: 0,
        reads: Vec::new(),
        funding: vec![FundingPopulationIntent {
            intent: intent(&f, 1),
            phase: FundingPhase::Callback(0),
            credit_digest: None,
        }],
    };
    update_candid_args::<(Result<(), Failure>,), _>(
        &f.harness.pic,
        f.service,
        f.operator,
        "populate_histories",
        (batch,),
        REPLY_LIMITS,
    )
    .map(|(reply,)| reply)
    .unwrap()
    .unwrap();
    let command = CreditCommand {
        intent: intent(&f, 1),
        accepted: 1,
        receipt_digest: digest(1),
        established: true,
        fault: None,
    };
    let denied = measured(&f, directory.path(), "denied", f.other, command);
    assert_eq!(denied.result, Err(Failure::Denied));
    assert!(denied.reads.iter().all(|r| r.calls == 0));
    let confirmed = measured(&f, directory.path(), "confirmed", f.operator, command);
    assert_eq!(confirmed.result, Ok(Some(true)));
    assert!(confirmed.reads.iter().any(|r| r.calls > 0));
    let replay = measured(&f, directory.path(), "replay", f.operator, command);
    assert_eq!(replay.result, Ok(Some(false)));
    let denied_again = measured(&f, directory.path(), "denied-again", f.other, command);
    assert_eq!(denied_again.result, Err(Failure::Denied));
    assert!(denied_again.reads.iter().all(|r| r.calls == 0));
    for observation in [denied, confirmed, replay, denied_again] {
        for row in observation.reads {
            assert!(!row.calls_saturated);
            assert!(!row.bytes_saturated);
            assert!(!row.instructions_saturated);
            assert!(row.instructions <= u128::from(observation.instructions));
        }
    }
}

#[test]
#[ignore = "opt-in receipt profile; needs a fresh BLOB_FUNDING_RECEIPT_PROFILE directory"]
fn receipt_populated_confirmation_and_restoration_profile() {
    let directory =
        std::path::PathBuf::from(std::env::var_os("BLOB_FUNDING_RECEIPT_PROFILE").unwrap());
    std::fs::create_dir(&directory).unwrap();
    for count in [100, 1_000, 10_000] {
        let path = directory.join(format!("receipts-{count}"));
        std::fs::create_dir(&path).unwrap();
        profile(&path, count);
    }
    retain(
        &directory,
        "summary.json",
        &serde_json::json!({
            "tiers":[100,1000,10000],"paid_cycles":"0","provider_requests":0,
            "limitations":["synthetic transport and host receipts", "fixture read observer overhead",
                "single local Linux observation, not production or provider qualification"]
        }),
    );
}

#[expect(
    clippy::too_many_lines,
    reason = "One retained tier binds canonical population, credit refusals, replay and actual restore"
)]
fn profile(directory: &Path, count: u32) {
    let harness = Harness::new();
    let operator = Fake::principal(1);
    let controller = Fake::principal(2);
    let service = harness.pic.create_canister_with_settings(
        Some(controller),
        Some(CanisterSettings {
            controllers: Some(vec![controller]),
            ..CanisterSettings::default()
        }),
    );
    let installation = ic_platform::candid::contract::encode_one(StorageProbeInstallation {
        operator,
        max_objects: 2,
        max_tenants: 2,
        max_object_bytes: 10,
        max_funding_attempts: count,
        max_read_sessions: 1,
        max_tenant_read_sessions: 1,
    })
    .unwrap();
    retain_bytes(directory, "installation.wire", &installation);
    harness.pic.install_canister(
        service,
        Fixture::wasm(),
        installation.clone(),
        Some(controller),
    );
    let f = Fixture {
        harness,
        service,
        operator,
        controller,
        tenant: Principal::anonymous(),
        uploader: Fake::principal(4),
        other: Fake::principal(5),
    };
    let started = Instant::now();
    for start in (0..count).step_by(16) {
        let batch = HistoryPopulationBatch {
            funding_start: u64::from(start),
            read_start: 0,
            reads: Vec::new(),
            funding: (start..(start + 16).min(count))
                .map(|index| {
                    let operation = u128::from(index + 1);
                    FundingPopulationIntent {
                        intent: intent(&f, operation),
                        phase: FundingPhase::Callback(0),
                        credit_digest: (index + 1 != count).then(|| digest(operation)),
                    }
                })
                .collect(),
        };
        let packet = ic_platform::candid::contract::encode_one(batch).unwrap();
        assert!(packet.len() <= STORAGE_PROBE_INPUT_BYTES);
        retain_bytes(
            directory,
            &format!("population-{start}-request.candid"),
            &packet,
        );
        let reply = f
            .harness
            .pic
            .update_call(service, operator, "populate_histories", packet);
        retain(
            directory,
            &format!("population-{start}-result.json"),
            &serde_json::json!({
                "elapsed_ms":started.elapsed().as_millis(),"result":format!("{reply:?}")
            }),
        );
        let reply = reply.unwrap();
        retain_bytes(directory, &format!("population-{start}-reply.cbor"), &reply);
        assert_eq!(
            decoder::decode_one_with_config::<Result<(), Failure>>(
                &reply,
                &decoder::DecoderConfig::new()
            )
            .unwrap(),
            Ok(())
        );
        assert!(
            started.elapsed() < std::time::Duration::from_secs(180),
            "local tier population budget exhausted"
        );
    }
    retain(
        directory,
        "population.json",
        &serde_json::json!({"intents":count,"confirmed":count-1,
        "elapsed_ms":started.elapsed().as_millis()}),
    );
    let original = f.local_status();
    assert_eq!(original.funding.transport_accepted, u128::from(count));
    assert_eq!(original.funding.uncredited_accepted, 1);
    let command = CreditCommand {
        intent: intent(&f, u128::from(count)),
        accepted: 1,
        receipt_digest: digest(u128::from(count)),
        established: true,
        fault: None,
    };
    let before = f.harness.pic.get_stable_memory(service);
    assert_eq!(
        measured(&f, directory, "denied", f.other, command).result,
        Err(Failure::Denied)
    );
    assert_eq!(
        measured(
            &f,
            directory,
            "reused",
            operator,
            CreditCommand {
                receipt_digest: digest(1),
                ..command
            }
        )
        .result,
        Err(Failure::Conflict)
    );
    assert!(
        f.harness.pic.get_stable_memory(service) == before,
        "refusal changed stable bytes"
    );
    assert_eq!(
        measured(&f, directory, "confirmed", operator, command).result,
        Ok(Some(true))
    );
    let after = f.harness.pic.get_stable_memory(service);
    assert_eq!(
        measured(&f, directory, "replay", operator, command).result,
        Ok(Some(false))
    );
    assert!(
        f.harness.pic.get_stable_memory(service) == after,
        "replay changed stable bytes"
    );
    let credited = f.local_status();
    assert_eq!(
        credited.funding.transport_accepted,
        original.funding.transport_accepted
    );
    assert_eq!(credited.funding.uncredited_accepted, 0);
    let upgrade =
        f.harness
            .pic
            .upgrade_canister(service, Fixture::wasm(), installation, Some(controller));
    retain(
        directory,
        "upgrade.json",
        &serde_json::json!({"result":format!("{upgrade:?}")}),
    );
    upgrade.unwrap();
    let restored = f.local_status();
    assert_eq!(
        restored.funding.transport_accepted,
        credited.funding.transport_accepted
    );
    assert_eq!(restored.funding.uncredited_accepted, 0);
    assert!(restored.funding.fenced);
    retain(directory, "restoration.json", &resources(&f));
    let reopened = f.harness.pic.get_stable_memory(service);
    assert_eq!(
        measured(&f, directory, "fenced", operator, command).result,
        Err(Failure::Fenced)
    );
    assert!(
        f.harness.pic.get_stable_memory(service) == reopened,
        "replay changed stable bytes"
    );
    retain_bytes(directory, "stable.bin", &reopened);
}
