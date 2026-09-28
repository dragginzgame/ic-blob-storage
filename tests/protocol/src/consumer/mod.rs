//! Bounded application substitute for local publication/outbox evidence, not Toko's API.
use candid::{CandidType, Deserialize};
use ic_blob_storage::dto::reference::{
    ReferenceChange, ReferenceCommand, ReferenceTransitionFailure,
};
/// Exact application intent, retained before permission or reference dispatch.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Registration {
    /// Never reused application asset identity.
    pub asset: u128,
    /// Complete bounded fixture asset payload, compared directly on retry.
    pub payload: Vec<u8>,
    /// First upload reference or an explicit retain of existing content.
    pub source: RegistrationSource,
    /// Reserved exact operation for eventual release of this reference.
    pub release_operation: u128,
}
/// Current supported registration sources, with no fabricated retain for a first reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum RegistrationSource {
    /// Add a distinct reference to existing content.
    Existing(ReferenceCommand),
    /// Observe completion of the exact upload that creates its first reference.
    Fresh(ic_blob_storage::dto::upload::admission::UploadAdmissionRequest),
}
/// Deliberate IC transaction interruption.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Fault {
    /// No interruption.
    None,
    /// Trap after intent storage, before sending.
    AfterIntent,
    /// Trap after remote admission before recording its acknowledgment.
    AfterAdmission,
    /// Trap after remote withdrawal before saving its acknowledgment.
    AfterRevocation,
    /// Trap after remote retain, before recording the result.
    AfterRetain,
    /// Trap after authenticated upload observation, before storing it locally.
    AfterUploadObservation,
    /// Trap after local publication writes.
    AfterPublish,
    /// Trap after storing the remote release outcome.
    AfterRelease,
}
/// Private scheduling controls, separate from the immutable registration intent.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Run {
    /// Exact application intent.
    pub registration: Registration,
    /// Fixture-only trap.
    pub fault: Fault,
    /// Hold after descriptor delivery, immediately before atomic publication.
    pub hold: bool,
    /// Encoded reference reply budget.
    pub max_reply_bytes: u32,
}
/// Read-only original intent and retained publication/cleanup state.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Read-only independent publication, tombstone and two dispatch facts"
)]
pub struct AssetView {
    /// Immutable intent and reserved cleanup operation.
    pub registration: Registration,
    /// Local tombstone blocks publication and new use.
    pub cancelled: bool,
    /// Currently published by the application.
    pub published: bool,
    /// A successful publication receipt remains even after cancellation.
    pub published_once: bool,
    /// Fresh permission dispatch intent has committed.
    pub admission_started: bool,
    /// Exact admission acknowledgment or typed refusal; absent after dispatch is uncertain.
    pub admission_result:
        Option<Result<(), ic_blob_storage::dto::upload::admission::UploadAdmissionFailure>>,
    /// Withdrawal dispatch intent committed after local cancellation.
    pub revocation_started: bool,
    /// Exact withdrawal acknowledgment or refusal; uncertainty retains the permission.
    pub revocation_result:
        Option<Result<(), ic_blob_storage::dto::upload::admission::UploadAdmissionFailure>>,
    /// Existing-content retain dispatch intent has committed; always false for fresh uploads.
    pub retain_started: bool,
    /// Existing-content retain result; absence after its dispatch is uncertainty.
    pub retain_result: Option<Result<ReferenceChange, ReferenceTransitionFailure>>,
    /// Last exact upload observation for a fresh registration; never a browser assertion.
    pub upload_state: Option<ic_blob_storage::dto::upload::UploadState>,
    /// Release dispatch intent has committed.
    pub release_started: bool,
    /// Original release result; an inner failure is not completed cleanup.
    pub release_result: Option<Result<ReferenceChange, ReferenceTransitionFailure>>,
    /// Number of currently attached fixture uses.
    pub uses: u32,
}
/// Fixture admission/coordination refusal; never a provider failure schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Failure {
    /// Actual caller lacks application authority.
    Denied,
    /// Malformed intent or configuration.
    Invalid,
    /// Same asset/reference/operation identity has changed arguments.
    Conflict,
    /// Lifetime asset/outbox slots are exhausted.
    Capacity,
    /// Unknown asset.
    Unknown,
    /// An uncertain operation still needs exact receipt recovery.
    Pending,
    /// Local state disallows the operation, including an outstanding use.
    State,
    /// Restored consumer cannot publish, dispatch or acknowledge effects.
    Fenced,
    /// Transport or response validation failed; intent remains retained.
    Transport,
}

/// Exact asset selected for release and an optional test interruption.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Release {
    /// Existing asset identity.
    pub asset: u128,
    /// Callback interruption control.
    pub fault: Fault,
}
/// Withdraw the saved fresh-upload permission after local cancellation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Revocation {
    /// Existing asset identity; the original permission is never supplied anew.
    pub asset: u128,
    /// Fixture callback interruption.
    pub fault: Fault,
    /// Encoded withdrawal reply budget.
    pub max_reply_bytes: u32,
}
/// Exact retained operation to inspect; never a new dispatch.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Recovery {
    /// Existing asset identity.
    pub asset: u128,
    /// Inspect release rather than the original retain or fresh upload.
    pub release: bool,
}
/// One bounded local application dependency change.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Use {
    /// Existing asset identity.
    pub asset: u128,
    /// Attach one use or remove one existing use.
    pub attach: bool,
}
