//! Many simultaneously live references; cleanup capacity remains reserved.
use super::*;
use blob_test_protocol::admission::{
    ContentLookup,
    release::{LifecycleCommand, ReferenceCapacity},
};

const REFERENCES: u64 = 256;

fn reference(request: Request, id: u64, retain: bool) -> Command {
    Command::FixtureLifecycle(LifecycleCommand::Reference {
        object: request,
        reference: u128::from(id),
        operation: u128::from(if retain { id - 1 } else { REFERENCES - 1 + id }),
        retain,
    })
}

fn capacity(f: &Fixture, request: Request) -> ReferenceCapacity {
    let result: Result<Option<ReferenceCapacity>, Failure> = f
        .harness
        .pic
        .query_candid_as(
            f.service,
            f.project,
            "reference_capacity",
            (ContentLookup {
                service: request.service,
                tenant: request.tenant,
                namespace: request.namespace,
                root: request.root,
            },),
        )
        .unwrap();
    result.unwrap().unwrap()
}

fn changed(replayed: bool) -> Outcome {
    Outcome::Reference {
        replayed,
        result: Ok(true),
    }
}

fn retain_all(f: &Fixture, input: Permission, samples: &mut Vec<serde_json::Value>) -> (u64, u64) {
    let mut max_retain = 0;
    let mut max_retain_workflow = 0;
    for id in 2..=REFERENCES {
        assert_eq!(
            f.call(f.project, reference(input.request, id, true)),
            Ok(changed(false))
        );
        let profile = f.sample(f.project);
        max_retain = max_retain.max(profile.after_work);
        max_retain_workflow = max_retain_workflow.max(profile.after_work - profile.before_work);
        assert!(profile.after_work < 10_000_000);
        let remaining = capacity(f, input.request);
        assert_eq!(remaining.reference_slots, REFERENCES - id);
        assert_eq!(remaining.fresh_retains, REFERENCES - id);
        assert_eq!(remaining.release_reserved_receipts, id);
        if [2, 16, 64, 128, REFERENCES].contains(&id) {
            samples.push(serde_json::json!({"phase":"retain", "references":id,
                "before_decode":profile.before_decode,"after_header":profile.after_header,
                "after_value":profile.after_value,"after_decode":profile.after_decode,
                "before_work":profile.before_work,"after_work":profile.after_work,
                "wasm_memory_bytes":f.wasm_bytes()}));
        }
    }
    (max_retain, max_retain_workflow)
}

fn release_all(f: &Fixture, input: Permission, samples: &mut Vec<serde_json::Value>) -> (u64, u64) {
    let mut max_release = 0;
    let mut max_release_workflow = 0;
    for id in 1..=REFERENCES {
        let command = reference(input.request, id, false);
        assert_eq!(f.call(f.project, command.clone()), Ok(changed(false)));
        let profile = f.sample(f.project);
        max_release = max_release.max(profile.after_work);
        max_release_workflow = max_release_workflow.max(profile.after_work - profile.before_work);
        assert!(profile.after_work < 10_000_000);
        assert_eq!(
            capacity(f, input.request).release_reserved_receipts,
            REFERENCES - id
        );
        if [1, 16, 64, 128, REFERENCES].contains(&id) {
            assert_eq!(f.call(f.project, command), Ok(changed(true)));
            samples.push(
                serde_json::json!({"phase":"release", "released_references":id,
                "before_decode":profile.before_decode,"after_header":profile.after_header,
                "after_value":profile.after_value,"after_decode":profile.after_decode,
                "before_work":profile.before_work,"after_work":profile.after_work,
                "wasm_memory_bytes":f.wasm_bytes()}),
            );
        }
    }
    (max_release, max_release_workflow)
}

fn settle(f: &Fixture, input: Permission) {
    let bytes = u128::from(input.request.bytes);
    let usage = f.observe(input).usage.unwrap();
    assert_eq!(
        (usage.logical, usage.physical, usage.liability),
        (0, bytes, bytes)
    );
    assert_eq!(
        f.call(
            f.operator,
            Command::FixtureLifecycle(LifecycleCommand::SubstituteDeletion(input.request))
        ),
        Ok(Outcome::Changed(true))
    );
    let usage = f.observe(input).usage.unwrap();
    assert_eq!(
        (usage.logical, usage.physical, usage.liability),
        (0, 0, bytes)
    );
    assert_eq!(
        f.call(
            f.operator,
            Command::FixtureLifecycle(LifecycleCommand::SubstituteSettlement(input.request))
        ),
        Ok(Outcome::Changed(true))
    );
    let usage = f.observe(input).usage.unwrap();
    assert_eq!((usage.logical, usage.physical, usage.liability), (0, 0, 0));
}

#[test]
fn resource_reference_history_reserves_every_release_and_keeps_exact_receipts() {
    let f = Fixture::with_workload(Workload::ReferenceHistory);
    f.enroll();
    let vector = vectors::vector("abc-text", 1);
    let input = f.permission(&vector);
    f.admit(input);
    f.prepare(input, &vector);
    assert_eq!(
        f.call(f.uploader, Command::Expose(input.request.root)),
        Ok(Outcome::Exposed)
    );
    assert_eq!(
        f.call(
            f.operator,
            Command::FixtureLifecycle(LifecycleCommand::SubstituteCompletion(input.request))
        ),
        Ok(Outcome::Changed(true))
    );
    let initial_heap = f.wasm_bytes();
    let mut samples = Vec::new();
    let (max_retain, max_retain_workflow) = retain_all(&f, input, &mut samples);
    let full = capacity(&f, input.request);
    assert_eq!(
        full,
        ReferenceCapacity {
            reference_slots: 0,
            unreserved_receipts: 0,
            release_reserved_receipts: REFERENCES,
            fresh_retains: 0
        }
    );
    let original = f.observe(input);
    // Fresh retain and unknown release must not consume reserved cleanup slots.
    for retain in [true, false] {
        assert_eq!(
            f.call(
                f.project,
                reference(input.request, 2 * REFERENCES + 1, retain)
            ),
            Err(Failure::Capacity)
        );
    }
    assert_eq!(capacity(&f, input.request), full);
    assert_eq!(f.observe(input), original);
    f.restart();
    assert_eq!(capacity(&f, input.request), full);
    assert_eq!(
        f.call(f.project, reference(input.request, REFERENCES, true)),
        Ok(changed(true))
    );
    let (max_release, max_release_workflow) = release_all(&f, input, &mut samples);
    assert_eq!(
        capacity(&f, input.request),
        ReferenceCapacity {
            reference_slots: 0,
            unreserved_receipts: 0,
            release_reserved_receipts: 0,
            fresh_retains: 0,
        }
    );
    settle(&f, input);
    assert_eq!(
        f.call(f.project, reference(input.request, REFERENCES, true)),
        Ok(changed(true))
    );
    assert_eq!(
        f.call(f.project, reference(input.request, REFERENCES, false)),
        Ok(changed(true))
    );
    assert_eq!(
        f.call(
            f.project,
            reference(input.request, 2 * REFERENCES + 1, true)
        ),
        Err(Failure::Capacity)
    );
    let final_heap = f.wasm_bytes();
    assert!(final_heap <= initial_heap + 2 * CHUNK as u64);
    let report = serde_json::json!({
        "scope":"One transient object with 256 simultaneous references and 511 receipts. Stop/start, exact retries, saturation and separate deletion/billing cleanup. Operator completion facts are substitutes. Counters exclude diagnostics/replies; allocated memory includes allocator headroom. No persistence or provider calls.",
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
