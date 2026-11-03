//! Service-owned report fields; Metrics owns recording and numeric admission.
use ic_metrics::{MeasurementAggregate, MeasurementCounter};
use serde::{Deserialize, Deserializer, Serialize};

/// A counter observation, including whether an actual overflow occurred.
#[cfg_attr(
    all(feature = "wire-schema", not(target_arch = "wasm32")),
    derive(ic_wire::Wire)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CounterMetrics {
    /// Saturating accumulated value; the exact cap need not mean overflow.
    pub total: u128,
    /// True only when recording exceeded the representable total.
    pub total_overflowed: bool,
}
impl From<MeasurementCounter> for CounterMetrics {
    fn from(value: MeasurementCounter) -> Self {
        Self {
            total: value.total(),
            total_overflowed: value.total_overflowed(),
        }
    }
}
impl<'de> Deserialize<'de> for CounterMetrics {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        MeasurementCounter::deserialize(decoder).map(Self::from)
    }
}

/// A completed-sample observation; an empty maximum differs from measured zero.
#[cfg_attr(
    all(feature = "wire-schema", not(target_arch = "wasm32")),
    derive(ic_wire::Wire)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AggregateMetrics {
    /// Number of completed samples.
    pub samples: u64,
    /// Saturating instruction sum across completed samples.
    pub total: u128,
    /// Largest sample, absent only for an empty observation.
    pub maximum: Option<u64>,
    /// True when recording exceeded the representable sample count.
    pub samples_overflowed: bool,
    /// True when recording exceeded the representable total.
    pub total_overflowed: bool,
}
impl From<MeasurementAggregate> for AggregateMetrics {
    fn from(value: MeasurementAggregate) -> Self {
        Self {
            samples: value.samples(),
            total: value.total(),
            maximum: value.maximum(),
            samples_overflowed: value.samples_overflowed(),
            total_overflowed: value.total_overflowed(),
        }
    }
}
impl<'de> Deserialize<'de> for AggregateMetrics {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        MeasurementAggregate::deserialize(decoder).map(Self::from)
    }
}
