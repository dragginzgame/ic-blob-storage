//! Native substrate composition only: no blob schema, canister lifecycle or recovery.

use ic_blob_storage::ic_storage as blob_memory;
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_storage::VectorMemory;
use ic_storage::adapter::StableCell;
use ic_storage::{
    AllocationGrants, GenericAllocationPolicy, MemoryAuthority, MemoryRequest, MemoryRuntime,
    RuntimeConstructionError, RuntimeOpenError, SealedDeclarationSnapshot, StorageConfig,
};

#[test]
fn content_work_does_not_bootstrap_or_change_the_host_runtime() {
    let backing = VectorMemory::default();
    let host = blob_memory::MemoryRuntime::new_with_config(
        backing.clone(),
        blob_memory::StorageConfig::default(),
    )
    .expect("explicit host runtime");
    let before = backing.borrow().clone();
    let allocations = host.memory_allocations().expect("observe host allocations");
    assert!(!host.is_bootstrapped());
    assert!(matches!(
        host.committed_generation(),
        Err(RuntimeOpenError::NotBootstrapped)
    ));
    let digest = ContentDigest::compute(b"no stable memory required");
    assert_eq!(digest.to_string().parse::<ContentDigest>(), Ok(digest));
    assert!(!host.is_bootstrapped());
    assert!(matches!(
        host.committed_generation(),
        Err(RuntimeOpenError::NotBootstrapped)
    ));
    assert_eq!(
        host.memory_allocations().expect("observe again"),
        allocations
    );
    assert_eq!(*backing.borrow(), before);
}

#[test]
fn exported_collections_use_host_grants_and_preserve_the_hosts_limits() {
    // These keys and IDs are fixture choices, not proposed product allocations.
    // The public factory must return the host's exact Memory dependency types.
    let blob_key = ic_storage::StableKey::parse("fixture.blob.data.v1").unwrap();
    let database_key = ic_storage::StableKey::parse("fixture.database.data.v1").unwrap();
    let ungranted_key = ic_storage::StableKey::parse("fixture.ungranted.data.v1").unwrap();
    let mut requests: Vec<MemoryRequest> =
        ic_blob_storage::ops::service::installation::requests("blob").expect("Blob requests");
    requests.extend(
        ["fixture.blob.data.v1", "fixture.database.data.v1"]
            .map(|key| MemoryRequest::new("fixture", key).expect("request")),
    );
    let pool = AllocationGrants::new(vec![
        MemoryAuthority::new("blob", "blob.").expect("Blob namespace grant"),
        MemoryAuthority::new("fixture", "fixture.").expect("host namespace grant"),
    ])
    .expect("host allocation pool");
    let declarations = SealedDeclarationSnapshot::new(requests).expect("snapshot");
    let backing = VectorMemory::default();
    let mut host = MemoryRuntime::new_with_config(backing.clone(), StorageConfig::default())
        .expect("host runtime");
    assert!(matches!(
        host.open_memory(&blob_key),
        Err(RuntimeOpenError::NotBootstrapped)
    ));
    host.bootstrap(&declarations, &pool, &GenericAllocationPolicy)
        .expect("host bootstrap");
    host.open_memory(
        &ic_storage::StableKey::parse(
            ic_blob_storage::ops::service::installation::INSTALLATION_MEMORY_KEY,
        )
        .unwrap(),
    )
    .expect("public installation request grants host memory");
    let committed: Vec<_> = host
        .memory_allocations()
        .unwrap()
        .into_iter()
        .map(|a| (a.stable_key, a.id))
        .collect();
    let mut database: StableCell<u64, VectorMemory> =
        StableCell::create(host.open_memory(&database_key).expect("database grant"), 0).unwrap();
    database.set(91).unwrap();
    // Direct Memory grants and Blob's public Memory re-export have one identity.
    let memory: blob_memory::RuntimeMemory<VectorMemory> =
        host.open_memory(&blob_key).expect("blob grant");
    let mut blob = StableCell::create(memory, 0_u64).unwrap();
    blob.set(37).unwrap();
    assert_eq!(*database.get(), 91);
    assert!(matches!(
        host.open_memory(&blob_key),
        Err(RuntimeOpenError::AlreadyOpen { .. })
    ));
    drop(blob);
    let reopened =
        StableCell::<u64, _>::load(host.open_memory(&blob_key).expect("same grant"), 8).unwrap();
    assert_eq!(*reopened.get(), 37);
    assert_eq!(
        host.memory_allocations()
            .unwrap()
            .into_iter()
            .map(|a| (a.stable_key, a.id))
            .collect::<Vec<_>>(),
        committed
    );
    assert_eq!(host.config().pages, StorageConfig::default().pages);
    assert!(matches!(
        host.open_memory(&ungranted_key),
        Err(RuntimeOpenError::NotAdmitted(_))
    ));
    // A composed host must not replace its runtime while either component still
    // owns a collection, even after the runtime itself is dropped.
    drop(host);
    let before = backing.borrow().clone();
    assert!(matches!(
        MemoryRuntime::new(backing.clone()),
        Err(RuntimeConstructionError::BackingAlreadyClaimed)
    ));
    drop(database);
    assert!(matches!(
        MemoryRuntime::new(backing.clone()),
        Err(RuntimeConstructionError::BackingAlreadyClaimed)
    ));
    assert_eq!(*backing.borrow(), before);
    drop(reopened);
    let mut restored = MemoryRuntime::new(backing).expect("all component leases released");
    restored
        .bootstrap(&declarations, &pool, &GenericAllocationPolicy)
        .expect("same declaration and retained ledger");
    assert_eq!(
        *StableCell::<u64, _>::load(restored.open_memory(&blob_key).unwrap(), 8)
            .unwrap()
            .get(),
        37
    );
    let database: StableCell<u64, VectorMemory> =
        StableCell::load(restored.open_memory(&database_key).unwrap(), 8).unwrap();
    assert_eq!(*database.get(), 91);
}
