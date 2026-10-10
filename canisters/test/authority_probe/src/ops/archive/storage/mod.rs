//! Host-owned memory for this experiment only. No product schema or migration.

use std::cell::RefCell;

use candid::{de::DecoderConfig, decode_one_with_config};
use ic_blob_storage::ic_memory::{
    GenericAllocationPolicy, MemoryAllocationPool, MemoryAuthority, MemoryManagerConfig,
    MemoryRequest, MemoryRuntime, RuntimeMemory, SchemaMetadata, SealedDeclarationSnapshot,
    ic_stable_structures::{Cell, DefaultMemoryImpl},
};

use crate::model::archive::AuthorityArchiveRecord;

const JOURNAL_KEY: &str = "fixture.authority.archive.v1";
const MAX_JOURNAL_BYTES: usize = 65_536;

struct HostStore {
    _runtime: MemoryRuntime<DefaultMemoryImpl>,
    journal: Cell<Vec<u8>, RuntimeMemory<DefaultMemoryImpl>>,
}

thread_local! {
    static STORE: RefCell<Option<HostStore>> = const { RefCell::new(None) };
}

pub(super) fn open() {
    let request = MemoryRequest::new("fixture", JOURNAL_KEY, SchemaMetadata::default())
        .expect("fixture memory request");
    let pool = MemoryAllocationPool::new(
        vec![MemoryAuthority::new("fixture", "fixture.").expect("host namespace grant")],
        vec![],
    )
    .expect("host allocation pool");
    let declarations = SealedDeclarationSnapshot::new(&[request]).expect("fixture declarations");
    let mut runtime = MemoryRuntime::new_with_config(
        DefaultMemoryImpl::default(),
        MemoryManagerConfig::new(16).expect("host bucket profile"),
    )
    .expect("host runtime");
    runtime
        .bootstrap(&declarations, &pool, &GenericAllocationPolicy)
        .expect("host bootstrap");
    let memory = runtime.open_memory(JOURNAL_KEY).expect("host grant");
    let journal = Cell::init(memory, Vec::new());
    STORE.with_borrow_mut(|store| {
        *store = Some(HostStore {
            _runtime: runtime,
            journal,
        });
    });
}

pub(super) fn save(state: &AuthorityArchiveRecord) {
    let bytes = candid::encode_one(state).expect("bounded fixture journal");
    assert!(bytes.len() <= MAX_JOURNAL_BYTES, "journal byte budget");
    STORE.with_borrow_mut(|store| {
        store.as_mut().expect("opened journal").journal.set(bytes);
    });
}

pub(super) fn load() -> AuthorityArchiveRecord {
    STORE.with_borrow(|store| {
        let bytes = store.as_ref().expect("opened journal").journal.get();
        // An absent/invalid cell must reject inspection, never invent an empty archive.
        assert!(
            !bytes.is_empty() && bytes.len() <= MAX_JOURNAL_BYTES,
            "retained journal required"
        );
        let mut limits = DecoderConfig::new();
        limits
            .set_decoding_quota(2_000_000)
            .set_skipping_quota(10_000)
            .set_max_type_len(64)
            .set_full_error_message(false);
        decode_one_with_config(bytes, &limits).expect("same-release journal")
    })
}
