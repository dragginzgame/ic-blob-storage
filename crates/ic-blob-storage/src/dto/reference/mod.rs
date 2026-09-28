//! Exact reference commands and historical outcomes; no implicit dispatch authority.
use candid::{CandidType, Deserialize, Principal};

/// Complete original upload binding. Upload and object IDs are independent.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ReferenceUpload {
    /// Storage service canister.
    pub service: Principal,
    /// Tenant project canister.
    pub tenant: Principal,
    /// Local provider namespace.
    pub namespace: u128,
    /// Original upload operation.
    pub upload: u128,
    /// Object identity.
    pub object: u128,
    /// Object lifetime.
    pub incarnation: u128,
    /// First reference admitted with the upload.
    pub first_reference: u128,
    /// Immutable provider root.
    pub root: [u8; 32],
    /// Original declared nonzero length.
    pub bytes: u64,
}
/// Exact reference operation kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ReferenceAction {
    /// Retain the named reference.
    Retain,
    /// Release the named reference.
    Release,
}
/// Passive exact intent shared by mutation and receipt lookup. Persist before dispatch.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ReferenceCommand {
    /// Original confirmed upload.
    pub upload: ReferenceUpload,
    /// Reference targeted by the operation.
    pub reference: u128,
    /// Exact operation identity, independent of upload and reference IDs.
    pub operation: u128,
    /// Exact original action.
    pub action: ReferenceAction,
}
/// Original successful transition; it says nothing about current liveness.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ReferenceChange {
    /// The original operation changed the reference.
    Changed,
    /// The original operation required no change.
    Unchanged,
}
/// Retained failure of an admitted operation, not failure to look up its receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ReferenceTransitionFailure {
    /// No such reference existed at the time.
    UnknownReference,
    /// The reference had already been released.
    Released,
    /// Lifetime reference capacity was exhausted.
    Limit,
    /// Object deletion had already queued.
    DeletionQueued,
}
/// Exact recorded outcome. Historical success cannot authorize publication.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ReferenceReceiptResponse {
    /// Complete original request, checked before disclosure by the client.
    pub request: ReferenceCommand,
    /// Original success or failure, preserved across subsequent transitions.
    pub result: Result<ReferenceChange, ReferenceTransitionFailure>,
}
/// Explicit presence avoids Candid optional-value coercion hiding malformed data.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[expect(
    clippy::large_enum_variant,
    reason = "One fixed-size bounded receipt; preserve allocation-free value semantics"
)]
pub enum ReferenceReceiptLookup {
    /// No receipt in this owner; never permission to repeat an uncertain operation.
    Absent,
    /// The exact original transition result.
    Found(ReferenceReceiptResponse),
}
/// Admission/lookup refusal, distinct from a recorded transition failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ReferenceFailure {
    /// Invalid identity or declaration.
    Invalid,
    /// Actual caller is not the named tenant.
    Denied,
    /// Service or namespace differs from the installed owner.
    Binding,
    /// No matching upload operation is retained.
    Unknown,
    /// Upload has not completed.
    Unconfirmed,
    /// Existing identity has different original arguments.
    Conflict,
    /// Fresh retain requires active enrollment. Exact replay and release remain allowed.
    Inactive,
    /// Restored owner permits inspection only, including for exact mutation retries.
    Fenced,
    /// No unreserved receipt slot; cleanup reservations remain protected.
    Capacity,
    /// Inconsistent storage or an unexpected internal result.
    Internal,
}

/// Exact original result plus whether this call recorded it or replayed history.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ReferenceMutationResponse {
    /// Original transition result, including an admitted lifecycle failure.
    pub receipt: ReferenceReceiptResponse,
    /// True only when this call returned an already recorded result.
    pub replayed: bool,
}
