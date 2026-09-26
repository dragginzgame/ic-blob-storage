//! Bounded inspection records, deliberately insufficient to resume hashing/authority.
use blob_test_protocol::authority::{ArchiveCatalog, ArchivePhase};
use blob_test_protocol::journey::JourneyVerification;
use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub(crate) struct AuthorityArchiveRecord {
    pub service: Principal,
    pub operator: Principal,
    pub tenants: [Principal; 2],
    pub namespace: u128,
    pub initial_gateway: Principal,
    pub sync_source: Principal,
    pub gateways: Vec<Principal>,
    pub last_sync: u64,
    pub pending_sync: Option<u64>,
    pub last_read: u64,
    pub pending_read: Option<ReadRecord>,
    pub armed_read_trap: Option<[u8; 32]>,
    pub trap_read_token: Option<u64>,
    pub objects: Vec<ObjectRecord>,
}

#[derive(CandidType, Deserialize)]
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

#[derive(CandidType, Deserialize)]
pub(crate) struct ContentRecord {
    pub chunks: Vec<[u8; 32]>,
    pub headers: Vec<(String, String)>,
    pub next_chunk: u64,
    pub verified_bytes: u64,
    pub verdict: JourneyVerification,
}

#[derive(Clone, CandidType, Deserialize)]
pub(crate) struct ReadRecord {
    pub token: u64,
    pub valid: bool,
    pub tenant: Principal,
    pub root: [u8; 32],
    pub index: u64,
    pub gateway: Principal,
}

impl AuthorityArchiveRecord {
    pub fn bounded(&self) -> bool {
        self.objects.len() <= 14
            && self.gateways.len() <= 1
            && self.objects.iter().all(|object| {
                object.content.as_ref().is_none_or(|content| {
                    content.chunks.len() <= 6
                        && content.headers.len() <= 8
                        && content
                            .headers
                            .iter()
                            .map(|(key, value)| key.len() + value.len() + 3)
                            .sum::<usize>()
                            <= 1024
                })
            })
    }
}
