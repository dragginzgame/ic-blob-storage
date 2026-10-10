use super::*;
use crate::model::billing::journal::FundingIntent;
use crate::ops::service::configuration::tests::candidate;
use crate::ops::service::configuration::validate_candidate;
use crate::ops::service::stores::ServiceStores;
use crate::workflow::operator::inspect;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_blob_storage_contracts::tenant::TenantUpdate;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_memory::{
    GenericAllocationPolicy, MemoryAllocationPool, MemoryAuthority, MemoryManagerConfig,
    MemoryManagerIdRange, MemoryRuntime, SealedDeclarationSnapshot,
    ic_stable_structures::VectorMemory,
};

fn pool(first: u8) -> MemoryAllocationPool {
    MemoryAllocationPool::new(
        vec![
            MemoryAuthority::new("test-blob", "blob.").unwrap(),
            MemoryAuthority::new("application", "application.").unwrap(),
        ],
        vec![
            MemoryManagerIdRange::new(0, first - 1).unwrap(),
            MemoryManagerIdRange::new(first + 17, 254).unwrap(),
        ],
    )
    .unwrap()
}
fn snapshot(missing: Option<usize>) -> SealedDeclarationSnapshot {
    let mut service = requests("test-blob").unwrap();
    if let Some(index) = missing {
        service.remove(index);
    }
    service.push(
        MemoryRequest::new(
            "application",
            "application.settings.v1",
            SchemaMetadata::default(),
        )
        .unwrap(),
    );
    SealedDeclarationSnapshot::new(&service).unwrap()
}
fn runtime(backing: VectorMemory) -> MemoryRuntime<VectorMemory> {
    MemoryRuntime::new_with_config(backing, MemoryManagerConfig::new(1).unwrap()).unwrap()
}

#[test]
fn opening_without_committed_grants_preserves_backing_memory() {
    let backing = VectorMemory::default();
    let runtime = runtime(backing.clone());
    let before = backing.borrow().clone();
    assert!(matches!(
        open(|key| runtime.open_memory(key)),
        Err(RuntimeOpenError::NotBootstrapped)
    ));
    assert_eq!(*backing.borrow(), before);
}

#[test]
fn host_namespace_mismatch_preserves_existing_allocations() {
    let backing = VectorMemory::default();
    let mut host = runtime(backing.clone());
    let declarations = snapshot(None);
    host.bootstrap(&declarations, &pool(40), &GenericAllocationPolicy)
        .unwrap();
    let neighbor = host.open_memory("application.settings.v1").unwrap();
    neighbor.grow(1).unwrap();
    neighbor.write(0, b"retained neighbor");
    drop(neighbor);
    drop(host);
    let mut host = runtime(backing.clone());
    let wrong_owner = MemoryAllocationPool::new(
        vec![
            MemoryAuthority::new("another-service", "blob.").unwrap(),
            MemoryAuthority::new("application", "application.").unwrap(),
        ],
        vec![],
    )
    .unwrap();
    let before = backing.borrow().clone();
    assert!(matches!(
        host.bootstrap(&declarations, &wrong_owner, &GenericAllocationPolicy),
        Err(ic_memory::RuntimeBootstrapError::Resolution(
            ic_memory::MemoryResolutionError::Pool(
                ic_memory::MemoryAllocationPoolError::AuthorityMismatch { .. }
            )
        ))
    ));
    assert!(matches!(
        open(|key| host.open_memory(key)),
        Err(RuntimeOpenError::NotBootstrapped)
    ));
    assert!(
        *backing.borrow() == before,
        "rejected ownership must retain bytes"
    );
    host.bootstrap(&declarations, &pool(40), &GenericAllocationPolicy)
        .unwrap();
    let mut retained = [0; 17];
    host.open_memory("application.settings.v1")
        .unwrap()
        .read(0, &mut retained);
    assert_eq!(&retained, b"retained neighbor");
}

#[test]
fn default_lookup_requires_existing_host_bootstrap() {
    // A fresh thread has no default runtime; do not initialize one for inspection.
    std::thread::spawn(|| {
        assert!(matches!(
            open_default(),
            Err(RuntimeOpenError::NotBootstrapped)
        ));
        assert!(!ic_memory::is_default_memory_manager_bootstrapped().unwrap());
        assert!(matches!(
            ic_memory::default_memory_manager_memory_allocations(),
            Err(ic_memory::RuntimeDiagnosticError::NotBootstrapped)
        ));
    })
    .join()
    .unwrap();
}

#[test]
fn incomplete_composed_grants_reject_without_allocating_or_repairing() {
    let backing = VectorMemory::default();
    let mut runtime = runtime(backing.clone());
    let declarations = snapshot(Some(15));
    runtime
        .bootstrap(&declarations, &pool(40), &GenericAllocationPolicy)
        .unwrap();
    let neighbor = runtime.open_memory("application.settings.v1").unwrap();
    neighbor.grow(1).expect("neighbor memory growth");
    neighbor.write(0, b"application-owned data");
    let before = backing.borrow().clone();
    assert!(matches!(
        open(|key| runtime.open_memory(key)),
        Err(RuntimeOpenError::StableKeyNotCommitted(key)) if key == "blob.read_tenants.v1"
    ));
    assert_eq!(*backing.borrow(), before);
    for request in requests("test-blob").unwrap().iter().take(15) {
        assert_eq!(
            runtime
                .open_memory(request.stable_key().as_str())
                .unwrap()
                .size(),
            0
        );
    }
}

#[test]
fn shared_grants_preserve_populated_owners_and_neighbor_under_host_selected_placement() {
    // Physical placement is host-owned. Both layouts use the same logical mapping.
    for first in [40, 120] {
        composed_journey(first);
    }
}
fn composed_journey(first: u8) {
    let backing = VectorMemory::default();
    let declarations = snapshot(None);
    let mut host = runtime(backing.clone());
    host.bootstrap(&declarations, &pool(first), &GenericAllocationPolicy)
        .unwrap();
    let neighbor = host.open_memory("application.settings.v1").unwrap();
    assert_eq!(neighbor.grow(1), Ok(0));
    neighbor.write(0, b"application-owned data");

    let input = candidate();
    let config = validate_candidate(input.service, input).unwrap();
    let context = UploadContext {
        service: input.service,
        actor: input.operator,
    };
    let scope = OperatorScope {
        service: input.service,
        namespace: input.namespace,
        cashier: input.billing.cashier,
        payment_account: input.payment_account,
    };
    let mut stores =
        ServiceStores::install(open(|key| host.open_memory(key)).unwrap(), config).unwrap();
    let enrollment = stores
        .uploads
        .update_tenant(
            context,
            TenantUpdate {
                tenant: input.payment_account,
                expected: None,
                active: true,
            },
        )
        .unwrap();
    let intent = FundingIntent {
        service: input.service,
        namespace: input.namespace.try_into().unwrap(),
        cashier: input.billing.cashier,
        account: input.payment_account,
        operation: u128::MAX.try_into().unwrap(),
        offered: 10.try_into().unwrap(),
        target_balance: None,
    };
    stores.funding.prepare(context, intent).unwrap();
    let before = inspect((&stores).into(), context, scope).unwrap();
    assert_eq!(before.funding.last_operation, Some(u128::MAX));
    assert_eq!(before.funding.reserved_or_uncertain, 10);
    drop(stores);
    drop(host);

    let mut host = runtime(backing.clone());
    host.bootstrap(&declarations, &pool(first), &GenericAllocationPolicy)
        .unwrap();
    let bytes = backing.borrow().clone();
    let restored = ServiceStores::open(open(|key| host.open_memory(key)).unwrap(), config).unwrap();
    let mut expected = before;
    expected.uploads.fenced = true;
    expected.funding.fenced = true;
    expected.gateways.fenced = true;
    expected.reads.fenced = true;
    assert_eq!(
        inspect((&restored).into(), context, scope).unwrap(),
        expected
    );
    assert_eq!(
        restored.uploads.tenant(context, input.payment_account),
        Ok(Some(enrollment))
    );
    let mut retained = [0; 22];
    host.open_memory("application.settings.v1")
        .unwrap()
        .read(0, &mut retained);
    assert_eq!(&retained, b"application-owned data");
    assert_eq!(*backing.borrow(), bytes);
}
