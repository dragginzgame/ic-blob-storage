//! Query the active owner only; never reload older stable evidence or call the peer.
use blob_test_protocol::funding::lookup::{
    FundingLookupFailure, FundingLookupRequest, FundingLookupState, FundingLookupView,
};
use candid::Principal;

pub(crate) fn lookup(
    caller: Principal,
    request: FundingLookupRequest,
) -> Result<FundingLookupView, FundingLookupFailure> {
    super::read(|record| {
        let entry = record.lookup(caller, request)?;
        Ok(FundingLookupView {
            request,
            fenced: record.fenced(),
            state: match entry {
                None => FundingLookupState::Absent,
                Some(entry) => match entry.observation {
                    None => FundingLookupState::Pending,
                    Some(observation) => FundingLookupState::Observed(observation),
                },
            },
        })
    })
}
