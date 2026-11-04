//! Service-owned report fields and admission; instrumentation owns its recorders.
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
impl<'de> Deserialize<'de> for CounterMetrics {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(remote = "CounterMetrics", deny_unknown_fields)]
        struct Fields {
            total: u128,
            total_overflowed: bool,
        }
        let value = Fields::deserialize(decoder)?;
        if value.total_overflowed && value.total != u128::MAX {
            return Err(serde::de::Error::custom(
                "overflow flag disagrees with numeric state",
            ));
        }
        Ok(value)
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
impl<'de> Deserialize<'de> for AggregateMetrics {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(remote = "AggregateMetrics", deny_unknown_fields)]
        struct Fields {
            samples: u64,
            total: u128,
            maximum: Option<u64>,
            samples_overflowed: bool,
            total_overflowed: bool,
        }
        let value = Fields::deserialize(decoder)?;
        if value.samples == 0 {
            return if value.total == 0
                && value.maximum.is_none()
                && !value.samples_overflowed
                && !value.total_overflowed
            {
                Ok(value)
            } else {
                Err(serde::de::Error::custom(
                    "nonempty state without observations",
                ))
            };
        }
        let Some(maximum) = value.maximum else {
            return Err(serde::de::Error::custom("observations without a maximum"));
        };
        if (value.samples_overflowed && value.samples != u64::MAX)
            || (value.total_overflowed && (value.total != u128::MAX || !value.samples_overflowed))
        {
            return Err(serde::de::Error::custom(
                "overflow flag disagrees with numeric state",
            ));
        }
        // The product of two u64 fields always fits in u128.
        if value.total < u128::from(maximum)
            || (maximum == 0 && value.total != 0)
            || (!value.samples_overflowed
                && value.total > u128::from(value.samples) * u128::from(maximum))
        {
            return Err(serde::de::Error::custom(
                "total disagrees with count or maximum",
            ));
        }
        Ok(value)
    }
}
