//! Fixture-owned admission revision; membership and callback tokens remain library-owned.
use blob_test_protocol::SyncFailure;
use ic_blob_storage::model::gateway::registry::{GatewayScope, GatewaySyncView};
use serde::Deserialize;

#[derive(Clone, Eq, PartialEq, serde::Serialize, Deserialize)]
pub(crate) struct SyncControlRecord {
    /// Exhaustion permanently prevents new syncs but never prevents revocation.
    pub revision: Option<u64>,
}

impl SyncControlRecord {
    pub fn new() -> Self {
        Self { revision: Some(0) }
    }

    pub fn revoke(&mut self) {
        self.revision = self.revision.and_then(|revision| revision.checked_add(1));
    }

    pub fn check(
        &self,
        scope: GatewayScope,
        view: GatewaySyncView,
        expected: GatewayScope,
        revision: u64,
        sequence: u64,
    ) -> Result<(), SyncFailure> {
        if scope != expected {
            return Err(SyncFailure::Binding);
        }
        if view.pending_sequence.is_some() {
            return Err(SyncFailure::InProgress);
        }
        let current = self.revision.ok_or(SyncFailure::Admission)?;
        let next = view
            .last_sequence
            .checked_add(1)
            .ok_or(SyncFailure::Admission)?;
        if revision != current || sequence != next {
            return Err(SyncFailure::Stale);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
