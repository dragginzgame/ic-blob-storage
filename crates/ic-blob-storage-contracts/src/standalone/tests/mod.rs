use super::*;
use crate::{dto::configuration::HostFailure, wire};
use ic_metrics::{MeasurementAggregate, MeasurementCounter};

pub(super) fn maximal_report() -> MetricsReport {
    let counter = MeasurementCounter::from_parts(u128::MAX, true)
        .unwrap()
        .into();
    MetricsReport {
        service: Principal::from_slice(&[1; 29]),
        namespace: u128::MAX,
        window_version: u64::MAX,
        observed_version: u64::MAX,
        started_at_ns: u64::MAX,
        observed_at_ns: u64::MAX,
        operations: MetricOperation::ALL.map(|operation| OperationMetrics {
            operation,
            attempts: counter,
            applied: counter,
            replayed: counter,
            refused: counter,
            provider_failed: counter,
            acknowledged: counter,
            request_bytes: counter,
            instructions: MeasurementAggregate::from_parts(
                u64::MAX,
                u128::MAX,
                Some(u64::MAX),
                true,
                true,
            )
            .unwrap()
            .into(),
        }),
    }
}
#[test]
fn standalone_report_is_bounded_direction_bound_and_does_not_extend_shared_mount() {
    assert!(
        !crate::endpoints::SERVICE_METHODS
            .iter()
            .any(|entry| entry.name == METRICS_METHOD.name)
    );
    assert!(
        crate::endpoints::SERVICE_METHODS
            .iter()
            .filter_map(|entry| entry.wire().ok())
            .all(|contract| contract.id != METRICS_WIRE.id)
    );
    let expected: Result<_, HostFailure> = Ok(maximal_report());
    let bytes = wire::encode_response(METRICS_METHOD.name, &expected).unwrap();
    assert!(bytes.len() < METRICS_WIRE.response.bytes);
    assert_eq!(
        wire::decode_response::<Result<MetricsReport, HostFailure>>(
            METRICS_METHOD.name,
            &bytes,
            METRICS_WIRE.response.bytes
        )
        .unwrap(),
        expected
    );
    assert!(wire::decode_request::<()>(METRICS_METHOD.name, &bytes, 64).is_err());
    assert!(
        wire::decode_response::<Result<MetricsReport, HostFailure>>(
            "blob_configuration",
            &bytes,
            4096
        )
        .is_err()
    );
    let mut trailing = wire::encode_request(METRICS_METHOD.name, ()).unwrap();
    trailing.push(0);
    assert!(wire::decode_request::<()>(METRICS_METHOD.name, &trailing, 64).is_err());
    assert!(wire::decode_request::<()>(METRICS_METHOD.name, &[0; 65], 65).is_err());
}

#[test]
fn report_decoding_rejects_invalid_numeric_quality() {
    let mut report = maximal_report();
    report.namespace = 7;
    report.operations = MetricOperation::ALL.map(|operation| OperationMetrics {
        operation,
        attempts: MeasurementCounter::ZERO.into(),
        applied: MeasurementCounter::ZERO.into(),
        replayed: MeasurementCounter::ZERO.into(),
        refused: MeasurementCounter::ZERO.into(),
        provider_failed: MeasurementCounter::ZERO.into(),
        acknowledged: MeasurementCounter::ZERO.into(),
        request_bytes: MeasurementCounter::ZERO.into(),
        instructions: MeasurementAggregate::EMPTY.into(),
    });
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(
        serde_json::from_value::<MetricsReport>(value.clone()).unwrap(),
        report
    );
    let mut invalid_counter = value.clone();
    invalid_counter["operations"][0]["attempts"]["total_overflowed"] = true.into();
    assert!(serde_json::from_value::<MetricsReport>(invalid_counter).is_err());
    let mut invalid_aggregate = value;
    invalid_aggregate["operations"][0]["instructions"]["total"] = 1.into();
    assert!(serde_json::from_value::<MetricsReport>(invalid_aggregate).is_err());
    for aggregate in [false, true] {
        let mut invalid = report.clone();
        if aggregate {
            invalid.operations[0].instructions.total = 1;
        } else {
            invalid.operations[0].attempts.total_overflowed = true;
        }
        let frame =
            wire::encode_response(METRICS_METHOD.name, Ok::<_, HostFailure>(invalid)).unwrap();
        assert!(
            wire::decode_response::<Result<MetricsReport, HostFailure>>(
                METRICS_METHOD.name,
                &frame,
                METRICS_WIRE.response.bytes,
            )
            .is_err()
        );
    }
}

#[cfg(feature = "wire-schema")]
#[test]
fn standalone_report_composes_service_owned_wire_metadata() {
    ic_wire::typescript::native_schema::<MetricsReport>().unwrap();
}

#[test]
fn report_projection_keeps_the_frozen_prior_metrics_frame() {
    // Captured with Wire 0.15.1 and Metrics 0.27.2 before the DTO/Types cut.
    let bytes = hex::decode(include_str!("prior-metrics.hex").trim()).unwrap();
    let expected = Ok::<_, HostFailure>(maximal_report());
    assert_eq!(
        wire::encode_response(METRICS_METHOD.name, &expected).unwrap(),
        bytes
    );
    assert_eq!(
        wire::decode_response::<Result<MetricsReport, HostFailure>>(
            METRICS_METHOD.name,
            &bytes,
            METRICS_WIRE.response.bytes,
        )
        .unwrap(),
        expected
    );
}

#[test]
fn metric_projections_keep_exact_caps_actual_overflow_and_empty_zero_distinct() {
    for overflowed in [false, true] {
        let canonical = MeasurementCounter::from_parts(u128::MAX, overflowed).unwrap();
        let dto = CounterMetrics::from(canonical);
        assert_eq!(dto.total, u128::MAX);
        assert_eq!(dto.total_overflowed, overflowed);
        let json = serde_json::to_string(&dto).unwrap();
        assert_eq!(json, serde_json::to_string(&canonical).unwrap());
        assert_eq!(serde_json::from_str::<CounterMetrics>(&json).unwrap(), dto);
    }
    let empty = AggregateMetrics::from(MeasurementAggregate::EMPTY);
    let zero = AggregateMetrics::from(
        MeasurementAggregate::from_parts(1, 0, Some(0), false, false).unwrap(),
    );
    assert_eq!(empty.maximum, None);
    assert_eq!(zero.maximum, Some(0));
    assert_ne!(empty, zero);
    for dto in [empty, zero] {
        let json = serde_json::to_value(dto).unwrap();
        assert_eq!(
            serde_json::from_value::<AggregateMetrics>(json).unwrap(),
            dto
        );
    }
}
