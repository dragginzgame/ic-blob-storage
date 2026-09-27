//! Saturated cancelled history: identity slots remain occupied while bytes release.
use super::*;

fn permission(f: &Fixture, tenant: Principal, id: u128) -> Permission {
    let mut input = f.permission(&vectors::vector("abc-text", 1));
    input.request.tenant = tenant;
    input.request.id = id;
    // Synthetic roots are sufficient for admission/cancellation; never prepare
    // a manifest or claim provider storage for these declarations.
    input.request.root = Sha256::digest(id.to_le_bytes()).into();
    input.request.bytes = 1;
    input
}

#[test]
fn resource_root_lookup_at_capacity_keeps_exact_last_permission_and_authority() {
    let f = Fixture::with_workload(Workload::RetainedHistory);
    f.enroll();
    assert_eq!(
        f.call(
            f.operator,
            Command::Enroll {
                tenant: f.other,
                expected: None,
                active: true
            }
        ),
        Ok(Outcome::Enrolled(Enrollment {
            generation: 1,
            active: true
        }))
    );
    for id in 1..256 {
        let tenant = if id <= 128 { f.project } else { f.other };
        let input = permission(&f, tenant, id);
        assert_eq!(f.call(tenant, Command::Admit(input)), Ok(Outcome::Admitted));
        assert_eq!(
            f.call(tenant, Command::Revoke(input.request)),
            Ok(Outcome::Changed(true))
        );
    }
    let vector = vectors::vector("abc-text", 1);
    let mut last = f.permission(&vector);
    last.request.id = 256;
    last.request.tenant = f.other;
    assert_eq!(f.call(f.other, Command::Admit(last)), Ok(Outcome::Admitted));
    let original = f.inspect(f.other, last.request).unwrap();
    assert_eq!(
        f.call(f.uploader, Command::Expose(last.request.root)),
        Err(Failure::Unprepared)
    );
    f.prepare(last, &vector);
    f.restart();
    for caller in [f.project, f.other, f.operator, f.controller] {
        assert_eq!(
            f.call(caller, Command::Expose(last.request.root)),
            Err(Failure::NotUploader)
        );
    }
    assert_eq!(
        f.call(
            f.uploader,
            Command::Expose(permission(&f, f.project, 1).request.root)
        ),
        Err(Failure::Revoked)
    );
    let unknown = permission(&f, f.other, 1000);
    assert_eq!(
        f.call(f.other, Command::Admit(unknown)),
        Err(Failure::TenantManifestCapacity)
    );
    assert_eq!(
        f.call(f.uploader, Command::Expose(unknown.request.root)),
        Err(Failure::Unknown)
    );
    assert_eq!(
        f.call(f.uploader, Command::Expose(last.request.root)),
        Ok(Outcome::Exposed)
    );
    assert!(f.sample(f.uploader).after_work < 5_000_000);
    let exposed = f.inspect(f.other, last.request).unwrap();
    assert_eq!(exposed.permission, last);
    assert_eq!(exposed.phase, Phase::ExposurePossible);
    assert_eq!(exposed.usage, original.usage);
    assert_eq!(
        f.call(f.uploader, Command::Expose(last.request.root)),
        Err(Failure::Phase)
    );
    assert_eq!(f.inspect(f.other, last.request).unwrap(), exposed);
}

#[test]
fn resource_history_saturation_keeps_retries_cleanup_and_other_tenant_available() {
    let f = Fixture::with_workload(Workload::RetainedHistory);
    f.enroll();
    assert_eq!(
        f.call(
            f.operator,
            Command::Enroll {
                tenant: f.other,
                expected: None,
                active: true,
            }
        ),
        Ok(Outcome::Enrolled(Enrollment {
            generation: 1,
            active: true
        }))
    );
    let initial_heap = f.wasm_bytes();
    let mut samples = Vec::new();
    for (tenant_index, tenant) in [f.project, f.other].into_iter().enumerate() {
        for index in 1..=128u128 {
            let input = permission(&f, tenant, tenant_index as u128 * 128 + index);
            assert_eq!(f.call(tenant, Command::Admit(input)), Ok(Outcome::Admitted));
            let admit = f.sample(tenant);
            assert_eq!(
                f.call(tenant, Command::Revoke(input.request)),
                Ok(Outcome::Changed(true))
            );
            let cancel = f.sample(tenant);
            // Broad per-message ceilings include decoder and allocator costs;
            // compiler/layout changes need not reproduce exact observations.
            assert!(admit.after_work < 5_000_000);
            assert!(cancel.after_work < 5_000_000);
            if [1, 64, 128].contains(&index) {
                samples.push(serde_json::json!({
                    "retained_operations": tenant_index as u128 * 128 + index,
                    "tenant_operations": index,
                    "admit_instructions": admit.after_work,
                    "admit_before_decode": admit.before_decode,
                    "admit_after_header": admit.after_header,
                    "admit_after_value": admit.after_value,
                    "admit_after_decode": admit.after_decode,
                    "admit_workflow_instructions": admit.after_work - admit.before_work,
                    "cancel_instructions": cancel.after_work,
                    "wasm_memory_bytes": f.wasm_bytes(),
                }));
            }
        }
        let rejected = permission(&f, tenant, 1000 + tenant_index as u128);
        assert_eq!(
            f.call(tenant, Command::Admit(rejected)),
            Err(Failure::TenantManifestCapacity)
        );
        assert_eq!(f.inspect(tenant, rejected.request), Err(Failure::Unknown));
    }
    f.restart();
    for (tenant, id) in [(f.project, 1), (f.other, 129)] {
        let input = permission(&f, tenant, id);
        // Reuse the retained permission, including its original deadline.
        let original = f.inspect(tenant, input.request).unwrap();
        assert_eq!(original.phase, Phase::Cancelled);
        let usage = original.usage.unwrap();
        assert_eq!((usage.logical, usage.physical, usage.liability), (0, 0, 0));
        assert_eq!(
            f.call(tenant, Command::Admit(original.permission)),
            Ok(Outcome::Existing(Phase::Cancelled))
        );
        assert_eq!(
            f.call(tenant, Command::Revoke(input.request)),
            Ok(Outcome::Changed(false))
        );
        assert_eq!(f.inspect(tenant, input.request).unwrap(), original);
    }
    let final_heap = f.wasm_bytes();
    assert!(final_heap <= initial_heap + 2 * CHUNK as u64);
    let report = serde_json::json!({
        "scope": "Local transient release Wasm, two tenants and 256 cancelled one-byte admissions. No manifests, provider calls, persistence or confirmed-object history. Instructions exclude diagnostic storage and reply encoding; memory is allocated Wasm pages.",
        "wasm_sha256": wasm_hash(),
        "initial_wasm_memory_bytes": initial_heap,
        "final_wasm_memory_bytes": final_heap,
        "samples": samples,
    });
    if let Some(path) = std::env::var_os("BLOB_ADMISSION_HISTORY_REPORT") {
        std::fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
    println!("admission history profile: {report}");
}
