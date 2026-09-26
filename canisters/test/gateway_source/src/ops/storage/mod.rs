//! Host-owned memory for this experiment only. No product schema or migration.

use std::cell::RefCell;

use candid::{de::DecoderConfig, decode_one_with_config};
use ic_blob_storage::ic_memory::{
    GenericRangePolicy, MemoryManagerAuthorityRecord, MemoryManagerConfig, MemoryManagerIdRange,
    MemoryManagerRangeMode, MemoryRequest, MemoryRuntime, RuntimeMemory, SchemaMetadata,
    SealedDeclarationSnapshot, StaticMemoryRangeDeclaration,
    ic_stable_structures::{Cell, DefaultMemoryImpl},
};

use crate::model::SourceJournalRecord;

const JOURNAL_KEY: &str = "fixture.source.journal.v1";
const MAX_JOURNAL_BYTES: usize = 1_114_112;

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
    let grant = StaticMemoryRangeDeclaration::new(
        MemoryManagerAuthorityRecord::new(
            MemoryManagerIdRange::new(120, 120).expect("fixture range"),
            "fixture",
            MemoryManagerRangeMode::Allowed,
            None,
        )
        .expect("fixture authority"),
    )
    .expect("fixture grant");
    let declarations =
        SealedDeclarationSnapshot::new(&[], &[grant], &[request]).expect("fixture declarations");
    let mut runtime = MemoryRuntime::new_with_config(
        DefaultMemoryImpl::default(),
        MemoryManagerConfig::new(16).expect("host bucket profile"),
    )
    .expect("host runtime");
    runtime
        .bootstrap(&declarations, &GenericRangePolicy)
        .expect("host bootstrap");
    let memory = runtime.open_memory_by_key(JOURNAL_KEY).expect("host grant");
    let journal = Cell::init(memory, Vec::new());
    STORE.with_borrow_mut(|store| {
        *store = Some(HostStore {
            _runtime: runtime,
            journal,
        });
    });
}

pub(super) fn save(state: &SourceJournalRecord) {
    let bytes = candid::encode_one(state).expect("bounded fixture journal");
    assert!(bytes.len() <= MAX_JOURNAL_BYTES, "journal byte budget");
    STORE.with_borrow_mut(|store| {
        store.as_mut().expect("opened journal").journal.set(bytes);
    });
}

pub(super) fn load() -> SourceJournalRecord {
    STORE.with_borrow(|store| {
        let bytes = store.as_ref().expect("opened journal").journal.get();
        // An absent/invalid cell must trap the upgrade, never seed an empty journal.
        assert!(
            !bytes.is_empty() && bytes.len() <= MAX_JOURNAL_BYTES,
            "retained journal required"
        );
        let mut limits = DecoderConfig::new();
        limits
            .set_decoding_quota(30_000_000)
            .set_skipping_quota(10_000)
            .set_max_type_len(64)
            .set_full_error_message(false);
        decode_one_with_config(bytes, &limits).expect("same-release journal")
    })
}
