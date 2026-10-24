use super::*;
use ic_memory::DefaultMemoryImpl;
use std::panic::{AssertUnwindSafe, catch_unwind};

#[test]
fn retained_bytes_reopen_exactly_and_never_reset_or_seed_on_failure() {
    let memory = DefaultMemoryImpl::default();
    let mut journal = FixtureJournal::create(memory.clone(), "fixture.journal.v1", 64, 10_000);
    assert!(catch_unwind(AssertUnwindSafe(|| journal.load::<u64>())).is_err());
    journal.save(&42u64);
    assert_eq!(journal.cell.bytes(), encode_one(42u64).unwrap());
    drop(journal);
    assert!(
        catch_unwind(AssertUnwindSafe(|| FixtureJournal::create(
            memory.clone(),
            "fixture.journal.v1",
            64,
            10_000
        )))
        .is_err()
    );
    let mut reopened = FixtureJournal::open(memory, "fixture.journal.v1", 64, 10_000);
    assert_eq!(reopened.load::<u64>(), 42);
    assert!(catch_unwind(AssertUnwindSafe(|| reopened.save(&vec![1u8; 128]))).is_err());
    assert_eq!(reopened.load::<u64>(), 42);
    reopened.cell.try_set(vec![0]).unwrap();
    assert!(catch_unwind(AssertUnwindSafe(|| reopened.load::<u64>())).is_err());
}

#[test]
fn absent_memory_and_exhausted_decoder_quota_refuse() {
    assert!(
        catch_unwind(AssertUnwindSafe(|| FixtureJournal::open(
            DefaultMemoryImpl::default(),
            "fixture.journal.v1",
            64,
            10_000
        )))
        .is_err()
    );
    let mut journal =
        FixtureJournal::create(DefaultMemoryImpl::default(), "fixture.journal.v1", 64, 0);
    journal.save(&42u64);
    assert!(catch_unwind(AssertUnwindSafe(|| journal.load::<u64>())).is_err());
}
