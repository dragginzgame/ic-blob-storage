//! The standalone host owns one ic-storage runtime and fourteen exclusive grants.
use ic_blob_storage::ic_storage::AllocationGrants;
use ic_blob_storage::ic_storage::DefaultMemoryImpl;
use ic_blob_storage::ic_storage::GenericAllocationPolicy;
use ic_blob_storage::ic_storage::Memory as _;
use ic_blob_storage::ic_storage::MemoryAuthority;
use ic_blob_storage::ic_storage::MemoryRuntime;
use ic_blob_storage::ic_storage::RuntimeMemory;
use ic_blob_storage::ic_storage::SealedDeclarationSnapshot;
use ic_blob_storage::ic_storage::StableKey;
use ic_blob_storage::ic_storage::StorageConfig;
use ic_blob_storage::ops::service::installation;
use ic_blob_storage::ops::service::installation::INSTALLATION_MEMORY_KEY;
use ic_blob_storage::ops::service::stores::ServiceMemories;
use ic_blob_storage::ops::service::stores::grants;
pub(super) type Memory = DefaultMemoryImpl;
pub(super) struct Grants {
    pub(super) runtime: MemoryRuntime<DefaultMemoryImpl>,
    pub(super) configuration: RuntimeMemory<Memory>,
    pub(super) stores: ServiceMemories<RuntimeMemory<Memory>>,
}
pub(super) fn open(fresh: bool) -> Grants {
    let backing = DefaultMemoryImpl::default();
    assert_eq!(backing.size() == 0, fresh, "installation memory state");
    let requests = installation::requests("blob").expect("host installation requests");
    let pool = AllocationGrants::new(vec![
        MemoryAuthority::new("blob", "blob.").expect("host namespace grant"),
    ])
    .expect("host allocation pool");
    let declarations = SealedDeclarationSnapshot::new(requests).expect("host declarations");
    let mut runtime = MemoryRuntime::new_with_config(backing, StorageConfig::default())
        .expect("host memory runtime");
    runtime
        .bootstrap(&declarations, &pool, &GenericAllocationPolicy)
        .expect("host memory bootstrap");
    let configuration = runtime
        .open_memory(&StableKey::parse(INSTALLATION_MEMORY_KEY).expect("configuration key"))
        .expect("exclusive configuration grant");
    let stores = grants::open(|key| runtime.open_memory(key)).expect("exclusive service grants");
    Grants {
        runtime,
        configuration,
        stores,
    }
}
