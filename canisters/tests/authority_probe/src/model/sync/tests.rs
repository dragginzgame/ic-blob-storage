use super::*;
use ic_platform::Principal;
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
