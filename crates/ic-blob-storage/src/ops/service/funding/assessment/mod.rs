//! Conversion of proposed ingress and passive host facts, with no effect authority.
use crate::{
    dto::funding::assessment::{
        FundingPreparationBlocker as D, FundingPreparationFailure as E, FundingPreparationRequest,
        FundingPreparationResponse,
    },
    model::billing::journal::{FundingIntent, FundingIntentError, FundingJournalScope},
    ops::service::{
        funding::{FundingJournalError, summary::FundingJournalSummary},
        operator::funding_status,
    },
    policy::billing::admission::{
        evidence::{FundingAdmissionEvidence, FundingEvidenceBlocker},
        journal::{FundingHostEvidence, FundingPreparationBlocker as B},
    },
};
use std::num::NonZeroU128;

/// Canonical passive service query; linking exports no endpoint.
pub const FUNDING_PREPARATION_METHOD: &str = "blob_funding_preparation_assessment";
pub(crate) fn input(
    request: FundingPreparationRequest,
) -> Result<(FundingIntent, FundingHostEvidence), E> {
    let positive = |v| NonZeroU128::new(v).ok_or(E::Invalid);
    let intent = FundingIntent {
        service: request.scope.service,
        cashier: request.scope.cashier,
        account: request.scope.payment_account,
        namespace: positive(request.scope.namespace)?,
        operation: positive(request.operation)?,
        offered: positive(request.offered)?,
        target_balance: request.target_balance.map(positive).transpose()?,
    };
    // These hosts have no authoritative production acquisition path. Observed
    // provider balances or zero local obligations cannot establish these facts.
    let host = FundingHostEvidence {
        scope: FundingJournalScope {
            service: intent.service,
            cashier: intent.cashier,
            account: intent.account,
            namespace: intent.namespace,
        },
        provider_qualified: false,
        admission: FundingAdmissionEvidence {
            available_cycles: None,
            recovery: None,
            activity: None,
        },
    };
    Ok((intent, host))
}
pub(crate) fn failure(error: FundingJournalError) -> E {
    match error {
        FundingJournalError::Binding => E::Binding,
        FundingJournalError::NotOperator => E::Denied,
        FundingJournalError::Intent(FundingIntentError::Conflict) => E::Conflict,
        _ => E::Internal,
    }
}
fn blocker(b: B) -> Result<D, E> {
    Ok(match b {
        B::ProviderUnqualified => D::ProviderUnqualified,
        B::JournalFenced => D::JournalFenced,
        B::JournalUncredited => D::JournalUncredited,
        B::IdentityRetained => D::IdentityRetained,
        B::IdentityStale => D::IdentityStale,
        B::JournalFull => D::JournalFull,
        B::AllocationReserve {
            transferable_cycles,
        } => D::AllocationReserve {
            transferable_cycles,
        },
        B::Evidence(FundingEvidenceBlocker::RecoveryUnknown) => D::RecoveryUnknown,
        B::Evidence(FundingEvidenceBlocker::FundingUnknown) => D::FundingUnknown,
        B::Evidence(FundingEvidenceBlocker::SpendabilityUnknown) => D::SpendabilityUnknown,
        // Known host admission facts cannot appear in the current passive profile.
        B::Evidence(FundingEvidenceBlocker::Admission(_)) => return Err(E::Internal),
    })
}
pub(crate) fn present(
    request: FundingPreparationRequest,
    journal: &FundingJournalSummary,
    blockers: &[B],
) -> Result<FundingPreparationResponse, E> {
    Ok(FundingPreparationResponse {
        request,
        journal: funding_status(journal),
        blockers: blockers
            .iter()
            .copied()
            .map(blocker)
            .collect::<Result<_, _>>()?,
    })
}
