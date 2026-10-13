//! The host's read window preserves zero observations and overflow provenance.
use super::*;
use ic_metrics::MeasurementAggregate;

#[test]
fn zero_read_and_exact_caps_remain_exact_until_overflow() {
    let mut row = ReadCounter::EMPTY;
    row.record(0, 0);
    assert_eq!(row.instructions.samples(), 1);
    assert_eq!(row.instructions.total(), 0);
    assert_eq!(row.instructions.maximum(), Some(0));

    row.instructions = MeasurementAggregate::from_parts(
        u64::MAX - 1,
        u128::from(u64::MAX - 1),
        Some(1),
        false,
        false,
    )
    .unwrap();
    row.record(1, u64::MAX);
    assert_eq!(row.instructions.samples(), u64::MAX);
    assert!(!row.instructions.samples_overflowed());
    assert_eq!(row.bytes, u64::MAX);
    assert!(!row.bytes_saturated);
    row.record(0, 0);
    assert!(row.instructions.samples_overflowed());
    assert!(!row.bytes_saturated);
    row.record(1, 1);
    assert!(row.bytes_saturated);
    row.record(0, 0);
    assert!(row.bytes_saturated);
}

#[test]
fn instruction_total_quality_survives_exact_cap_and_overflow() {
    // Count is already clipped: only explicit provenance permits this total.
    let mut row = ReadCounter {
        instructions: MeasurementAggregate::from_parts(
            u64::MAX,
            u128::MAX - 1,
            Some(u64::MAX),
            true,
            false,
        )
        .unwrap(),
        ..ReadCounter::EMPTY
    };
    row.record(1, 0);
    assert_eq!(row.instructions.total(), u128::MAX);
    assert!(!row.instructions.total_overflowed());
    row.record(1, 0);
    assert_eq!(row.instructions.total(), u128::MAX);
    assert!(row.instructions.total_overflowed());
    row.record(0, 0);
    assert!(row.instructions.total_overflowed());
}

#[test]
fn completed_and_disabled_windows_cannot_leak_into_the_next_profile() {
    begin_read_profile(true);
    READS.with_borrow_mut(|rows| {
        rows[0].record(0, 0);
        rows[1].record(7, 13);
        rows[2] = ReadCounter {
            instructions: MeasurementAggregate::from_parts(
                u64::MAX,
                u128::MAX,
                Some(u64::MAX),
                true,
                true,
            )
            .unwrap(),
            bytes: u64::MAX,
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
