//! One owner of the local receive call and its current platform cost observation.
use super::{ObservedFundingCall, read};
use blob_test_protocol::funding::{FundingOutcome, FundingRequest};
use candid::Principal;
use ic_blob_storage::policy::billing::liquidity::FundingLiquidity;
use ic_blob_storage_contracts::funding::transfer::FundingTransfer;
use ic_cdk::call::Call;
use std::num::NonZeroU128;

pub(crate) struct PreparedFundingCall {
    pub call: Call<'static, 'static>,
    pub offered: NonZeroU128,
    pub liquidity: FundingLiquidity,
}

pub(crate) fn prepare(peer: Principal, request: FundingRequest) -> PreparedFundingCall {
    let budget = read(crate::model::FundingJournalRecord::budget);
    let call = Call::unbounded_wait(peer, "receive").with_arg(request);
    let call_cost = call.get_cost(); // No attachment yet: avoid counting it twice.
    let call = call.with_cycles(request.offered);
    let liquidity = FundingLiquidity {
        call_cost,
        operating_reserve: NonZeroU128::new(budget.operating_reserve)
            .expect("validated operating slack"),
        other_liabilities: budget.other_liabilities,
        liquid_cycles: ic_cdk::api::canister_liquid_cycle_balance(),
    };
    PreparedFundingCall {
        call,
        offered: NonZeroU128::new(request.offered).expect("positive full offer"),
        liquidity,
    }
}

pub(crate) fn blocked(prepared: PreparedFundingCall) -> ObservedFundingCall {
    // Dropping a Call that was never awaited cannot enqueue it. Its durable intent
    // still consumes its identity, with known zero transport acceptance.
    drop(prepared.call);
    ObservedFundingCall {
        transfer: FundingTransfer::not_enqueued(prepared.offered),
        outcome: FundingOutcome::LiquidityBlocked,
    }
}

pub(crate) fn preview_request(
    request: blob_test_protocol::funding::preview::FundingPreviewRequest,
) -> FundingRequest {
    FundingRequest {
        id: request.id,
        offered: request.requested_cycles,
        accept: request.requested_cycles,
        reply: blob_test_protocol::funding::FundingReplyMode::Success,
        trap_callback: false,
    }
}

#[cfg(test)]
mod tests;
