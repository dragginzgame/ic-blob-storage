//! Opt-in larger lifetime funding histories.
use super::*;
use blob_test_protocol::storage::{
    funding::{Intent, Phase as FundingPhase, Request as FundingRequest},
    gateways::{Action, Command, Outcome, Scope},
    resources::{FundingPopulationIntent, HistoryPopulationBatch},
};
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_platform::pocket_ic::RejectResponse;

#[derive(Clone, Copy)]
struct HistoryWorkload {
    name: &'static str,
    funding: u32,
    tenants: u32,
    final_phase: FundingPhase,
}

fn send(
    f: &Fixture,
    directory: &Path,
    name: &str,
    actor: Principal,
    method: &str,
    packet: Vec<u8>,
) -> Result<Vec<u8>, RejectResponse> {
    retain_bytes(directory, &format!("{name}.cbor"), &packet);
    assert!(packet.len() <= STORAGE_PROBE_INPUT_BYTES);
    let result = f.harness.pic.update_call(f.service, actor, method, packet);
    retain(
        directory,
        &format!("{name}.json"),
        &serde_json::json!({"method": method, "actor": actor.to_text(), "result": format!("{result:?}")}),
    );
    result
}

fn populate(
    f: &Fixture,
    directory: &Path,
    name: &str,
    batch: &HistoryPopulationBatch,
) -> Result<(), Failure> {
    let reply = send(
        f,
        directory,
        name,
        f.operator,
        "populate_histories",
        ic_blob_storage_pocketic_tests::control_calls::encode_control_one(batch).unwrap(),
    )
    .unwrap();
    ic_blob_storage_pocketic_tests::control_calls::decode_control_one(&reply, 4 * 1024 * 1024)
        .unwrap()
}

fn funding(f: &Fixture, workload: HistoryWorkload, index: u32) -> FundingPopulationIntent {
    FundingPopulationIntent {
        credit_digest: None,
        intent: Intent {
            service: f.service.into(),
            cashier: f.operator.into(),
            account: f.service.into(),
            namespace: 1,
            operation: u128::from(index + 1),
            offered: if index == 0 { 100 } else { 1 },
            target_balance: None,
        },
        phase: if index + 1 == workload.funding {
            workload.final_phase
        } else if index == 0 {
            FundingPhase::Callback(25)
        } else if index.is_multiple_of(2) {
            FundingPhase::NotEnqueued
        } else {
            FundingPhase::Callback(1)
        },
    }
}

fn selected_funding(
    f: &Fixture,
    workload: HistoryWorkload,
) -> Vec<(Intent, FundingPhase, FundingRequest)> {
    [0, workload.funding / 2, workload.funding - 1]
        .into_iter()
        .map(|index| {
            let input = funding(f, workload, index);
            let phase: Result<Option<FundingPhase>, Failure> = f
                .harness
                .pic
                .query_control_as(
                    f.service,
                    f.operator,
                    "funding_lookup",
                    (input.intent,),
                    REPLY_BYTES,
                )
                .map(|(reply,)| reply)
                .unwrap();
            assert_eq!(phase, Ok(Some(input.phase)));
            let request: Result<FundingRequest, Failure> = f
                .harness
                .pic
                .query_control_as(
                    f.service,
                    f.operator,
                    "funding_request",
                    (input.intent,),
                    REPLY_BYTES,
                )
                .map(|(reply,)| reply)
                .unwrap();
            (input.intent, input.phase, request.unwrap())
        })
        .collect()
}

#[test]
#[ignore = "opt-in larger history profile; needs a fresh BLOB_SCALED_HISTORY_REPORT directory"]
fn scaled_occupied_history_restoration_profile() {
    let directory =
        std::path::PathBuf::from(std::env::var_os("BLOB_SCALED_HISTORY_REPORT").unwrap());
    std::fs::create_dir(&directory).unwrap();
    for workload in [
        HistoryWorkload {
            name: "funding-100",
            funding: 100,

            tenants: 1,
            final_phase: FundingPhase::Prepared,
        },
        HistoryWorkload {
            name: "funding-1000",
            funding: 1000,

            tenants: 1,
            final_phase: FundingPhase::Uncertain,
        },
        HistoryWorkload {
            name: "funding-10000",
            funding: 10_000,

            tenants: 1,
            final_phase: FundingPhase::Uncertain,
        },
    ] {
        let path = directory.join(workload.name);
        std::fs::create_dir(&path).unwrap();
        profile_workload(&path, workload);
    }
    retain(
        &directory,
        "summary.json",
        &serde_json::json!({"workloads_passed": 3,
        "live_provider_requests": 0, "paid_cycles": "0",
        "limitations": ["synthetic funding outcomes", "instrumented observer overhead", "no production, million-object or macOS qualification"]}),
    );
}

#[expect(
    clippy::too_many_lines,
    reason = "One retained local workload binds population, rollback/refusals, funding inspection and actual upgrade"
)]
fn profile_workload(directory: &Path, workload: HistoryWorkload) {
    let harness = Harness::new();
    let operator = fake_principal(1);
    let controller = fake_principal(2);
    let service = harness.pic.create_canister_with_settings(
        Some(controller),
        Some(CanisterSettings {
            controllers: Some(vec![controller]),
            ..CanisterSettings::default()
        }),
    );
    let profile = StorageProbeInstallation {
        operator,
        max_objects: workload.tenants * 4,
        max_tenants: workload.tenants,
        max_object_bytes: 10,
        max_funding_attempts: workload.funding,
    };
    let installation =
        blob_test_protocol::fixture_wire::encode_one("fixture_init", profile).unwrap();
    let restoration =
        blob_test_protocol::fixture_wire::encode_one("fixture_post_upgrade", profile).unwrap();
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
        uploader: fake_principal(4),
        other: fake_principal(5),
    };
    let objects = Workload {
        name: workload.name,
        objects: workload.tenants * 4,
        tenants: workload.tenants,
        bytes: 10,
    };
    for start in (0..objects.objects).step_by(16) {
        let batch = PopulationBatch {
            start,
            uploads: (start..(start + 16).min(objects.objects))
                .map(|i| preparation(&f, objects, i))
                .collect(),
        };
        let reply = send(
            &f,
            directory,
            &format!("uploads-{start}"),
            operator,
            "populate_resources",
            ic_blob_storage_pocketic_tests::control_calls::encode_control_one(batch).unwrap(),
        )
        .unwrap();
        assert_eq!(
            ic_blob_storage_pocketic_tests::control_calls::decode_control_one::<Result<(), Failure>>(
                &reply,
                4 * 1024 * 1024
            )
            .unwrap(),
            Ok(())
        );
    }
    let command = Command {
        scope: Scope {
            service: service.into(),
            cashier: operator.into(),
            namespace: 1,
        },
        action: Action::Add(operator.into()),
        fault: false,
    };
    let reply = send(
        &f,
        directory,
        "gateway",
        operator,
        "fixture_gateways",
        ic_blob_storage_pocketic_tests::control_calls::encode_control_one(command).unwrap(),
    )
    .unwrap();
    assert!(
        ic_blob_storage_pocketic_tests::control_calls::decode_control_one::<Result<Outcome, Failure>>(
            &reply,
            4 * 1024 * 1024
        )
        .unwrap()
        .is_ok()
    );
    let empty = f.local_status();
    let first = HistoryPopulationBatch {
        funding_start: 0,

        funding: vec![funding(&f, workload, 0)],
    };
    let denied = send(
        &f,
        directory,
        "wrong-actor",
        f.other,
        "populate_histories",
        ic_blob_storage_pocketic_tests::control_calls::encode_control_one(&first).unwrap(),
    )
    .unwrap();
    assert_eq!(
        ic_blob_storage_pocketic_tests::control_calls::decode_control_one::<Result<(), Failure>>(
            &denied,
            4 * 1024 * 1024
        )
        .unwrap(),
        Err(Failure::Denied)
    );
    assert_eq!(
        populate(
            &f,
            directory,
            "stale-counter",
            &HistoryPopulationBatch {
                funding_start: 1,
                ..first.clone()
            }
        ),
        Err(Failure::Conflict)
    );
    let mut invalid = funding(&f, workload, 1);
    invalid.intent.namespace = 2;
    let failure = send(
        &f,
        directory,
        "funding-rollback",
        operator,
        "populate_histories",
        ic_blob_storage_pocketic_tests::control_calls::encode_control_one(HistoryPopulationBatch {
            funding_start: 0,

            funding: vec![funding(&f, workload, 0), invalid],
        })
        .unwrap(),
    )
    .unwrap_err();
    assert_eq!(failure.reject_code, RejectCode::CanisterError);
    assert_eq!(f.local_status(), empty);
    let population_started = Instant::now();
    for start in (0..workload.funding).step_by(16) {
        let end = (start + 16).min(workload.funding);
        let batch = HistoryPopulationBatch {
            funding_start: u64::from(start),

            funding: (start..end).map(|i| funding(&f, workload, i)).collect(),
        };
        assert_eq!(
            populate(&f, directory, &format!("funding-{start}"), &batch),
            Ok(())
        );
        if end / 1000 != start / 1000 || end == workload.funding {
            retain(
                directory,
                &format!("funding-checkpoint-{end}.json"),
                &serde_json::json!({"local_status": format!("{:?}", f.local_status()), "elapsed_ms": population_started.elapsed().as_millis()}),
            );
        }
    }
    let before = f.local_status();
    assert_eq!(before.funding.retained_intents, u64::from(workload.funding));
    assert_eq!(before.funding.reserved_or_uncertain, 1);
    assert_eq!(before.funding.transport_accepted, 75);
    let funding_before = selected_funding(&f, workload);
    assert_eq!(f.local_status(), before);
    retain(
        directory,
        "before-reopen.json",
        &serde_json::json!({"local_status": format!("{before:?}"), "selected_funding": format!("{funding_before:?}"), "resources": resources(&f), "population_ms": population_started.elapsed().as_millis()}),
    );
    let started = Instant::now();
    let result =
        f.harness
            .pic
            .upgrade_canister(service, Fixture::wasm(), restoration, Some(controller));
    retain(
        directory,
        "upgrade-result.json",
        &serde_json::json!({"result": format!("{result:?}"), "elapsed_ms": started.elapsed().as_millis()}),
    );
    result.unwrap();
    let restored = resources(&f);
    let mut expected = before;
    expected.uploads.fenced = true;
    expected.funding.fenced = true;
    expected.gateways.fenced = true;

    assert_eq!(f.local_status(), expected);
    assert_eq!(selected_funding(&f, workload), funding_before);
    assert_eq!(
        populate(
            &f,
            directory,
            "fenced-population",
            &HistoryPopulationBatch {
                funding_start: u64::from(workload.funding),

                funding: first.funding,
            }
        ),
        Err(Failure::Fenced)
    );
    assert_eq!(f.local_status(), expected);
    assert_eq!(resources(&f), restored);
    retain(
        directory,
        "reopened.json",
        &serde_json::json!({"funding": workload.funding, "tenants": workload.tenants,
        "restored": restored, "whole_service_and_selected_funding_preserved": true,
        "rollback_authority_counter_capacity_and_fence_refusals_preserved": true,
        "live_provider_requests": 0, "paid_cycles": "0"}),
    );
    println!("scaled history {}: {restored}", workload.name);
}
