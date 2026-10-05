//! Opt-in larger lifetime funding histories and workflow-admitted read occupancy.
use super::*;
use blob_test_protocol::{
    admission::{ContentLookup, input::RetainedDescriptorInput},
    storage::{
        funding::{Intent, Phase as FundingPhase, Request as FundingRequest},
        gateways::{Action, Command, Outcome, ReadSessionInput, Scope},
        resources::{
            FundingPopulationIntent, HistoryPopulationBatch, RestorationRead, RestorationReadPage,
        },
    },
};
use ic_testkit::pocket_ic::RejectResponse;

#[derive(Clone, Copy)]
struct HistoryWorkload {
    name: &'static str,
    funding: u32,
    reads: u32,
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
    retain_bytes(directory, &format!("{name}.candid"), &packet);
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
        candid::encode_one(batch).unwrap(),
    )
    .unwrap();
    candid::decode_one(&reply).unwrap()
}

fn funding(f: &Fixture, workload: HistoryWorkload, index: u32) -> FundingPopulationIntent {
    FundingPopulationIntent {
        intent: Intent {
            service: f.service,
            cashier: f.operator,
            account: f.service,
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

fn read_target(f: &Fixture, workload: HistoryWorkload, tenant: u32) -> ReadSessionInput {
    let p = preparation(
        f,
        Workload {
            name: workload.name,
            objects: workload.tenants * 4,
            tenants: workload.tenants,
            bytes: 10,
        },
        tenant * 4 + 1,
    );
    ReadSessionInput {
        index: 0,
        admit_fault: None,
        callback_fault: None,
        cashier: f.operator,
        gateway: f.operator,
        target: RetainedDescriptorInput {
            content: ContentLookup {
                service: f.service,
                tenant: p.request.tenant,
                namespace: 1,
                root: p.request.root,
            },
            object: p.request.id,
            incarnation: 1,
            reference: 1,
        },
    }
}

fn read_page(
    f: &Fixture,
    actor: Principal,
    page: RestorationReadPage,
) -> Result<Vec<RestorationRead>, Failure> {
    f.harness
        .pic
        .query_candid_as(f.service, actor, "restoration_read_page", (page,))
        .unwrap()
}

fn occupied_reads(f: &Fixture) -> Vec<RestorationRead> {
    let mut reads = Vec::new();
    let mut after = 0;
    loop {
        let page = read_page(f, f.operator, RestorationReadPage { after, limit: 32 }).unwrap();
        let Some(last) = page.last() else {
            break;
        };
        assert!(last.sequence > after);
        after = last.sequence;
        reads.extend(page);
    }
    reads
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
                .query_candid_as(f.service, f.operator, "funding_lookup", (input.intent,))
                .unwrap();
            assert_eq!(phase, Ok(Some(input.phase)));
            let request: Result<FundingRequest, Failure> = f
                .harness
                .pic
                .query_candid_as(f.service, f.operator, "funding_request", (input.intent,))
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
            reads: 0,
            tenants: 1,
            final_phase: FundingPhase::Prepared,
        },
        HistoryWorkload {
            name: "funding-1000",
            funding: 1000,
            reads: 0,
            tenants: 1,
            final_phase: FundingPhase::Uncertain,
        },
        HistoryWorkload {
            name: "funding-10000",
            funding: 10_000,
            reads: 0,
            tenants: 1,
            final_phase: FundingPhase::Uncertain,
        },
        HistoryWorkload {
            name: "reads-32",
            funding: 4,
            reads: 32,
            tenants: 1,
            final_phase: FundingPhase::Uncertain,
        },
        HistoryWorkload {
            name: "reads-1024-tenants-32",
            funding: 1000,
            reads: 1024,
            tenants: 32,
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
        &serde_json::json!({"workloads_passed": 5,
        "live_provider_requests": 0, "paid_cycles": "0",
        "limitations": ["synthetic funding outcomes and workflow-admitted undispatched reads", "instrumented observer overhead", "no production, million-object or macOS qualification"]}),
    );
}

#[expect(
    clippy::too_many_lines,
    reason = "One retained local workload binds population, rollback/refusals, full read inspection and actual upgrade"
)]
fn profile_workload(directory: &Path, workload: HistoryWorkload) {
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
        max_objects: workload.tenants * 4,
        max_tenants: workload.tenants,
        max_object_bytes: 10,
        max_funding_attempts: workload.funding,
        max_read_sessions: workload.reads.max(1),
        max_tenant_read_sessions: (workload.reads / workload.tenants).max(1),
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
            candid::encode_one(batch).unwrap(),
        )
        .unwrap();
        assert_eq!(
            candid::decode_one::<Result<(), Failure>>(&reply).unwrap(),
            Ok(())
        );
    }
    let command = Command {
        scope: Scope {
            service,
            cashier: operator,
            namespace: 1,
        },
        action: Action::Add(operator),
        fault: false,
    };
    let reply = send(
        &f,
        directory,
        "gateway",
        operator,
        "fixture_gateways",
        candid::encode_one(command).unwrap(),
    )
    .unwrap();
    assert!(
        candid::decode_one::<Result<Outcome, Failure>>(&reply)
            .unwrap()
            .is_ok()
    );
    let empty = f.local_status();
    let first = HistoryPopulationBatch {
        funding_start: 0,
        read_start: 0,
        funding: vec![funding(&f, workload, 0)],
        reads: vec![],
    };
    let denied = send(
        &f,
        directory,
        "wrong-actor",
        f.other,
        "populate_histories",
        candid::encode_one(&first).unwrap(),
    )
    .unwrap();
    assert_eq!(
        candid::decode_one::<Result<(), Failure>>(&denied).unwrap(),
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
        candid::encode_one(HistoryPopulationBatch {
            funding_start: 0,
            read_start: 0,
            funding: vec![funding(&f, workload, 0), invalid],
            reads: vec![],
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
            read_start: 0,
            funding: (start..end).map(|i| funding(&f, workload, i)).collect(),
            reads: vec![],
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
    if workload.reads != 0 {
        let valid = read_target(&f, workload, 0);
        let mut invalid = valid;
        invalid.target.incarnation = 2;
        let before = f.local_status();
        let failure = send(
            &f,
            directory,
            "read-rollback",
            operator,
            "populate_histories",
            candid::encode_one(HistoryPopulationBatch {
                funding_start: u64::from(workload.funding),
                read_start: 0,
                funding: vec![],
                reads: vec![valid, invalid],
            })
            .unwrap(),
        )
        .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
        assert_eq!(f.local_status(), before);
        assert_eq!(occupied_reads(&f), []);
    }
    for start in (0..workload.reads).step_by(16) {
        let end = (start + 16).min(workload.reads);
        assert_eq!(
            populate(
                &f,
                directory,
                &format!("reads-{start}"),
                &HistoryPopulationBatch {
                    funding_start: u64::from(workload.funding),
                    read_start: u64::from(start),
                    funding: vec![],
                    reads: (start..end)
                        .map(|i| read_target(&f, workload, i / (workload.reads / workload.tenants)))
                        .collect(),
                }
            ),
            Ok(())
        );
        if workload.tenants > 1 && end == workload.reads / workload.tenants {
            // Tenant capacity refuses while global capacity still has room.
            let target = read_target(&f, workload, 0);
            let before = f.local_status();
            let reply = send(
                &f,
                directory,
                "tenant-capacity-refusal",
                target.target.content.tenant,
                "fixture_read_chunk",
                candid::encode_one(target).unwrap(),
            )
            .unwrap();
            assert_eq!(
                candid::decode_one::<
                    Result<blob_test_protocol::journey::readback::JourneyReadChunk, Failure>,
                >(&reply)
                .unwrap(),
                Err(Failure::Capacity)
            );
            assert_eq!(f.local_status(), before);
        }
    }
    let before = f.local_status();
    assert_eq!(before.funding.retained_intents, u64::from(workload.funding));
    assert_eq!(before.funding.reserved_or_uncertain, 1);
    assert_eq!(before.funding.transport_accepted, 75);
    assert_eq!(before.reads.sessions, workload.reads);
    assert_eq!(
        before.reads.reserved_bytes,
        u64::from(workload.reads) * 2048
    );
    assert_eq!(before.reads.last_sequence, u64::from(workload.reads));
    let reads = occupied_reads(&f);
    for (index, read) in reads.iter().enumerate() {
        assert_eq!(read.sequence, u64::try_from(index + 1).unwrap());
        assert_eq!(
            read.target,
            read_target(
                &f,
                workload,
                u32::try_from(index).unwrap() / (workload.reads / workload.tenants)
            )
        );
    }
    assert_eq!(reads.len(), workload.reads as usize);
    let page = RestorationReadPage {
        after: 0,
        limit: 32,
    };
    assert_eq!(read_page(&f, f.other, page), Err(Failure::Denied));
    assert_eq!(
        read_page(&f, operator, RestorationReadPage { limit: 65, ..page }),
        Err(Failure::Invalid)
    );
    if workload.reads != 0 {
        let target = read_target(&f, workload, 0);
        let reply = send(
            &f,
            directory,
            "capacity-refusal",
            target.target.content.tenant,
            "fixture_read_chunk",
            candid::encode_one(target).unwrap(),
        )
        .unwrap();
        assert_eq!(
            candid::decode_one::<
                Result<blob_test_protocol::journey::readback::JourneyReadChunk, Failure>,
            >(&reply)
            .unwrap(),
            Err(Failure::Capacity)
        );
    }
    let funding_before = selected_funding(&f, workload);
    assert_eq!(f.local_status(), before);
    retain(
        directory,
        "before-reopen.json",
        &serde_json::json!({"local_status": format!("{before:?}"), "selected_funding": format!("{funding_before:?}"), "all_reads": format!("{reads:?}"), "resources": resources(&f), "population_ms": population_started.elapsed().as_millis()}),
    );
    let started = Instant::now();
    let result =
        f.harness
            .pic
            .upgrade_canister(service, Fixture::wasm(), installation, Some(controller));
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
    expected.reads.fenced = true;
    assert_eq!(f.local_status(), expected);
    assert_eq!(occupied_reads(&f), reads);
    assert_eq!(selected_funding(&f, workload), funding_before);
    assert_eq!(
        populate(
            &f,
            directory,
            "fenced-population",
            &HistoryPopulationBatch {
                funding_start: u64::from(workload.funding),
                read_start: u64::from(workload.reads),
                funding: first.funding,
                reads: vec![],
            }
        ),
        Err(Failure::Fenced)
    );
    assert_eq!(f.local_status(), expected);
    assert_eq!(resources(&f), restored);
    retain(
        directory,
        "reopened.json",
        &serde_json::json!({"funding": workload.funding, "reads": workload.reads, "tenants": workload.tenants,
        "restored": restored, "whole_service_selected_funding_and_every_read_preserved": true,
        "rollback_authority_counter_capacity_and_fence_refusals_preserved": true,
        "live_provider_requests": 0, "paid_cycles": "0"}),
    );
    println!("scaled history {}: {restored}", workload.name);
}
