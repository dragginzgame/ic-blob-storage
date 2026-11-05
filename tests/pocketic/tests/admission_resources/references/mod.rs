//! Many simultaneously live references; cleanup capacity remains reserved.
use super::*;
use blob_test_protocol::admission::release::LifecycleCommand;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityFailure;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityRequest;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityResponse;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceHeadroom;
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;

const REFERENCES: u64 = 256;

fn reference(request: Request, id: u64, retain: bool) -> Command {
    Command::FixtureLifecycle(LifecycleCommand::Reference {
        object: request,
        reference: u128::from(id),
        operation: u128::from(if retain { id - 1 } else { REFERENCES - 1 + id }),
        retain,
    })
}

async fn capacity(f: &Fixture, request: Request) -> ReferenceHeadroom {
    let input = ReferenceCapacityRequest {
        scope: TenantScope {
            service: request.service.into(),
            tenant: request.tenant.into(),
            namespace: request.namespace,
        },
        root: request.root,
    };
    let result: Result<ReferenceCapacityResponse, ReferenceCapacityFailure> = f
        .harness
        .pic
        .query_blob_as(
            f.service,
            f.project,
            "blob_reference_capacity",
            input,
            REPLY_BYTES,
        )
        .await
        .unwrap();
    let response = result.unwrap();
    assert_eq!(response.request, input);
    assert!(!response.fenced);
    response.headroom.unwrap()
}

fn changed(replayed: bool) -> Outcome {
    Outcome::Reference {
        replayed,
        result: Ok(true),
    }
}

async fn retain_all(
    f: &Fixture,
    input: Permission,
    samples: &mut Vec<serde_json::Value>,
) -> (u64, u64) {
    let mut max_retain = 0;
    let mut max_retain_workflow = 0;
    for id in 2..=REFERENCES {
        assert_eq!(
            f.call(f.project, reference(input.request, id, true)).await,
            Ok(changed(false))
        );
        let profile = f.sample(f.project).await;
        max_retain = max_retain.max(profile.after_work);
        max_retain_workflow = max_retain_workflow.max(profile.after_work - profile.before_work);
        assert!(profile.after_work < 10_000_000);
        let remaining = capacity(f, input.request).await;
        assert_eq!(remaining.reference_slots, REFERENCES - id);
        assert_eq!(remaining.fresh_retains, REFERENCES - id);
        assert_eq!(remaining.release_reserved_receipts, id);
        if [2, 16, 64, 128, REFERENCES].contains(&id) {
            samples.push(serde_json::json!({"phase":"retain", "references":id,
                "before_decode":profile.before_decode,
                "after_decode":profile.after_decode,
                "before_work":profile.before_work,"after_work":profile.after_work,
                "wasm_memory_bytes":f.wasm_bytes().await}));
        }
    }
    (max_retain, max_retain_workflow)
}

async fn release_all(
    f: &Fixture,
    input: Permission,
    samples: &mut Vec<serde_json::Value>,
) -> (u64, u64) {
    let mut max_release = 0;
    let mut max_release_workflow = 0;
    for id in 1..=REFERENCES {
        let command = reference(input.request, id, false);
        assert_eq!(f.call(f.project, command.clone()).await, Ok(changed(false)));
        let profile = f.sample(f.project).await;
        max_release = max_release.max(profile.after_work);
        max_release_workflow = max_release_workflow.max(profile.after_work - profile.before_work);
        assert!(profile.after_work < 10_000_000);
        assert_eq!(
            capacity(f, input.request).await.release_reserved_receipts,
            REFERENCES - id
        );
        if [1, 16, 64, 128, REFERENCES].contains(&id) {
            assert_eq!(f.call(f.project, command).await, Ok(changed(true)));
            samples.push(
                serde_json::json!({"phase":"release", "released_references":id,
                "before_decode":profile.before_decode,
                "after_decode":profile.after_decode,
                "before_work":profile.before_work,"after_work":profile.after_work,
                "wasm_memory_bytes":f.wasm_bytes().await}),
            );
        }
    }
    (max_release, max_release_workflow)
}

async fn settle(f: &Fixture, input: Permission) {
    let bytes = u128::from(input.request.bytes);
    let usage = f.observe(input).await.usage.unwrap();
    assert_eq!(
        (usage.logical, usage.physical, usage.liability),
        (0, bytes, bytes)
    );
    assert_eq!(
        f.call(
            f.operator,
            Command::FixtureLifecycle(LifecycleCommand::SubstituteDeletion(input.request))
        )
        .await,
        Ok(Outcome::Changed(true))
    );
    let usage = f.observe(input).await.usage.unwrap();
    assert_eq!(
        (usage.logical, usage.physical, usage.liability),
        (0, 0, bytes)
    );
    assert_eq!(
        f.call(
            f.operator,
            Command::FixtureLifecycle(LifecycleCommand::SubstituteSettlement(input.request))
        )
        .await,
        Ok(Outcome::Changed(true))
    );
    let usage = f.observe(input).await.usage.unwrap();
    assert_eq!((usage.logical, usage.physical, usage.liability), (0, 0, 0));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn resource_reference_history_reserves_every_release_and_keeps_exact_receipts() {
    let f = Fixture::with_workload(Workload::ReferenceHistory).await;
    f.enroll().await;
    let vector = vectors::vector("abc-text", 1);
    let input = f.permission(&vector).await;
    f.admit(input).await;
    f.prepare(input, &vector).await;
    assert_eq!(
        f.call(f.uploader, Command::Expose(input.request.root))
            .await,
        Ok(Outcome::Exposed)
    );
    let initial_heap = f.wasm_bytes().await;
    let mut samples = Vec::new();
    let (max_retain, max_retain_workflow) = retain_all(&f, input, &mut samples).await;
    let full = capacity(&f, input.request).await;
    assert_eq!(
        full,
        ReferenceHeadroom {
            reference_slots: 0,
            unreserved_receipts: 0,
            release_reserved_receipts: REFERENCES,
            fresh_retains: 0
        }
    );
    let original = f.observe(input).await;
    // Fresh retain and unknown release must not consume reserved cleanup slots.
    for retain in [true, false] {
        assert_eq!(
            f.call(
                f.project,
                reference(input.request, 2 * REFERENCES + 1, retain)
            )
            .await,
            Err(Failure::Capacity)
        );
    }
    assert_eq!(capacity(&f, input.request).await, full);
    assert_eq!(f.observe(input).await, original);
    f.restart().await;
    assert_eq!(capacity(&f, input.request).await, full);
    assert_eq!(
        f.call(f.project, reference(input.request, REFERENCES, true))
            .await,
        Ok(changed(true))
    );
    let (max_release, max_release_workflow) = release_all(&f, input, &mut samples).await;
    assert_eq!(
        capacity(&f, input.request).await,
        ReferenceHeadroom {
            reference_slots: 0,
            unreserved_receipts: 0,
            release_reserved_receipts: 0,
            fresh_retains: 0,
        }
    );
    settle(&f, input).await;
    assert_eq!(
        f.call(f.project, reference(input.request, REFERENCES, true))
            .await,
        Ok(changed(true))
    );
    assert_eq!(
        f.call(f.project, reference(input.request, REFERENCES, false))
            .await,
        Ok(changed(true))
    );
    assert_eq!(
        f.call(
            f.project,
            reference(input.request, 2 * REFERENCES + 1, true)
        )
        .await,
        Err(Failure::Capacity)
    );
    let final_heap = f.wasm_bytes().await;
    assert!(final_heap <= initial_heap + 2 * CHUNK as u64);
    let report = serde_json::json!({
        "scope":"One durable object with 256 simultaneous references and 511 receipts. Stop/start, exact retries, saturation and separate deletion/billing cleanup. Operator completion facts are substitutes. Counters exclude diagnostics/replies; allocated memory includes allocator headroom. Stable bookkeeping is included; no upgrade/restore or provider calls.",
        "wasm_sha256":wasm_hash(), "references":REFERENCES,"receipts":2 * REFERENCES - 1,
        "initial_wasm_memory_bytes":initial_heap,"final_wasm_memory_bytes":final_heap,
        "max_retain_instructions":max_retain,"max_release_instructions":max_release,
        "max_retain_workflow_instructions":max_retain_workflow,"max_release_workflow_instructions":max_release_workflow,
        "samples":samples,
    });
    if let Some(path) = std::env::var_os("BLOB_REFERENCE_HISTORY_REPORT") {
        std::fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
    println!("reference history profile: {report}");
}
