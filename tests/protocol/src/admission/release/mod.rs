//! Local measurement controls. Operator-supplied facts do not qualify Caffeine.
use super::Request;
use candid::CandidType;
use serde::Deserialize;

/// In-process dispatch for a single shared-owner mutation, never a wire envelope.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleCommand {
    /// Operator supplies a substitute completion fact after local exposure.
    SubstituteCompletion(Request),
    /// Operator supplies a substitute physical-deletion fact.
    SubstituteDeletion(Request),
    /// Operator supplies a separate substitute billing-cessation fact.
    SubstituteSettlement(Request),
    /// Tenant retains/releases one exact reference under an exact request ID.
    Reference {
        /// Original object binding; this fixture fixes incarnation to one.
        object: Request,
        /// Nonzero lifetime reference ID.
        reference: u128,
        /// Nonzero receipt ID; exact retries preserve every argument.
        operation: u128,
        /// True retains; false releases.
        retain: bool,
    },
}

/// Typed inner lifecycle results, distinct from receipt admission errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ReferenceFailure {
    /// Reference belongs to a different object binding.
    Binding,
    /// Released identity cannot be reused.
    Released,
    /// Unknown identity cannot release another reference.
    Unknown,
    /// Lifetime reference slots are exhausted.
    Capacity,
    /// Current lifecycle forbids the operation.
    Phase,
}

/// Passive tenant-authorized history headroom, not an admission reservation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ReferenceCapacity {
    /// Unused lifetime reference identities.
    pub reference_slots: u64,
    /// Unreserved receipt slots.
    pub unreserved_receipts: u64,
    /// Cleanup slots reserved for active references.
    pub release_reserved_receipts: u64,
    /// Fresh distinct retains possible at observation time.
    pub fresh_retains: u64,
}
