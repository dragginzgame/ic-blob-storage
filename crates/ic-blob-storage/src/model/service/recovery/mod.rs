//! Platform history observations, separate from counters retained in a backup.

/// One independently obtained IC management change.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InstanceChange {
    /// IC-maintained version after the change, never supplied by ingress.
    pub version: u64,
    /// Classification of the platform's current change details.
    pub kind: InstanceChangeKind,
}

/// Management operations relevant to retaining the current instance's journals.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstanceChangeKind {
    /// Ordinary code upgrade retains stable memory; release identity is checked separately.
    Upgrade,
    /// Controller changes do not change tenant authority or replace stable state.
    Controllers,
    /// Creation, installation, reinstallation or code removal starts another state boundary.
    Replacement,
    /// Snapshot loads may discard later identities, effects and liabilities.
    Snapshot,
    /// Renaming or missing/unknown details cannot establish this installation's continuity.
    Unqualified,
}

/// Bounded platform observations, not a resumable checkpoint or proof by themselves.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstanceHistory {
    /// All changes ever recorded, including changes outside the retained window.
    pub total_changes: u64,
    /// Oldest to newest; the IC retains at least its latest twenty changes.
    pub changes: Vec<InstanceChange>,
}
