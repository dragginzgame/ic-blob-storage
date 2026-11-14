//! Experimental reference-application wire contract; publication is uploader consent.

use ic_blob_storage::local::{Error as BlobError, Root, UploadId};
use ic_platform_types::Principal;
use serde::{Deserialize, Serialize};

/// Immutable installation. Only these users may create and manage their own drafts.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Installation {
    /// Dedicated standalone Blob host whose application authority is this canister.
    pub blob: Principal,
    /// One to eight distinct, non-anonymous application users.
    pub users: Vec<Principal>,
}

/// Immutable draft input. The captured caller becomes its uploader; ID also names its reference.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct DraftInput {
    /// Globally unique lifetime ID; never evicted or reassigned.
    pub id: UploadId,
    /// Asset name in the creating user's catalog, at most 64 UTF-8 bytes.
    pub asset: String,
    /// Original declared root.
    pub root: Root,
    /// Original declared length, not a provider-enforced bound.
    pub bytes: u64,
    /// Exclusive IC nanosecond deadline for Blob's original grant.
    pub expires_at: u64,
    /// Original manifest leaves, retained for exact authorization reconciliation.
    pub chunks: Vec<Root>,
    /// Original metadata, retained for the same purpose.
    pub headers: Vec<(String, String)>,
}

/// Application disposition, independent of Blob's retention and provider availability.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum Phase {
    /// Saved, possibly authorized; publication requires current Blob inspection.
    Draft,
    /// Present in this user's catalog; its reference must remain retained.
    Published,
    /// Permanently unpublished; remote cleanup remains unresolved.
    Cancelled,
    /// Exact authorization resolved and first reference released or retired.
    Cleaned,
}

/// Owner-visible durable draft. No inspection result grants later publication rights.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct DraftView {
    /// Exact immutable original input.
    pub input: DraftInput,
    /// Captured creating user.
    pub uploader: Principal,
    /// Current local disposition.
    pub phase: Phase,
}

/// Ordinary refusals; cancelled drafts remain durable even when cleanup returns an error.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum Error {
    /// Caller is not allowed or does not own the draft.
    Denied,
    /// No local draft with this ID; this is not successful cancellation.
    Unknown,
    /// Immutable input changed or another draft occupies the asset name.
    Conflict,
    /// Invalid or oversized input.
    Invalid,
    /// All sixteen lifetime draft slots are occupied.
    Capacity,
    /// The draft was cancelled and can never publish again.
    Cancelled,
    /// Blob has no matching currently live reference.
    NotLive,
    /// Blob refused the original request; pending local state remains retained.
    Blob(BlobError),
    /// Remote outcome could not be established; inspect/reconcile the same draft.
    Uncertain,
}
