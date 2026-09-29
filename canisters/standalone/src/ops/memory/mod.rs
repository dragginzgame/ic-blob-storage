//! The standalone host owns one ic-memory runtime and seventeen exclusive grants.
use ic_blob_storage::{
    ic_memory::{
        GenericRangePolicy, MemoryManagerAuthorityRecord, MemoryManagerConfig,
        MemoryManagerIdRange, MemoryManagerRangeMode, MemoryRequest, MemoryRuntime, RuntimeMemory,
        SchemaMetadata, SealedDeclarationSnapshot, StaticMemoryRangeDeclaration,
        ic_stable_structures::{DefaultMemoryImpl, Memory as _},
    },
    ops::service::{
        funding::FundingMemories, reads::ReadSessionMemories, stores::ServiceMemories,
        uploads::UploadMemories,
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
    let keys = [
        "blob.configuration.v1",
        "blob.tenants.v1",
        "blob.roots.v1",
        "blob.root_objects.v1",
        "blob.permissions.v1",
        "blob.usage.v1",
        "blob.manifests.v1",
        "blob.confirmed.v1",
        "blob.references.v1",
        "blob.receipts.v1",
        "blob.root_requests.v1",
        "blob.funding_accounting.v1",
        "blob.funding_intents.v1",
        "blob.gateways.v1",
        "blob.read_journal.v1",
        "blob.read_sessions.v1",
        "blob.read_tenants.v1",
    ];
    let requests = keys.map(|key| {
        MemoryRequest::new("blob", key, SchemaMetadata::default()).expect("host request")
    });
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
    let [
        configuration,
        tenants,
        roots,
        root_objects,
        permissions,
        usage,
        manifests,
        confirmed,
        references,
        receipts,
        root_requests,
        funding_accounting,
        funding_intents,
        gateways,
        read_journal,
        read_sessions,
        read_tenants,
    ] = keys.map(|key| {
        runtime
            .open_memory_by_key(key)
            .expect("exclusive host grant")
    });
    Grants {
        runtime,
        configuration,
        stores: ServiceMemories {
            uploads: UploadMemories {
                tenants,
                roots,
                objects: root_objects,
                permissions,
                usage,
                manifests,
                confirmed,
                references,
                receipts,
                root_requests,
            },
            funding: FundingMemories {
                accounting: funding_accounting,
                intents: funding_intents,
            },
            gateways,
            reads: ReadSessionMemories {
                journal: read_journal,
                sessions: read_sessions,
                tenants: read_tenants,
            },
        },
    }
}
