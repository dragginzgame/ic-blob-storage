use super::*;

fn amount(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).unwrap()
}

#[test]
fn full_request_preserves_costs_operating_slack_and_other_liabilities() {
    let observed = FundingLiquidity {
        liquid_cycles: 1000,
        call_cost: 200,
        operating_reserve: amount(100),
        other_liabilities: 300,
    };
    assert_eq!(
        assess_funding_liquidity(amount(400), observed),
        FundingLiquidityDecision::Fits
    );
    assert_eq!(
        assess_funding_liquidity(amount(401), observed),
        FundingLiquidityDecision::Insufficient {
            transferable_cycles: 400
        }
    );
    for reduced in [
        FundingLiquidity {
            liquid_cycles: 999,
            ..observed
        },
        FundingLiquidity {
            call_cost: 201,
            ..observed
        },
        FundingLiquidity {
            other_liabilities: 301,
            ..observed
        },
        FundingLiquidity {
            operating_reserve: amount(101),
            ..observed
        },
    ] {
        assert_eq!(
            assess_funding_liquidity(amount(400), reduced),
            FundingLiquidityDecision::Insufficient {
                transferable_cycles: 399
            }
        );
    }
}

#[test]
fn extreme_holds_cannot_overflow_into_available_funds() {
    let observed = FundingLiquidity {
        liquid_cycles: u128::MAX,
        call_cost: 0,
        operating_reserve: amount(1),
        other_liabilities: 0,
    };
    assert_eq!(
        assess_funding_liquidity(amount(u128::MAX - 1), observed),
        FundingLiquidityDecision::Fits
    );
    for exhausted in [
        FundingLiquidity {
            liquid_cycles: 0,
            ..observed
        },
        FundingLiquidity {
            call_cost: u128::MAX,
            ..observed
        },
        FundingLiquidity {
            other_liabilities: u128::MAX,
            ..observed
        },
        FundingLiquidity {
            operating_reserve: amount(u128::MAX),
            call_cost: u128::MAX,
            other_liabilities: u128::MAX,
            ..observed
        },
    ] {
        assert_eq!(
            assess_funding_liquidity(amount(1), exhausted),
            FundingLiquidityDecision::Insufficient {
                transferable_cycles: 0
            }
        );
    }
}
