//! Host-owned memory for this experiment only. No product schema or migration.

use std::cell::RefCell;

use candid::{de::DecoderConfig, decode_one_with_config};
use ic_memory::{DefaultMemoryImpl, MemoryManagerConfig, testing::FixtureCell};

use crate::model::FundingJournalRecord;

const JOURNAL_KEY: &str = "fixture.funding.journal.v1";
const MAX_JOURNAL_BYTES: usize = 65_536;

thread_local! {
    static STORE: RefCell<Option<FixtureCell<DefaultMemoryImpl>>> = const { RefCell::new(None) };
}

pub(super) fn open() {
    let journal = FixtureCell::new(
        DefaultMemoryImpl::default(),
        "fixture",
        "fixture.",
        JOURNAL_KEY,
        MemoryManagerConfig::new(16).expect("host bucket profile"),
    );
    STORE.with_borrow_mut(|store| *store = Some(journal));
}

pub(super) fn save(state: &FundingJournalRecord) {
    let bytes = candid::encode_one(state).expect("bounded fixture journal");
    assert!(bytes.len() <= MAX_JOURNAL_BYTES, "journal byte budget");
    STORE.with_borrow_mut(|store| {
        store.as_mut().expect("opened journal").set(bytes);
    });
}

pub(super) fn load() -> FundingJournalRecord {
    STORE.with_borrow(|store| {
        let bytes = store.as_ref().expect("opened journal").bytes();
        // An absent/invalid cell must trap the upgrade, never seed an empty journal.
        assert!(
            !bytes.is_empty() && bytes.len() <= MAX_JOURNAL_BYTES,
            "retained journal required"
        );
        let mut limits = DecoderConfig::new();
        limits
            .set_decoding_quota(1_000_000)
            .set_skipping_quota(10_000)
            .set_max_type_len(64)
            .set_full_error_message(false);
        decode_one_with_config(bytes, &limits).expect("same-release journal")
    })
}
