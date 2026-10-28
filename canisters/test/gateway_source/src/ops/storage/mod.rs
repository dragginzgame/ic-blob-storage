//! Probe-owned journal identity and budgets; shared plumbing owns the bounded Wire payload.
use crate::model::SourceJournalRecord;
use blob_test_protocol::fixture_storage::FixtureJournal;
use ic_memory::DefaultMemoryImpl;
use ic_wire::Limits;
use std::cell::RefCell;
const JOURNAL_KEY: &str = "fixture.source.wire.v1";
const MAX_JOURNAL_BYTES: usize = 1_114_112;
const LIMITS: Limits = Limits {
    bytes: MAX_JOURNAL_BYTES,
    // Wire conservatively charges bulk payload storage as well as output bytes.
    // Preserve the admitted 1 MiB leaf plus bounded source history.
    allocation: MAX_JOURNAL_BYTES * 16,
    nodes: 262_144,
    depth: 32,
};
thread_local! {
    static STORE: RefCell<Option<FixtureJournal<DefaultMemoryImpl>>> = const { RefCell::new(None) };
}
pub(super) fn create() {
    STORE.with_borrow_mut(|store| {
        *store = Some(FixtureJournal::create(
            DefaultMemoryImpl::default(),
            JOURNAL_KEY,
            LIMITS,
        ));
    });
}
pub(super) fn open() {
    STORE.with_borrow_mut(|store| {
        drop(store.take());
        *store = Some(FixtureJournal::open(
            DefaultMemoryImpl::default(),
            JOURNAL_KEY,
            LIMITS,
        ));
    });
}
pub(super) fn save(state: &SourceJournalRecord) {
    STORE.with_borrow_mut(|store| store.as_mut().expect("opened journal").save(state));
}
pub(super) fn load() -> SourceJournalRecord {
    STORE.with_borrow(|store| store.as_ref().expect("opened journal").load())
}
