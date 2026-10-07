//! Local observer fixture controls; the service request remains the shared DTO.
use candid::{CandidType, Deserialize};
use ic_blob_storage::dto::funding::FundingHistoryRequest;
/// One bounded history inspection, with no client persistence or mutation.
#[derive(Clone, Copy, Debug, CandidType, Deserialize)]
pub struct FundingHistoryInspection {
    /// Exact shared request.
    pub request: FundingHistoryRequest,
    /// Decoder budget, used to exercise discarded replies.
    pub max_reply_bytes: u32,
}
