use super::*;
use crate::workflow::uploads::capacity::inspect;
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityFailure as F;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityResponse;
fn scope() -> TenantScope {
    TenantScope {
        service: p(1),
        tenant: p(4),
        namespace: 1,
    }
}

#[test]
fn capacity_boundary_keeps_full_width_namespace_and_byte_headroom() {
    let original = config();
    let mut bindings = original.bindings();
    bindings.namespace = NonZeroU128::new(u128::MAX).unwrap();
    let mut limits = original.limits();
    limits.catalog.max_physical_bytes = NonZeroU128::new(u128::MAX).unwrap();
    limits.catalog.max_liability_bytes = NonZeroU128::new(u128::MAX).unwrap();
    limits.catalog.max_tenant_logical_bytes = NonZeroU128::new(u128::MAX).unwrap();
    let config = ServiceConfiguration::new(bindings, limits, original.billing()).unwrap();
    let mut store = StableUploads::install(memory(), config).unwrap();
    enroll(&mut store);
    let scope = TenantScope {
        namespace: u128::MAX,
        ..scope()
    };
    let observed = inspect(&store, context(4), scope).unwrap();
    assert_eq!(observed.scope, scope);
    assert_eq!(observed.remaining_bytes, u128::MAX);
    let bytes = candid::encode_one(observed).unwrap();
    assert_eq!(
        candid::decode_one::<UploadCapacityResponse>(&bytes).unwrap(),
        observed
    );
}

#[test]
fn capacity_boundary_checks_scope_actor_and_enrollment_before_disclosing_counters() {
    let store = StableUploads::install(memory(), config()).unwrap();
    assert_eq!(inspect(&store, context(4), scope()), Err(F::NotEnrolled));
    for actor in [2, 5, 6] {
        assert_eq!(inspect(&store, context(actor), scope()), Err(F::Denied));
    }
    for scope in [
        TenantScope {
            service: p(9),
            ..scope()
        },
        TenantScope {
            namespace: 2,
            ..scope()
        },
    ] {
        assert_eq!(inspect(&store, context(4), scope), Err(F::Binding));
    }
    assert_eq!(
        inspect(
            &store,
            UploadContext {
                service: p(9),
                actor: p(4)
            },
            TenantScope {
                service: p(9),
                ..scope()
            }
        ),
        Err(F::Binding)
    );
    assert_eq!(
        inspect(
            &store,
            context(4),
            TenantScope {
                namespace: 0,
                ..scope()
            }
        ),
        Err(F::Invalid)
    );
    for tenant in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            inspect(
                &store,
                UploadContext {
                    actor: tenant,
                    ..context(4)
                },
                TenantScope { tenant, ..scope() }
            ),
            Err(F::Invalid)
        );
    }
}

#[test]
fn capacity_boundary_preserves_heap_parity_and_reports_durable_restore_fence() {
    let memory = memory();
    let mut store = StableUploads::install(clone_memory(&memory), config()).unwrap();
    let mut heap = UploadAdmissions::new(config());
    let enrollment = enroll(&mut store);
    heap.update_tenant(
        context(2),
        TenantUpdate {
            tenant: p(4),
            expected: None,
            active: true,
        },
    )
    .unwrap();
    let permission = permission(1);
    store.admit(context(4), permission, 1).unwrap();
    heap.admit(context(4), permission, 1).unwrap();
    let reserved = inspect(&store, context(4), scope()).unwrap();
    assert_eq!(inspect(&heap, context(4), scope()), Ok(reserved));
    assert!(!reserved.fenced);
    let suspension = TenantUpdate {
        tenant: p(4),
        expected: Some(enrollment),
        active: false,
    };
    store.revoke(context(4), permission.request).unwrap();
    heap.revoke(context(4), permission.request).unwrap();
    store.update_tenant(context(2), suspension).unwrap();
    heap.update_tenant(context(2), suspension).unwrap();
    let cancelled = inspect(&store, context(4), scope()).unwrap();
    assert_eq!(inspect(&heap, context(4), scope()), Ok(cancelled));
    assert!(!cancelled.enrollment.active);
    assert_eq!(cancelled.remaining_objects, reserved.remaining_objects);
    assert_eq!(
        cancelled.remaining_manifest_chunks,
        reserved.remaining_manifest_chunks
    );
    assert_eq!(cancelled.remaining_bytes, reserved.remaining_bytes + 10);
    drop(store);
    let restored = StableUploads::open(memory, config()).unwrap();
    let expected = UploadCapacityResponse {
        fenced: true,
        ..cancelled
    };
    assert_eq!(inspect(&restored, context(4), scope()), Ok(expected));
    let bytes = candid::encode_one(expected).unwrap();
    assert_eq!(
        candid::decode_one::<UploadCapacityResponse>(&bytes).unwrap(),
        expected
    );
}
