//! Host-owned memory for this experiment only. No product schema or migration.

use std::cell::RefCell;

use ic_memory::{DefaultMemoryImpl, MemoryManagerConfig, testing::FixtureCell};
use ic_platform::candid::decoder::{DecoderConfig, decode_one_with_config};

use crate::model::archive::AuthorityArchiveRecord;

const JOURNAL_KEY: &str = "fixture.authority.archive.v1";
const MAX_JOURNAL_BYTES: usize = 65_536;

thread_local! {
    static STORE: RefCell<Option<FixtureCell<DefaultMemoryImpl>>> = const { RefCell::new(None) };
}

pub(super) fn create() {
    let journal = FixtureCell::create_bounded(
        DefaultMemoryImpl::default(),
        "fixture",
        "fixture.",
        JOURNAL_KEY,
        MemoryManagerConfig::new(16).expect("host bucket profile"),
        MAX_JOURNAL_BYTES,
    )
    .expect("fresh bounded fixture journal");
    STORE.with_borrow_mut(|store| *store = Some(journal));
}

pub(super) fn open() {
    let journal = FixtureCell::open_bounded(
        DefaultMemoryImpl::default(),
        "fixture",
        "fixture.",
        JOURNAL_KEY,
        MemoryManagerConfig::new(16).expect("host bucket profile"),
        MAX_JOURNAL_BYTES,
    )
    .expect("retained bounded fixture journal");
    STORE.with_borrow_mut(|store| *store = Some(journal));
}

pub(super) fn save(state: &AuthorityArchiveRecord) {
    let bytes = ic_platform::candid::contract::encode_one(state).expect("bounded fixture journal");
    STORE.with_borrow_mut(|store| {
        store
            .as_mut()
            .expect("opened journal")
            .try_set(bytes)
            .expect("journal byte budget");
    });
}

pub(super) fn load() -> AuthorityArchiveRecord {
    STORE.with_borrow(|store| {
        let bytes = store.as_ref().expect("opened journal").bytes();
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
