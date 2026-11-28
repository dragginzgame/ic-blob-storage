//! Requests, replies and configuration shared by Blob hosts.

use ic_platform_types::Principal;
use serde::{Deserialize, Serialize};

use crate::{
    local::{
        Limits, ObjectView, PrincipalOverview, ReferenceView, Root, UploadId, UploadRequest,
        UploadView, Usage,
    },
    references::ReferenceId,
};

///
/// Installation
/// Immutable authorities and resource limits for a fresh Blob installation.
///

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Installation {
    /// Authority for application grants and references.
    pub application: Principal,
    /// Authority for draining and activating; captured ingress must match.
    pub manager: Principal,
    /// Immutable per-installation resource limits.
    pub limits: Limits,
}

///
/// ManagementView
/// Configuration and maintenance state returned to the manager by replicated inspection.
/// Transport authentication and continuity checks belong to the caller.
///

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct ManagementView {
    /// Canister principal bound into the durable record at creation.
    pub installation: Principal,
    /// Immutable authorities and limits loaded by the service.
    pub configuration: Installation,
    /// Format accepted and written by this service; not a software release version.
    pub state_version: u32,
    /// Current maintenance attempt and admission gate.
    pub maintenance: Maintenance,
}

/// Synchronous local mutation. Caller and time are supplied by the host, never this payload.
#[derive(Debug, Deserialize, Serialize)]
pub enum Command {
    /// Validate a declaration and reserve its first reference or attach an existing object.
    Authorize {
        /// Exact immutable declaration.
        request: UploadRequest,
        /// Manifest leaves; discarded after admission.
        chunks: Vec<Root>,
        /// Original metadata; discarded after admission.
        headers: Vec<(String, String)>,
    },
    /// Rehearse registration without returning upload authority.
    Register(Root),
    /// Acquire an application reference.
    Retain(Root, ReferenceId),
    /// Release a reference or cancel an unconsumed grant.
    Release(ReferenceId),
    /// Retire an expired grant.
    Expire(UploadId),
    /// Close admission for the next monotonically increasing maintenance attempt.
    Drain(u64),
    /// Reopen admission for this exact current attempt, after supported management checks.
    Activate(u64),
}

/// Mutation outcome. It is never a provider proof or settlement receipt.
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum Outcome {
    /// Local transition completed or its historical success replayed.
    Done,
    /// Current disposition of the exact original authorization.
    Upload(UploadView),
    /// Current management state, including historical command replay.
    Maintenance(Maintenance),
}

///
/// Maintenance
/// Current admission state and attempt number used to reject stale activation requests.
///

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Maintenance {
    /// Zero before first maintenance, otherwise monotonically increasing.
    pub attempt: u64,
    /// Whether fresh local admissions are permitted.
    pub active: bool,
}

/// Read-only inspection, still subject to the library's caller rules.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Inspection {
    /// Application authority or that upload's bound uploader.
    Upload(UploadId),
    /// Application authority only.
    Reference(ReferenceId),
    /// Application authority only.
    Object(Root),
    /// Application authority only.
    Usage,
    /// Configured manager only.
    Maintenance,
    /// Application authority only; historical objects associated with this uploader.
    Overview(Principal),
}

/// Read-only projection of existing state.
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum Observation {
    /// Exact authorization and current disposition.
    Upload(UploadView),
    /// Immutable target and current reference liveness.
    Reference(ReferenceView),
    /// Original upload and current object retention.
    Object(ObjectView),
    /// Declared accounting and occupied slots.
    Usage(Usage),
    /// Current management attempt and admission.
    Maintenance(Maintenance),
    /// Distinct associated objects, matching grants and application references.
    Overview(PrincipalOverview),
}
