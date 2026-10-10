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
fn library_linking_and_content_work_do_not_claim_or_bootstrap_memory() {
    assert!(!blob_memory::is_default_memory_manager_bootstrapped().expect("observe"));
    assert_eq!(
        blob_memory::committed_allocations(),
        Err(RuntimeOpenError::NotBootstrapped)
    );
    let before = blob_memory::sealed_declaration_snapshot().expect("linked declarations");
    assert_eq!(before.requests(), []);
    let digest = ContentDigest::compute(b"no stable memory required");
    assert_eq!(digest.to_string().parse::<ContentDigest>(), Ok(digest));
    assert!(!blob_memory::is_default_memory_manager_bootstrapped().expect("observe again"));
    assert_eq!(
        blob_memory::committed_allocations(),
        Err(RuntimeOpenError::NotBootstrapped)
    );
    assert_eq!(
        blob_memory::sealed_declaration_snapshot()
            .expect("same snapshot")
            .requests(),
        []
    );
}

#[test]
fn exported_collections_use_host_grants_and_preserve_the_hosts_bucket_profile() {
    // These keys and IDs are fixture choices, not proposed product allocations.
    let requests = ["fixture.blob.data.v1", "fixture.database.data.v1"]
        .map(|key| MemoryRequest::new("fixture", key, SchemaMetadata::default()).expect("request"));
    let pool = MemoryAllocationPool::new(
        vec![MemoryAuthority::new("fixture", "fixture.").expect("host namespace grant")],
        vec![],
    )
    .expect("host allocation pool");
    let declarations = SealedDeclarationSnapshot::new(&requests).expect("snapshot");
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
            .bucket_size_pages,
        16
    );
    assert!(matches!(
        host.open_memory("fixture.ungranted.data.v1"),
        Err(RuntimeOpenError::StableKeyNotCommitted(_))
    ));
}
