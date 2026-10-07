//! Independent raw-byte source, bounded to one pending reply and 64 lifetime reads.
use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub(crate) struct BalanceSourceRecord {
    pub config: Option<ReplyRecord>,
    pub requests: u64,
    pub pending: bool,
    pub ready: bool,
}

#[derive(Clone, CandidType, Deserialize)]
pub(crate) struct ReplyRecord {
    pub account: Principal,
    pub bytes: Vec<u8>,
    pub reject: bool,
    pub hold: bool,
}

impl BalanceSourceRecord {
    /// Passive response inspection shares the bounded raw-byte configuration.
    /// A held/rejected/busy source cannot be bypassed through the query path.
    pub fn inspection_reply(&self, account: Principal) -> Option<Vec<u8>> {
        let config = self.config.as_ref()?;
        if self.pending || config.hold || config.reject || config.account != account {
            return None;
        }
        Some(config.bytes.clone())
    }

    pub fn new() -> Self {
        Self {
            config: None,
            requests: 0,
            pending: false,
            ready: false,
        }
    }
    pub fn configure(&mut self, input: ReplyRecord) -> bool {
        if self.pending
            || input.bytes.len() > 4097
            || input.account == Principal::anonymous()
            || input.account == Principal::management_canister()
        {
            return false;
        }
        self.config = Some(input);
        true
    }
    pub fn begin(&mut self, account: Principal) -> Option<ReplyRecord> {
        let config = self.config.as_ref()?;
        if self.pending || self.requests == 64 || config.account != account {
            return None;
        }
        self.requests += 1;
        self.pending = true;
        self.ready = !config.hold;
        Some(config.clone())
    }
    pub fn resume(&mut self) -> bool {
        if !self.pending || self.ready {
            return false;
        }
        self.ready = true;
        true
    }
    pub fn finish(&mut self) {
        self.pending = false;
        self.ready = false;
    }
    pub fn valid(&self) -> bool {
        self.requests <= 64
            && (!self.pending || (self.config.is_some() && self.requests > 0))
            && (!self.ready || self.pending)
            && self.config.as_ref().is_none_or(|c| {
                c.bytes.len() <= 4097
                    && c.account != Principal::anonymous()
                    && c.account != Principal::management_canister()
            })
    }
}
