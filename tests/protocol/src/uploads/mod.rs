//! Passive outcomes of fixed local upload fixtures, not a service/provider API.

use serde::Deserialize;

/// Coarse local observation of a fixed fixture root; no deletion authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub enum UploadProbeState {
    /// Capacity reserved, no possible exposure recorded.
    Reserved,
    /// Cancelled before exposure, with retained root history.
    Cancelled,
    /// Confirmed fixture object; says nothing about a deployed provider.
    Registered,
}
