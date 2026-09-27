//! Local journal composition with the shared actual IC transport.
use super::{Failure, STATE, TRAP_WRITE, UploadContext, failure, intent};
use blob_test_protocol::{
    funding::{FundingOutcome, FundingProviderErrorView},
    storage::funding::transport::{CallbackIdentityFault, Input, Observation},
};
use ic_blob_storage::{
    model::billing::journal::FundingIntent,
    ops::caffeine::funding::{
        TopUpProviderError, TopUpReply, TopUpReplyLimits,
        transport::{CashierTopUpObservation, CashierTopUpStatus, PreparedCashierTopUp},
    },
};
use std::num::NonZeroUsize;

pub(crate) fn begin(
    execution: UploadContext,
    input: Input,
) -> Result<(FundingIntent, PreparedCashierTopUp), Failure> {
    let original = intent(input.intent)?;
    let request = STATE
        .with_borrow_mut(|state| {
            state
                .as_mut()
                .unwrap()
                .funding
                .mark_attempted(execution, original)
        })
        .map_err(failure)?;
    // No stable writes after this observation until dispatch or a proven unsent result.
    Ok((original, PreparedCashierTopUp::new(&request)))
}
pub(crate) fn limits() -> TopUpReplyLimits {
    let n = |v| NonZeroUsize::new(v).unwrap();
    TopUpReplyLimits {
        max_bytes: n(4096),
        decoding_quota: n(100_000),
        skipping_quota: n(1000),
        max_type_entries: n(32),
    }
}
pub(crate) fn complete(
    execution: UploadContext,
    original: FundingIntent,
    call: CashierTopUpObservation,
    input: Input,
    call_cost: u128,
) -> Result<Observation, Failure> {
    let original = if let Some(fault) = input.callback_identity_fault {
        let mut changed = input.intent;
        match fault {
            CallbackIdentityFault::Offered => changed.offered -= 1,
            CallbackIdentityFault::Account => changed.account = candid::Principal::anonymous(),
            CallbackIdentityFault::TargetBalance => {
                changed.target_balance = changed.target_balance.is_none().then_some(1);
            }
        }
        intent(changed)?
    } else {
        original
    };
    if call.transfer().refunded().is_some() {
        TRAP_WRITE.set(input.callback_fault);
    }
    let result = STATE.with_borrow_mut(|state| {
        state
            .as_mut()
            .unwrap()
            .funding
            .record_observation(execution, original, call)
    });
    TRAP_WRITE.set(None);
    result.map_err(failure)?;
    Ok(Observation {
        refunded: call.transfer().refunded(),
        accepted: call
            .transfer()
            .accepted()
            .expect("completed local transport"),
        outcome: outcome(call.status()),
        call_cost,
    })
}
pub(super) fn outcome(status: CashierTopUpStatus) -> FundingOutcome {
    match status {
        CashierTopUpStatus::NotDispatched => FundingOutcome::LiquidityBlocked,
        CashierTopUpStatus::NotEnqueued => FundingOutcome::NotEnqueued,
        CashierTopUpStatus::Rejected(code) => FundingOutcome::Rejected(code),
        CashierTopUpStatus::Replied(Err(_)) => FundingOutcome::InvalidReply,
        CashierTopUpStatus::Replied(Ok(TopUpReply::ReportedSuccess { .. })) => {
            FundingOutcome::ReportedSuccess
        }
        CashierTopUpStatus::Replied(Ok(TopUpReply::ProviderFailure(error))) => {
            FundingOutcome::ProviderError(match error {
                TopUpProviderError::NotAuthorized(p) => FundingProviderErrorView::NotAuthorized(p),
                TopUpProviderError::AccountBalanceOverflow => {
                    FundingProviderErrorView::AccountBalanceOverflow
                }
                TopUpProviderError::InternalError => FundingProviderErrorView::InternalError,
                TopUpProviderError::TopUpWithoutCycles => {
                    FundingProviderErrorView::TopUpWithoutCycles
                }
            })
        }
    }
}
