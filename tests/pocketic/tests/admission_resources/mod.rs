//! Local Wasm cost observations and rejected-ingress atomicity, not provider pricing.
use super::*;
use blob_test_protocol::{
    admission::{ExecutionProfile, RESOURCE_SAMPLE_CAPACITY, input::PreparationInput},
    journey::JourneyManifest,
};
use ic_blob_storage_pocketic_tests::fixture_calls;
use sha2::{Digest, Sha256};

mod descriptors;
mod history;
mod references;
mod releases;

const CHUNK: usize = 1024 * 1024;

#[test]
fn resource_typed_endpoints_preserve_provider_and_tenant_authority() {
    use blob_test_protocol::admission::release::LifecycleCommand;
    let f = Fixture::new();
    f.enroll();
    let vector = vectors::vector("abc-text", 1);
    let permission = f.permission(&vector);
    f.admit(permission);
    let original = f.observe(permission);
    for command in [
        LifecycleCommand::SubstituteDeletion(permission.request),
        LifecycleCommand::SubstituteSettlement(permission.request),
    ] {
        for actor in [
            f.project,
            f.uploader,
            f.other,
            f.controller,
            Principal::anonymous(),
        ] {
            assert_eq!(
                f.call(actor, Command::FixtureLifecycle(command)),
                Err(Failure::NotOperator)
            );
        }
    }
    for actor in [
        f.operator,
        f.uploader,
        f.other,
        f.controller,
        Principal::anonymous(),
    ] {
        for retain in [true, false] {
            assert_eq!(
                f.call(
                    actor,
                    Command::FixtureLifecycle(LifecycleCommand::Reference {
                        object: permission.request,
                        reference: 1,
                        operation: 1,
                        retain,
                    })
                ),
                Err(Failure::NotProject)
            );
        }
    }
    assert_eq!(f.observe(permission), original);
    f.prepare(permission, &vector);
    assert_eq!(
        f.call(f.uploader, Command::Expose(permission.request.root)),
        Ok(Outcome::Exposed)
    );
}

impl Fixture {
    fn resources(&self, caller: Principal, limit: usize) -> Result<Vec<ExecutionProfile>, Failure> {
        fixture_calls::query_fixture(
            &self.harness.pic,
            self.service,
            caller,
            "resources",
            (u8::try_from(limit).unwrap(),),
        )
        .map(|(reply,)| reply)
        .unwrap()
    }

    fn sample(&self, caller: Principal) -> ExecutionProfile {
        let profile = *self.resources(self.operator, 1).unwrap().last().unwrap();
        check_profile(profile, caller);
        profile
    }

    fn wasm_bytes(&self) -> u64 {
        let status = self
            .harness
            .pic
            .canister_status(self.service, Some(self.controller))
            .unwrap();
        u64::try_from(status.memory_metrics.wasm_memory_size.0).unwrap()
    }

    fn reject_input(&self, bytes: Vec<u8>) {
        let error = self
            .harness
            .pic
            .update_call(self.service, self.uploader, "prepare", bytes)
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
    }
}

fn check_profile(profile: ExecutionProfile, caller: Principal) {
    assert_eq!(profile.caller, caller);
    assert!(profile.after_decode > profile.before_decode);
    assert!(profile.before_work >= profile.after_decode);
    assert!(profile.after_work > profile.before_work);
}

#[test]
fn resource_observations_are_operator_only_and_reads_preserve_the_sample() {
    let f = Fixture::new();
    assert_eq!(
        f.resources(f.operator, RESOURCE_SAMPLE_CAPACITY),
        Ok(vec![])
    );
    f.enroll();
    let sample = f.sample(f.operator);
    for caller in [f.controller, f.project, f.uploader, Principal::anonymous()] {
        assert_eq!(
            f.resources(caller, RESOURCE_SAMPLE_CAPACITY),
            Err(Failure::NotOperator)
        );
    }
    assert_eq!(
        f.resources(f.operator, RESOURCE_SAMPLE_CAPACITY),
        Ok(vec![sample])
    );
    assert_eq!(
        f.call(f.controller, f.enrollment(None, false)),
        Err(Failure::NotOperator)
    );
    f.sample(f.controller);
}

#[test]
fn resource_window_preserves_order_and_overwrites_only_the_oldest_samples() {
    let f = Fixture::new();
    f.enroll();
    let initial = f.sample(f.operator).sequence;
    let callers: Vec<_> = (0..RESOURCE_SAMPLE_CAPACITY + 3)
        .map(|index| fake_principal(u32::try_from(index + 20).unwrap()))
        .collect();
    for caller in &callers {
        assert_eq!(
            f.call(*caller, f.enrollment(None, false)),
            Err(Failure::NotOperator)
        );
    }
    let profiles = f.resources(f.operator, RESOURCE_SAMPLE_CAPACITY).unwrap();
    assert_eq!(profiles.len(), RESOURCE_SAMPLE_CAPACITY);
    let discarded = callers.len() - RESOURCE_SAMPLE_CAPACITY;
    for (index, (profile, caller)) in profiles.iter().zip(&callers[discarded..]).enumerate() {
        check_profile(*profile, *caller);
        assert_eq!(profile.sequence, initial + (discarded + index + 1) as u64);
    }
    f.restart();
    assert_eq!(
        f.resources(f.operator, RESOURCE_SAMPLE_CAPACITY),
        Ok(profiles.clone())
    );
    for limit in [1, 3, RESOURCE_SAMPLE_CAPACITY] {
        assert_eq!(
            f.resources(f.operator, limit),
            Ok(profiles[profiles.len() - limit..].to_vec())
        );
    }
    for limit in [0, RESOURCE_SAMPLE_CAPACITY + 1] {
        assert_eq!(f.resources(f.operator, limit), Err(Failure::InvalidInput));
        assert_eq!(f.resources(f.controller, limit), Err(Failure::NotOperator));
    }
    assert_eq!(
        f.resources(f.controller, RESOURCE_SAMPLE_CAPACITY),
        Err(Failure::NotOperator)
    );
    assert_eq!(
        f.resources(f.operator, RESOURCE_SAMPLE_CAPACITY),
        Ok(profiles)
    );
}

#[test]
fn resource_envelope_rejects_before_workflow_and_preserves_the_original_permission() {
    let f = Fixture::new();
    f.enroll();
    let v = vectors::vector("abc-text", 1);
    let p = f.permission(&v);
    f.admit(p);
    let original = f.observe(p);
    f.sample(f.project);
    let profiles = f.resources(f.operator, RESOURCE_SAMPLE_CAPACITY).unwrap();
    let valid = PreparationInput {
        request: p.request,
        manifest: v.manifest.clone(),
    };
    let frame = blob_test_protocol::fixture_wire::encode_one("prepare", &valid).unwrap();
    let mut trailing = frame.clone();
    trailing.push(0);
    let mut truncated = frame.clone();
    truncated.pop();
    let too_large = PreparationInput {
        request: p.request,
        manifest: JourneyManifest {
            chunks: v.manifest.chunks.clone(),
            headers: vec![("X".to_owned(), "x".repeat(16 * 1024))],
        },
    };
    for bytes in [
        blob_test_protocol::fixture_wire::encode_one("prepare", too_large).unwrap(),
        blob_test_protocol::fixture_wire::encode_one("admit", &valid).unwrap(),
        blob_test_protocol::fixture_wire::encode_reply("prepare", &valid).unwrap(),
        blob_test_protocol::fixture_wire::encode_args("prepare", (&valid, 7_u64)).unwrap(),
        ic_platform::candid::contract::encode_one(7_u64).unwrap(),
        trailing,
        truncated,
        vec![],
    ] {
        f.reject_input(bytes);
    }
    assert_eq!(f.observe(p), original);
    assert_eq!(
        f.resources(f.operator, RESOURCE_SAMPLE_CAPACITY),
        Ok(profiles)
    );
    // A valid input still works after every rejected message.
    f.prepare(p, &v);
    assert_eq!(f.observe(p).manifest, ManifestState::Bound);
}
#[test]
fn resource_manifest_conversion_rejects_large_collections_without_hash_work() {
    let f = Fixture::new();
    f.enroll();
    let v = vectors::vector("abc-text", 1);
    let p = f.permission(&v);
    f.admit(p);
    let original = f.observe(p);
    for manifest in [
        JourneyManifest {
            chunks: vec![[0; 32]; 256],
            headers: vec![],
        },
        JourneyManifest {
            chunks: v.manifest.chunks.clone(),
            headers: vec![(String::new(), String::new()); 256],
        },
        JourneyManifest {
            chunks: v.manifest.chunks.clone(),
            headers: vec![("X".to_owned(), "x".repeat(4096))],
        },
    ] {
        assert_eq!(
            f.call(f.uploader, Command::Prepare(p.request, manifest)),
            Err(Failure::Manifest)
        );
        let sample = f.sample(f.uploader);
        // Coarse work budget excludes Candid decode but includes argument destruction;
        // it detects accidental conversion/hashing of these oversized inputs.
        assert!(sample.after_work - sample.before_work < 1_000_000);
        assert_eq!(f.observe(p), original);
    }
    f.prepare(p, &v);
}

#[test]
fn resource_profile_ten_mib_direct_manifest_and_exposure() {
    let f = Fixture::new();
    f.enroll();
    let v = vectors::vector("media-10485760", 1);
    let p = f.permission(&v);
    let initial_heap = f.wasm_bytes();
    let mut messages = Vec::new();
    let mut total = 0;
    for (step, caller, command, expected) in [
        ("admit", f.project, Command::Admit(p), Outcome::Admitted),
        (
            "prepare",
            f.uploader,
            Command::Prepare(p.request, v.manifest.clone()),
            Outcome::Changed(true),
        ),
        (
            "prepare_retry",
            f.uploader,
            Command::Prepare(p.request, v.manifest),
            Outcome::Changed(false),
        ),
        (
            "expose",
            f.uploader,
            Command::Expose(p.request.root),
            Outcome::Exposed,
        ),
    ] {
        let wire_bytes = admission_wire::encode(command.clone()).1.len();
        assert!(wire_bytes < 4096);
        assert_eq!(f.call(caller, command), Ok(expected));
        let sample = f.sample(caller);
        let heap = f.wasm_bytes();
        total += sample.after_work;
        // Headroom for normal compiler/allocator changes; bound metadata work.
        assert!(sample.after_work < 5_000_000);
        assert!(heap <= initial_heap + CHUNK as u64);
        messages.push(serde_json::json!({"step":step,"wire_bytes":wire_bytes,"before_decode":sample.before_decode,"after_decode":sample.after_decode,"before_work":sample.before_work,"after_work":sample.after_work,"wasm_memory_bytes":heap}));
    }
    assert!(total < 10_000_000);
    let view = f.observe(p);
    assert_eq!(view.manifest, ManifestState::Bound);
    assert_eq!(view.phase, Phase::Registered);
    assert_eq!(view.usage.unwrap().liability, u128::from(p.request.bytes));
    let wasm_sha256 = wasm_hash();
    let report = serde_json::json!({
        "scope":"Local release Wasm; bounded manifest admission and local exposure only. No file bytes or provider call. Counters exclude diagnostic storage and reply encoding; allocated Wasm memory is not live allocation size.",
        "wasm_sha256":wasm_sha256,"object_bytes":p.request.bytes,
        "initial_wasm_memory_bytes":initial_heap,"instructions_sum":total,"messages":messages
    });
    if let Some(path) = std::env::var_os("BLOB_ADMISSION_RESOURCE_REPORT") {
        std::fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
    println!("admission resource profile: {report}");
}

fn wasm_hash() -> String {
    hex::encode(Sha256::digest(Fixture::wasm()))
}
