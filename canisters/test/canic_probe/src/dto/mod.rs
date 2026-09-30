//! Passive managed boundary data; the core never depends on Canic.
use candid::{CandidType, Deserialize};
/// A managed reply keeps the exact shared error's Candid shape.
/// Artifact-owned ops converts generic Canic refusals; domain errors remain unchanged.
#[derive(CandidType, Deserialize)]
pub struct TransportFailure<E>(pub E);

/// Refusal from the local exposure-state substitute, never a provider result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ProbeExposureFailure {
    /// Only the installed operator may configure this controlled fixture cut.
    Denied,
    /// Malformed identity or a foreign service/namespace binding.
    Binding,
    /// The supplied permission differs from the original retained permission.
    Conflict,
    /// Local phase, manifest, enrollment, clock or restore fence refused the cut.
    State,
}
