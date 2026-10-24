//! Explicit bounded tenant contracts for IC Wire.
//!
//! These projections preserve native DTOs and their existing persisted/provider
//! uses. Serde declaration order owns wire order. Principals are bounded bytes;
//! conversions grant no authority and existing tenant policy still validates them.
use crate::dto::tenant as native;
use ic_wire::{BytesRef, Limits};
use serde::{Deserialize, Serialize};

/// Exact selected service, namespace and tenant, without authentication claims.
#[cfg_attr(
    all(feature = "wire-schema", not(target_arch = "wasm32")),
    derive(ic_wire::Wire)
)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Scope<'a> {
    /// Intended storage service, at most 29 principal bytes.
    #[serde(borrow)]
    pub service: BytesRef<'a, 29>,
    /// Installed local namespace.
    pub namespace: u128,
    /// Intended tenant, at most 29 principal bytes.
    #[serde(borrow)]
    pub tenant: BytesRef<'a, 29>,
}
/// One operator-authorized compare-and-set, without dispatch or retry policy.
#[cfg_attr(
    all(feature = "wire-schema", not(target_arch = "wasm32")),
    derive(ic_wire::Wire)
)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UpdateRequest<'a> {
    /// Explicit destination and tenant binding.
    #[serde(borrow)]
    pub scope: Scope<'a>,
    /// Exact prior observation; absence requires a never-enrolled tenant.
    pub expected: Option<native::TenantEnrollment>,
    /// Requested admission state.
    pub active: bool,
}
/// Correlated observation; no historical operation receipt or retry authority.
#[cfg_attr(
    all(feature = "wire-schema", not(target_arch = "wasm32")),
    derive(ic_wire::Wire)
)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Observation<'a> {
    /// Echoed original scope.
    #[serde(borrow)]
    pub scope: Scope<'a>,
    /// Current enrollment, including explicit absence.
    pub enrollment: Option<native::TenantEnrollment>,
    /// Current owner is inspection-only after restore.
    pub fenced: bool,
}
/// Request byte ceiling shared by method contracts and host preallocation admission.
pub const MAX_REQUEST_BYTES: usize = 512;
pub(super) const LIMITS: Limits = Limits {
    bytes: MAX_REQUEST_BYTES,
    // Native visitor charges include repeated value-size accounting. These
    // conservative bounds admit the full borrowed response, not arbitrary DTOs.
    allocation: 16 * 1024,
    nodes: 64,
    depth: 16,
};

impl<'a> From<&'a native::TenantScope> for Scope<'a> {
    fn from(value: &'a native::TenantScope) -> Self {
        Self {
            service: BytesRef::new(value.service.as_slice()).expect("principal byte bound"),
            namespace: value.namespace,
            tenant: BytesRef::new(value.tenant.as_slice()).expect("principal byte bound"),
        }
    }
}
impl From<Scope<'_>> for native::TenantScope {
    fn from(value: Scope<'_>) -> Self {
        Self {
            service: ic_platform::Principal::from_slice(value.service.as_slice()),
            namespace: value.namespace,
            tenant: ic_platform::Principal::from_slice(value.tenant.as_slice()),
        }
    }
}
impl<'a> From<&'a native::TenantUpdateRequest> for UpdateRequest<'a> {
    fn from(value: &'a native::TenantUpdateRequest) -> Self {
        Self {
            scope: (&value.scope).into(),
            expected: value.expected,
            active: value.active,
        }
    }
}
impl From<UpdateRequest<'_>> for native::TenantUpdateRequest {
    fn from(value: UpdateRequest<'_>) -> Self {
        Self {
            scope: value.scope.into(),
            expected: value.expected,
            active: value.active,
        }
    }
}
impl<'a> From<&'a native::TenantEnrollmentResponse> for Observation<'a> {
    fn from(value: &'a native::TenantEnrollmentResponse) -> Self {
        Self {
            scope: (&value.scope).into(),
            enrollment: value.enrollment,
            fenced: value.fenced,
        }
    }
}
impl From<Observation<'_>> for native::TenantEnrollmentResponse {
    fn from(value: Observation<'_>) -> Self {
        Self {
            scope: value.scope.into(),
            enrollment: value.enrollment,
            fenced: value.fenced,
        }
    }
}

/// Convert a native observation without changing application failure meaning.
fn response(
    value: &Result<native::TenantEnrollmentResponse, native::TenantFailure>,
) -> Result<Observation<'_>, native::TenantFailure> {
    value.as_ref().map(Into::into).map_err(|e| *e)
}
/// Convert an admitted reply; native semantic/binding validators still run afterward.
fn native_response(
    value: Result<Observation<'_>, native::TenantFailure>,
) -> Result<native::TenantEnrollmentResponse, native::TenantFailure> {
    value.map(Into::into)
}

#[cfg(test)]
mod tests;

/// Encode the exact inspection request; save these bytes when retaining intent.
/// # Errors
/// Refuses codec admission or allocation failure before dispatch.
pub fn inspection_request(value: native::TenantScope) -> Result<Vec<u8>, ic_wire::Error> {
    ic_wire::native::encode_request(
        super::identity(crate::protocol::TENANT_INSPECTION_METHOD)?,
        &Scope::from(&value),
        LIMITS,
    )
}
/// Decode only this inspection method and request direction.
/// # Errors
/// Rejects foreign identity/direction, malformed, noncanonical or excessive input.
pub fn decode_inspection_request(bytes: &[u8]) -> Result<native::TenantScope, ic_wire::Error> {
    ic_wire::native::decode_request::<Scope<'_>>(
        super::identity(crate::protocol::TENANT_INSPECTION_METHOD)?,
        bytes,
        LIMITS,
    )
    .map(Into::into)
}
/// Encode one compare-and-set; encoding itself grants no effect authority.
/// # Errors
/// Refuses codec admission or allocation failure before dispatch.
pub fn update_request(value: native::TenantUpdateRequest) -> Result<Vec<u8>, ic_wire::Error> {
    ic_wire::native::encode_request(
        super::identity(crate::protocol::TENANT_UPDATE_METHOD)?,
        &UpdateRequest::from(&value),
        LIMITS,
    )
}
/// Decode only the update method and request direction.
/// # Errors
/// Rejects foreign identity/direction, malformed, noncanonical or excessive input.
pub fn decode_update_request(bytes: &[u8]) -> Result<native::TenantUpdateRequest, ic_wire::Error> {
    ic_wire::native::decode_request::<UpdateRequest<'_>>(
        super::identity(crate::protocol::TENANT_UPDATE_METHOD)?,
        bytes,
        LIMITS,
    )
    .map(Into::into)
}
/// Encode an inspection observation or application refusal.
/// # Errors
/// Refuses codec admission or allocation failure; adapters must trap after mutation.
pub fn inspection_response(
    value: Result<native::TenantEnrollmentResponse, native::TenantFailure>,
) -> Result<Vec<u8>, ic_wire::Error> {
    ic_wire::native::encode_response(
        super::identity(crate::protocol::TENANT_INSPECTION_METHOD)?,
        &response(&value),
        LIMITS,
    )
}
/// Encode an update acknowledgment or application refusal.
/// # Errors
/// Refuses codec admission or allocation failure; adapters must trap after mutation.
pub fn update_response(
    value: Result<native::TenantEnrollmentResponse, native::TenantFailure>,
) -> Result<Vec<u8>, ic_wire::Error> {
    ic_wire::native::encode_response(
        super::identity(crate::protocol::TENANT_UPDATE_METHOD)?,
        &response(&value),
        LIMITS,
    )
}
/// Decode only an inspection reply; semantic/binding checks remain caller-owned.
/// # Errors
/// Rejects foreign identity/direction, malformed, noncanonical or excessive input.
pub fn decode_inspection_response(
    bytes: &[u8],
) -> Result<Result<native::TenantEnrollmentResponse, native::TenantFailure>, ic_wire::Error> {
    ic_wire::native::decode_response(
        super::identity(crate::protocol::TENANT_INSPECTION_METHOD)?,
        bytes,
        LIMITS,
    )
    .map(native_response)
}
/// Decode only an update reply; an unusable reply never proves non-execution.
/// # Errors
/// Rejects foreign identity/direction, malformed, noncanonical or excessive input.
pub fn decode_update_response(
    bytes: &[u8],
) -> Result<Result<native::TenantEnrollmentResponse, native::TenantFailure>, ic_wire::Error> {
    ic_wire::native::decode_response(
        super::identity(crate::protocol::TENANT_UPDATE_METHOD)?,
        bytes,
        LIMITS,
    )
    .map(native_response)
}
