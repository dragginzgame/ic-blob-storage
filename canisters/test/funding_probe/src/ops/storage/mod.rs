//! Probe-owned journal identity and budgets; shared plumbing retains exact Candid bytes.
use crate::model::FundingJournalRecord;
use blob_test_protocol::fixture_storage::FixtureJournal;
use ic_memory::DefaultMemoryImpl;
use std::cell::RefCell;
const JOURNAL_KEY: &str = "fixture.funding.journal.v1";
const MAX_JOURNAL_BYTES: usize = 65_536;
const DECODING_QUOTA: usize = 1_000_000;
thread_local! {
    static STORE: RefCell<Option<FixtureJournal<DefaultMemoryImpl>>> = const { RefCell::new(None) };
}
pub(super) fn create() {
    STORE.with_borrow_mut(|store| {
        *store = Some(FixtureJournal::create(
            DefaultMemoryImpl::default(),
            JOURNAL_KEY,
            MAX_JOURNAL_BYTES,
            DECODING_QUOTA,
        ));
    });
}
pub(super) fn open() {
    STORE.with_borrow_mut(|store| {
        *store = Some(FixtureJournal::open(
            DefaultMemoryImpl::default(),
            JOURNAL_KEY,
            MAX_JOURNAL_BYTES,
            DECODING_QUOTA,
        ));
    });
}
pub(super) fn save(state: &FundingJournalRecord) {
    STORE.with_borrow_mut(|store| store.as_mut().expect("opened journal").save(state));
}
pub(super) fn load() -> FundingJournalRecord {
    STORE.with_borrow(|store| store.as_ref().expect("opened journal").load())
}
