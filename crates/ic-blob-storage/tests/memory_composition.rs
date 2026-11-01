//! Native substrate composition only: no blob schema, canister lifecycle or recovery.

use ic_blob_storage::ic_memory as blob_memory;
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_memory::VectorMemory;
use ic_memory::{
    GenericAllocationPolicy, MemoryAllocationPool, MemoryAuthority, MemoryManagerConfig,
    MemoryRequest, MemoryRuntime, RuntimeConstructionError, RuntimeOpenError,
    SealedDeclarationSnapshot,
};
use ic_storage::adapter::StableCell;

#[test]
fn content_work_does_not_bootstrap_or_change_the_host_runtime() {
    let backing = VectorMemory::default();
    let host = blob_memory::MemoryRuntime::new_with_config(
        backing.clone(),
        blob_memory::MemoryManagerConfig::new(
            std::num::NonZeroU16::new(16).expect("nonzero bucket pages"),
        ),
    )
    .expect("explicit host runtime");
    let before = backing.borrow().clone();
    let allocations = host.memory_allocations().expect("observe host allocations");
    assert!(!host.is_bootstrapped());
    assert_eq!(
        host.committed_allocations(),
        Err(RuntimeOpenError::NotBootstrapped)
    );
    let digest = ContentDigest::compute(b"no stable memory required");
    assert_eq!(digest.to_string().parse::<ContentDigest>(), Ok(digest));
    assert!(!host.is_bootstrapped());
    assert_eq!(
        host.committed_allocations(),
        Err(RuntimeOpenError::NotBootstrapped)
    );
    assert_eq!(
        host.memory_allocations().expect("observe again"),
        allocations
    );
    assert_eq!(*backing.borrow(), before);
}

#[test]
fn exported_collections_use_host_grants_and_preserve_the_hosts_bucket_profile() {
    // These keys and IDs are fixture choices, not proposed product allocations.
    // The public factory must return the host's exact Memory dependency types.
    let blob_key = ic_memory::StableKey::parse("fixture.blob.data.v1").unwrap();
    let database_key = ic_memory::StableKey::parse("fixture.database.data.v1").unwrap();
    let ungranted_key = ic_memory::StableKey::parse("fixture.ungranted.data.v1").unwrap();
    let mut requests: Vec<MemoryRequest> =
        ic_blob_storage::ops::service::installation::requests("blob").expect("Blob requests");
    requests.extend(
        ["fixture.blob.data.v1", "fixture.database.data.v1"]
            .map(|key| MemoryRequest::new("fixture", key).expect("request")),
    );
    let pool = MemoryAllocationPool::new(
        vec![
            MemoryAuthority::new("blob", "blob.").expect("Blob namespace grant"),
            MemoryAuthority::new("fixture", "fixture.").expect("host namespace grant"),
        ],
        vec![],
    )
    .expect("host allocation pool");
    let declarations = SealedDeclarationSnapshot::new(requests).expect("snapshot");
    let backing = VectorMemory::default();
    let mut host = MemoryRuntime::new_with_config(
        backing.clone(),
        MemoryManagerConfig::new(std::num::NonZeroU16::new(16).expect("nonzero bucket pages")),
    )
    .expect("host runtime");
    assert!(matches!(
        host.open_memory(&blob_key),
        Err(RuntimeOpenError::NotBootstrapped)
    ));
    host.bootstrap(&declarations, &pool, &GenericAllocationPolicy)
        .expect("host bootstrap");
    host.open_memory(
        &ic_memory::StableKey::parse(
            ic_blob_storage::ops::service::installation::INSTALLATION_MEMORY_KEY,
        )
        .unwrap(),
    )
    .expect("public installation request grants host memory");
    let committed = host.committed_allocations().expect("host capability");
    let mut database: StableCell<u64, VectorMemory> =
        StableCell::create(host.open_memory(&database_key).expect("database grant"), 0);
    database.set(91);
    // Direct Memory grants and Blob's public Memory re-export have one identity.
    let memory: blob_memory::RuntimeMemory<VectorMemory> =
        host.open_memory(&blob_key).expect("blob grant");
    let mut blob = StableCell::create(memory, 0_u64);
    blob.set(37);
    assert_eq!(*database.get(), 91);
    assert!(matches!(
        host.open_memory(&blob_key),
        Err(RuntimeOpenError::AlreadyOpen { .. })
    ));
    drop(blob);
    let reopened =
        StableCell::<u64, _>::load(host.open_memory(&blob_key).expect("same grant")).unwrap();
    assert_eq!(*reopened.get(), 37);
    assert_eq!(
        host.committed_allocations().expect("unchanged capability"),
        committed
    );
    assert_eq!(
        host.memory_allocations()
            .expect("host profile preserved")
            .summary
            .bucket_size_pages,
        16
    );
    assert!(matches!(
        host.open_memory(&ungranted_key),
        Err(RuntimeOpenError::StableKeyNotCommitted(_))
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
        *StableCell::<u64, _>::load(restored.open_memory(&blob_key).unwrap())
            .unwrap()
            .get(),
        37
    );
    let database: StableCell<u64, VectorMemory> =
        StableCell::load(restored.open_memory(&database_key).unwrap()).unwrap();
    assert_eq!(*database.get(), 91);
}
