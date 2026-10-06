//! Opt-in fully credited local funding histories; receipts are synthetic host facts.
use super::*;
use blob_test_protocol::storage::{
    funding::{CreditCommand, Intent, Phase as FundingPhase},
    resources::{FundingCreditResources, FundingPopulationIntent, HistoryPopulationBatch},
};

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
    let packet = candid::encode_one(input).unwrap();
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
    retain_bytes(directory, &format!("{name}-reply.candid"), &reply);
    let observation: FundingCreditResources = candid::decode_one(&reply).unwrap();
    retain(
        directory,
        &format!("{name}-metrics.json"),
        &serde_json::json!({
            "instructions":observation.instructions,"heap_bytes":observation.heap_bytes,
            "stable_bytes":observation.stable_bytes,
            "reads":observation.reads.iter().map(|r| serde_json::json!({"memory":r.memory,
                "calls":r.calls,"bytes":r.bytes,"instructions":r.instructions})).collect::<Vec<_>>()
        }),
    );
    observation
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
    let installation = candid::encode_one(StorageProbeInstallation {
        operator,
        max_objects: 2,
        max_tenants: 2,
        max_object_bytes: 10,
        max_funding_attempts: count,
        max_read_sessions: 1,
        max_tenant_read_sessions: 1,
    })
    .unwrap();
    retain_bytes(directory, "installation.candid", &installation);
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
        let packet = candid::encode_one(batch).unwrap();
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
        retain_bytes(
            directory,
            &format!("population-{start}-reply.candid"),
            &reply,
        );
        assert_eq!(
            candid::decode_one::<Result<(), Failure>>(&reply).unwrap(),
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
