//! Fixture-owned admission revision; membership and callback tokens remain library-owned.
use blob_test_protocol::SyncFailure;
use candid::CandidType;
use ic_blob_storage::model::gateway::registry::{GatewayScope, GatewaySyncView};
use serde::Deserialize;

#[derive(Clone, Eq, PartialEq, CandidType, Deserialize)]
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
mod tests {
    use super::*;
    use candid::Principal;
    use std::num::NonZeroU128;

    #[test]
    fn edits_invalidate_previews_and_exhaustion_never_blocks_revocation() {
        let scope = GatewayScope::new(
            Principal::from_slice(&[1]),
            NonZeroU128::new(1).unwrap(),
            Principal::from_slice(&[2]),
        )
        .unwrap();
        let mut record = SyncControlRecord::new();
        let mut view = GatewaySyncView {
            last_sequence: 0,
            pending_sequence: None,
        };
        assert_eq!(record.check(scope, view, scope, 0, 1), Ok(()));
        record.revoke();
        assert_eq!(
            record.check(scope, view, scope, 0, 1),
            Err(SyncFailure::Stale)
        );
        assert_eq!(record.check(scope, view, scope, 1, 1), Ok(()));
        view.last_sequence = 1;
        assert_eq!(
            record.check(scope, view, scope, 1, 1),
            Err(SyncFailure::Stale)
        );
        view.pending_sequence = Some(1);
        assert_eq!(
            record.check(scope, view, scope, 1, 2),
            Err(SyncFailure::InProgress)
        );
        view.pending_sequence = None;
        record.revision = Some(u64::MAX);
        record.revoke();
        record.revoke();
        assert_eq!(record.revision, None);
        assert_eq!(
            record.check(scope, view, scope, u64::MAX, 2),
            Err(SyncFailure::Admission)
        );
    }
}
