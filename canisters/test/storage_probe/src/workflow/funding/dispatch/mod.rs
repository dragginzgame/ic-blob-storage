//! Local IC experiment with synthetic host facts; not a production payment API.
use crate::ops::funding::{self, admission, dispatch::FixtureDispatchJournal, transport};
use blob_test_protocol::storage::{
    Failure,
    funding::transport::{DispatchInput, DispatchResult, EvidenceScenario},
};
use ic_blob_storage::{
    model::service::upload::UploadContext,
    policy::billing::{
        RecoveryState,
        admission::{FundingActivity, attempt::FundingAttemptEvidence},
    },
    workflow::funding::dispatch::{
        FundingDispatchEvidence, FundingDispatchHolds, FundingDispatchResult, dispatch,
    },
};
use std::num::NonZeroU128;

pub(crate) async fn run(
    execution: UploadContext,
    input: DispatchInput,
) -> Result<DispatchResult, Failure> {
    let intent = funding::intent(input.intent)?;
    let reserve = NonZeroU128::new(input.operating_reserve).ok_or(Failure::Invalid)?;
    let host = FixtureDispatchJournal {
        execution,
        intent,
        attempt_fault: input.attempt_fault,
        callback_fault: input.callback_fault,
    };
    let result = dispatch(
        &host,
        execution,
        intent,
        || {
            let attempt = match input.evidence {
                EvidenceScenario::Unknown => admission::unknown_attempt_evidence(intent),
                EvidenceScenario::Complete | EvidenceScenario::MissingHolds => {
                    FundingAttemptEvidence {
                        intent,
                        provider_qualified: true,
                        available_for_offer: Some(1000),
                        recovery: Some(RecoveryState::Reconciled),
                        other_activity: Some(FundingActivity::Clear),
                    }
                }
            };
            // Fixed synthetic observations for this local experiment only. No balance
            // report, caller assertion or journal summary establishes these in production.
            let holds =
                (input.evidence == EvidenceScenario::Complete).then_some(FundingDispatchHolds {
                    operating_reserve: reserve,
                    other_liabilities: input.other_liabilities,
                });
            FundingDispatchEvidence { attempt, holds }
        },
        transport::limits(),
    )
    .await
    .map_err(funding::failure)?;
    Ok(match result {
        FundingDispatchResult::Blocked {
            attempt,
            holds_unknown,
        } => DispatchResult::Blocked {
            blockers: admission::attempt_blockers(attempt.blockers()),
            holds_unknown,
        },
        FundingDispatchResult::Settled {
            observation,
            call_cost,
        } => DispatchResult::Settled(transport::present(*observation, call_cost)),
    })
}
