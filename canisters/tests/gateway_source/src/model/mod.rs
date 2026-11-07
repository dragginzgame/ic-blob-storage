//! Same-release fixture records; no product persistence or reconciliation schema.
use ic_platform::principal::ExternalPrincipal;
pub(crate) mod balance;

use blob_test_protocol::SourceMode;
use ic_platform::Principal;
use serde::Deserialize;

const MAX_EFFECTS: usize = 64;

#[derive(serde::Serialize, Deserialize)]
pub(crate) struct SourceJournalRecord {
    pub service: ExternalPrincipal,
    pub gateway: ExternalPrincipal,
    pub driver: ExternalPrincipal,
    pub fenced: bool,
    pub mode: SourceMode,
    pub requests: u64,
    pub sync: SyncRecord,
    pub balance: balance::BalanceSourceRecord,
    pub effects: Vec<EffectRecord>,
}

// Required record boundary: an older journal cannot decode a missing optional
// held reply as an empty new state. Fixture schema remains v1, reinstall only.
#[derive(serde::Serialize, Deserialize)]
pub(crate) struct SyncRecord {
    pub held: Option<HeldSyncRecord>,
}

#[derive(serde::Serialize, Deserialize)]
pub(crate) struct HeldSyncRecord {
    pub sequence: u64,
    pub gateway: ExternalPrincipal,
    pub ready: bool,
}

#[derive(Clone, serde::Serialize, Deserialize)]
pub(crate) enum ActionRecord {
    Delete(Vec<[u8; 32]>),
}

#[derive(serde::Serialize, Deserialize)]
pub(crate) struct EffectRecord {
    pub action: ActionRecord,
    pub succeeded: Option<bool>,
}

impl SourceJournalRecord {
    /// A passive gateway observation cannot run a scripted scheduling/effect mode.
    pub fn gateway_inspection(&self) -> Option<GatewayInspection> {
        (!self.fenced && self.sync.held.is_none()).then_some(GatewayInspection {
            gateway: self.gateway.into(),
            mode: self.mode,
        })
    }

    pub fn new(service: Principal, gateway: Principal, driver: Principal) -> Self {
        Self {
            service: service.into(),
            gateway: gateway.into(),
            driver: driver.into(),
            fenced: false,
            mode: SourceMode::Valid,
            requests: 0,
            sync: SyncRecord { held: None },
            balance: balance::BalanceSourceRecord::new(),
            effects: Vec::new(),
        }
    }

    pub fn valid(&self) -> bool {
        self.balance.valid()
            && self.effects.len() <= MAX_EFFECTS
            && self.effects.iter().all(|effect| effect.action.valid())
            && self.sync.held.as_ref().is_none_or(|held| {
                held.sequence != 0
                    && held.sequence == self.requests
                    && held.gateway != Principal::anonymous().into()
                    && held.gateway != Principal::management_canister().into()
            })
    }

    pub fn busy(&self) -> bool {
        self.balance.pending
            || self.sync.held.is_some()
            || self.effects.iter().any(|effect| effect.succeeded.is_none())
    }

    pub fn begin_held_sync(&mut self, gateway: Principal) -> u64 {
        assert!(
            !self.fenced && self.sync.held.is_none(),
            "one active held reply"
        );
        let sequence = self.requests;
        assert!(sequence != 0, "received request required");
        self.sync.held = Some(HeldSyncRecord {
            sequence,
            gateway: gateway.into(),
            ready: false,
        });
        sequence
    }

    pub fn resume_held_sync(&mut self) -> bool {
        assert!(!self.fenced, "active source required");
        let Some(held) = &mut self.sync.held else {
            return false;
        };
        if held.ready {
            return false;
        }
        held.ready = true;
        true
    }

    pub fn finish_held_sync(&mut self, token: u64) {
        assert!(!self.fenced, "active source required");
        assert!(
            self.sync
                .held
                .as_ref()
                .is_some_and(|held| held.sequence == token),
            "exact held reply"
        );
        self.sync.held = None;
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

#[derive(Clone, Copy)]
pub(crate) struct GatewayInspection {
    pub gateway: Principal,
    pub mode: SourceMode,
}

impl ActionRecord {
    fn valid(&self) -> bool {
        match self {
            Self::Delete(roots) => roots.len() <= 8,
        }
    }
}
