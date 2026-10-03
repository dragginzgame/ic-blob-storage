//! Pure current-instance continuity checks; no locally restored counter grants freshness.
use crate::model::service::recovery::{InstanceChangeKind, InstanceHistory};

/// A current installation cannot be proved continuous from the supplied IC history.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum InstanceContinuityError {
    /// No real platform installation version was captured by the installing host.
    #[error("platform installation anchor is unavailable")]
    MissingAnchor,
    /// The returned bounded history cannot cover every change since installation.
    #[error("platform history does not cover the installation")]
    IncompleteHistory,
    /// Ordering, counts or versions disagree with the running platform observation.
    #[error("invalid platform history")]
    InvalidHistory,
    /// A snapshot load can have erased later obligations; this path never repairs it.
    #[error("snapshot restoration requires surviving obligation evidence")]
    SnapshotRestored,
    /// Installation/reinstallation/removal changed the state boundary.
    #[error("platform state was replaced")]
    Replaced,
    /// A rename or missing change details leave continuity unqualified.
    #[error("unqualified platform change")]
    UnqualifiedChange,
}

/// Prove that every IC management change after the immutable installation anchor
/// preserves this instance's stable journals. Ordinary message counters are not
/// used as completeness evidence. A truncated history is accepted only when its
/// oldest retained change reaches the installation anchor. No snapshot restore,
/// replacement, unknown detail or window rotation is accepted.
///
/// The caller must independently authenticate the platform history and current
/// version; this pure predicate alone grants no operational authority.
/// # Errors
/// Rejects missing anchors, incomplete/invalid history and noncontinuous changes.
pub fn assess_instance_continuity(
    installation_version: u64,
    current_version: u64,
    history: &InstanceHistory,
) -> Result<(), InstanceContinuityError> {
    use InstanceContinuityError as Error;
    if installation_version == 0 {
        return Err(Error::MissingAnchor);
    }
    if current_version < installation_version
        || history.changes.len() > 20
        || history.total_changes < history.changes.len() as u64
        || history
            .changes
            .iter()
            .any(|change| change.version > current_version)
        || history
            .changes
            .windows(2)
            .any(|pair| pair[0].version >= pair[1].version)
    {
        return Err(Error::InvalidHistory);
    }
    let Some(first) = history.changes.first() else {
        return Err(Error::IncompleteHistory);
    };
    if first.version > installation_version {
        return Err(Error::IncompleteHistory);
    }
    for change in history
        .changes
        .iter()
        .filter(|change| change.version > installation_version)
    {
        match change.kind {
            InstanceChangeKind::Upgrade | InstanceChangeKind::Controllers => {}
            InstanceChangeKind::Snapshot => return Err(Error::SnapshotRestored),
            InstanceChangeKind::Replacement => return Err(Error::Replaced),
            InstanceChangeKind::Unqualified => return Err(Error::UnqualifiedChange),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
