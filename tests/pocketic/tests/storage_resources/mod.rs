//! Opt-in population/reopen measurements on normal application limits, outside default CI.
mod credits;
mod histories;
mod reads;
use super::*;
use blob_test_protocol::{
    admission::input::ReferenceInput,
    storage::resources::{
        PopulationBatch, RestorationResources, STORAGE_PROBE_INPUT_BYTES, StorageProbeInstallation,
    },
};
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestInspection;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestResponse;
use ic_blob_storage_contracts::identity::caffeine::CAFFEINE_CHUNK_BYTES;
use std::{fs::OpenOptions, io::Write, path::Path, time::Instant};

pub(super) fn retain(directory: &Path, name: &str, value: &serde_json::Value) {
    let mut bytes = serde_json::to_vec_pretty(value).unwrap();
    bytes.push(b'\n');
    retain_bytes(directory, name, &bytes);
}

pub(super) fn retain_bytes(directory: &Path, name: &str, bytes: &[u8]) {
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(directory.join(name))
        .unwrap();
    output.write_all(bytes).unwrap();
    output.sync_all().unwrap();
}

pub(super) fn resources(f: &Fixture) -> serde_json::Value {
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

#[derive(Clone, Copy)]
struct Workload {
    name: &'static str,
    objects: u32,
    tenants: u32,
    bytes: u64,
}

fn preparation(f: &Fixture, workload: Workload, index: u32) -> PreparationInput {
    let length = workload.bytes.to_string();
    let headers = [
        CaffeineHeader {
            name: "Content-Length",
            value: &length,
        },
        CaffeineHeader {
            name: "Content-Type",
            value: "application/octet-stream",
        },
    ];
    let mut builder = CaffeineManifestBuilder::new(
        workload.bytes,
        &headers,
        CaffeineHashLimits {
            max_content_bytes: NonZeroU64::new(workload.bytes).unwrap(),
            max_append_bytes: NonZeroUsize::new(CAFFEINE_CHUNK_BYTES).unwrap(),
            max_headers: NonZeroUsize::new(8).unwrap(),
            max_header_bytes: NonZeroUsize::new(1024).unwrap(),
        },
        NonZeroUsize::new(
            usize::try_from(workload.bytes.div_ceil(CAFFEINE_CHUNK_BYTES as u64)).unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    // One bounded off-canister frame; every object and leaf has distinct bytes.
    let mut frame =
        vec![0; usize::try_from(workload.bytes.min(CAFFEINE_CHUNK_BYTES as u64)).unwrap()];
    frame[..4].copy_from_slice(&index.to_le_bytes());
    while builder.remaining_bytes() != 0 {
        let offset = builder.received_bytes();
        frame[4..8].copy_from_slice(
            &u32::try_from(offset / CAFFEINE_CHUNK_BYTES as u64)
                .unwrap()
                .to_le_bytes(),
        );
        let bytes =
            usize::try_from(builder.remaining_bytes().min(CAFFEINE_CHUNK_BYTES as u64)).unwrap();
        builder.append(offset, &frame[..bytes]).unwrap();
    }
    let built = builder.finish().unwrap();
    let tenant = Principal::from_slice(&[
        43,
        u8::try_from((index / 4) % workload.tenants).unwrap() + 1,
    ]);
    PreparationInput {
        request: Request {
            service: f.service,
            tenant,
            namespace: 1,
            // IDs intentionally overlap across tenants; roots remain distinct.
            id: u128::from((index / (4 * workload.tenants)) * 4 + index % 4 + 1),
            root: *built.hashes().provider_root.as_bytes(),
            bytes: workload.bytes,
        },
        manifest: JourneyManifest {
            chunks: built
                .manifest()
                .chunks()
                .iter()
                .map(|h| *h.as_bytes())
                .collect(),
            headers: headers
                .iter()
                .map(|h| (h.name.to_owned(), h.value.to_owned()))
                .collect(),
        },
    }
}

fn manifest(f: &Fixture, input: &PreparationInput) -> UploadManifestResponse {
    let permission = admission_input(Permission {
        request: input.request,
        uploader: input.request.tenant,
        expires_at_ns: u64::MAX,
    });
    let result: Result<UploadManifestResponse, UploadManifestFailure> = f
        .harness
        .pic
        .query_candid_as(
            f.service,
            input.request.tenant,
            "blob_upload_manifest",
            (permission,),
        )
        .unwrap();
    result.unwrap()
}

fn references(samples: &[PreparationInput]) -> Vec<ReferenceInput> {
    samples
        .as_chunks::<4>()
        .0
        .iter()
        .flat_map(|group| {
            [
                ReferenceInput {
                    object: group[1].request,
                    reference: 2,
                    operation: 1,
                    retain: true,
                },
                ReferenceInput {
                    object: group[2].request,
                    reference: 1,
                    operation: 1,
                    retain: false,
                },
            ]
        })
        .collect()
}

#[test]
#[ignore = "opt-in population profile; needs a fresh BLOB_RESTORE_RESOURCE_REPORT directory"]
fn populated_service_restoration_profile() {
    let directory =
        std::path::PathBuf::from(std::env::var_os("BLOB_RESTORE_RESOURCE_REPORT").unwrap());
    std::fs::create_dir(&directory).unwrap();
    let workloads = [
        Workload {
            name: "single-100",
            objects: 100,
            tenants: 1,
            bytes: 10,
        },
        Workload {
            name: "single-1000",
            objects: 1_000,
            tenants: 1,
            bytes: 10,
        },
        Workload {
            name: "single-10000",
            objects: 10_000,
            tenants: 1,
            bytes: 10,
        },
        Workload {
            name: "tenants-32",
            objects: 1_024,
            tenants: 32,
            bytes: 10,
        },
        Workload {
            name: "manifests-16mib",
            objects: 32,
            tenants: 4,
            bytes: 16 * 1_048_576,
        },
        Workload {
            name: "manifests-64mib",
            objects: 4,
            tenants: 1,
            bytes: 64 * 1_048_576,
        },
    ];
    let mut failures = Vec::new();
    for workload in workloads {
        if !profile_workload(&directory, workload) {
            failures.push(workload.name);
        }
    }
    retain(
        &directory,
        "summary.json",
        &serde_json::json!({"failed_restore_workloads":failures,
        "provider_requests":0,"paid_cycles":"0","subnet":"normal application",
        "limitations":["synthetic client-prepared manifests and local completion facts", "instrumented reads include observer overhead", "allocated linear memory is not live heap", "same-release synchronous reopen; no freshness resume", "no million-object or populated funding/read-history qualification"]}),
    );
    assert!(failures.is_empty(), "failed restoration workloads retained");
}

#[expect(
    clippy::too_many_lines,
    reason = "One retained workload couples population, actual reopen and boundary evidence"
)]
fn profile_workload(directory: &Path, workload: Workload) -> bool {
    let count = workload.objects;
    let name = workload.name;
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
        max_tenants: workload.tenants,
        max_object_bytes: workload.bytes,
        max_funding_attempts: 4,
        max_read_sessions: 1,
        max_tenant_read_sessions: 1,
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
    let batch_objects = if workload.bytes <= CAFFEINE_CHUNK_BYTES as u64 {
        16
    } else {
        1
    };
    for start in (0..count).step_by(usize::try_from(batch_objects).unwrap()) {
        let uploads: Vec<_> = (start..(start + batch_objects).min(count))
            .map(|i| preparation(&f, workload, i))
            .collect();
        samples.extend(uploads.iter().filter(|p| p.request.id <= 4).cloned());
        let end = start + u32::try_from(uploads.len()).unwrap();
        let batch = PopulationBatch { start, uploads };
        let packet = candid::encode_one(&batch).unwrap();
        retain_bytes(
            directory,
            &format!("request-{name}-{start}.candid"),
            &packet,
        );
        assert!(packet.len() <= STORAGE_PROBE_INPUT_BYTES);
        let result = f.harness.pic.update_candid_as::<Result<(), Failure>, _>(
            service,
            operator,
            "populate_resources",
            (batch,),
        );
        if !matches!(result, Ok(Ok(()))) {
            retain(
                directory,
                &format!("population-failure-{name}-{start}.json"),
                &serde_json::json!({"operations_before":start,"result":format!("{result:?}")}),
            );
            panic!("population failed; retained original result");
        }
        if end / 1000 != start / 1000 || end == count {
            retain(
                directory,
                &format!("checkpoint-{name}-{end}.json"),
                &serde_json::json!({"operations":f.status().operations,"population_ms":started.elapsed().as_millis(),"resources":resources(&f)}),
            );
        }
    }
    assert_eq!(
        samples.len(),
        usize::try_from(workload.tenants * 4).unwrap()
    );
    let before = f.status();
    assert_eq!(before.operations, u64::from(count));
    assert_eq!(before.active, u64::from(count / 4));
    assert_eq!(
        before.bytes,
        u128::from(count / 4) * u128::from(workload.bytes)
    );
    assert_eq!(
        before.usage.logical,
        u128::from(count / 2) * u128::from(workload.bytes)
    );
    assert_eq!(
        before.usage.physical,
        u128::from(count / 4 * 3) * u128::from(workload.bytes)
    );
    assert_eq!(before.usage.liability, before.usage.physical);
    let observations: Vec<_> = samples
        .iter()
        .map(|p| f.lookup(p.request.tenant, p.request).unwrap())
        .collect();
    for group in observations.as_chunks::<4>().0 {
        assert_eq!(
            group.iter().map(|r| r.phase).collect::<Vec<_>>(),
            vec![
                Phase::ExposurePossible,
                Phase::Confirmed,
                Phase::Confirmed,
                Phase::Cancelled
            ]
        );
    }
    let manifests: Vec<_> = samples.iter().map(|p| manifest(&f, p)).collect();
    for (index, (input, observed)) in samples.iter().zip(&manifests).enumerate() {
        if index % 4 == 3 {
            assert_eq!(observed.manifest, UploadManifestInspection::Unprepared);
        } else {
            let UploadManifestInspection::Prepared(declaration) = &observed.manifest else {
                panic!("missing exact manifest");
            };
            assert_eq!(declaration.chunks, input.manifest.chunks);
            assert_eq!(
                declaration
                    .headers
                    .iter()
                    .map(|h| (h.name.clone(), h.value.clone()))
                    .collect::<Vec<_>>(),
                input.manifest.headers
            );
        }
    }
    let references = references(&samples);
    let receipts: Vec<_> = references
        .iter()
        .map(|r| {
            f.tenant = r.object.tenant;
            assert_eq!(f.live(*r).unwrap(), r.retain);
            f.receipt(*r).unwrap()
        })
        .collect();
    retain(
        directory,
        &format!("before-reopen-{name}.json"),
        &serde_json::json!({
        "count":count,"tenants":workload.tenants,"object_bytes":workload.bytes,"initial":initial,
        "populated":resources(&f),"status":format!("{before:?}"),"observations":format!("{observations:?}"),
        "manifests":format!("{manifests:?}"),"receipts":format!("{receipts:?}"),"population_ms":started.elapsed().as_millis()}),
    );
    let restore_started = Instant::now();
    if let Err(error) =
        f.harness
            .pic
            .upgrade_canister(service, Fixture::wasm(), installation, Some(controller))
    {
        retain(
            directory,
            &format!("reopen-failure-{name}.json"),
            &serde_json::json!({
            "count":count,"error":format!("{error:?}"),"elapsed_ms":restore_started.elapsed().as_millis()}),
        );
        return false;
    }
    let restored = resources(&f);
    assert_eq!(
        f.status(),
        Status {
            fenced: true,
            ..before
        }
    );
    for ((input, observed), declared) in samples.iter().zip(&observations).zip(&manifests) {
        assert_eq!(f.lookup(input.request.tenant, input.request), Ok(*observed));
        assert_eq!(manifest(&f, input), *declared);
        assert_eq!(f.lookup(f.other, input.request), Err(Failure::Denied));
    }
    for (reference, receipt) in references.iter().zip(&receipts) {
        f.tenant = reference.object.tenant;
        assert_eq!(f.live(*reference).unwrap(), reference.retain);
        assert_eq!(f.receipt(*reference).unwrap(), *receipt);
    }
    let refused: Result<(), Failure> = f
        .harness
        .pic
        .update_candid_as(
            service,
            operator,
            "populate_resources",
            (PopulationBatch {
                start: count,
                uploads: vec![samples[0].clone()],
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
        directory,
        &format!("reopened-{name}.json"),
        &serde_json::json!({
        "count":count,"tenants":workload.tenants,"object_bytes":workload.bytes,"restored":restored,
        "elapsed_ms":restore_started.elapsed().as_millis(),"accounting_observations_manifests_references_receipts_preserved":true,
        "fenced_mutation_refused":true,"foreign_tenant_observation_refused":true}),
    );
    println!("restoration profile {name}: {restored}");
    true
}
