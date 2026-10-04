//! Opt-in population/reopen measurements on normal application limits, outside default CI.
use super::*;
use blob_test_protocol::{
    admission::input::ReferenceInput,
    storage::resources::{PopulationBatch, RestorationResources, StorageProbeInstallation},
};
use std::{fs::OpenOptions, io::Write, path::Path, time::Instant};

fn retain(directory: &Path, name: &str, value: &serde_json::Value) {
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(directory.join(name))
        .unwrap();
    output
        .write_all(&serde_json::to_vec_pretty(value).unwrap())
        .unwrap();
    output.write_all(b"\n").unwrap();
    output.sync_all().unwrap();
}

fn resources(f: &Fixture) -> serde_json::Value {
    let result: Result<RestorationResources, Failure> = f
        .harness
        .pic
        .query_candid_as(f.service, f.operator, "restoration_resources", ())
        .unwrap();
    let r = result.unwrap();
    let status = f
        .harness
        .pic
        .canister_status(f.service, Some(f.controller))
        .unwrap();
    serde_json::json!({"restored":r.restored, "initialization_instructions":r.initialization_instructions,
        "stores_instructions":r.stores_instructions, "initialization_heap_bytes":r.heap_bytes,
        "initialization_stable_bytes":r.stable_bytes,
        "reads":r.reads.iter().map(|row| serde_json::json!({"memory":row.memory,"calls":row.calls,"bytes":row.bytes,"instructions":row.instructions})).collect::<Vec<_>>(),
        "current_heap_bytes":status.memory_metrics.wasm_memory_size.0.to_string(),
        "current_stable_bytes":status.memory_metrics.stable_memory_size.0.to_string()})
}

#[test]
#[ignore = "opt-in population profile; needs a fresh BLOB_RESTORE_RESOURCE_REPORT directory"]
#[expect(
    clippy::too_many_lines,
    reason = "One retained workload follows population, actual upgrade, accounting and fence checks"
)]
fn populated_service_restoration_profile() {
    let directory =
        std::path::PathBuf::from(std::env::var_os("BLOB_RESTORE_RESOURCE_REPORT").unwrap());
    // Caller selects a new directory: never overwrite an earlier failure or profile.
    std::fs::create_dir(&directory).unwrap();
    let mut failures = Vec::new();
    for count in [100_u32, 1_000, 10_000] {
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
            max_objects: count,
        })
        .unwrap();
        harness.pic.install_canister(
            service,
            Fixture::wasm(),
            installation.clone(),
            Some(controller),
        );
        let mut f = Fixture {
            harness,
            service,
            operator,
            controller,
            tenant: Principal::anonymous(),
            uploader: Fake::principal(4),
            other: Fake::principal(5),
        };
        let initial = resources(&f);
        let started = Instant::now();
        let mut samples = Vec::new();
        for start in (0..count).step_by(100) {
            let batch = PopulationBatch { start, count: 100 };
            let result = f
                .harness
                .pic
                .update_candid_as::<Result<Vec<Request>, Failure>, _>(
                    service,
                    operator,
                    "populate_resources",
                    (batch,),
                );
            let Ok(Ok(rows)) = result else {
                retain(
                    &directory,
                    &format!("population-failure-{count}-{start}.json"),
                    &serde_json::json!({"operations_before":start,"result":format!("{result:?}")}),
                );
                panic!("population failed; retained original result");
            };
            if samples.is_empty() {
                samples = rows;
                f.tenant = samples[0].tenant;
            }
            if (start + 100) % 1000 == 0 || start + 100 == count {
                retain(
                    &directory,
                    &format!("checkpoint-{count}-{}.json", start + 100),
                    &serde_json::json!({"operations":f.status().operations,"population_ms":started.elapsed().as_millis(),"resources":resources(&f)}),
                );
            }
        }
        let before = f.status();
        assert_eq!(before.operations, u64::from(count));
        assert_eq!(before.active, u64::from(count / 4));
        assert_eq!(before.bytes, u128::from(count / 4) * 10);
        assert_eq!(before.usage.logical, u128::from(count / 2) * 10);
        assert_eq!(before.usage.physical, u128::from(count / 4 * 3) * 10);
        assert_eq!(before.usage.liability, before.usage.physical);
        let observations: Vec<_> = samples
            .iter()
            .map(|r| f.lookup(f.tenant, *r).unwrap())
            .collect();
        assert_eq!(
            observations.iter().map(|r| r.phase).collect::<Vec<_>>(),
            vec![
                Phase::ExposurePossible,
                Phase::Confirmed,
                Phase::Confirmed,
                Phase::Cancelled
            ]
        );
        let retained = ReferenceInput {
            object: samples[1],
            reference: 2,
            operation: 1,
            retain: true,
        };
        let released = ReferenceInput {
            object: samples[2],
            reference: 1,
            operation: 1,
            retain: false,
        };
        assert!(f.live(retained).unwrap());
        assert!(!f.live(released).unwrap());
        let receipts = [f.receipt(retained).unwrap(), f.receipt(released).unwrap()];
        let populated = resources(&f);
        retain(
            &directory,
            &format!("before-reopen-{count}.json"),
            &serde_json::json!({
            "count":count,"initial":initial,"populated":populated,"status":format!("{before:?}"),
            "observations":format!("{observations:?}"),"receipts":format!("{receipts:?}"),
            "population_ms":started.elapsed().as_millis()}),
        );
        let restore_started = Instant::now();
        let result = f.harness.pic.upgrade_canister(
            service,
            Fixture::wasm(),
            installation,
            Some(controller),
        );
        if let Err(error) = result {
            retain(
                &directory,
                &format!("reopen-failure-{count}.json"),
                &serde_json::json!({
                "count":count,"error":format!("{error:?}"),"elapsed_ms":restore_started.elapsed().as_millis()}),
            );
            failures.push(count);
            continue;
        }
        let restored = resources(&f);
        assert_eq!(
            f.status(),
            Status {
                fenced: true,
                ..before
            }
        );
        for (request, observed) in samples.iter().zip(&observations) {
            assert_eq!(f.lookup(f.tenant, *request), Ok(*observed));
        }
        assert!(f.live(retained).unwrap());
        assert!(!f.live(released).unwrap());
        assert_eq!(
            [f.receipt(retained).unwrap(), f.receipt(released).unwrap()],
            receipts
        );
        let refused: Result<Vec<Request>, Failure> = f
            .harness
            .pic
            .update_candid_as(
                service,
                operator,
                "populate_resources",
                (PopulationBatch {
                    start: count,
                    count: 1,
                },),
            )
            .unwrap();
        assert_eq!(refused, Err(Failure::Fenced));
        assert_eq!(
            f.status(),
            Status {
                fenced: true,
                ..before
            }
        );
        assert_eq!(resources(&f), restored);
        retain(
            &directory,
            &format!("reopened-{count}.json"),
            &serde_json::json!({
            "count":count,"restored":restored,"elapsed_ms":restore_started.elapsed().as_millis(),
            "accounting_observations_references_receipts_preserved":true,"fenced_mutation_refused":true}),
        );
        println!("restoration profile {count}: {restored}");
    }
    retain(
        &directory,
        "summary.json",
        &serde_json::json!({"failed_restore_tiers":failures,
        "provider_requests":0,"paid_cycles":"0","subnet":"normal application",
        "limitations":["one tenant, ten-byte single-leaf objects", "completion is a local fact substitute", "allocated linear memory is not live heap", "same-release synchronous reopen; no freshness resume"]}),
    );
    assert!(failures.is_empty(), "failed restoration tiers retained");
}
