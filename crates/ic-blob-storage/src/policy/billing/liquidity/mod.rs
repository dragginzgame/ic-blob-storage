//! Full-request liquidity checks, independent of attachment allocations and credit.
use std::num::NonZeroU128;

/// Same-message observations and holds for one prospective outgoing call.
/// The workflow must establish completeness and recheck before dispatch; a query
/// result is not a reusable authorization or an independent recovery proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingLiquidity {
    /// Platform liquid cycles, excluding platform execution/freezing reservations.
    pub liquid_cycles: u128,
    /// Platform cost bound for this method and encoded payload, excluding attachment.
    pub call_cost: u128,
    /// Additional operating slack, including anticipated memory-growth costs.
    pub operating_reserve: NonZeroU128,
    /// Complete additional liabilities still payable from the observed liquid funds.
    /// Do not subtract transfers or platform reservations already excluded from them.
    pub other_liabilities: u128,
}

/// Local arithmetic result; neither variant establishes provider/account authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingLiquidityDecision {
    /// The complete attachment fits after all supplied holds and call costs.
    Fits,
    /// Do not dispatch or substitute a smaller attachment.
    Insufficient {
        /// Maximum attachment from these observations, for diagnosis only.
        transferable_cycles: u128,
    },
}

/// Check the full attachment without overflow, even when combined holds exceed u128.
/// Unknown liabilities or costs must remain unknown at the workflow boundary;
/// callers must not replace them with zero to construct this complete input.
#[must_use]
pub const fn assess_funding_liquidity(
    requested_cycles: NonZeroU128,
    observation: FundingLiquidity,
) -> FundingLiquidityDecision {
    let transferable_cycles = observation
        .liquid_cycles
        .saturating_sub(observation.operating_reserve.get())
        .saturating_sub(observation.other_liabilities)
        .saturating_sub(observation.call_cost);
    if requested_cycles.get() > transferable_cycles {
        FundingLiquidityDecision::Insufficient {
            transferable_cycles,
        }
    } else {
        FundingLiquidityDecision::Fits
    }
}

#[cfg(test)]
mod tests;
