//! Inspection-only local archive; never a resumable service checkpoint.
use crate::journey::{JourneyManifest, JourneyProgress};
use candid::{CandidType, Principal};
use serde::Deserialize;

/// Catalogs remain separate owners within the test fixture.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ArchiveCatalog {
    /// Two initial confirmed sample objects.
    Samples,
    /// Four initial sample upload operations.
    Uploads,
    /// Dynamically admitted content-bearing uploads.
    Journey,
}

/// Local lifecycle, including byte-free history that must not disappear.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ArchivePhase {
    /// Reserved before certificate exposure.
    Reserved,
    /// Certificate may have escaped; completion remains unknown.
    ExposurePossible,
    /// Cancelled before exposure, with immutable identity retained.
    Cancelled,
    /// Confirmed object with a live reference.
    Live,
    /// Released but still physically present and billed.
    DeletionPending,
    /// Physically deleted but still billed.
    ProviderDeleted,
    /// Billing ended; lifetime identity and receipts remain.
    Settled,
}

/// Read-only object history from a bounded persisted inspection archive.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ArchivedObjectView {
    /// Owning fixture catalog; equal roots across catalogs are independent.
    pub catalog: ArchiveCatalog,
    /// Exact tenant binding; service and namespace are in the enclosing archive.
    pub tenant: Principal,
    /// Fixture object ID; also upload request ID when digest is present.
    pub id: u8,
    /// Immutable provider root.
    pub root: [u8; 32],
    /// Exact original declared length.
    pub bytes: u64,
    /// Declared raw digest for uploads; absent only for supplied sample objects.
    pub digest: Option<[u8; 32]>,
    /// Current local phase, not provider evidence.
    pub phase: ArchivePhase,
    /// Current charged logical bytes including reservations.
    pub logical: u64,
    /// Current charged physical bytes including reservations.
    pub physical: u64,
    /// Current charged liability bytes including reservations.
    pub liability: u64,
    /// Original result of release request 1 for reference 1; true means changed.
    /// The actor is this object's tenant and the operation is always Release.
    pub release_receipt: Option<bool>,
    /// Original admitted journey manifest; absent for supplied sample uploads.
    pub manifest: Option<JourneyManifest>,
    /// Last observed verification prefix/verdict, without resumable SHA state.
    pub progress: Option<JourneyProgress>,
}

/// Exact outstanding source read, retained even when its authority was invalidated.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ArchivedReadView {
    /// Allocated read token.
    pub token: u64,
    /// Whether revocation/sync has invalidated this token.
    pub valid: bool,
    /// Bound tenant.
    pub tenant: Principal,
    /// Bound root (resolves the complete object in the journey archive).
    pub root: [u8; 32],
    /// Requested leaf index.
    pub index: u64,
    /// Exact source principal selected before dispatch.
    pub gateway: Principal,
}

/// Operator-only inspection of all retained fixture owners, read from stable memory.
/// Namespace, object incarnation and reference identity are explicitly fixed at 1.
/// No method consumes this view to resume work or grant authority.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct AuthorityArchiveView {
    /// Bound service instance.
    pub service: Principal,
    /// Explicit archive inspection authority, independent of controller status.
    pub operator: Principal,
    /// Two configured tenants.
    pub tenants: [Principal; 2],
    /// Provider namespace for all fixture bindings.
    pub namespace: u128,
    /// Original configured gateway used by the revocation control.
    pub initial_gateway: Principal,
    /// Exact configured gateway-list source (a local substitute).
    pub sync_source: Principal,
    /// Current gateway membership, which may be empty after revocation.
    pub gateways: Vec<Principal>,
    /// Last allocated sync sequence; no token can be reconstructed from this view.
    pub last_sync: u64,
    /// Pending sync sequence, if any.
    pub pending_sync: Option<u64>,
    /// Last allocated read sequence, including completed reads.
    pub last_read: u64,
    /// Exact pending read, if any.
    pub pending_read: Option<ArchivedReadView>,
    /// Operator-armed fault plan awaiting a read of this root.
    pub armed_read_trap: Option<[u8; 32]>,
    /// Exact token whose callback is configured to trap.
    pub trap_read_token: Option<u64>,
    /// All lifetime objects/operations, including cancelled and settled history.
    pub objects: Vec<ArchivedObjectView>,
}
