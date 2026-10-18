//! Local observer fixture controls; the service request remains the shared DTO.
use ic_blob_storage_contracts::dto::funding::FundingHistoryRequest;
use serde::{Deserialize, Serialize};
/// One bounded history inspection, with no client persistence or mutation.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FundingHistoryInspection {
    /// Exact shared request.
    pub request: FundingHistoryRequest,
    /// Decoder budget, used to exercise discarded replies.
    pub max_reply_bytes: u32,
}
