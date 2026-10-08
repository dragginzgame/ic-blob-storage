//! Explicit IC effects for the canonical Cashier request; no endpoint or retry loop.
//!
//! The host must authenticate the operator, qualify the provider/account, establish
//! complete activity/spendability and persist the exact attempt before dispatch.
//! Unbounded wait preserves exact attachment refund accounting but a stalled peer
//! can obstruct upgrades. This primitive does not qualify a deployment choice.
//! See <https://docs.internetcomputer.org/references/message-execution-properties/>.
use super::{
    TopUpReply, TopUpReplyError, TopUpReplyLimits, decode_top_up_reply,
    request::CashierTopUpRequest,
};
use crate::model::billing::journal::FundingTransportContext;
use crate::model::billing::journal::FundingTransportOutcome;
use crate::policy::billing::liquidity::FundingLiquidity;
use candid::Principal;
use ic_blob_storage_contracts::funding::transfer::FundingTransfer;
use ic_cdk::call::{Call, CallFailed};
use std::num::NonZeroU128;

pub(crate) fn running_service() -> Principal {
    ic_cdk::api::canister_self()
}

/// One owned, unpolled call; constructing or dropping it sends nothing.
/// Never retain its liquidity observation across an await or treat it as permission.
#[must_use]
pub struct PreparedCashierTopUp {
    call: Call<'static, 'static>,
    source: FundingTransportContext,
    offered: NonZeroU128,
    call_cost: u128,
    account: Principal,
    target_balance: Option<NonZeroU128>,
}
impl PreparedCashierTopUp {
    /// Bind the exact method, arguments and full attachment without dispatch.
    /// Must run on the IC, after durable intent/attempt writes and their memory growth.
    pub fn new(request: &CashierTopUpRequest) -> Self {
        let call = Call::unbounded_wait(request.cashier(), request.method_name())
            .take_raw_args(request.arguments().to_vec());
        let call_cost = call.get_cost(); // The attachment is accounted separately.
        Self {
            call: call.with_cycles(request.offered().get()),
            source: FundingTransportContext {
                service: ic_cdk::api::canister_self(),
                cashier: request.cashier(),
            },
            offered: request.offered(),
            call_cost,
            account: request.account(),
            target_balance: request.target_balance(),
        }
    }
    /// Original complete attachment, never reduced to fit available funds.
    #[must_use]
    pub const fn offered(&self) -> NonZeroU128 {
        self.offered
    }

    /// Current platform liquidity and the cost of this exact call, plus explicit
    /// host-established operating slack and other liabilities. Unknown holds must
    /// not be replaced by zero. Assess policy and dispatch in the same message.
    #[must_use]
    pub fn liquidity(
        &self,
        operating_reserve: NonZeroU128,
        other_liabilities: u128,
    ) -> FundingLiquidity {
        FundingLiquidity {
            liquid_cycles: ic_cdk::api::canister_liquid_cycle_balance(),
            call_cost: self.call_cost,
            operating_reserve,
            other_liabilities,
        }
    }
    /// Consume a call that was never polled; positive proof that it was not sent.
    /// This does not cancel an executing future or resolve a lost callback.
    #[must_use]
    pub fn cancel(self) -> CashierTopUpObservation {
        drop(self.call);
        CashierTopUpObservation {
            source: self.source,
            transfer: FundingTransfer::not_enqueued(self.offered),
            status: CashierTopUpStatus::NotDispatched,
            account: self.account,
            target_balance: self.target_balance,
        }
    }
    /// Dispatch once after the host's admission and same-message liquidity checks.
    /// Capture this call's refund before decoding or any other await. The host must
    /// retain the returned evidence against its exact intent before yielding again.
    /// Dropped futures, traps and missing responses confer no retry authority.
    /// Reply limits bound decoding, not the CDK's transport response buffer.
    /// # Panics
    /// An impossible platform refund traps; the durable attempt must remain uncertain.
    pub async fn execute(self, limits: TopUpReplyLimits) -> CashierTopUpObservation {
        let result = self.call.await;
        let transfer = match &result {
            Ok(_) | Err(CallFailed::CallRejected(_)) => {
                // This is the matching unbounded callback, not a later observation.
                let refunded = ic_cdk::api::msg_cycles_refunded();
                FundingTransfer::unbounded_callback(self.offered, refunded)
                    .expect("exact callback refund")
            }
            Err(
                CallFailed::InsufficientLiquidCycleBalance(_) | CallFailed::CallPerformFailed(_),
            ) => {
                // No callback exists: never sample an ambient refund here.
                FundingTransfer::not_enqueued(self.offered)
            }
        };
        let status = match result {
            Ok(response) => {
                CashierTopUpStatus::Replied(decode_top_up_reply(&response.into_bytes(), limits))
            }
            Err(CallFailed::CallRejected(error)) => {
                CashierTopUpStatus::Rejected(error.raw_reject_code())
            }
            Err(
                CallFailed::InsufficientLiquidCycleBalance(_) | CallFailed::CallPerformFailed(_),
            ) => CashierTopUpStatus::NotEnqueued,
        };
        CashierTopUpObservation {
            source: self.source,
            transfer,
            status,
            account: self.account,
            target_balance: self.target_balance,
        }
    }
}

/// Independent reply interpretation; no variant proves provider credit or permits retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CashierTopUpStatus {
    /// The host consumed the prepared call without polling it.
    NotDispatched,
    /// The platform refused to enqueue the call; there was no callback.
    NotEnqueued,
    /// Actual rejection code, independent of any accepted attachment.
    Rejected(u32),
    /// Bounded structured reply or decode failure, independent of the exact refund.
    Replied(Result<TopUpReply, TopUpReplyError>),
}
/// Call-correlated transport facts, not a durable receipt or provider-credit proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CashierTopUpObservation {
    source: FundingTransportContext,
    transfer: FundingTransfer,
    status: CashierTopUpStatus,
    account: Principal,
    target_balance: Option<NonZeroU128>,
}
impl CashierTopUpObservation {
    /// Explicit account in the original request, never taken from the response.
    #[must_use]
    pub const fn account(self) -> Principal {
        self.account
    }
    /// Exact optional target in the original request, independent of attachment.
    #[must_use]
    pub const fn target_balance(self) -> Option<NonZeroU128> {
        self.target_balance
    }
    /// Actual running service and original call target, never read from reply payloads.
    #[must_use]
    pub const fn source(self) -> FundingTransportContext {
        self.source
    }
    /// Validated attachment arithmetic, excluding execution fees and provider credit.
    #[must_use]
    pub const fn transfer(self) -> FundingTransfer {
        self.transfer
    }
    /// Evidence suitable for the journal only with the original exact intent.
    #[must_use]
    pub const fn outcome(self) -> FundingTransportOutcome {
        match self.transfer.refunded() {
            Some(refunded) => FundingTransportOutcome::Callback { refunded },
            None => FundingTransportOutcome::NotEnqueued,
        }
    }
    /// Provider result or transport failure, kept separate from attachment accounting.
    #[must_use]
    pub const fn status(self) -> CashierTopUpStatus {
        self.status
    }
}
