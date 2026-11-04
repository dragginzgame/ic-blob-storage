use super::*;

#[test]
fn limits_reject_each_invalid_boundary() {
    for (values, error) in [
        ((0, 10, 100), FundingLimitsError::ZeroReserve),
        ((1, 0, 100), FundingLimitsError::ZeroMinimumBalance),
        ((1, 10, 0), FundingLimitsError::ZeroTargetBalance),
        ((1, 101, 100), FundingLimitsError::MinimumExceedsTarget),
    ] {
        assert_eq!(FundingLimits::new(values.0, values.1, values.2), Err(error));
    }
}

#[test]
fn limits_accept_equal_thresholds_and_maximum_cycle_values() {
    let limits = FundingLimits::new(u128::MAX, u128::MAX, u128::MAX).expect("valid limits");
    assert_eq!(limits.reserve(), u128::MAX);
    assert_eq!(limits.minimum_balance(), u128::MAX);
    assert_eq!(limits.target_balance(), u128::MAX);
}
