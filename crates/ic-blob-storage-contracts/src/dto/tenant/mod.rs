//! Explicit tenant enrollment scope and compare-and-set commands.
use candid::CandidType;
use candid::Deserialize;
use candid::Principal;

/// Exact service namespace and tenant; supplied identity alone grants no authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct TenantScope {
    /// Intended running storage service.
    pub service: Principal,
    /// Positive installed local provider namespace.
    pub namespace: u128,
    /// Tenant project, never inferred from the operator or controller.
    pub tenant: Principal,
}

/// Passive current enrollment, also used as an exact update precondition.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct TenantEnrollment {
    /// Positive local activation generation; not a restore-safe allocation authority.
    pub generation: u64,
    /// Fresh admission is enabled; suspension preserves obligations and cleanup.
    pub active: bool,
}

/// Operator-authorized compare-and-set; inspect after a lost reply before deciding again.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct TenantUpdateRequest {
    /// Exact destination and tenant.
    pub scope: TenantScope,
    /// Exact observed enrollment; `None` requires a never-enrolled tenant.
    pub expected: Option<TenantEnrollment>,
    /// Desired admission state; suspended tenants still consume lifetime capacity.
    pub active: bool,
}

/// Correlated observation, not a durable operation receipt or upload permission.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct TenantEnrollmentResponse {
    /// Original requested service, namespace and tenant.
    pub scope: TenantScope,
    /// Current enrollment, absent only for a never-enrolled tenant.
    pub enrollment: Option<TenantEnrollment>,
    /// This owner is inspection-only after restoration.
    pub fenced: bool,
}

/// Typed refusal without granting authority or erasing tenant history.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum TenantFailure {
    /// Wrong running service or installed namespace.
    Binding,
    /// Mutation requires the operator; inspection requires the operator or tenant.
    Denied,
    /// Invalid tenant principal or zero expected generation.
    Invalid,
    /// Current enrollment differs from the expected state; inspect before retrying.
    Conflict,
    /// All lifetime tenant slots are occupied, including suspended entries.
    Capacity,
    /// Reactivation cannot increment the generation; suspension remains possible.
    GenerationExhausted,
    /// Restored owners reject all updates, including no-ops.
    Fenced,
    /// Inconsistent retained state or unexpected internal result.
    Internal,
}
