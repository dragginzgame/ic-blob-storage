//! Fixed heap telemetry using upstream arithmetic; no stable backing or business authority.
use ic_blob_storage_contracts::standalone::{MetricOperation, MetricsReport, OperationMetrics};
use ic_metrics::{MeasurementAggregate, MeasurementCounter};
use std::cell::RefCell;

#[derive(Clone, Copy)]
struct Row {
    attempts: MeasurementCounter,
    outcomes: [MeasurementCounter; 5],
    bytes: MeasurementCounter,
    instructions: MeasurementAggregate,
}
impl Row {
    const EMPTY: Self = Self {
        attempts: MeasurementCounter::ZERO,
        outcomes: [MeasurementCounter::ZERO; 5],
        bytes: MeasurementCounter::ZERO,
        instructions: MeasurementAggregate::EMPTY,
    };
    fn view(self, operation: MetricOperation) -> OperationMetrics {
        OperationMetrics {
            operation,
            attempts: self.attempts,
            applied: self.outcomes[0],
            replayed: self.outcomes[1],
            refused: self.outcomes[2],
            provider_failed: self.outcomes[3],
            acknowledged: self.outcomes[4],
            request_bytes: self.bytes,
            instructions: self.instructions,
        }
    }
}
pub(crate) enum Outcome {
    Applied,
    Replayed,
    Refused,
    ProviderFailed,
    Acknowledged,
}
/// Exact local measurement interval; a callback cannot enter another version's window.
pub(crate) struct Started {
    operation: MetricOperation,
    version: u64,
    instructions: u64,
}
struct Metrics {
    version: u64,
    started_at: u64,
    rows: [Row; 6],
}
impl Metrics {
    const fn new(version: u64, started_at: u64) -> Self {
        Self {
            version,
            started_at,
            rows: [Row::EMPTY; 6],
        }
    }
    fn begin(&mut self, operation: MetricOperation, bytes: u128) {
        let row = &mut self.rows[operation as usize];
        row.attempts.add(1);
        row.bytes.add(bytes);
    }
    fn complete(
        &mut self,
        operation: MetricOperation,
        version: u64,
        instructions: u64,
        outcome: Outcome,
    ) {
        if version != self.version {
            return;
        }
        let row = &mut self.rows[operation as usize];
        row.outcomes[outcome as usize].add(1);
        row.instructions.record(instructions);
    }
}
thread_local! { static METRICS: RefCell<Metrics> = const { RefCell::new(Metrics::new(0, 0)) }; }
pub(crate) fn reset() {
    METRICS.with_borrow_mut(|state| {
        *state = Metrics::new(
            ic_platform::ic0::canister_version(),
            ic_platform::cdk::api::time(),
        );
    });
}
pub(crate) fn begin(operation: MetricOperation, bytes: usize) -> Started {
    let version = METRICS.with_borrow_mut(|state| {
        state.begin(operation, bytes as u128);
        state.version
    });
    Started {
        operation,
        version,
        instructions: ic_platform::cdk::api::call_context_instruction_counter(),
    }
}
#[expect(
    clippy::needless_pass_by_value,
    reason = "consume the measurement token exactly once"
)]
pub(crate) fn complete(started: Started, outcome: Outcome) {
    let instructions = ic_platform::cdk::api::call_context_instruction_counter()
        .saturating_sub(started.instructions);
    METRICS.with_borrow_mut(|state| {
        state.complete(started.operation, started.version, instructions, outcome);
    });
}
pub(crate) fn report(service: ic_platform::Principal, namespace: u128) -> MetricsReport {
    METRICS.with_borrow(|state| MetricsReport {
        service,
        namespace,
        window_version: state.version,
        observed_version: ic_platform::ic0::canister_version(),
        started_at_ns: state.started_at,
        observed_at_ns: ic_platform::cdk::api::time(),
        operations: MetricOperation::ALL
            .map(|operation| state.rows[operation as usize].view(operation)),
    })
}

#[cfg(test)]
mod tests;
