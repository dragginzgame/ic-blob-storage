//! Passive admission assessment; the raw transfer experiment is not operator funding.
use crate::ops;
use blob_test_protocol::funding::preview::{
    FundingPreviewFailure, FundingPreviewRequest, FundingPreviewView,
};
use candid::Principal;
use ic_blob_storage::policy::billing::{
    RecoveryState,
    admission::evidence::{FundingAdmissionEvidence, assess_funding_evidence},
    reconciliation::{assess_funding_reconciliation, assess_uncredited_activity},
};

pub(crate) fn preview(
    service: Principal,
    caller: Principal,
    request: FundingPreviewRequest,
) -> Result<FundingPreviewView, FundingPreviewFailure> {
    let snapshot = ops::preview::snapshot(service, caller, request)?;
    let activity = assess_uncredited_activity(
        snapshot
            .transfers
            .iter()
            .copied()
            .map(assess_funding_reconciliation),
    );
    let assessment = assess_funding_evidence(
        None,
        snapshot.requested_cycles,
        FundingAdmissionEvidence {
            available_cycles: None,
            recovery: snapshot.fenced.then_some(RecoveryState::Fenced),
            activity: Some(activity),
        },
    );
    let liquidity = ic_blob_storage::policy::billing::liquidity::assess_funding_liquidity(
        snapshot.requested_cycles,
        snapshot.liquidity,
    );
    Ok(ops::preview::view(
        request,
        &snapshot,
        &assessment,
        liquidity,
    ))
}
