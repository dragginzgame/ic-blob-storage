use super::*;
mod admission_boundary;
mod capacity_boundary;
mod certificate_boundary;
mod exposure_boundary;
mod manifest_boundary;
mod reference_capacity_boundary;
mod revocation;
use crate::{
    model::{
        catalog::admission::{UploadObject, UploadRequestId},
        identity::{
            ProviderRootHash,
            caffeine::{
                CaffeineHashLimits, CaffeineHeader, manifest::builder::CaffeineManifestBuilder,
            },
        },
        lifecycle::{
            ReferenceId,
            binding::{ObjectBinding, ObjectIdentity, ReferenceKey},
        },
        service::upload::UploadAdmissions,
    },
    ops::service::tenant::tests::config,
};
use ic_memory::ic_stable_structures::VectorMemory;
use std::num::NonZeroU128;
fn p(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}
fn context(n: u8) -> UploadContext {
    UploadContext {
        service: p(1),
        actor: p(n),
    }
}
fn memory() -> UploadMemories<VectorMemory> {
    UploadMemories {
        tenants: VectorMemory::default(),
        roots: VectorMemory::default(),
        objects: VectorMemory::default(),
        permissions: VectorMemory::default(),
        usage: VectorMemory::default(),
        manifests: VectorMemory::default(),
        confirmed: VectorMemory::default(),
        references: VectorMemory::default(),
        receipts: VectorMemory::default(),
        root_requests: VectorMemory::default(),
    }
}
fn clone_memory(m: &UploadMemories<VectorMemory>) -> UploadMemories<VectorMemory> {
    UploadMemories {
        tenants: m.tenants.clone(),
        roots: m.roots.clone(),
        objects: m.objects.clone(),
        permissions: m.permissions.clone(),
        usage: m.usage.clone(),
        manifests: m.manifests.clone(),
        confirmed: m.confirmed.clone(),
        references: m.references.clone(),
        receipts: m.receipts.clone(),
        root_requests: m.root_requests.clone(),
    }
}
fn permission(id: u128) -> UploadPermission {
    let n = NonZeroU128::new(id).unwrap();
    UploadPermission {
        request: UploadRequest {
            id: UploadRequestId::new(n),
            object: UploadObject {
                root: ProviderRootHash::try_from([u8::try_from(id).unwrap(); 32].as_slice())
                    .unwrap(),
                bytes: 10,
                first: ReferenceKey::new(
                    ObjectBinding::new(
                        p(1),
                        p(4),
                        ObjectIdentity {
                            namespace: NonZeroU128::MIN,
                            object: n,
                            incarnation: NonZeroU128::MIN,
                        },
                    )
                    .unwrap(),
                    ReferenceId::new(NonZeroU128::MIN),
                ),
            },
        },
        uploader: p(5),
        expires_at_ns: 100,
    }
}
fn enroll(store: &mut StableUploads<VectorMemory>) -> TenantEnrollmentView {
    store
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: None,
                active: true,
            },
        )
        .unwrap()
}

#[test]
fn exact_admission_and_cancellation_match_heap_and_survive_fenced_reopen() {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    let mut heap = UploadAdmissions::new(config());
    let command = TenantUpdate {
        tenant: p(4),
        expected: None,
        active: true,
    };
    store.update_tenant(context(2), command).unwrap();
    heap.update_tenant(context(2), command).unwrap();
    for input in [permission(1), permission(1), permission(2)] {
        assert_eq!(
            store.admit(context(4), input, 1),
            heap.admit(context(4), input, 1)
                .map_err(UploadStoreError::Admission)
        );
        assert_eq!(store.usage().unwrap(), heap.catalog().usage());
        planning::parity(&store, &heap, input);
    }
    let input = permission(1);
    assert_eq!(
        store.revoke(context(4), input.request),
        heap.revoke(context(4), input.request)
            .map_err(UploadStoreError::Admission)
    );
    assert_eq!(store.usage().unwrap(), heap.catalog().usage());
    planning::parity(&store, &heap, input);
    assert_eq!(
        store.admit(context(4), input, 101),
        Ok(UploadAdmission::Existing(UploadPhase::Cancelled))
    );
    let before = store.lookup(context(5), input.request).unwrap();
    let totals = store.usage().unwrap();
    drop(store);
    let mut restored = StableUploads::open(clone_memory(&m), config()).unwrap();
    assert_eq!(restored.lookup(context(5), input.request), Ok(before));
    assert_eq!(restored.usage(), Ok(totals));
    assert_eq!(
        restored.revoke(context(4), input.request),
        Err(UploadStoreError::Fenced)
    );
    assert_eq!(
        restored.admit(context(4), input, 1),
        Err(UploadStoreError::Fenced)
    );
}

#[test]
fn rejected_admissions_cannot_claim_roots_or_charge_capacity() {
    let mut store = StableUploads::install(memory(), config()).unwrap();
    let input = permission(1);
    assert_eq!(
        store.admit(context(4), input, 1),
        Err(UploadAdmissionError::Tenant(TenantError::NotEnrolled).into())
    );
    enroll(&mut store);
    let before = store.usage().unwrap();
    assert_eq!(
        store.admit(context(5), input, 1),
        Err(UploadAdmissionError::NotProject.into())
    );
    assert_eq!(
        store.admit(context(4), input, 100),
        Err(UploadAdmissionError::Expired.into())
    );
    assert_eq!(store.usage(), Ok(before));
    assert_eq!(store.roots.slots(), 0);
    store.admit(context(4), input, 1).unwrap();
    let changed = UploadPermission {
        expires_at_ns: 99,
        ..input
    };
    assert_eq!(
        store.admit(context(4), changed, 1),
        Err(UploadAdmissionError::PermissionConflict.into())
    );
    let duplicate = UploadPermission {
        request: UploadRequest {
            id: permission(2).request.id,
            ..input.request
        },
        ..input
    };
    assert_eq!(
        store.admit(context(4), duplicate, 1),
        Err(UploadError::Root(RootClaimError::RootAlreadyClaimed).into())
    );
    assert_eq!(store.roots.slots(), 1);
    assert_eq!(store.usage().unwrap().operations, 1);
}

#[test]
fn inconsistent_counters_or_missing_claims_are_rejected_without_repair() {
    for damage in 0..3 {
        let m = memory();
        let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
        enroll(&mut store);
        store.admit(context(4), permission(1), 1).unwrap();
        match damage {
            0 => {
                store.usage.insert(p(4), UploadUsageRecord::empty());
            }
            1 => {
                store.usage.remove(&p(4));
            }
            _ => {
                store.permissions.remove(&key(permission(1).request));
            }
        }
        drop(store);
        let before = m.usage.borrow().clone();
        assert!(matches!(
            StableUploads::open(clone_memory(&m), config()),
            Err(UploadStoreError::InvalidRecord)
        ));
        assert_eq!(*m.usage.borrow(), before);
    }
}

const HEADERS: [CaffeineHeader<'static>; 2] = [
    CaffeineHeader {
        name: "Content-Length",
        value: "10",
    },
    CaffeineHeader {
        name: "Content-Type",
        value: "image/png",
    },
];
fn built() -> crate::model::identity::caffeine::manifest::builder::CaffeineBuiltManifest {
    use std::num::NonZeroUsize;
    let mut builder = CaffeineManifestBuilder::new(
        10,
        &HEADERS,
        CaffeineHashLimits {
            max_content_bytes: NonZeroU64::new(10).unwrap(),
            max_append_bytes: NonZeroUsize::new(10).unwrap(),
            max_headers: NonZeroUsize::new(8).unwrap(),
            max_header_bytes: NonZeroUsize::new(1024).unwrap(),
        },
        NonZeroUsize::MIN,
    )
    .unwrap();
    builder.append(0, &[1; 10]).unwrap();
    builder.finish().unwrap()
}

#[test]
fn manifest_exposure_and_revocation_retain_uncertain_bytes_and_original_metadata() {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    enroll(&mut store);
    let built = built();
    let mut input = permission(1);
    input.request.object.root = built.hashes().provider_root;
    store.admit(context(4), input, 1).unwrap();
    assert_eq!(
        store.expose(context(5), input.request, 2),
        Err(UploadAdmissionError::ManifestNotPrepared.into())
    );
    let manifest = UploadManifest {
        chunks: built.manifest().chunks(),
        headers: &HEADERS,
    };
    assert_eq!(
        store.prepare_manifest(context(4), input.request, manifest, 2),
        Err(UploadAdmissionError::NotUploader.into())
    );
    assert_eq!(
        store.prepare_manifest(context(5), input.request, manifest, 2),
        Ok(LifecycleChange::Changed)
    );
    let reversed = [HEADERS[1], HEADERS[0]];
    assert_eq!(
        store.prepare_manifest(
            context(5),
            input.request,
            UploadManifest {
                headers: &reversed,
                ..manifest
            },
            2
        ),
        Ok(LifecycleChange::Unchanged)
    );
    let original = UploadManifestRecord::new(manifest);
    // A one-leaf declaration must not allocate a tree node sized for several
    // maximum 64 KiB records. This is a native storage ceiling, not IC sizing.
    assert!(m.manifests.size() * 65_536 <= 128 * 1024);
    assert_eq!(
        store.manifests.get(&key(input.request)),
        Some(original.clone())
    );
    store.expose(context(5), input.request, 3).unwrap();
    let totals = store.usage().unwrap();
    store.revoke(context(4), input.request).unwrap();
    assert_eq!(store.usage(), Ok(totals));
    assert_eq!(totals.reserved_bytes, 10);
    assert_eq!(
        store.lookup(context(5), input.request).unwrap().phase,
        UploadPhase::ExposurePossible
    );
    assert_eq!(
        store.expose(context(5), input.request, 4),
        Err(UploadAdmissionError::Revoked.into())
    );
    drop(store);
    let restored = StableUploads::open(clone_memory(&m), config()).unwrap();
    assert_eq!(restored.usage(), Ok(totals));
    assert_eq!(restored.manifests.get(&key(input.request)), Some(original));
    assert!(restored.lookup(context(4), input.request).unwrap().revoked);
    assert_eq!(
        restored.lookup(context(6), input.request),
        Err(UploadAdmissionError::NotObserver.into())
    );
}

#[test]
fn suspension_and_reactivation_do_not_renew_old_permissions() {
    let mut store = StableUploads::install(memory(), config()).unwrap();
    let active = enroll(&mut store);
    let built = built();
    let mut input = permission(1);
    input.request.object.root = built.hashes().provider_root;
    store.admit(context(4), input, 1).unwrap();
    let manifest = UploadManifest {
        chunks: built.manifest().chunks(),
        headers: &HEADERS,
    };
    for (now, error) in [
        (0, UploadAdmissionError::ClockReversed),
        (100, UploadAdmissionError::Expired),
    ] {
        assert_eq!(
            store.prepare_manifest(context(5), input.request, manifest, now),
            Err(error.into())
        );
    }
    let suspended = store
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: Some(active),
                active: false,
            },
        )
        .unwrap();
    assert_eq!(
        store.prepare_manifest(context(5), input.request, manifest, 2),
        Err(UploadAdmissionError::Tenant(TenantError::Suspended).into())
    );
    store
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: Some(suspended),
                active: true,
            },
        )
        .unwrap();
    assert_eq!(
        store.admit(context(4), input, 2),
        Ok(UploadAdmission::Existing(UploadPhase::Reserved))
    );
    assert_eq!(
        store.prepare_manifest(context(5), input.request, manifest, 2),
        Err(UploadAdmissionError::Tenant(TenantError::StalePermission).into())
    );
    store.revoke(context(4), input.request).unwrap();
    assert_eq!(store.usage().unwrap().reserved_bytes, 0);
    assert_eq!(store.total(p(4)).unwrap().chunks(), 1);
}

#[test]
fn missing_or_substituted_manifest_is_not_repaired_on_reopen() {
    for substitute in [false, true] {
        let m = memory();
        let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
        enroll(&mut store);
        let built = built();
        let mut input = permission(1);
        input.request.object.root = built.hashes().provider_root;
        store.admit(context(4), input, 1).unwrap();
        store
            .prepare_manifest(
                context(5),
                input.request,
                UploadManifest {
                    chunks: built.manifest().chunks(),
                    headers: &HEADERS,
                },
                2,
            )
            .unwrap();
        if substitute {
            store.manifests.insert(
                key(input.request),
                UploadManifestRecord::new(UploadManifest {
                    chunks: built.manifest().chunks(),
                    headers: &[],
                }),
            );
        } else {
            store.manifests.remove(&key(input.request));
        }
        drop(store);
        let before = m.manifests.borrow().clone();
        assert!(matches!(
            StableUploads::open(clone_memory(&m), config()),
            Err(UploadStoreError::InvalidRecord)
        ));
        assert_eq!(*m.manifests.borrow(), before);
    }
}

#[test]
fn codec_envelope_is_checked_before_installation_and_binds_reopening() {
    use ic_memory::ic_stable_structures::Storable;
    use std::num::NonZeroUsize;
    let base = config();
    let mut limits = base.limits();
    limits.max_header_bytes = NonZeroUsize::new(65_536).unwrap();
    let oversized = ServiceConfiguration::new(base.bindings(), limits, base.billing()).unwrap();
    let m = memory();
    assert!(matches!(
        StableUploads::install(clone_memory(&m), oversized),
        Err(UploadStoreError::UnsupportedEnvelope)
    ));
    assert_eq!(m.tenants.size(), 0);
    assert_eq!(m.manifests.size(), 0);
    limits.max_header_bytes = NonZeroUsize::new(64_000).unwrap();
    let wide = ServiceConfiguration::new(base.bindings(), limits, base.billing()).unwrap();
    let mut store = StableUploads::install(clone_memory(&m), wide).unwrap();
    enroll(&mut store);
    let value = "x".repeat(63_950);
    let headers = [
        HEADERS[0],
        CaffeineHeader {
            name: "X",
            value: &value,
        },
    ];
    let mut builder = CaffeineManifestBuilder::new(
        10,
        &headers,
        CaffeineHashLimits {
            max_content_bytes: NonZeroU64::new(10).unwrap(),
            max_append_bytes: NonZeroUsize::new(10).unwrap(),
            max_headers: limits.max_headers,
            max_header_bytes: limits.max_header_bytes,
        },
        NonZeroUsize::MIN,
    )
    .unwrap();
    builder.append(0, &[9; 10]).unwrap();
    let built = builder.finish().unwrap();
    let mut input = permission(1);
    input.request.object.root = built.hashes().provider_root;
    store.admit(context(4), input, 1).unwrap();
    store
        .prepare_manifest(
            context(5),
            input.request,
            UploadManifest {
                chunks: built.manifest().chunks(),
                headers: &headers,
            },
            2,
        )
        .unwrap();
    let record = store.manifests.get(&key(input.request)).unwrap();
    assert_eq!(UploadManifestRecord::from_bytes(record.to_bytes()), record);
    drop(store);
    assert!(matches!(
        StableUploads::open(clone_memory(&m), base),
        Err(UploadStoreError::Binding)
    ));
    let reopened = StableUploads::open(clone_memory(&m), wide).unwrap();
    assert_eq!(reopened.manifests.get(&key(input.request)), Some(record));
}

mod lifecycle;

mod callbacks;
mod download;
mod planning;
mod read;
mod read_authority;
mod read_chunks;
mod read_sessions;
mod reference_receipts;
mod upload_status;
