//! Maximal retained metadata under the existing lifetime object envelope.
use super::*;
use blob_test_protocol::admission::{ContentDescriptor, ContentLookup, ContentState};
use ic_blob_storage::model::identity::caffeine::{
    CaffeineContentHasher, CaffeineHashLimits, CaffeineHeader,
};
use std::num::{NonZeroU64, NonZeroUsize};

fn wide(f: &Fixture, id: u8, tenant: Principal) -> (Permission, JourneyManifest) {
    let v = vectors::vector("abc-text", id);
    let mut manifest = v.manifest.clone();
    manifest.headers.push(("X-Object".into(), id.to_string()));
    for n in 0..4 {
        manifest.headers.push((format!("X-{n}"), "value".into()));
    }
    manifest.headers.push(("X-Padding".into(), String::new()));
    let used: usize = manifest
        .headers
        .iter()
        .map(|(name, value)| name.len() + value.len() + 3)
        .sum();
    manifest.headers.last_mut().unwrap().1 = "x".repeat(1024 - used);
    assert_eq!(manifest.headers.len(), 8);
    assert_eq!(
        manifest
            .headers
            .iter()
            .map(|(name, value)| name.len() + value.len() + 3)
            .sum::<usize>(),
        1024
    );
    let headers: Vec<_> = manifest
        .headers
        .iter()
        .map(|(name, value)| CaffeineHeader { name, value })
        .collect();
    let mut hasher = CaffeineContentHasher::new(
        3,
        &headers,
        CaffeineHashLimits {
            max_content_bytes: NonZeroU64::new(3).unwrap(),
            max_append_bytes: NonZeroUsize::new(3).unwrap(),
            max_headers: NonZeroUsize::new(8).unwrap(),
            max_header_bytes: NonZeroUsize::new(1024).unwrap(),
        },
    )
    .unwrap();
    hasher.append(0, b"abc").unwrap();
    let mut p = f.permission(&v);
    p.request.root = *hasher.finish().unwrap().provider_root.as_bytes();
    p.request.tenant = tenant;
    (p, manifest)
}

fn descriptor(f: &Fixture, p: Permission) -> ContentDescriptor {
    let result: Result<Option<ContentDescriptor>, Failure> = f
        .harness
        .pic
        .query_candid_as(
            f.service,
            p.request.tenant,
            "content_descriptor",
            (ContentLookup {
                service: f.service,
                tenant: p.request.tenant,
                namespace: 1,
                root: p.request.root,
            },),
        )
        .unwrap();
    result.unwrap().unwrap()
}

fn bind(f: &Fixture, p: Permission, manifest: &JourneyManifest) -> u64 {
    assert_eq!(
        f.call(p.request.tenant, Command::Admit(p)),
        Ok(Outcome::Admitted)
    );
    let mut oversized = manifest.clone();
    oversized.headers.last_mut().unwrap().1.push('x');
    assert_eq!(
        f.call(f.uploader, Command::Prepare(p.request, oversized)),
        Err(Failure::Manifest)
    );
    assert_eq!(
        f.call(f.uploader, Command::Prepare(p.request, manifest.clone())),
        Ok(Outcome::Changed(true))
    );
    let instructions = f.sample(f.uploader).after_work;
    let original = descriptor(f, p);
    assert_eq!(original.headers, manifest.headers);
    let mut reordered = manifest.clone();
    reordered.headers.reverse();
    for _ in 0..8 {
        assert_eq!(
            f.call(f.uploader, Command::Prepare(p.request, reordered.clone())),
            Ok(Outcome::Changed(false))
        );
        assert_eq!(descriptor(f, p), original);
    }
    assert_eq!(
        f.call(p.request.tenant, Command::Revoke(p.request)),
        Ok(Outcome::Changed(true))
    );
    instructions
}

#[test]
fn descriptors_keep_maximum_metadata_at_lifetime_capacity() {
    let f = Fixture::new();
    let enrolled = f.enroll();
    f.call(
        f.operator,
        Command::Enroll {
            tenant: f.other,
            expected: None,
            active: true,
        },
    )
    .unwrap();
    let initial = f.wasm_bytes();
    let mut retained = Vec::new();
    let mut samples = Vec::new();
    for id in 1..=4 {
        let tenant = if id <= 2 { f.project } else { f.other };
        let (p, manifest) = wide(&f, id, tenant);
        let instructions = bind(&f, p, &manifest);
        samples.push(serde_json::json!({"objects":id,"prepare_instructions":instructions,"wasm_memory_bytes":f.wasm_bytes()}));
        retained.push((p, manifest));
    }
    let (extra, _) = wide(&f, 5, f.project);
    assert_eq!(
        f.call(f.project, Command::Admit(extra)),
        Err(Failure::Catalog)
    );
    f.call(f.operator, f.enrollment(Some(enrolled), false))
        .unwrap();
    f.restart();
    for (p, manifest) in retained {
        let view = descriptor(&f, p);
        assert_eq!(view.content.state, ContentState::Cancelled);
        assert_eq!(view.headers, manifest.headers);
        assert_eq!(
            f.inspect(p.request.tenant, p.request)
                .unwrap()
                .usage
                .unwrap()
                .logical,
            0
        );
    }
    let final_heap = f.wasm_bytes();
    assert!(final_heap <= initial + 1024 * 1024);
    let report = serde_json::json!({
        "scope":"Four prepared then cancelled objects across two tenants, each with eight headers and 1024 framed metadata bytes. Reordered retries, overflow rejection, full lifetime history, suspension and stop/start. Transient probe; no production persistence, certified descriptor or provider calls. Counters exclude reply encoding/diagnostics.",
        "wasm_sha256":wasm_hash(),"retained_framed_metadata_bytes":4096,
        "initial_wasm_memory_bytes":initial,"final_wasm_memory_bytes":final_heap,"samples":samples,
    });
    if let Some(path) = std::env::var_os("BLOB_DESCRIPTOR_RESOURCE_REPORT") {
        std::fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
    println!("descriptor resource profile: {report}");
}
