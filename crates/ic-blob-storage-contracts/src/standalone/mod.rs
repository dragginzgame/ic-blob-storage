//! Standalone-only operator telemetry. Embedded hosts retain their own reporting policy.
use crate::endpoints::{MethodCodec, MethodMode, MethodSpec, WireContract};
use ic_metrics::{MeasurementAggregate, MeasurementCounter};
use ic_platform::Principal;
use serde::{Deserialize, Serialize};

/// Extra standalone query; deliberately absent from the shared service mount/catalog.
pub const METRICS_METHOD: MethodSpec = MethodSpec {
    name: "blob_metrics",
    mode: MethodMode::Query,
    codec: MethodCodec::IcWire,
};
/// Fixed-roster reporting budgets, including the Wire header.
pub const METRICS_WIRE: WireContract = WireContract {
    id: 0x424c_4f42_0042_fe01,
    request: ic_wire::Limits {
        bytes: 64,
        allocation: 128,
        nodes: 8,
        depth: 4,
    },
    response: ic_wire::Limits {
        bytes: 4096,
        allocation: 262_144,
        nodes: 2048,
        depth: 12,
    },
};
/// Fixed operations observed by the standalone host; never derive labels from ingress.
#[cfg_attr(
    all(feature = "wire-schema", not(target_arch = "wasm32")),
    derive(ic_wire::Wire)
)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MetricOperation {
    /// Project-authorized upload admission.
    Admission,
    /// Uploader manifest preparation.
    Manifest,
    /// Certificate issuance and atomic local retention registration.
    Registration,
    /// Original reference command and receipt.
    Reference,
    /// Gateway deletion acknowledgment, never evidence of deletion or billing cessation.
    GatewayAcknowledgment,
    /// Bounded provider membership query; no payment is sent.
    GatewaySync,
}
impl MetricOperation {
    /// Complete bounded roster in report order.
    pub const ALL: [Self; 6] = [
        Self::Admission,
        Self::Manifest,
        Self::Registration,
        Self::Reference,
        Self::GatewayAcknowledgment,
        Self::GatewaySync,
    ];
}
/// One fixed update path. Traps and query execution cannot promise retained observations.
#[cfg_attr(
    all(feature = "wire-schema", not(target_arch = "wasm32")),
    derive(ic_wire::Wire)
)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperationMetrics {
    /// Fixed operation identity.
    pub operation: MetricOperation,
    /// Decoded attempts that reached the workflow boundary, including incomplete async work.
    pub attempts: MeasurementCounter,
    /// Fresh successful local mutations with explicit domain evidence of novelty.
    pub applied: MeasurementCounter,
    /// Original receipt/declaration replays; never another applied operation.
    pub replayed: MeasurementCounter,
    /// Normally returned typed refusals, including failed original reference receipts.
    pub refused: MeasurementCounter,
    /// Normally returned provider query submission, rejection or reply-admission failures.
    pub provider_failed: MeasurementCounter,
    /// Successful sync/GC callback returns with no claim of a new logical effect.
    pub acknowledged: MeasurementCounter,
    /// Encoded argument bytes for the observed attempts, not uploaded content bytes.
    pub request_bytes: MeasurementCounter,
    /// Completed samples in this window; interrupted work can leave unmatched attempts.
    pub instructions: MeasurementAggregate,
}
/// Authenticated heap telemetry, never accounting, retry or restore authority.
#[cfg_attr(
    all(feature = "wire-schema", not(target_arch = "wasm32")),
    derive(ic_wire::Wire)
)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MetricsReport {
    /// Actual service, not a caller-supplied identity.
    pub service: Principal,
    /// Installed local namespace; not provider provisioning evidence.
    pub namespace: u128,
    /// Platform version at this heap window's start.
    pub window_version: u64,
    /// Current platform version; ordinary updates advance it. Not a restore detector.
    pub observed_version: u64,
    /// Platform timestamp of window initialization/reset.
    pub started_at_ns: u64,
    /// Platform timestamp of the read-only report.
    pub observed_at_ns: u64,
    /// Fixed roster; no principal, tenant, object or arbitrary error dimensions.
    pub operations: [OperationMetrics; 6],
}

#[cfg(test)]
mod tests;
