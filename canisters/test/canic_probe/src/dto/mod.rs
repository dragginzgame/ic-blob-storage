//! Passive managed boundary data; the core never depends on Canic.
use candid::{CandidType, Deserialize};
/// A managed reply keeps the exact shared error's Candid shape.
/// Artifact-owned ops converts generic Canic refusals; domain errors remain unchanged.
#[derive(CandidType, Deserialize)]
pub struct TransportFailure<E>(pub E);
