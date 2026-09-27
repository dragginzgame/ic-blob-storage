//! One guarded IC dispatch and durable settlement, with no exposed pending permit.
use super::attempt::{FundingAttemptResult, inspect_attempt, mark_first_attempt};
use crate::{
    model::{billing::journal::FundingIntent, service::upload::UploadContext},
    ops::{
        caffeine::funding::{
            TopUpReplyLimits,
            transport::{CashierTopUpObservation, PreparedCashierTopUp, running_service},
        },
        service::funding::{FundingJournalError, access::FundingJournalAccess},
    },
    policy::billing::{
        admission::attempt::{FundingAttemptAssessment, FundingAttemptEvidence},
        liquidity::{FundingLiquidityDecision, assess_funding_liquidity},
    },
};
use std::num::NonZeroU128;

/// Complete host holds, applied to actual platform liquidity after journal writes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingDispatchHolds {
    /// Additional positive operating slack, beyond platform freezing reservations.
    pub operating_reserve: NonZeroU128,
    /// Other liabilities still payable from platform liquid cycles. Excludes this
    /// exact unsent offer and funds already excluded by the platform.
    pub other_liabilities: u128,
}
/// Trusted observations acquired synchronously at the start of this dispatch.
/// Not ingress data or proof tokens. The host must establish freshness, exact
/// scope and complete account activity, including other installations/payers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingDispatchEvidence {
    /// Exact intent-bound first-attempt evidence.
    pub attempt: FundingAttemptEvidence,
    /// Missing holds block before mutation; they must never be defaulted to zero.
    pub holds: Option<FundingDispatchHolds>,
}
/// Completed handler result, never provider-credit or retry authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FundingDispatchResult {
    /// Nothing was marked or sent; the existing reservation remains intact.
    Blocked {
        /// All local and external first-attempt blockers.
        attempt: FundingAttemptAssessment,
        /// Independent absence of complete liquidity holds.
        holds_unknown: bool,
    },
    /// Observation and accounting are durably committed. Includes positively
    /// unsent liquidity refusals; inspect the status separately from acceptance.
    Settled {
        /// Call-correlated facts, not verified provider credit.
        observation: Box<CashierTopUpObservation>,
        /// Exact platform call-cost bound, excluding the attachment.
        call_cost: u128,
    },
}

/// Compose guarded marking, post-write liquidity, one unbounded call and settlement.
/// `observe` runs once on first polling, after authenticated exact request lookup,
/// rather than when the async future is constructed. It must establish current
/// facts synchronously, not return cached query/await observations. Linking this
/// handler installs no endpoint, timer, automatic retry or lifecycle hook.
///
/// No borrow or dispatch permit crosses the await. The original execution/intent
/// stays captured through the exact callback. Refund capture and durable outcome
/// recording occur before another await. Callback errors/traps leave the original
/// attempt uncertain; callers must not catch traps or infer retry from failure.
/// A liquidity refusal consumes an unpolled call and durably records it as unsent.
/// # Errors
/// Rejects authority, actual service, exact identity or journal settlement failures.
/// # Panics
/// IC system API and stable traps propagate for whole-message rollback. This
/// handler must run as an IC update, never a query or native platform substitute.
pub async fn dispatch<H: FundingJournalAccess>(
    host: &H,
    execution: UploadContext,
    input: FundingIntent,
    observe: impl FnOnce() -> FundingDispatchEvidence,
    limits: TopUpReplyLimits,
) -> Result<FundingDispatchResult, FundingJournalError> {
    // Authenticate before invoking trusted host acquisition, including on failures.
    host.with_funding_journal(|journal| journal.request(execution, input))?;
    if running_service() != execution.service {
        return Err(FundingJournalError::TransportBinding);
    }
    let evidence = observe();
    let Some(holds) = evidence.holds else {
        let attempt = host.with_funding_journal(|journal| {
            inspect_attempt(journal, execution, input, evidence.attempt)
        })?;
        return Ok(FundingDispatchResult::Blocked {
            attempt,
            holds_unknown: true,
        });
    };
    let marked = host.with_funding_journal(|journal| {
        mark_first_attempt(journal, execution, input, evidence.attempt)
    })?;
    let request = match marked {
        FundingAttemptResult::Blocked(attempt) => {
            return Ok(FundingDispatchResult::Blocked {
                attempt,
                holds_unknown: false,
            });
        }
        FundingAttemptResult::Marked(request) => request,
    };
    // All durable intent writes and memory growth precede this observation.
    let call = PreparedCashierTopUp::new(&request);
    let liquidity = call.liquidity(holds.operating_reserve, holds.other_liabilities);
    let observation = match assess_funding_liquidity(input.offered, liquidity) {
        FundingLiquidityDecision::Fits => call.execute(limits).await,
        FundingLiquidityDecision::Insufficient { .. } => call.cancel(),
    };
    host.with_funding_journal(|journal| journal.record_observation(execution, input, observation))?;
    Ok(FundingDispatchResult::Settled {
        observation: Box::new(observation),
        call_cost: liquidity.call_cost,
    })
}
