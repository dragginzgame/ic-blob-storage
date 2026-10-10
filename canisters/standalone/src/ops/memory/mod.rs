//! The standalone host owns one ic-memory runtime and seventeen exclusive grants.
use ic_blob_storage::ic_memory::GenericAllocationPolicy;
use ic_blob_storage::ic_memory::MemoryAllocationPool;
use ic_blob_storage::ic_memory::MemoryAuthority;
use ic_blob_storage::ic_memory::MemoryManagerConfig;
use ic_blob_storage::ic_memory::MemoryRuntime;
use ic_blob_storage::ic_memory::RuntimeMemory;
use ic_blob_storage::ic_memory::SealedDeclarationSnapshot;
use ic_blob_storage::ic_memory::ic_stable_structures::DefaultMemoryImpl;
use ic_blob_storage::ic_memory::ic_stable_structures::Memory as _;
use ic_blob_storage::ops::service::installation;
use ic_blob_storage::ops::service::installation::INSTALLATION_MEMORY_KEY;
use ic_blob_storage::ops::service::stores::ServiceMemories;
use ic_blob_storage::ops::service::stores::grants;
pub(super) type Memory = RuntimeMemory<DefaultMemoryImpl>;
pub(super) struct Grants {
    pub(super) runtime: MemoryRuntime<DefaultMemoryImpl>,
    pub(super) configuration: Memory,
    pub(super) stores: ServiceMemories<Memory>,
}
pub(super) fn open(fresh: bool) -> Grants {
    let backing = DefaultMemoryImpl::default();
    assert_eq!(backing.size() == 0, fresh, "installation memory state");
    let requests = installation::requests("blob").expect("host installation requests");
    let pool = MemoryAllocationPool::new(
        vec![MemoryAuthority::new("blob", "blob.").expect("host namespace grant")],
        vec![],
    )
    .expect("host allocation pool");
    let declarations = SealedDeclarationSnapshot::new(&requests).expect("host declarations");
    let mut runtime = MemoryRuntime::new_with_config(
        backing,
        MemoryManagerConfig::new(16).expect("host bucket policy"),
    )
    .expect("host memory runtime");
    runtime
        .bootstrap(&declarations, &pool, &GenericAllocationPolicy)
        .expect("host memory bootstrap");
    let configuration = runtime
        .open_memory(INSTALLATION_MEMORY_KEY)
        .expect("exclusive configuration grant");
    let stores = grants::open(|key| runtime.open_memory(key)).expect("exclusive service grants");
    Grants {
        runtime,
        configuration,
        stores,
    }
}
