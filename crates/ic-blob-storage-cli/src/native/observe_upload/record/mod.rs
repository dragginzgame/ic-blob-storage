//! Fresh, no-clobber observation artifacts. Partial runs are never resumed automatically.
//! String parameters permit borrowed writes and owned strict reads of the same schema.
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(in crate::native) struct ObservationPlanRecord<S> {
    pub schema: u8,
    pub runner_version: S,
    pub runner_source_sha256: String,
    pub network: S,
    pub service_url: S,
    pub service: String,
    pub namespace: String,
    pub verifier: String,
    pub gateway_origin: S,
    pub max_content_bytes: String,
    pub max_provider_requests: u8,
    pub request_deadline_seconds: u8,
    pub provider_charges: S,
    pub started_at_ns: String,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(in crate::native) struct DownloadRequestRecord<S> {
    pub method: S,
    pub url: S,
    pub service_reply_sha256: String,
    pub owner: String,
    pub project: S,
    pub bytes: String,
    pub started_at_ns: String,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(in crate::native) struct ObservationSummaryRecord<S> {
    pub schema: u8,
    pub observation: S,
    pub network: S,
    pub service: String,
    pub namespace: String,
    pub verifier: String,
    pub owner: String,
    pub project: S,
    pub gateway_origin: S,
    pub root: String,
    pub bytes: String,
    pub content_digest: String,
    pub observed_at_ns: String,
    pub statement_sha256: String,
    pub authentication: S,
    pub attestation_dispatched: bool,
    pub retry_authorized: bool,
    pub future_retention: S,
    pub billing_cessation: S,
}
