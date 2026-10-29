use super::*;

#[test]
fn fixed_roster_keeps_units_outcomes_empty_zero_and_actual_overflow() {
    let mut state = Metrics::new(7, 100);
    assert_eq!(
        state.rows[0]
            .view(MetricOperation::Admission)
            .instructions
            .maximum(),
        None
    );
    state.begin(MetricOperation::Admission, u128::MAX);
    assert!(
        !state.rows[0]
            .view(MetricOperation::Admission)
            .request_bytes
            .total_overflowed()
    );
    state.complete(MetricOperation::Admission, 7, 0, Outcome::Applied);
    state.begin(MetricOperation::Admission, 1);
    state.complete(MetricOperation::Admission, 7, 12, Outcome::Replayed);
    state.begin(MetricOperation::Reference, 9);
    state.complete(MetricOperation::Reference, 7, 0, Outcome::Refused);
    let row = state.rows[0].view(MetricOperation::Admission);
    assert_eq!(row.attempts.total(), 2);
    assert_eq!(row.applied.total(), 1);
    assert_eq!(row.replayed.total(), 1);
    assert_eq!(row.request_bytes.total(), u128::MAX);
    assert!(row.request_bytes.total_overflowed());
    assert_eq!(row.instructions.samples(), 2);
    assert_eq!(row.instructions.total(), 12);
    assert_eq!(row.instructions.maximum(), Some(12));
    let refusal = state.rows[3].view(MetricOperation::Reference);
    assert_eq!(refusal.refused.total(), 1);
    assert_eq!(refusal.instructions.maximum(), Some(0));
    assert_eq!(refusal.request_bytes.total(), 9);
    assert_eq!(state.rows.len(), MetricOperation::ALL.len());
}

#[test]
fn only_lifecycle_reset_discards_inflight_contexts() {
    let mut state = Metrics::new(7, 100);
    state.begin(MetricOperation::Admission, 100);
    state.complete(MetricOperation::Admission, 7, 900, Outcome::Applied);
    state.begin(MetricOperation::Admission, 20);
    assert_eq!(state.rows[0].attempts.total(), 2);
    state = Metrics::new(20, 200);
    state.begin(MetricOperation::Admission, 20);
    state.complete(MetricOperation::Admission, 7, 900, Outcome::Applied);
    state.complete(MetricOperation::Admission, 20, 2, Outcome::Refused);
    assert_eq!(state.version, 20);
    assert_eq!(state.started_at, 200);
    assert_eq!(state.rows[0].attempts.total(), 1);
    assert_eq!(state.rows[0].bytes.total(), 20);
    assert_eq!(state.rows[0].instructions.samples(), 1);
    assert_eq!(state.rows[0].outcomes[0].total(), 0);
    assert_eq!(state.rows[0].outcomes[2].total(), 1);
}
