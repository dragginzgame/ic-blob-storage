//! Private fixture controls shared by local test canisters and the host harness.
//! These are not a production service API or an independently defined provider contract.

use serde::Deserialize;

pub mod admission;
pub mod consumer;
pub mod content;
pub mod funding;
pub mod journey;
pub mod source;
pub mod storage;

/// Deliberate source behavior for one local gateway-list experiment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub enum SourceMode {
    /// Return the configured member.
    Valid,
    /// Hold one captured gateway-list reply until the driver releases it.
    Hold,
    /// Return bytes that cannot decode as a principal list.
    Malformed,
    /// Exceed the probe's explicit reply byte budget.
    Oversized,
    /// Return an invalid empty membership candidate.
    Empty,
    /// Reject the inter-canister call.
    Reject,
}

/// Bounded observations of calls made to the controlled source.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct SourceObservation {
    /// Number of list requests observed, used to check denial before effects.
    pub requests: u64,
}

pub mod balance;

pub mod fixture_wire;
pub mod provider_candid;

/// Bounded same-release probe journals; excluded unless a fixture selects them.
#[cfg(feature = "fixture-storage")]
pub mod fixture_storage;
