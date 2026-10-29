//! One authenticated standalone telemetry query; no retries, resets or provider reads.
use super::{Failure, arguments::Options};
use ic_blob_storage_contracts::{
    dto::configuration::HostFailure,
    standalone::{METRICS_METHOD, METRICS_WIRE, MetricOperation, MetricsReport},
    wire,
};
use ic_metrics::MeasurementCounter;
use ic_platform::Principal;
use serde_json::{Value, json};

pub(super) async fn run(
    options: &Options,
    service: Principal,
    namespace: u128,
) -> Result<Value, Failure> {
    let bytes = super::query(
        options,
        service,
        METRICS_METHOD.name,
        wire::encode_request(METRICS_METHOD.name, ()).map_err(|_| Failure::Arguments)?,
    )
    .await?;
    output(service, namespace, &bytes)
}
fn counter(value: MeasurementCounter) -> Value {
    json!({"total":value.total().to_string(), "saturated":value.total_overflowed()})
}
fn output(service: Principal, namespace: u128, bytes: &[u8]) -> Result<Value, Failure> {
    let reply: Result<MetricsReport, HostFailure> =
        wire::decode_response(METRICS_METHOD.name, bytes, METRICS_WIRE.response.bytes)
            .map_err(|_| Failure::InvalidReply)?;
    let report = reply.map_err(|_| Failure::Denied)?;
    if report.service != service || report.namespace != namespace {
        return Err(Failure::Binding);
    }
    if report
        .operations
        .iter()
        .zip(MetricOperation::ALL)
        .any(|(row, operation)| row.operation != operation)
    {
        return Err(Failure::InvalidReply);
    }
    let operations: Vec<_> = report.operations.iter().map(|row| json!({
        "operation":row.operation,"attempts":counter(row.attempts),"applied":counter(row.applied),
        "replayed":counter(row.replayed),"refused":counter(row.refused),"provider_failed":counter(row.provider_failed),
        "acknowledged":counter(row.acknowledged),"request_bytes":counter(row.request_bytes),
        "instructions":{"samples":row.instructions.samples().to_string(),"samples_saturated":row.instructions.samples_overflowed(),
            "total":row.instructions.total().to_string(),"total_saturated":row.instructions.total_overflowed(),
            "maximum":row.instructions.maximum().map(|value| value.to_string())},
    })).collect();
    Ok(
        json!({"schema":1,"observation":"standalone_heap_metrics","service":service.to_text(),"namespace":namespace.to_string(),
        "window_version":report.window_version.to_string(),"observed_version":report.observed_version.to_string(),
        "started_at_ns":report.started_at_ns.to_string(),"observed_at_ns":report.observed_at_ns.to_string(),"operations":operations}),
    )
}

#[cfg(test)]
mod tests;
