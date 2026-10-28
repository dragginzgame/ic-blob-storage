use super::*;
use ic_memory::DefaultMemoryImpl;
use std::panic::{AssertUnwindSafe, catch_unwind};

const KEY: &str = "fixture.journal.wire.v1";
const LIMITS: Limits = Limits {
    bytes: 128,
    allocation: 8192,
    nodes: 128,
    depth: 16,
};

#[test]
fn retained_wire_bytes_reopen_and_failed_writes_or_creation_never_reset_them() {
    let backing = DefaultMemoryImpl::default();
    let mut journal = FixtureJournal::create(backing.clone(), KEY, LIMITS);
    assert!(catch_unwind(AssertUnwindSafe(|| journal.load::<u64>())).is_err());
    journal.save(&42_u64);
    let original = journal.cell.bytes().to_vec();
    assert_eq!(
        original,
        native::encode_request(JOURNAL_ID, &(KEY, 42_u64), LIMITS).unwrap()
    );
    for key in [KEY, "fixture.another.wire.v1"] {
        assert!(
            catch_unwind(AssertUnwindSafe(|| FixtureJournal::create(
                backing.clone(),
                key,
                LIMITS
            )))
            .is_err()
        );
    }
    assert!(catch_unwind(AssertUnwindSafe(|| journal.save(&vec![1_u8; 512]))).is_err());
    assert_eq!(journal.cell.bytes(), original);
    drop(journal);
    let reopened = FixtureJournal::open(backing.clone(), KEY, LIMITS);
    assert_eq!(reopened.load::<u64>(), 42);
    drop(reopened);
    let limited = FixtureJournal::open(backing.clone(), KEY, Limits { nodes: 1, ..LIMITS });
    assert!(catch_unwind(AssertUnwindSafe(|| limited.load::<u64>())).is_err());
    drop(limited);
    assert_eq!(FixtureJournal::open(backing, KEY, LIMITS).load::<u64>(), 42);
}

#[test]
fn candid_foreign_direction_key_and_corrupt_data_refuse_without_replacement() {
    let mut journal = FixtureJournal::create(DefaultMemoryImpl::default(), KEY, LIMITS);
    let valid = native::encode_request(JOURNAL_ID, &(KEY, 42_u64), LIMITS).unwrap();
    let mut trailing = valid.clone();
    trailing.push(0);
    for bytes in [
        ic_platform::candid::contract::encode_one(42_u64).unwrap(),
        native::encode_response(JOURNAL_ID, &(KEY, 42_u64), LIMITS).unwrap(),
        native::encode_request(JOURNAL_ID, &("fixture.other.wire.v1", 42_u64), LIMITS).unwrap(),
        native::encode_request(JOURNAL_ID + 1, &(KEY, 42_u64), LIMITS).unwrap(),
        valid[..valid.len() - 1].to_vec(),
        trailing,
        vec![0],
    ] {
        journal.cell.try_set(bytes.clone()).unwrap();
        assert!(catch_unwind(AssertUnwindSafe(|| journal.load::<u64>())).is_err());
        assert_eq!(journal.cell.bytes(), bytes);
    }
    journal.save(&42_u64);
    assert_eq!(journal.load::<u64>(), 42);
}

#[test]
fn custom_consumer_envelope_and_missing_or_oversized_cells_never_seed_empty_state() {
    assert!(
        catch_unwind(AssertUnwindSafe(|| FixtureJournal::open(
            DefaultMemoryImpl::default(),
            KEY,
            LIMITS
        )))
        .is_err()
    );
    let backing = DefaultMemoryImpl::default();
    let mut journal = FixtureJournal::create(backing.clone(), KEY, LIMITS);
    journal.save(&42_u64);
    drop(journal);
    assert!(
        catch_unwind(AssertUnwindSafe(|| FixtureJournal::open(
            backing.clone(),
            KEY,
            Limits {
                bytes: 16,
                ..LIMITS
            }
        )))
        .is_err()
    );
    let journal = FixtureJournal::open(backing.clone(), KEY, LIMITS);
    let memory = journal.cell.into_memory();
    let body = ic_platform::candid::contract::encode_one(42_u64).unwrap();
    memory.write(0, b"CONS\x01");
    memory.write(5, &u32::try_from(body.len()).unwrap().to_le_bytes());
    memory.write(9, &body);
    let mut original = vec![0; 9 + body.len()];
    memory.read(0, &mut original);
    // Release custody so the next open reaches envelope validation rather than
    // merely refusing a second owner of the same backing.
    drop(memory);
    assert!(
        catch_unwind(AssertUnwindSafe(|| FixtureJournal::open(
            backing.clone(),
            KEY,
            LIMITS
        )))
        .is_err()
    );
    // Inspect the damaged cell through its canonical grant, without asking the
    // journal to accept it. Runtime ledger admission is separate from cell bytes.
    let memory = ic_memory::testing::fixture_runtime(
        backing,
        "fixture",
        "fixture.",
        &[KEY],
        MemoryManagerConfig::new(16).unwrap(),
    )
    .unwrap()
    .open_memory(KEY)
    .unwrap();
    let mut retained = vec![0; original.len()];
    memory.read(0, &mut retained);
    assert_eq!(retained, original);
}
