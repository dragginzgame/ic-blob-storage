//! Bounded application substitute for local publication/outbox evidence, not Toko's API.
use candid::{CandidType, Deserialize};
use ic_blob_storage::dto::reference::{
    ReferenceChange, ReferenceCommand, ReferenceTransitionFailure,
};
/// Exact application intent, retained before reference dispatch.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Registration {
    /// Never reused application asset identity.
    pub asset: u128,
    /// Complete bounded fixture asset payload, compared directly on retry.
    pub payload: Vec<u8>,
    /// Exact retain of a distinct reference to existing confirmed content.
    pub retain: ReferenceCommand,
    /// Reserved exact operation for eventual release of this reference.
    pub release_operation: u128,
}
/// Deliberate IC transaction interruption.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Fault {
    /// No interruption.
    None,
    /// Trap after intent storage, before sending.
    AfterIntent,
    /// Trap after remote retain, before recording the result.
    AfterRetain,
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
    /// Reference dispatch intent has committed.
    pub retain_started: bool,
    /// Original retain result; absence after dispatch is uncertainty.
    pub retain_result: Option<Result<ReferenceChange, ReferenceTransitionFailure>>,
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
/// Exact retained operation to inspect; never a new dispatch.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Recovery {
    /// Existing asset identity.
    pub asset: u128,
    /// Inspect release rather than retain.
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
