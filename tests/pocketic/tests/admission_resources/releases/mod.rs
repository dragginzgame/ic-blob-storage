//! Synthetic multi-file manifests and explicit host-fact substitutes, not Miner acceptance.
use super::*;
use blob_test_protocol::admission::{
    ContentLookup,
    release::{LifecycleCommand, ReferenceCapacity},
};
use ic_blob_storage::model::identity::caffeine::{
    CaffeineContentHasher, CaffeineHashLimits, CaffeineHeader,
};
use std::num::{NonZeroU64, NonZeroUsize};

const OBJECTS: usize = 704;
const DECLARED_BYTES: u128 = 288 * 1024 * 1024;

fn content(f: &Fixture, object: usize) -> (Permission, JourneyManifest) {
    let bytes = if object < 640 { CHUNK / 4 } else { 2 * CHUNK };
    let mut chunk = vec![0; bytes.min(CHUNK)];
    chunk[..8].copy_from_slice(&(object as u64).to_le_bytes());
    let length = bytes.to_string();
    let headers = [CaffeineHeader {
        name: "Content-Length",
        value: &length,
    }];
    let mut hasher = CaffeineContentHasher::new(
        bytes as u64,
        &headers,
        CaffeineHashLimits {
            max_content_bytes: NonZeroU64::new(bytes as u64).unwrap(),
            max_append_bytes: NonZeroUsize::new(CHUNK).unwrap(),
            max_headers: NonZeroUsize::new(1).unwrap(),
            max_header_bytes: NonZeroUsize::new(1024).unwrap(),
        },
    )
    .unwrap();
    let mut chunks = Vec::new();
    for offset in (0..bytes).step_by(CHUNK) {
        hasher.append(offset as u64, &chunk).unwrap();
        // Independent fixture leaf calculation; only this test owns fabricated bytes.
        let mut leaf = Sha256::new();
        leaf.update(b"icfs-chunk/");
        leaf.update(&chunk);
        chunks.push(leaf.finalize().into());
    }
    let root = *hasher.finish().unwrap().provider_root.as_bytes();
    let request = Request {
        service: f.service,
        tenant: f.project,
        namespace: 1,
        id: object as u128 + 1,
        root,
        bytes: bytes as u64,
    };
    (
        Permission {
            request,
            uploader: f.uploader,
            expires_at_ns: f.harness.pic.get_time().as_nanos_since_unix_epoch() + 3_600_000_000_000,
        },
        JourneyManifest {
            chunks,
            headers: vec![("Content-Length".into(), length)],
        },
    )
}

fn capacity(
    f: &Fixture,
    actor: Principal,
    request: Request,
) -> Result<Option<ReferenceCapacity>, Failure> {
    f.harness
        .pic
        .query_candid_as(
            f.service,
            actor,
            "reference_capacity",
            (ContentLookup {
                service: request.service,
                tenant: request.tenant,
                namespace: request.namespace,
                root: request.root,
            },),
        )
        .unwrap()
}

fn reference(request: Request, generation: u128, retain: bool) -> Command {
    Command::FixtureLifecycle(LifecycleCommand::Reference {
        object: request,
        reference: generation,
        operation: if retain {
            2 * generation - 3
        } else {
            2 * generation
        },
        retain,
    })
}

fn measured(
    f: &Fixture,
    actor: Principal,
    command: Command,
    expected: Outcome,
) -> ExecutionProfile {
    assert_eq!(f.call(actor, command), Ok(expected));
    let profile = f.sample(actor);
    assert!(profile.after_work < 10_000_000);
    profile
}

fn reference_result(replayed: bool) -> Outcome {
    Outcome::Reference {
        replayed,
        result: Ok(true),
    }
}

fn populate(f: &Fixture, samples: &mut Vec<serde_json::Value>) -> Vec<Permission> {
    let mut inputs = Vec::new();
    for index in 0..OBJECTS {
        let (input, manifest) = content(f, index);
        let admit = measured(f, f.project, Command::Admit(input), Outcome::Admitted);
        let prepare = measured(
            f,
            f.uploader,
            Command::Prepare(input.request, manifest),
            Outcome::Changed(true),
        );
        measured(
            f,
            f.uploader,
            Command::Expose(input.request.root),
            Outcome::Exposed,
        );
        let confirm = measured(
            f,
            f.operator,
            Command::FixtureLifecycle(LifecycleCommand::SubstituteCompletion(input.request)),
            Outcome::Changed(true),
        );
        if [0, 63, 255, 639, 703].contains(&index) {
            samples.push(serde_json::json!({
                "phase":"populate", "objects":index + 1,
                "admit_instructions":admit.after_work, "admit_before_work":admit.before_work,
                "admit_before_decode":admit.before_decode, "admit_after_header":admit.after_header,
                "admit_after_value":admit.after_value, "admit_after_decode":admit.after_decode,
                "prepare_instructions":prepare.after_work, "prepare_before_work":prepare.before_work,
                "prepare_before_decode":prepare.before_decode, "prepare_after_header":prepare.after_header,
                "prepare_after_value":prepare.after_value, "prepare_after_decode":prepare.after_decode,
                "confirm_instructions":confirm.after_work, "confirm_before_work":confirm.before_work,
                "confirm_before_decode":confirm.before_decode, "confirm_after_header":confirm.after_header,
                "confirm_after_value":confirm.after_value, "confirm_after_decode":confirm.after_decode,
                "wasm_memory_bytes":f.wasm_bytes(),
            }));
        }
        inputs.push(input);
    }
    println!("release history: all manifests retained and substitute completions applied");
    inputs
}

fn overlap(f: &Fixture, inputs: &[Permission], samples: &mut Vec<serde_json::Value>) {
    for generation in 2..=4 {
        let mut max_retain = 0;
        let mut max_release = 0;
        let mut max_retain_workflow = 0;
        let mut max_release_workflow = 0;
        for input in inputs {
            let retain = measured(
                f,
                f.project,
                reference(input.request, generation, true),
                reference_result(false),
            );
            let release = measured(
                f,
                f.project,
                reference(input.request, generation - 1, false),
                reference_result(false),
            );
            max_retain = max_retain.max(retain.after_work);
            max_release = max_release.max(release.after_work);
            max_retain_workflow = max_retain_workflow.max(retain.after_work - retain.before_work);
            max_release_workflow =
                max_release_workflow.max(release.after_work - release.before_work);
        }
        samples.push(serde_json::json!({"phase":"overlap", "generation":generation, "max_retain_instructions":max_retain, "max_release_instructions":max_release, "max_retain_workflow_instructions":max_retain_workflow, "max_release_workflow_instructions":max_release_workflow, "wasm_memory_bytes":f.wasm_bytes()}));
        println!(
            "release history: reference generation {generation} retained; previous references released"
        );
    }
}

fn cleanup(f: &Fixture, inputs: &[Permission]) {
    for input in inputs {
        // The seventh receipt is reserved for final release; never use a fresh ID
        // to recover the earlier retain result after release has changed liveness.
        let final_release = Command::FixtureLifecycle(LifecycleCommand::Reference {
            object: input.request,
            reference: 4,
            operation: 7,
            retain: false,
        });
        measured(f, f.project, final_release.clone(), reference_result(false));
        assert_eq!(f.call(f.project, final_release), Ok(reference_result(true)));
        assert_eq!(
            capacity(f, f.project, input.request)
                .unwrap()
                .unwrap()
                .release_reserved_receipts,
            0
        );
    }
    let first = inputs[0];
    let usage = f.observe(first).usage.unwrap();
    assert_eq!(
        (usage.logical, usage.physical, usage.liability),
        (0, DECLARED_BYTES, DECLARED_BYTES)
    );
    for input in inputs {
        measured(
            f,
            f.operator,
            Command::FixtureLifecycle(LifecycleCommand::SubstituteDeletion(input.request)),
            Outcome::Changed(true),
        );
    }
    let usage = f.observe(first).usage.unwrap();
    assert_eq!(
        (usage.logical, usage.physical, usage.liability),
        (0, 0, DECLARED_BYTES)
    );
    for input in inputs {
        measured(
            f,
            f.operator,
            Command::FixtureLifecycle(LifecycleCommand::SubstituteSettlement(input.request)),
            Outcome::Changed(true),
        );
    }
    let usage = f.observe(first).usage.unwrap();
    assert_eq!((usage.logical, usage.physical, usage.liability), (0, 0, 0));
}

#[test]
fn resource_multifile_release_history_preserves_cleanup_at_capacity() {
    let f = Fixture::with_workload(Workload::ReleaseHistory);
    f.enroll();
    let initial_heap = f.wasm_bytes();
    let mut samples = Vec::new();
    let inputs = populate(&f, &mut samples);
    let first = inputs[0];
    for actor in [f.operator, f.uploader, f.controller, f.other] {
        assert_eq!(capacity(&f, actor, first.request), Err(Failure::NotProject));
        assert_eq!(
            f.call(
                actor,
                Command::FixtureLifecycle(LifecycleCommand::SubstituteCompletion(first.request))
            ),
            if actor == f.operator {
                Ok(Outcome::Changed(false))
            } else {
                Err(Failure::NotOperator)
            }
        );
    }
    assert_eq!(
        capacity(&f, f.project, first.request)
            .unwrap()
            .unwrap()
            .fresh_retains,
        3
    );
    let original = f.observe(first);
    assert_eq!(original.usage.unwrap().physical, DECLARED_BYTES);
    overlap(&f, &inputs, &mut samples);
    f.restart();
    assert_eq!(f.observe(first), original);
    for input in &inputs {
        assert_eq!(
            capacity(&f, f.project, input.request),
            Ok(Some(ReferenceCapacity {
                reference_slots: 0,
                unreserved_receipts: 0,
                release_reserved_receipts: 1,
                fresh_retains: 0
            }))
        );
    }
    assert_eq!(
        f.call(f.project, reference(first.request, 4, true)),
        Ok(reference_result(true))
    );
    assert_eq!(
        f.call(f.project, reference(first.request, 5, true)),
        Err(Failure::Capacity)
    );
    let (extra, _) = content(&f, OBJECTS);
    assert_eq!(
        f.call(f.project, Command::Admit(extra)),
        Err(Failure::TenantManifestCapacity)
    );
    cleanup(&f, &inputs);
    assert_eq!(
        f.call(f.project, Command::Admit(extra)),
        Err(Failure::TenantManifestCapacity)
    );
    assert_eq!(
        f.call(f.project, Command::Admit(first)),
        Ok(Outcome::Existing(Phase::Confirmed))
    );
    let final_heap = f.wasm_bytes();
    assert!(final_heap <= initial_heap + 16 * CHUNK as u64);
    let report = serde_json::json!({
        "scope":"Local transient owner; synthetic 640 x 256 KiB and 64 x 2 MiB files, 768 retained leaves, four reference generations. Operator-only completion/deletion/billing facts are substitutes, not Caffeine evidence. No file bytes enter the canister. Instructions exclude diagnostic writes/reply encoding; memory is allocated Wasm pages. No persistence or read sessions.",
        "wasm_sha256":wasm_hash(), "objects":OBJECTS, "declared_bytes":DECLARED_BYTES,
        "retained_references":OBJECTS * 4, "retained_receipts":OBJECTS * 7,
        "initial_wasm_memory_bytes":initial_heap, "final_wasm_memory_bytes":final_heap, "samples":samples,
    });
    if let Some(path) = std::env::var_os("BLOB_RELEASE_HISTORY_REPORT") {
        std::fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
    println!("release history profile: {report}");
}
