//! The standalone host owns one ic-memory runtime and seventeen exclusive grants.
use ic_blob_storage::{
    ic_memory::{
        GenericRangePolicy, MemoryManagerAuthorityRecord, MemoryManagerConfig,
        MemoryManagerIdRange, MemoryManagerRangeMode, MemoryRequest, MemoryRuntime, RuntimeMemory,
        SchemaMetadata, SealedDeclarationSnapshot, StaticMemoryRangeDeclaration,
        ic_stable_structures::{DefaultMemoryImpl, Memory as _},
    },
    ops::service::{
        installation::INSTALLATION_MEMORY_KEY,
        stores::{ServiceMemories, grants},
    },
};
pub(super) type Memory = RuntimeMemory<DefaultMemoryImpl>;
pub(super) struct Grants {
    pub(super) runtime: MemoryRuntime<DefaultMemoryImpl>,
    pub(super) configuration: Memory,
    pub(super) stores: ServiceMemories<Memory>,
}
pub(super) fn open(fresh: bool) -> Grants {
    let backing = DefaultMemoryImpl::default();
    assert_eq!(backing.size() == 0, fresh, "installation memory state");
    let mut requests = vec![
        MemoryRequest::new("blob", INSTALLATION_MEMORY_KEY, SchemaMetadata::default())
            .expect("host configuration request"),
    ];
    requests.extend(grants::requests("blob").expect("host service requests"));
    let grant = StaticMemoryRangeDeclaration::new(
        MemoryManagerAuthorityRecord::new(
            MemoryManagerIdRange::new(120, 136).expect("host range"),
            "blob",
            MemoryManagerRangeMode::Allowed,
            None,
        )
        .expect("host authority"),
    )
    .expect("host declaration");
    let declarations =
        SealedDeclarationSnapshot::new(&[], &[grant], &requests).expect("host declarations");
    let mut runtime = MemoryRuntime::new_with_config(
        backing,
        MemoryManagerConfig::new(16).expect("host bucket policy"),
    )
    .expect("host memory runtime");
    runtime
        .bootstrap(&declarations, &GenericRangePolicy)
        .expect("host memory bootstrap");
    let configuration = runtime
        .open_memory_by_key(INSTALLATION_MEMORY_KEY)
        .expect("exclusive configuration grant");
    let stores =
        grants::open(|key| runtime.open_memory_by_key(key)).expect("exclusive service grants");
    Grants {
        runtime,
        configuration,
        stores,
    }
}
