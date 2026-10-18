//! Local observer fixture controls; the service request remains the shared DTO.
use ic_blob_storage_contracts::dto::funding::FundingHistoryRequest;
use ic_platform::candid::contract::CandidType;
use ic_platform::candid::contract::serde::Deserialize;
/// One bounded history inspection, with no client persistence or mutation.
#[derive(Clone, Copy, Debug, CandidType, Deserialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct FundingHistoryInspection {
    /// Exact shared request.
    pub request: FundingHistoryRequest,
    /// Decoder budget, used to exercise discarded replies.
    pub max_reply_bytes: u32,
}
