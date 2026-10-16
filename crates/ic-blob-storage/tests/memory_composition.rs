//! Native substrate composition only: no blob schema, canister lifecycle or recovery.

use ic_blob_storage::ic_memory as blob_memory;
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_memory::{
    GenericAllocationPolicy, MemoryAllocationPool, MemoryAuthority, MemoryManagerConfig,
    MemoryRequest, MemoryRuntime, RuntimeMemory, RuntimeOpenError, SchemaMetadata,
    SealedDeclarationSnapshot,
    ic_stable_structures::{Cell, VectorMemory},
};

// This host uses ic-memory directly. Blob consumers use the crate's re-export;
// both paths must accept exactly the same runtime memory and collection types.
fn open_blob_cell(
    memory: RuntimeMemory<VectorMemory>,
) -> blob_memory::ic_stable_structures::Cell<u64, blob_memory::RuntimeMemory<VectorMemory>> {
    blob_memory::ic_stable_structures::Cell::init(memory, 0)
}

#[test]
fn content_work_does_not_bootstrap_or_change_the_host_runtime() {
    let backing = VectorMemory::default();
    let host = blob_memory::MemoryRuntime::new_with_config(
        backing.clone(),
        blob_memory::MemoryManagerConfig::new(16).expect("host profile"),
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
    let mut requests: Vec<MemoryRequest> =
        ic_blob_storage::ops::service::installation::requests("blob").expect("Blob requests");
    requests.extend(
        ["fixture.blob.data.v1", "fixture.database.data.v1"].map(|key| {
            MemoryRequest::new("fixture", key, SchemaMetadata::default()).expect("request")
        }),
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
    let mut host = MemoryRuntime::new_with_config(
        VectorMemory::default(),
        MemoryManagerConfig::new(16).expect("host profile"),
    )
    .expect("host runtime");
    assert!(matches!(
        host.open_memory("fixture.blob.data.v1"),
        Err(RuntimeOpenError::NotBootstrapped)
    ));
    host.bootstrap(&declarations, &pool, &GenericAllocationPolicy)
        .expect("host bootstrap");
    host.open_memory(ic_blob_storage::ops::service::installation::INSTALLATION_MEMORY_KEY)
        .expect("public installation request grants host memory");
    let committed = host.committed_allocations().expect("host capability");
    let mut database: Cell<u64, RuntimeMemory<VectorMemory>> = Cell::init(
        host.open_memory("fixture.database.data.v1")
            .expect("database grant"),
        0,
    );
    database.set(91);
    let mut blob = open_blob_cell(
        host.open_memory("fixture.blob.data.v1")
            .expect("blob grant"),
    );
    blob.set(37);
    assert_eq!(*database.get(), 91);
    let reopened = open_blob_cell(
        host.open_memory("fixture.blob.data.v1")
            .expect("same grant"),
    );
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
        host.open_memory("fixture.ungranted.data.v1"),
        Err(RuntimeOpenError::StableKeyNotCommitted(_))
    ));
}
