//! Authority fields shared by setup intent writers and recovery readers.
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Existing flat JSON fields; observational metadata is not recovery authority.
#[derive(Deserialize, Serialize)]
pub(in crate::native) struct SetupIntentRecord {
    pub schema: u64,
    pub network: String,
    pub service_url: String,
    pub actor: String,
    pub service: String,
    pub namespace: String,
    pub permission: Value,
    pub method: String,
    pub request_sha256: String,
}
