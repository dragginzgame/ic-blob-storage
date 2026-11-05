//! The host's read window preserves zero observations and overflow provenance.
use super::*;

fn report(row: ReadCounter) -> RestorationMemoryReads {
    begin_read_profile(true);
    READS.with_borrow_mut(|rows| rows[0] = row);
    finish_read_profile().remove(0)
}

#[test]
fn zero_read_and_exact_caps_remain_exact_until_overflow() {
    let mut row = ReadCounter::EMPTY;
    row.record(0, 0);
    assert_eq!(row.calls, 1);
    assert_eq!(row.instructions, 0);

    row.calls = u64::MAX - 1;
    row.instructions = u128::from(u64::MAX - 1);
    row.record(1, u64::MAX);
    assert_eq!(row.calls, u64::MAX);
    assert!(!row.calls_saturated);
    assert_eq!(report(row).bytes, u64::MAX);
    assert!(!report(row).bytes_saturated);
    row.record(0, 0);
    assert!(row.calls_saturated);
    assert!(!report(row).bytes_saturated);
    row.record(1, 1);
    assert!(report(row).bytes_saturated);
    row.record(0, 0);
    assert!(report(row).bytes_saturated);
}

#[test]
fn instruction_total_quality_survives_exact_cap_and_overflow() {
    // Count is already clipped: only explicit provenance permits this total.
    let mut row = ReadCounter {
        calls: u64::MAX,
        calls_saturated: true,
        instructions: u128::MAX - 1,
        ..ReadCounter::EMPTY
    };
    row.record(1, 0);
    assert_eq!(row.instructions, u128::MAX);
    assert!(!row.instructions_saturated);
    row.record(1, 0);
    assert_eq!(row.instructions, u128::MAX);
    assert!(row.instructions_saturated);
    row.record(0, 0);
    assert!(row.instructions_saturated);
}

#[test]
fn completed_and_disabled_windows_cannot_leak_into_the_next_profile() {
    begin_read_profile(true);
    READS.with_borrow_mut(|rows| {
        rows[0].record(0, 0);
        rows[1].record(7, 13);
        rows[2] = ReadCounter {
            calls: u64::MAX,
            instructions: u128::MAX,
            bytes: u64::MAX,
            calls_saturated: true,
            instructions_saturated: true,
            bytes_saturated: true,
        };
    });
    let first = finish_read_profile();
    assert_eq!((first[0].calls, first[0].instructions), (1, 0));
    assert_eq!(
        (first[1].calls, first[1].instructions, first[1].bytes),
        (1, 7, 13)
    );
    assert_eq!(
        (first[2].calls, first[2].instructions, first[2].bytes),
        (u64::MAX, u128::MAX, u64::MAX)
    );
    assert!(first[2].calls_saturated);
    assert!(first[2].bytes_saturated);
    assert!(first[2].instructions_saturated);
    assert!(finish_read_profile().is_empty());
    assert!(start_read().is_none());
    begin_read_profile(false);
    assert!(finish_read_profile().is_empty());
    begin_read_profile(true);
    assert!(finish_read_profile().iter().all(|r| r.calls == 0
        && r.bytes == 0
        && r.instructions == 0
        && !r.calls_saturated
        && !r.bytes_saturated
        && !r.instructions_saturated));
}

#[test]
fn byte_overflow_stays_clipped_and_flagged_after_zero_observations() {
    let mut row = ReadCounter {
        bytes: u64::MAX,
        bytes_saturated: true,
        ..ReadCounter::EMPTY
    };
    row.record(0, 0);
    let observed = report(row);
    assert_eq!(observed.bytes, u64::MAX);
    assert!(observed.bytes_saturated);
}
