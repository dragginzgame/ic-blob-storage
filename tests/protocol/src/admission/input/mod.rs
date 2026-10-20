//! Operation-specific fixture inputs keep unrelated command types off the wire.
use ic_platform::principal::ExternalPrincipal;

use super::{ContentLookup, Enrollment, Request};
use crate::journey::JourneyManifest;
use ic_platform::candid::contract::CandidType;
use serde::Deserialize;

/// Exact consumer reference to inspect, never an allocation or retain request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize, serde::Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct RetainedDescriptorInput {
    /// Explicit service, tenant, namespace and provider root.
    pub content: ContentLookup,
    /// Logical object identity.
    pub object: u128,
    /// Exact object lifetime.
    pub incarnation: u128,
    /// Exact retained reference identity.
    pub reference: u128,
}

/// Passive reference-qualified observation, not a publication permission.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize, serde::Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct RetainedDescriptor {
    /// Checked reference identity, observed atomically with the descriptor.
    pub reference: RetainedDescriptorInput,
    /// Confirmed content declaration and original metadata.
    pub descriptor: super::ContentDescriptor,
}

/// Operator compare-and-set enrollment request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct EnrollmentInput {
    /// Tenant whose admission state changes.
    pub tenant: ExternalPrincipal,
    /// Exact prior enrollment observation.
    pub expected: Option<Enrollment>,
    /// Desired admission state.
    pub active: bool,
}

/// Uploader's bounded manifest for an existing permission.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize, serde::Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct PreparationInput {
    /// Exact admitted operation.
    pub request: Request,
    /// Manifest declaration, not a provider completion fact.
    pub manifest: JourneyManifest,
}

/// One exact tenant reference mutation, shared by retain and release.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct ReferenceInput {
    /// Original upload binding.
    pub object: Request,
    /// Nonzero lifetime reference identity.
    pub reference: u128,
    /// Nonzero receipt identity, binding retries to the complete request.
    pub operation: u128,
    /// True retains; false releases.
    pub retain: bool,
}
