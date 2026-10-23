//! Passive reference headroom for one tenant-scoped root.
use crate::dto::tenant::TenantScope;
use serde::Deserialize;

/// Root lookup; a digest is not tenant authority or proof of global absence.
#[cfg_attr(
    all(feature = "wire-schema", not(target_arch = "wasm32")),
    derive(ic_wire::Wire)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceCapacityRequest {
    /// Actual service, positive namespace and tenant caller must match.
    pub scope: TenantScope,
    /// Provider root to inspect within this tenant's retained history.
    pub root: [u8; 32],
}
/// Independent lifetime capacity dimensions, without allocating a reference or receipt.
#[cfg_attr(
    all(feature = "wire-schema", not(target_arch = "wasm32")),
    derive(ic_wire::Wire)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceHeadroom {
    /// Unused lifetime reference identities.
    pub reference_slots: u64,
    /// Receipt slots not already reserved for cleanup.
    pub unreserved_receipts: u64,
    /// Cleanup receipt slots reserved for active references.
    pub release_reserved_receipts: u64,
    /// Fresh distinct retains permitted by these counters and lifecycle only.
    /// Enrollment, identity, scope and restore checks still apply at mutation time.
    pub fresh_retains: u64,
}
/// Correlated observation, not reservation, liveness or mutation authority.
#[cfg_attr(
    all(feature = "wire-schema", not(target_arch = "wasm32")),
    derive(ic_wire::Wire)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceCapacityResponse {
    /// Exact original tenant/root request.
    pub request: ReferenceCapacityRequest,
    /// Absent for unknown, foreign or unconfirmed roots; never proof of global absence.
    pub headroom: Option<ReferenceHeadroom>,
    /// Restored owner allows inspection only, including when headroom is absent.
    pub fenced: bool,
}
/// Typed inspection refusal; never interpreted as absent content.
#[cfg_attr(
    all(feature = "wire-schema", not(target_arch = "wasm32")),
    derive(ic_wire::Wire)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum ReferenceCapacityFailure {
    /// Zero namespace or invalid tenant principal.
    Invalid,
    /// Actual caller is not the tenant named in the request.
    Denied,
    /// Service or namespace differs from the installed owner.
    Binding,
    /// Inconsistent retained counters or failed boundary conversion.
    Internal,
}
