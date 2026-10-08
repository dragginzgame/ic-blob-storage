//! Passive journal/platform observations and boundary projection; no transfer.
use blob_test_protocol::funding::preview::{
    FundingPreviewBlocker as Blocker, FundingPreviewFailure, FundingPreviewRequest,
    FundingPreviewView,
};
use candid::Principal;
use ic_blob_storage::policy::billing::admission::FundingAdmissionBlocker;
use ic_blob_storage::policy::billing::admission::evidence::FundingAdmissionAssessment;
use ic_blob_storage::policy::billing::admission::evidence::FundingEvidenceBlocker;
use ic_blob_storage_contracts::funding::transfer::FundingTransfer;
use std::num::NonZeroU128;

pub(crate) struct FundingPreviewSnapshot {
    pub liquidity: ic_blob_storage::policy::billing::liquidity::FundingLiquidity,
    pub budget: crate::model::budget::FundingBudgetSnapshot,
    pub identity: crate::model::FundingIdentityView,
    pub fenced: bool,
    pub transfers: Vec<FundingTransfer>,
    pub requested_cycles: NonZeroU128,
}

pub(crate) fn snapshot(
    service: Principal,
    caller: Principal,
    request: FundingPreviewRequest,
) -> Result<FundingPreviewSnapshot, FundingPreviewFailure> {
    super::read(|record| {
        let identity = record.preview_identity(service, caller, request.peer, request.id)?;
        if request.service != service {
            return Err(FundingPreviewFailure::Binding);
        }
        let requested_cycles = NonZeroU128::new(request.requested_cycles)
            .ok_or(FundingPreviewFailure::InvalidAmount)?;
        Ok(FundingPreviewSnapshot {
            liquidity: super::liquidity::prepare(
                request.peer,
                super::liquidity::preview_request(request),
            )
            .liquidity,
            budget: record.budget(),
            identity,
            fenced: record.fenced(),
            requested_cycles,
            transfers: record
                .all_attempts()
                .iter()
                .map(super::status::transfer)
                .collect(),
        })
    })
}

pub(crate) fn view(
    request: FundingPreviewRequest,
    snapshot: &FundingPreviewSnapshot,
    assessment: &FundingAdmissionAssessment,
    liquidity: ic_blob_storage::policy::billing::liquidity::FundingLiquidityDecision,
) -> FundingPreviewView {
    let mut blockers = vec![Blocker::ProviderUnqualified];
    if snapshot.identity.already_admitted {
        blockers.push(Blocker::AlreadyAdmitted);
    }
    if snapshot.identity.journal_full {
        blockers.push(Blocker::JournalFull);
    }
    if request.requested_cycles > crate::model::MAX_OFFERED {
        blockers.push(Blocker::AmountLimitExceeded {
            maximum_cycles: crate::model::MAX_OFFERED,
        });
    }
    if request.revision != snapshot.budget.revision {
        blockers.push(Blocker::BudgetRevisionStale);
    }
    if request.requested_cycles > snapshot.budget.transferable() {
        blockers.push(Blocker::BudgetReserveWouldBeViolated {
            transferable_cycles: snapshot.budget.transferable(),
        });
    }
    blockers.extend(assessment.blockers().iter().copied().map(blocker));
    if let ic_blob_storage::policy::billing::liquidity::FundingLiquidityDecision::Insufficient {
        transferable_cycles,
    } = liquidity
    {
        blockers.push(Blocker::LiquidityWouldBeViolated {
            transferable_cycles,
        });
    }
    FundingPreviewView {
        request,
        available_cycles: None,
        budget: budget_view(&snapshot.budget),
        liquidity: blob_test_protocol::funding::preview::FundingLiquidityView {
            liquid_cycles: snapshot.liquidity.liquid_cycles,
            call_cost: snapshot.liquidity.call_cost,
        },
        blockers,
    }
}

pub(crate) fn budget_view(
    value: &crate::model::budget::FundingBudgetSnapshot,
) -> blob_test_protocol::funding::budget::FundingBudgetView {
    blob_test_protocol::funding::budget::FundingBudgetView {
        operating_reserve: value.operating_reserve,
        other_liabilities: value.other_liabilities,
        allocated: value.allocated,
        reserve: value.reserve,
        revision: value.revision,
        available: value.available,
        accepted: value.accepted,
        refunded: value.refunded,
        not_enqueued: value.not_enqueued,
        reserved_or_uncertain: value.reserved_or_uncertain,
    }
}

fn blocker(value: FundingEvidenceBlocker) -> Blocker {
    match value {
        FundingEvidenceBlocker::RecoveryUnknown => Blocker::RecoveryUnknown,
        FundingEvidenceBlocker::SpendabilityUnknown => Blocker::SpendabilityUnknown,
        FundingEvidenceBlocker::FundingUnknown => Blocker::FundingUnknown,
        FundingEvidenceBlocker::Admission(value) => match value {
            FundingAdmissionBlocker::RecoveryFenced => Blocker::RecoveryFenced,
            FundingAdmissionBlocker::FundingInProgress => Blocker::FundingInProgress,
            FundingAdmissionBlocker::FundingUncertain => Blocker::FundingUncertain,
            FundingAdmissionBlocker::NotConfigured => Blocker::NotConfigured,
            FundingAdmissionBlocker::ReserveWouldBeViolated {
                requested_cycles,
                transferable_cycles,
            } => Blocker::ReserveWouldBeViolated {
                requested_cycles: requested_cycles.get(),
                transferable_cycles,
            },
        },
    }
}
