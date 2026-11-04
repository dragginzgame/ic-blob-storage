use super::*;

fn funding() -> FundingLimits {
    FundingLimits::new(1, 10, 100).expect("valid funding limits")
}

#[test]
fn special_cashier_principals_reject_without_claiming_other_principals_are_deployed() {
    for principal in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            BillingConfiguration::new(principal, funding(), 8, 4),
            Err(BillingConfigurationError::InvalidCashier { principal })
        );
    }
    let candidate = Principal::self_authenticating(b"candidate identity");
    let config = BillingConfiguration::new(candidate, funding(), 8, 4).expect("valid candidate");
    assert_eq!(config.cashier(), candidate);
    assert_eq!(config.funding_limits(), funding());
    assert_eq!(config.gateway_limits().max_entries.get(), 8);
    assert_eq!(config.gateway_limits().max_unique.get(), 4);
}

#[test]
fn both_gateway_bounds_reject_zero_and_host_only_sizes() {
    let cashier = Principal::from_slice(&[1, 1]);
    for (entries, unique, field) in [
        (0, 4, GatewayLimitField::Entries),
        (8, 0, GatewayLimitField::Unique),
    ] {
        assert_eq!(
            BillingConfiguration::new(cashier, funding(), entries, unique),
            Err(BillingConfigurationError::ZeroGatewayLimit { field })
        );
    }
    for oversized in [u64::from(u32::MAX) + 1, u64::MAX] {
        for (entries, unique, field) in [
            (oversized, 4, GatewayLimitField::Entries),
            (8, oversized, GatewayLimitField::Unique),
        ] {
            assert_eq!(
                BillingConfiguration::new(cashier, funding(), entries, unique),
                Err(BillingConfigurationError::GatewayLimitOutOfRange { field })
            );
        }
    }
}

#[test]
fn maximum_representable_configuration_needs_no_collection_allocation() {
    let funding = FundingLimits::new(u128::MAX, u128::MAX, u128::MAX).expect("valid limits");
    let max = u64::from(u32::MAX);
    let config = BillingConfiguration::new(Principal::from_slice(&[1, 1]), funding, max, max)
        .expect("32-bit bounds");
    assert_eq!(config.funding_limits(), funding);
    assert_eq!(
        config.gateway_limits().max_entries.get(),
        usize::try_from(max).expect("target width")
    );
    assert_eq!(
        config.gateway_limits().max_unique.get(),
        usize::try_from(max).expect("target width")
    );
}
