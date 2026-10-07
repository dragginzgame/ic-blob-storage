//! Bounded fixture records; hash reconstruction does not resume service authority.
use blob_test_protocol::authority::{ArchiveCatalog, ArchivePhase};
use blob_test_protocol::journey::JourneyVerification;
use candid::{CandidType, Principal};
use serde::Deserialize;

// Bind even terminal-only archives to the workspace release and dependency
// selection. The unpublished fixture package itself always has version 0.0.0.
pub(crate) fn release_binding() -> [u8; 32] {
    *ic_blob_storage::model::identity::ContentDigest::compute(include_bytes!(
        "../../../../../Cargo.lock"
    ))
    .as_bytes()
}

#[derive(Clone, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct AuthorityArchiveRecord {
    pub release: [u8; 32],
    pub fenced: bool,
    pub service: Principal,
    pub operator: Principal,
    pub tenants: [Principal; 2],
    pub namespace: u128,
    pub initial_gateway: Principal,
    pub sync_source: Principal,
    pub gateways: Vec<Principal>,
    pub last_sync: u64,
    pub pending_sync: Option<u64>,
    pub sync_control: super::sync::SyncControlRecord,
    pub last_read: u64,
    pub pending_read: Option<ReadRecord>,
    pub armed_read_trap: Option<[u8; 32]>,
    pub trap_read_token: Option<u64>,
    pub objects: Vec<ObjectRecord>,
    pub balance: super::balance::BalanceJournalRecord,
}

#[derive(Clone, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct ObjectRecord {
    pub catalog: ArchiveCatalog,
    pub tenant: Principal,
    pub id: u8,
    pub root: [u8; 32],
    pub bytes: u64,
    pub digest: Option<[u8; 32]>,
    pub phase: ArchivePhase,
    pub logical: u64,
    pub physical: u64,
    pub liability: u64,
    pub release_receipt: Option<bool>,
    pub content: Option<ContentRecord>,
}

#[derive(Clone, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct ContentRecord {
    pub chunks: Vec<[u8; 32]>,
    pub headers: Vec<(String, String)>,
    pub next_chunk: u64,
    pub verified_bytes: u64,
    pub verdict: JourneyVerification,
    // Protected hash state, never included in the public inspection view.
    pub checkpoint: Vec<u8>,
}

#[derive(Clone, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct ReadRecord {
    pub token: u64,
    pub valid: bool,
    pub tenant: Principal,
    pub root: [u8; 32],
    pub index: u64,
    pub gateway: Principal,
}

impl AuthorityArchiveRecord {
    /// Pending operations remain evidence, including invalidated reads. These
    /// checks cannot make a restored sequence or gateway authoritative again.
    pub fn pending_valid(&self) -> bool {
        let read_valid = self
            .pending_read
            .as_ref()
            .is_none_or(|read| self.read_valid(read));
        let trap_valid = self.trap_read_token.is_none_or(|token| {
            self.pending_read
                .as_ref()
                .is_some_and(|read| read.token == token)
        });
        let plan_valid = self.armed_read_trap.is_none_or(|root| {
            self.objects
                .iter()
                .any(|o| o.catalog == ArchiveCatalog::Journey && o.root == root)
        });
        read_valid && trap_valid && plan_valid
    }

    fn read_valid(&self, read: &ReadRecord) -> bool {
        if read.token == 0 || read.token != self.last_read {
            return false;
        }
        if read.gateway == Principal::anonymous()
            || read.gateway == Principal::management_canister()
        {
            return false;
        }
        if read.valid && !self.gateways.contains(&read.gateway) {
            return false;
        }
        self.objects.iter().any(|entry| {
            let bound = entry.catalog == ArchiveCatalog::Journey
                && entry.root == read.root
                && entry.tenant == read.tenant;
            let confirmed = matches!(
                entry.phase,
                ArchivePhase::Live
                    | ArchivePhase::DeletionPending
                    | ArchivePhase::ProviderDeleted
                    | ArchivePhase::Settled
            );
            let in_range = entry.content.as_ref().is_some_and(|content| {
                usize::try_from(read.index).is_ok_and(|index| index < content.chunks.len())
            });
            bound && confirmed && in_range
        })
    }

    pub fn bounded(&self) -> bool {
        let content_limits = super::content::manifest_limits();
        self.release == release_binding()
            && self.balance.valid(self.service, self.namespace)
            && self.objects.len() <= 14
            && self.gateways.len() <= 1
            && self.objects.iter().all(|object| {
                object.content.as_ref().is_none_or(|content| {
                    content.checkpoint.len() <= ic_blob_storage::model::identity::caffeine::manifest::verification::ordered::checkpoint::CaffeineVerificationCheckpointRecord::ENCODED_BYTES
                        && content.chunks.len() <= content_limits.max_chunks.get()
                        && content.headers.len() <= content_limits.max_headers.get()
                        && content
                            .headers
                            .iter()
                            .map(|(key, value)| key.len() + value.len() + 3)
                            .sum::<usize>()
                            <= content_limits.max_header_bytes.get()
                })
            })
    }
}
