use super::*;
use crate::model::service::upload::UploadAdmissions;
use ic_blob_storage_contracts::configuration::billing::BillingConfiguration;
use ic_blob_storage_contracts::configuration::funding::FundingLimits;
use ic_blob_storage_contracts::configuration::limits::CatalogLimits;
use ic_blob_storage_contracts::configuration::limits::UploadLimits;
use ic_blob_storage_contracts::configuration::service::ServiceBindings;
use ic_blob_storage_contracts::configuration::service::ServiceLimits;
use ic_blob_storage_contracts::configuration::service::ServiceManifestLimits;
use ic_memory::{
    GenericRangePolicy, MemoryManagerAuthorityRecord, MemoryManagerConfig, MemoryManagerIdRange,
    MemoryManagerRangeMode, MemoryRequest, MemoryRuntime, SchemaMetadata,
    SealedDeclarationSnapshot, StaticMemoryRangeDeclaration, ic_stable_structures::VectorMemory,
};
use std::num::{NonZeroU64, NonZeroU128, NonZeroUsize};

fn p(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}
fn context(actor: u8) -> UploadContext {
    UploadContext {
        service: p(1),
        actor: p(actor),
    }
}
pub(in crate::ops::service) fn config() -> ServiceConfiguration {
    ServiceConfiguration::new(
        ServiceBindings {
            service: p(1),
            operator: p(2),
            payment_account: p(1),
            namespace: NonZeroU128::MIN,
        },
        ServiceLimits {
            max_tenants: NonZeroUsize::new(2).unwrap(),
            max_object_bytes: NonZeroU64::new(10).unwrap(),
            max_headers: NonZeroUsize::new(8).unwrap(),
            max_header_bytes: NonZeroUsize::new(1024).unwrap(),
            manifests: ServiceManifestLimits {
                max_chunks: NonZeroUsize::new(4).unwrap(),
                max_tenant_chunks: NonZeroUsize::new(4).unwrap(),
            },
            catalog: CatalogLimits {
                max_objects: NonZeroUsize::new(2).unwrap(),
                max_tenant_objects: NonZeroUsize::new(2).unwrap(),
                max_physical_bytes: NonZeroU128::new(20).unwrap(),
                max_liability_bytes: NonZeroU128::new(20).unwrap(),
                max_tenant_logical_bytes: NonZeroU128::new(20).unwrap(),
                max_references_per_object: NonZeroUsize::new(2).unwrap(),
                max_receipts_per_object: NonZeroUsize::new(3).unwrap(),
            },
            uploads: UploadLimits {
                max_active: NonZeroUsize::new(2).unwrap(),
                max_tenant_active: NonZeroUsize::new(2).unwrap(),
            },
        },
        BillingConfiguration::new(p(3), FundingLimits::new(1, 10, 100).unwrap(), 8, 4).unwrap(),
    )
    .unwrap()
}
fn update(tenant: u8, expected: Option<TenantEnrollmentView>, active: bool) -> TenantUpdate {
    TenantUpdate {
        tenant: p(tenant),
        expected,
        active,
    }
}

#[test]
fn host_granted_memory_reopens_with_retained_state_and_an_enforced_fence() {
    let request =
        MemoryRequest::new("fixture", "fixture.tenants.v1", SchemaMetadata::default()).unwrap();
    let grant = StaticMemoryRangeDeclaration::new(
        MemoryManagerAuthorityRecord::new(
            MemoryManagerIdRange::new(120, 120).unwrap(),
            "fixture",
            MemoryManagerRangeMode::Allowed,
            None,
        )
        .unwrap(),
    )
    .unwrap();
    let declarations = SealedDeclarationSnapshot::new(&[], &[grant], &[request]).unwrap();
    let raw = VectorMemory::default();
    let mut runtime =
        MemoryRuntime::new_with_config(raw.clone(), MemoryManagerConfig::new(16).unwrap()).unwrap();
    runtime
        .bootstrap(&declarations, &GenericRangePolicy)
        .unwrap();
    let memory = runtime.open_memory_by_key("fixture.tenants.v1").unwrap();
    let mut store = StableTenantEnrollments::install(memory, config()).unwrap();
    let active = store.update(context(2), update(4, None, true)).unwrap();
    let suspended = store
        .update(context(2), update(4, Some(active), false))
        .unwrap();
    drop(store);
    drop(runtime);
    let mut runtime =
        MemoryRuntime::new_with_config(raw, MemoryManagerConfig::new(16).unwrap()).unwrap();
    runtime
        .bootstrap(&declarations, &GenericRangePolicy)
        .unwrap();
    let memory = runtime.open_memory_by_key("fixture.tenants.v1").unwrap();
    let mut store = StableTenantEnrollments::open(memory, config()).unwrap();
    assert!(store.is_fenced());
    assert_eq!(store.inspect(context(4), p(4)), Ok(Some(suspended)));
    assert_eq!(
        store.update(context(2), update(4, Some(suspended), true)),
        Err(TenantStoreError::Fenced)
    );
    assert_eq!(store.inspect(context(2), p(4)), Ok(Some(suspended)));
    assert_eq!(
        store.inspect(context(5), p(4)),
        Err(TenantStoreError::NotObserver)
    );
}

#[test]
fn stable_and_heap_owners_share_compare_and_set_generation_and_capacity_rules() {
    let mut heap = UploadAdmissions::new(config());
    let mut stable = StableTenantEnrollments::install(VectorMemory::default(), config()).unwrap();
    for (tenant, active) in [
        (4, true),
        (4, false),
        (4, true),
        (5, false),
        (6, true),
        (5, true),
    ] {
        let current = heap.tenant(context(2), p(tenant)).unwrap();
        let command = update(tenant, current, active);
        let expected = heap.update_tenant(context(2), command);
        let actual = stable.update(context(2), command);
        match expected {
            Ok(view) => assert_eq!(actual, Ok(view)),
            Err(crate::model::service::upload::UploadAdmissionError::Tenant(error)) => {
                assert_eq!(actual, Err(TenantStoreError::Tenant(error)));
            }
            other => panic!("unexpected heap result: {other:?}"),
        }
        assert_eq!(
            stable.inspect(context(2), p(tenant)).unwrap(),
            heap.tenant(context(2), p(tenant)).unwrap()
        );
    }
    assert_eq!(
        stable.update(context(2), update(4, None, true)),
        Err(TenantError::Conflict.into())
    );
    for tenant in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            stable.update(
                context(2),
                TenantUpdate {
                    tenant,
                    expected: None,
                    active: true
                }
            ),
            Err(TenantError::InvalidTenant.into())
        );
    }
}

#[test]
fn rejected_authority_never_changes_stable_bytes() {
    let memory = VectorMemory::default();
    let mut store = StableTenantEnrollments::install(memory.clone(), config()).unwrap();
    let before = memory.borrow().clone();
    assert_eq!(
        store.update(context(4), update(4, None, true)),
        Err(TenantStoreError::NotOperator)
    );
    let wrong = UploadContext {
        service: p(9),
        ..context(2)
    };
    assert_eq!(
        store.update(wrong, update(4, None, true)),
        Err(TenantStoreError::WrongService)
    );
    assert_eq!(
        store.inspect(wrong, p(4)),
        Err(TenantStoreError::WrongService)
    );
    assert_eq!(*memory.borrow(), before);
}

#[test]
fn missing_or_mismatched_storage_is_never_initialized_or_rebound() {
    let memory = VectorMemory::default();
    assert!(matches!(
        StableTenantEnrollments::open(memory.clone(), config()),
        Err(TenantStoreError::Missing)
    ));
    assert_eq!(memory.size(), 0);
    let store = StableTenantEnrollments::install(memory.clone(), config()).unwrap();
    drop(store);
    let before = memory.borrow().clone();
    assert!(matches!(
        StableTenantEnrollments::install(memory.clone(), config()),
        Err(TenantStoreError::AlreadyAllocated)
    ));
    for field in 0..4 {
        let original = config();
        let mut bindings = original.bindings();
        let mut limits = original.limits();
        match field {
            0 => bindings.service = p(9),
            1 => bindings.operator = p(9),
            2 => bindings.namespace = NonZeroU128::new(2).unwrap(),
            _ => limits.max_tenants = NonZeroUsize::new(3).unwrap(),
        }
        let changed = ServiceConfiguration::new(bindings, limits, original.billing()).unwrap();
        assert!(matches!(
            StableTenantEnrollments::open(memory.clone(), changed),
            Err(TenantStoreError::Binding)
        ));
        assert_eq!(*memory.borrow(), before);
    }
}

#[test]
fn invalid_records_and_over_capacity_state_fail_restoration_without_repair() {
    for invalid_tenant in [p(4), Principal::anonymous()] {
        let memory = VectorMemory::default();
        let mut store = StableTenantEnrollments::install(memory.clone(), config()).unwrap();
        store.entries.insert(
            invalid_tenant,
            TenantStoreRecord::Metadata(TenantStoreMetadataRecord::new(&config())),
        );
        drop(store);
        let before = memory.borrow().clone();
        assert!(matches!(
            StableTenantEnrollments::open(memory.clone(), config()),
            Err(TenantStoreError::InvalidRecord)
        ));
        assert_eq!(*memory.borrow(), before);
    }
    let memory = VectorMemory::default();
    let mut store = StableTenantEnrollments::install(memory.clone(), config()).unwrap();
    for tenant in [4, 5, 6] {
        store.entries.insert(
            p(tenant),
            TenantStoreRecord::Enrollment(TenantEnrollmentRecord::new(TenantEnrollmentView {
                generation: NonZeroU64::MIN,
                active: true,
            })),
        );
    }
    drop(store);
    assert!(matches!(
        StableTenantEnrollments::open(memory, config()),
        Err(TenantStoreError::InvalidRecord)
    ));
}

#[test]
fn exhausted_generations_preserve_suspension_and_stable_history() {
    let memory = VectorMemory::default();
    let mut store = StableTenantEnrollments::install(memory.clone(), config()).unwrap();
    let last = TenantEnrollmentView {
        generation: NonZeroU64::MAX,
        active: true,
    };
    store.entries.insert(
        p(4),
        TenantStoreRecord::Enrollment(TenantEnrollmentRecord::new(last)),
    );
    let suspended = store
        .update(context(2), update(4, Some(last), false))
        .unwrap();
    let before = memory.borrow().clone();
    assert_eq!(
        store.update(context(2), update(4, Some(suspended), true)),
        Err(TenantError::GenerationExhausted.into())
    );
    assert_eq!(*memory.borrow(), before);
    drop(store);
    let reopened = StableTenantEnrollments::open(memory, config()).unwrap();
    assert_eq!(reopened.inspect(context(4), p(4)), Ok(Some(suspended)));
}

#[test]
fn unrecognized_memory_and_missing_metadata_never_seed_a_fresh_store() {
    let memory = VectorMemory::default();
    memory.grow(1);
    memory.write(0, b"unrecognized stable data");
    let before = memory.borrow().clone();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        StableTenantEnrollments::open(memory.clone(), config())
    }));
    assert!(result.is_err());
    assert_eq!(*memory.borrow(), before);
    assert!(matches!(
        StableTenantEnrollments::install(memory, config()),
        Err(TenantStoreError::AlreadyAllocated)
    ));
    let memory = VectorMemory::default();
    let mut store = StableTenantEnrollments::install(memory.clone(), config()).unwrap();
    store.entries.remove(&Principal::management_canister());
    drop(store);
    let before = memory.borrow().clone();
    assert!(matches!(
        StableTenantEnrollments::open(memory.clone(), config()),
        Err(TenantStoreError::Binding)
    ));
    assert_eq!(*memory.borrow(), before);
}
