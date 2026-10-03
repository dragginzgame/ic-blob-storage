use super::*;
use crate::model::service::recovery::{InstanceChange, InstanceChangeKind as Kind};

fn history(changes: &[(u64, Kind)]) -> InstanceHistory {
    InstanceHistory {
        total_changes: changes.len() as u64,
        changes: changes
            .iter()
            .map(|&(version, kind)| InstanceChange { version, kind })
            .collect(),
    }
}

#[test]
fn current_upgrades_preserve_continuity_without_rotating_the_installation_anchor() {
    let mut retained = history(&[
        (1, Kind::Replacement),
        (4, Kind::Upgrade),
        (8, Kind::Controllers),
    ]);
    assert_eq!(assess_instance_continuity(1, 13, &retained), Ok(()));
    retained.total_changes = 50;
    assert_eq!(assess_instance_continuity(1, 13, &retained), Ok(()));
    retained.changes.remove(0);
    assert_eq!(
        assess_instance_continuity(1, 13, &retained),
        Err(InstanceContinuityError::IncompleteHistory)
    );
    retained.total_changes = retained.changes.len() as u64;
    assert_eq!(
        assess_instance_continuity(1, 13, &retained),
        Err(InstanceContinuityError::IncompleteHistory)
    );
}

#[test]
fn every_noncontinuous_change_blocks_even_with_a_plausible_local_counter() {
    for (kind, error) in [
        (Kind::Snapshot, InstanceContinuityError::SnapshotRestored),
        (Kind::Replacement, InstanceContinuityError::Replaced),
        (
            Kind::Unqualified,
            InstanceContinuityError::UnqualifiedChange,
        ),
    ] {
        assert_eq!(
            assess_instance_continuity(1, 200, &history(&[(1, Kind::Replacement), (9, kind)])),
            Err(error)
        );
    }
}

#[test]
fn unknown_order_counts_future_versions_and_missing_anchors_never_prove_freshness() {
    let good = history(&[(1, Kind::Replacement)]);
    assert_eq!(
        assess_instance_continuity(0, 1, &good),
        Err(InstanceContinuityError::MissingAnchor)
    );
    assert_eq!(
        assess_instance_continuity(2, 1, &good),
        Err(InstanceContinuityError::InvalidHistory)
    );
    for candidate in [
        history(&[(2, Kind::Upgrade), (1, Kind::Replacement)]),
        history(&[(1, Kind::Replacement), (1, Kind::Upgrade)]),
        history(&[(10, Kind::Upgrade)]),
    ] {
        assert_eq!(
            assess_instance_continuity(1, 9, &candidate),
            Err(InstanceContinuityError::InvalidHistory)
        );
    }
    assert_eq!(
        assess_instance_continuity(1, 9, &history(&[])),
        Err(InstanceContinuityError::IncompleteHistory)
    );
}
