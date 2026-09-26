//! Same-release fixture records; no product persistence or reconciliation schema.
use blob_test_protocol::{SourceMode, SyncFailure, journey::readback::ReadSourceMode};
use candid::{CandidType, Principal};
use serde::Deserialize;

pub(crate) const CHUNK: usize = 1024 * 1024;
const MAX_EFFECTS: usize = 64;

#[derive(CandidType, Deserialize)]
pub(crate) struct SourceJournalRecord {
    pub service: Principal,
    pub gateway: Principal,
    pub driver: Principal,
    pub fenced: bool,
    pub mode: SourceMode,
    pub requests: u64,
    pub nested_sync: Option<Result<(), SyncFailure>>,
    pub read: ReadRecord,
    pub effects: Vec<EffectRecord>,
}

#[derive(CandidType, Deserialize)]
pub(crate) struct ReadRecord {
    pub config: Option<LeafRecord>,
    pub requests: u64,
    pub pending: bool,
    pub ready: bool,
}

#[derive(Clone, CandidType, Deserialize)]
pub(crate) struct LeafRecord {
    pub root: [u8; 32],
    pub index: u64,
    pub bytes: Vec<u8>,
    pub mode: ReadSourceMode,
    pub hold: bool,
}

#[derive(Clone, CandidType, Deserialize)]
pub(crate) enum ActionRecord {
    Sync,
    Revoke,
    Delete(Vec<[u8; 32]>),
}

#[derive(CandidType, Deserialize)]
pub(crate) struct EffectRecord {
    pub action: ActionRecord,
    pub succeeded: Option<bool>,
}

impl SourceJournalRecord {
    pub fn new(service: Principal, gateway: Principal, driver: Principal) -> Self {
        Self {
            service,
            gateway,
            driver,
            fenced: false,
            mode: SourceMode::Valid,
            requests: 0,
            nested_sync: None,
            read: ReadRecord {
                config: None,
                requests: 0,
                pending: false,
                ready: false,
            },
            effects: Vec::new(),
        }
    }

    pub fn valid(&self) -> bool {
        self.read.config.as_ref().is_none_or(LeafRecord::valid)
            && (!self.read.pending || self.read.config.is_some())
            && self.effects.len() <= MAX_EFFECTS
            && self.effects.iter().all(|effect| effect.action.valid())
    }

    pub fn busy(&self) -> bool {
        self.read.pending || self.effects.iter().any(|effect| effect.succeeded.is_none())
    }

    // Fencing is one-way, including when the loaded journal predates the fence.
    pub fn fence(&mut self) {
        self.fenced = true;
    }

    pub fn begin_effect(&mut self, action: ActionRecord) -> usize {
        assert!(
            !self.fenced && action.valid(),
            "active source and bounded action required"
        );
        assert!(self.effects.len() < MAX_EFFECTS, "lifetime effect budget");
        let id = self.effects.len();
        self.effects.push(EffectRecord {
            action,
            succeeded: None,
        });
        id
    }

    pub fn finish_effect(&mut self, id: usize, succeeded: bool) {
        assert!(!self.fenced, "restored source cannot resolve callbacks");
        let effect = self.effects.get_mut(id).expect("admitted effect");
        assert!(effect.succeeded.is_none(), "exact pending effect");
        effect.succeeded = Some(succeeded);
    }
}

impl ActionRecord {
    fn valid(&self) -> bool {
        match self {
            Self::Delete(roots) => roots.len() <= 8,
            Self::Sync | Self::Revoke => true,
        }
    }
}

impl LeafRecord {
    pub fn valid(&self) -> bool {
        !self.bytes.is_empty() && self.bytes.len() <= CHUNK && self.index < 6
    }
}
