//! Current descriptor contract. Hosts expose it through authenticated update execution.
use candid::{CandidType, Principal};
use serde::Deserialize;

/// Exact tenant-owned reference, never an allocation or a publication lease.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct DownloadRequest {
    /// Intended storage service, also checked against actual execution.
    pub service: Principal,
    /// Must equal the endpoint's authenticated caller.
    pub tenant: Principal,
    /// Explicit local namespace; never a provider project identifier.
    pub namespace: u128,
    /// Caffeine tree root, not a raw-content digest.
    pub root: [u8; 32],
    /// Nonzero object identity.
    pub object: u128,
    /// Nonzero exact object lifetime.
    pub incarnation: u128,
    /// Nonzero currently live reference.
    pub reference: u128,
}
/// Original accepted hash metadata; HTTP response headers must not replace it.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct DownloadHeader {
    /// Original header name.
    pub name: String,
    /// Original header value.
    pub value: String,
}
/// A current observation. Consumers must authenticate delivery and coordinate
/// publication/reference lifetime; saved descriptors are not revocation mechanisms.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct DownloadResponse {
    /// Complete request echoed by the same synchronous observation.
    pub request: DownloadRequest,
    /// Storage owner, independent of payer and tenant.
    pub owner: Principal,
    /// Explicit provider project; no origin or credential is returned.
    pub project: String,
    /// Declared exact nonzero body length.
    pub bytes: u64,
    /// First-accepted metadata in its retained order/spelling.
    pub headers: Vec<DownloadHeader>,
}
/// Descriptor refusal; no error authorizes body disclosure, retry of paid effects
/// or release of the consumer's reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum DownloadFailure {
    /// Malformed or zero identity.
    Invalid,
    /// Caller is not the named tenant.
    Denied,
    /// Service, namespace or configured serving identity differs.
    Binding,
    /// No confirmed content with that exact live reference.
    Unavailable,
    /// Tenant is absent or suspended.
    Inactive,
    /// Restored installation refuses operational delivery.
    Fenced,
    /// Stored invariants could not be established.
    Internal,
}
