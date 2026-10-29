use super::*;
use ic_blob_storage_contracts::standalone::OperationMetrics;
use ic_metrics::MeasurementAggregate;

fn report() -> MetricsReport {
    let exact = MeasurementCounter::from_parts(u128::MAX, false).unwrap();
    MetricsReport {
        service: Principal::from_slice(&[1; 29]),
        namespace: u128::MAX,
        window_version: 4,
        observed_version: 5,
        started_at_ns: 100,
        observed_at_ns: 200,
        operations: MetricOperation::ALL.map(|operation| OperationMetrics {
            operation,
            attempts: exact,
            applied: exact,
            replayed: exact,
            refused: exact,
            provider_failed: exact,
            acknowledged: exact,
            request_bytes: exact,
            instructions: MeasurementAggregate::EMPTY,
        }),
    }
}
fn packet(report: MetricsReport) -> Vec<u8> {
    wire::encode_response(METRICS_METHOD.name, Ok::<_, HostFailure>(report)).unwrap()
}
#[test]
fn report_binds_scope_roster_denial_and_full_width_decimal_output() {
    let report = report();
    let json = output(report.service, report.namespace, &packet(report.clone())).unwrap();
    assert_eq!(
        json["operations"][0]["attempts"]["total"],
        u128::MAX.to_string()
    );
    assert_eq!(
        json["operations"][0]["instructions"]["maximum"],
        Value::Null
    );
    assert_eq!(json["window_version"], "4");
    assert_eq!(json["observed_version"], "5");
    assert_eq!(
        output(report.service, 1, &packet(report.clone())),
        Err(Failure::Binding)
    );
    let mut malformed = report.clone();
    malformed.operations[1].operation = MetricOperation::Admission;
    assert_eq!(
        output(report.service, report.namespace, &packet(malformed)),
        Err(Failure::InvalidReply)
    );
    let denied = wire::encode_response(
        METRICS_METHOD.name,
        Err::<MetricsReport, _>(HostFailure::Denied),
    )
    .unwrap();
    assert_eq!(
        output(report.service, report.namespace, &denied),
        Err(Failure::Denied)
    );
}
