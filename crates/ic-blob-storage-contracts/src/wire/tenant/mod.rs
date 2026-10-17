//! Explicit bounded tenant contracts for IC Wire 0.3.
//!
//! These projections preserve native DTOs and their existing persisted/provider
//! uses. Field and variant IDs own wire order. Principals are bounded bytes;
//! conversions grant no authority and existing tenant policy still validates them.
use crate::dto::tenant as native;
use ic_wire::{Bytes, Limits, Method, Wire};

/// Exact selected service, namespace and tenant, without authentication claims.
#[derive(Clone, Debug, Eq, PartialEq, Wire)]
pub struct Scope {
    /// Intended storage service, at most 29 principal bytes.
    #[wire(id = 1)]
    pub service: Bytes<29>,
    /// Installed local namespace.
    #[wire(id = 2)]
    pub namespace: u128,
    /// Intended tenant, at most 29 principal bytes.
    #[wire(id = 3)]
    pub tenant: Bytes<29>,
}
/// Exact compare-and-set enrollment observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Wire)]
pub struct Enrollment {
    /// Existing policy requires a positive generation.
    #[wire(id = 1)]
    pub generation: u64,
    /// Fresh admission state; suspension retains obligations.
    #[wire(id = 2)]
    pub active: bool,
}
/// One operator-authorized compare-and-set, without dispatch or retry policy.
#[derive(Clone, Debug, Eq, PartialEq, Wire)]
pub struct UpdateRequest {
    /// Explicit destination and tenant binding.
    #[wire(id = 1)]
    pub scope: Scope,
    /// Exact prior observation; absence requires a never-enrolled tenant.
    #[wire(id = 2)]
    pub expected: Option<Enrollment>,
    /// Requested admission state.
    #[wire(id = 3)]
    pub active: bool,
}
/// Correlated observation; no historical operation receipt or retry authority.
#[derive(Clone, Debug, Eq, PartialEq, Wire)]
pub struct Observation {
    /// Echoed original scope.
    #[wire(id = 1)]
    pub scope: Scope,
    /// Current enrollment, including explicit absence.
    #[wire(id = 2)]
    pub enrollment: Option<Enrollment>,
    /// Current owner is inspection-only after restore.
    #[wire(id = 3)]
    pub fenced: bool,
}
/// Existing application refusals, independent of codec and transport errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Wire)]
pub enum Failure {
    /// Wrong actual/installed service or namespace.
    #[wire(id = 1)]
    Binding,
    /// Actual caller lacks authority.
    #[wire(id = 2)]
    Denied,
    /// Invalid principal or generation.
    #[wire(id = 3)]
    Invalid,
    /// Exact compare-and-set precondition differs.
    #[wire(id = 4)]
    Conflict,
    /// Lifetime tenant capacity is exhausted.
    #[wire(id = 5)]
    Capacity,
    /// Activation cannot advance the generation.
    #[wire(id = 6)]
    GenerationExhausted,
    /// Restored owner refuses mutation.
    #[wire(id = 7)]
    Fenced,
    /// Retained state is inconsistent.
    #[wire(id = 8)]
    Internal,
}

/// Tenant observation method; its identity cannot be used as an update envelope.
pub struct Inspect;
/// Tenant compare-and-set method; retain exact original request bytes before effects.
pub struct Update;
const LIMITS: Limits = Limits {
    bytes: 512,
    allocation: 1024,
    nodes: 32,
    depth: 8,
};
impl Method for Inspect {
    type Request = Scope;
    type Response = Result<Observation, Failure>;
    const ID: u64 = 0x424c_4f42_3332_0001;
    const NAME: &'static str = crate::protocol::TENANT_INSPECTION_METHOD;
    const REQUEST_LIMITS: Limits = LIMITS;
    const RESPONSE_LIMITS: Limits = LIMITS;
}
impl Method for Update {
    type Request = UpdateRequest;
    type Response = Result<Observation, Failure>;
    const ID: u64 = 0x424c_4f42_3332_0002;
    const NAME: &'static str = crate::protocol::TENANT_UPDATE_METHOD;
    const REQUEST_LIMITS: Limits = LIMITS;
    const RESPONSE_LIMITS: Limits = LIMITS;
}

impl From<native::TenantScope> for Scope {
    fn from(value: native::TenantScope) -> Self {
        Self {
            service: Bytes::new(value.service.as_slice().to_vec()).expect("principal byte bound"),
            namespace: value.namespace,
            tenant: Bytes::new(value.tenant.as_slice().to_vec()).expect("principal byte bound"),
        }
    }
}
impl From<Scope> for native::TenantScope {
    fn from(value: Scope) -> Self {
        Self {
            service: candid::Principal::from_slice(value.service.as_slice()),
            namespace: value.namespace,
            tenant: candid::Principal::from_slice(value.tenant.as_slice()),
        }
    }
}
impl From<native::TenantEnrollment> for Enrollment {
    fn from(value: native::TenantEnrollment) -> Self {
        Self {
            generation: value.generation,
            active: value.active,
        }
    }
}
impl From<Enrollment> for native::TenantEnrollment {
    fn from(value: Enrollment) -> Self {
        Self {
            generation: value.generation,
            active: value.active,
        }
    }
}
impl From<native::TenantUpdateRequest> for UpdateRequest {
    fn from(value: native::TenantUpdateRequest) -> Self {
        Self {
            scope: value.scope.into(),
            expected: value.expected.map(Into::into),
            active: value.active,
        }
    }
}
impl From<UpdateRequest> for native::TenantUpdateRequest {
    fn from(value: UpdateRequest) -> Self {
        Self {
            scope: value.scope.into(),
            expected: value.expected.map(Into::into),
            active: value.active,
        }
    }
}
impl From<native::TenantEnrollmentResponse> for Observation {
    fn from(value: native::TenantEnrollmentResponse) -> Self {
        Self {
            scope: value.scope.into(),
            enrollment: value.enrollment.map(Into::into),
            fenced: value.fenced,
        }
    }
}
impl From<Observation> for native::TenantEnrollmentResponse {
    fn from(value: Observation) -> Self {
        Self {
            scope: value.scope.into(),
            enrollment: value.enrollment.map(Into::into),
            fenced: value.fenced,
        }
    }
}

impl From<native::TenantFailure> for Failure {
    fn from(value: native::TenantFailure) -> Self {
        match value {
            native::TenantFailure::Binding => Self::Binding,
            native::TenantFailure::Denied => Self::Denied,
            native::TenantFailure::Invalid => Self::Invalid,
            native::TenantFailure::Conflict => Self::Conflict,
            native::TenantFailure::Capacity => Self::Capacity,
            native::TenantFailure::GenerationExhausted => Self::GenerationExhausted,
            native::TenantFailure::Fenced => Self::Fenced,
            native::TenantFailure::Internal => Self::Internal,
        }
    }
}

impl From<Failure> for native::TenantFailure {
    fn from(value: Failure) -> Self {
        match value {
            Failure::Binding => Self::Binding,
            Failure::Denied => Self::Denied,
            Failure::Invalid => Self::Invalid,
            Failure::Conflict => Self::Conflict,
            Failure::Capacity => Self::Capacity,
            Failure::GenerationExhausted => Self::GenerationExhausted,
            Failure::Fenced => Self::Fenced,
            Failure::Internal => Self::Internal,
        }
    }
}

/// Convert a native observation without changing application failure meaning.
fn response(
    value: Result<native::TenantEnrollmentResponse, native::TenantFailure>,
) -> Result<Observation, Failure> {
    value.map(Into::into).map_err(Into::into)
}
/// Convert an admitted reply; native semantic/binding validators still run afterward.
fn native_response(
    value: Result<Observation, Failure>,
) -> Result<native::TenantEnrollmentResponse, native::TenantFailure> {
    value.map(Into::into).map_err(Into::into)
}

#[cfg(test)]
mod tests;

/// Encode the exact inspection request; save these bytes when retaining intent.
/// # Errors
/// Refuses codec admission or allocation failure before dispatch.
pub fn inspection_request(value: native::TenantScope) -> Result<Vec<u8>, ic_wire::Error> {
    ic_wire::encode_request::<Inspect>(&value.into()).map(ic_wire::RequestBytes::into_bytes)
}
/// Decode only this inspection method and request direction.
/// # Errors
/// Rejects foreign identity/direction, malformed, noncanonical or excessive input.
pub fn decode_inspection_request(bytes: &[u8]) -> Result<native::TenantScope, ic_wire::Error> {
    ic_wire::decode_request::<Inspect>(bytes).map(Into::into)
}
/// Encode one compare-and-set; encoding itself grants no effect authority.
/// # Errors
/// Refuses codec admission or allocation failure before dispatch.
pub fn update_request(value: native::TenantUpdateRequest) -> Result<Vec<u8>, ic_wire::Error> {
    ic_wire::encode_request::<Update>(&value.into()).map(ic_wire::RequestBytes::into_bytes)
}
/// Decode only the update method and request direction.
/// # Errors
/// Rejects foreign identity/direction, malformed, noncanonical or excessive input.
pub fn decode_update_request(bytes: &[u8]) -> Result<native::TenantUpdateRequest, ic_wire::Error> {
    ic_wire::decode_request::<Update>(bytes).map(Into::into)
}
/// Encode an inspection observation or application refusal.
/// # Errors
/// Refuses codec admission or allocation failure; adapters must trap after mutation.
pub fn inspection_response(
    value: Result<native::TenantEnrollmentResponse, native::TenantFailure>,
) -> Result<Vec<u8>, ic_wire::Error> {
    ic_wire::encode_response::<Inspect>(&response(value)).map(ic_wire::ResponseBytes::into_bytes)
}
/// Encode an update acknowledgment or application refusal.
/// # Errors
/// Refuses codec admission or allocation failure; adapters must trap after mutation.
pub fn update_response(
    value: Result<native::TenantEnrollmentResponse, native::TenantFailure>,
) -> Result<Vec<u8>, ic_wire::Error> {
    ic_wire::encode_response::<Update>(&response(value)).map(ic_wire::ResponseBytes::into_bytes)
}
/// Decode only an inspection reply; semantic/binding checks remain caller-owned.
/// # Errors
/// Rejects foreign identity/direction, malformed, noncanonical or excessive input.
pub fn decode_inspection_response(
    bytes: &[u8],
) -> Result<Result<native::TenantEnrollmentResponse, native::TenantFailure>, ic_wire::Error> {
    ic_wire::decode_response::<Inspect>(bytes).map(native_response)
}
/// Decode only an update reply; an unusable reply never proves non-execution.
/// # Errors
/// Rejects foreign identity/direction, malformed, noncanonical or excessive input.
pub fn decode_update_response(
    bytes: &[u8],
) -> Result<Result<native::TenantEnrollmentResponse, native::TenantFailure>, ic_wire::Error> {
    ic_wire::decode_response::<Update>(bytes).map(native_response)
}
