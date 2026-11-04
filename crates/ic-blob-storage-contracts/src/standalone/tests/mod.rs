use super::*;
use crate::{dto::configuration::HostFailure, wire};

pub(super) fn maximal_report() -> MetricsReport {
    let counter = CounterMetrics {
        total: u128::MAX,
        total_overflowed: true,
    };
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
            instructions: AggregateMetrics {
                samples: u64::MAX,
                total: u128::MAX,
                maximum: Some(u64::MAX),
                samples_overflowed: true,
                total_overflowed: true,
            },
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
        attempts: CounterMetrics {
            total: 0,
            total_overflowed: false,
        },
        applied: CounterMetrics {
            total: 0,
            total_overflowed: false,
        },
        replayed: CounterMetrics {
            total: 0,
            total_overflowed: false,
        },
        refused: CounterMetrics {
            total: 0,
            total_overflowed: false,
        },
        provider_failed: CounterMetrics {
            total: 0,
            total_overflowed: false,
        },
        acknowledged: CounterMetrics {
            total: 0,
            total_overflowed: false,
        },
        request_bytes: CounterMetrics {
            total: 0,
            total_overflowed: false,
        },
        instructions: AggregateMetrics {
            samples: 0,
            total: 0,
            maximum: None,
            samples_overflowed: false,
            total_overflowed: false,
        },
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
        let dto = CounterMetrics {
            total: u128::MAX,
            total_overflowed: overflowed,
        };
        assert_eq!(dto.total, u128::MAX);
        assert_eq!(dto.total_overflowed, overflowed);
        let json = serde_json::to_string(&dto).unwrap();
        assert_eq!(
            json,
            format!(
                "{{\"total\":{},\"total_overflowed\":{overflowed}}}",
                u128::MAX
            )
        );
        assert_eq!(serde_json::from_str::<CounterMetrics>(&json).unwrap(), dto);
    }
    let empty = AggregateMetrics {
        samples: 0,
        total: 0,
        maximum: None,
        samples_overflowed: false,
        total_overflowed: false,
    };
    let zero = AggregateMetrics {
        samples: 1,
        maximum: Some(0),
        ..empty
    };
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

#[test]
fn report_numeric_admission_preserves_refusal_priority_and_strict_fields() {
    let empty = AggregateMetrics {
        samples: 0,
        total: 0,
        maximum: None,
        samples_overflowed: false,
        total_overflowed: false,
    };
    let one = AggregateMetrics {
        samples: 1,
        maximum: Some(1),
        total: 1,
        ..empty
    };
    for (invalid, message) in [
        (
            AggregateMetrics {
                total: 1,
                total_overflowed: true,
                ..empty
            },
            "nonempty state without observations",
        ),
        (
            AggregateMetrics {
                samples: 1,
                samples_overflowed: true,
                ..empty
            },
            "observations without a maximum",
        ),
        (
            AggregateMetrics {
                samples_overflowed: true,
                ..one
            },
            "overflow flag disagrees with numeric state",
        ),
        (
            AggregateMetrics {
                total_overflowed: true,
                ..one
            },
            "overflow flag disagrees with numeric state",
        ),
        (
            AggregateMetrics { total: 0, ..one },
            "total disagrees with count or maximum",
        ),
        (
            AggregateMetrics { total: 2, ..one },
            "total disagrees with count or maximum",
        ),
        (
            AggregateMetrics {
                maximum: Some(0),
                ..one
            },
            "total disagrees with count or maximum",
        ),
    ] {
        let value = serde_json::to_value(invalid).unwrap();
        assert!(
            serde_json::from_value::<AggregateMetrics>(value)
                .unwrap_err()
                .to_string()
                .contains(message)
        );
        let bytes = crate::document::encode(invalid).unwrap();
        assert!(crate::document::decode::<AggregateMetrics>(&bytes, 4096).is_err());
    }
    let mut extra = serde_json::to_value(one).unwrap();
    extra["extra"] = 0.into();
    assert!(serde_json::from_value::<AggregateMetrics>(extra).is_err());
    let invalid = CounterMetrics {
        total: 0,
        total_overflowed: true,
    };
    assert!(
        serde_json::from_value::<CounterMetrics>(serde_json::to_value(invalid).unwrap())
            .unwrap_err()
            .to_string()
            .contains("overflow flag disagrees with numeric state")
    );
}
