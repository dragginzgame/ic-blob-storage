//! Opt-in measurements of ordinary bounded funding history and interrupted reads.
use super::*;
use crate::storage_resources::{resources, retain, retain_bytes};
use blob_test_protocol::storage::funding::{
    Action as FundingAction, Command as FundingCommand, Intent, Phase as FundingPhase,
    Request as FundingRequest,
};
use std::{path::Path, time::Instant};

fn funding_command(
    f: &Fixture,
    directory: &Path,
    intent: Intent,
    action: FundingAction,
) -> Result<bool, Failure> {
    let packet = candid::encode_one(FundingCommand {
        intent,
        action,
        fault: None,
        source: None,
    })
    .unwrap();
    let name = format!("funding-{}-{action:?}", intent.operation);
    retain_bytes(directory, &format!("{name}.candid"), &packet);
    let result = f
        .harness
        .pic
        .update_call(f.service, f.operator, "fixture_funding", packet);
    retain(
        directory,
        &format!("{name}.json"),
        &serde_json::json!({"result": format!("{result:?}")}),
    );
    candid::decode_one(&result.unwrap()).unwrap()
}

fn read_command(
    f: &Fixture,
    directory: &Path,
    name: &str,
    input: ReadSessionInput,
    interrupted: bool,
) -> Result<(), Failure> {
    let packet = candid::encode_one(input).unwrap();
    retain_bytes(directory, &format!("{name}.candid"), &packet);
    let result = f
        .harness
        .pic
        .update_call(f.service, f.tenant, "fixture_read_chunk", packet);
    retain(
        directory,
        &format!("{name}.json"),
        &serde_json::json!({"intentional_callback_trap": interrupted, "result": format!("{result:?}")}),
    );
    if interrupted {
        assert_eq!(result.unwrap_err().reject_code, RejectCode::CanisterError);
        Ok(())
    } else {
        let result: Result<JourneyReadChunk, Failure> =
            candid::decode_one(&result.unwrap()).unwrap();
        result.map(|chunk| assert_eq!(chunk.bytes, vec![1; 10]))
    }
}

fn funding_lookup(
    f: &Fixture,
    actor: Principal,
    intent: Intent,
) -> Result<Option<FundingPhase>, Failure> {
    f.harness
        .pic
        .query_candid_as(f.service, actor, "funding_lookup", (intent,))
        .unwrap()
}

fn funding_request(f: &Fixture, intent: Intent) -> FundingRequest {
    f.harness
        .pic
        .query_candid_as::<Result<FundingRequest, Failure>, _>(
            f.service,
            f.operator,
            "funding_request",
            (intent,),
        )
        .unwrap()
        .unwrap()
}

#[test]
#[ignore = "opt-in occupied-history profile; needs a fresh BLOB_HISTORY_RESOURCE_REPORT directory"]
fn occupied_history_restoration_profile() {
    let directory =
        std::path::PathBuf::from(std::env::var_os("BLOB_HISTORY_RESOURCE_REPORT").unwrap());
    std::fs::create_dir(&directory).unwrap();
    for (name, final_phase) in [
        ("empty", None),
        ("terminal", Some(FundingPhase::Callback(100))),
        ("prepared", Some(FundingPhase::Prepared)),
        ("uncertain", Some(FundingPhase::Uncertain)),
    ] {
        let workload = directory.join(name);
        std::fs::create_dir(&workload).unwrap();
        profile_workload(&workload, final_phase);
    }
    retain(
        &directory,
        "summary.json",
        &serde_json::json!({
            "workloads_passed": ["empty", "terminal", "prepared", "uncertain"],
            "live_provider_requests": 0, "paid_cycles": "0",
            "limitations": ["four funding intents and one occupied read at ordinary fixture limits",
                "completed read rows are removed; only sequence history persists",
                "local labelled substitutes, not deployed provider evidence",
                "observer overhead; no production or expanded-scale qualification"]
        }),
    );
}

#[expect(
    clippy::too_many_lines,
    reason = "One retained restore compares exact funding, interrupted read occupancy and whole-service fences"
)]
fn profile_workload(directory: &Path, final_phase: Option<FundingPhase>) {
    let f = Fixture::with_gateway_source();
    let installation = Fixture::installation(f.operator);
    retain_bytes(directory, "installation.candid", &installation);
    let input = f.read_input();
    let mut intents = Vec::new();
    if let Some(final_phase) = final_phase {
        for (operation, phase) in [
            (1, FundingPhase::Callback(25)),
            (2, FundingPhase::NotEnqueued),
            (3, FundingPhase::Callback(100)),
            (4, final_phase),
        ] {
            let intent = f.funding_intent(operation, 100);
            assert_eq!(
                funding_command(&f, directory, intent, FundingAction::Prepare),
                Ok(true)
            );
            if phase != FundingPhase::Prepared {
                assert_eq!(
                    funding_command(&f, directory, intent, FundingAction::Attempt),
                    Ok(true)
                );
            }
            match phase {
                FundingPhase::Callback(refunded) => {
                    assert_eq!(
                        funding_command(&f, directory, intent, FundingAction::Callback(refunded)),
                        Ok(true)
                    );
                }
                FundingPhase::NotEnqueued => {
                    assert_eq!(
                        funding_command(&f, directory, intent, FundingAction::NotEnqueued),
                        Ok(true)
                    );
                }
                _ => {}
            }
            assert_eq!(funding_lookup(&f, f.operator, intent), Ok(Some(phase)));
            intents.push((intent, phase, funding_request(&f, intent)));
        }
        for sequence in 1..=64 {
            assert_eq!(
                read_command(
                    &f,
                    directory,
                    &format!("completed-read-{sequence}"),
                    input,
                    false,
                ),
                Ok(())
            );
        }
        assert_eq!(f.session_view().last_sequence, 64);
        assert_eq!(f.session_view().sessions, 0);
        assert_eq!(f.session_view().reserved_bytes, 0);
    }
    let interrupted = matches!(
        final_phase,
        Some(FundingPhase::Prepared | FundingPhase::Uncertain)
    );
    if interrupted {
        assert_eq!(
            read_command(
                &f,
                directory,
                "interrupted-read",
                ReadSessionInput {
                    callback_fault: Some(WriteFault::ReadJournal),
                    ..input
                },
                true,
            ),
            Ok(())
        );
        let before = f.session_view();
        assert_eq!(
            (before.last_sequence, before.sessions, before.reserved_bytes),
            (65, 1, 2048)
        );
        assert_eq!(
            read_command(&f, directory, "capacity-refusal", input, false),
            Err(Failure::Capacity)
        );
        assert_eq!(f.session_view(), before);
    }
    let before = f.local_status();
    assert_eq!(before.funding.retained_intents, intents.len() as u64);
    let chunks = f.chunk_observation().requests;
    retain(
        directory,
        "before-reopen.json",
        &serde_json::json!({
            "local_status": format!("{before:?}"), "resources": resources(&f),
            "exact_funding": format!("{intents:?}"), "local_chunk_calls": chunks
        }),
    );
    let started = Instant::now();
    let result = f.harness.pic.upgrade_canister(
        f.service,
        Fixture::wasm(),
        installation,
        Some(f.controller),
    );
    let elapsed = started.elapsed().as_millis();
    retain(
        directory,
        "upgrade-result.json",
        &serde_json::json!({"result": format!("{result:?}"), "elapsed_ms": elapsed}),
    );
    result.unwrap();
    let restored = resources(&f);
    let mut expected = before;
    expected.uploads.fenced = true;
    expected.funding.fenced = true;
    expected.gateways.fenced = true;
    expected.reads.fenced = true;
    assert_eq!(f.local_status(), expected);
    for (intent, phase, request) in intents {
        assert_eq!(funding_lookup(&f, f.operator, intent), Ok(Some(phase)));
        assert_eq!(funding_request(&f, intent), request);
        assert_eq!(funding_lookup(&f, f.other, intent), Err(Failure::Denied));
        assert_eq!(
            funding_lookup(
                &f,
                f.operator,
                Intent {
                    offered: 101,
                    ..intent
                }
            ),
            Err(Failure::Conflict)
        );
    }
    assert_eq!(
        read_command(&f, directory, "fenced-read-refusal", input, false),
        Err(Failure::Fenced)
    );
    assert_eq!(
        funding_command(
            &f,
            directory,
            f.funding_intent(5, 100),
            FundingAction::Prepare
        ),
        Err(Failure::Fenced)
    );
    assert_eq!(f.chunk_observation().requests, chunks);
    assert_eq!(f.local_status(), expected);
    assert_eq!(resources(&f), restored);
    retain(
        directory,
        "reopened.json",
        &serde_json::json!({
            "restored": restored, "elapsed_ms": elapsed,
            "whole_service_and_exact_funding_preserved": true,
            "read_occupancy_and_high_water_preserved": true,
            "mutation_and_foreign_funding_refused": true,
            "local_chunk_calls": chunks
        }),
    );
    println!(
        "occupied history {}: {restored}",
        directory.file_name().unwrap().to_str().unwrap()
    );
}
