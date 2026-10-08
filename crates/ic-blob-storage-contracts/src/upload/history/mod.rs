//! Pure upload history contracts.
pub mod reply;

use crate::dto::upload::history::UploadHistoryFilter;
/// Distinct logical, physical and economic stages of a confirmed object's release.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecyclePhase {
    /// At least one live reference remains.
    Live,
    /// No live references remain; physical storage and billing are unresolved.
    DeletionPending,
    /// Physical deletion is confirmed; financial obligations remain unresolved.
    ProviderDeleted,
    /// Deletion and final billing cessation are both confirmed.
    Settled,
}

/// Local root state spanning reservations and confirmed lifecycle history.
/// None of these observations grants deletion permission or proves provider state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadRootState {
    /// Unexposed reservation; must not be mistaken for an unknown/dead root.
    Reserved,
    /// Authority may have escaped; preserve the unresolved object's reservation.
    ExposurePossible,
    /// Cancelled before exposure; root history remains claimed, not reusable.
    Cancelled,
    /// Current confirmed-object phase, including retained settlement history.
    Confirmed(LifecyclePhase),
}

/// Current local states to return; none grants permission to repeat an effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadScanFilter {
    /// Every retained operation, including cancellation and settlement history.
    All,
    /// Reserved or possibly exposed uploads.
    Active,
    /// Last reference released, physical deletion still unresolved.
    DeletionPending,
    /// Active uploads or confirmed objects with any unresolved obligation.
    Outstanding,
}
impl UploadScanFilter {
    /// Test only the passive state tag; no snapshot or authority is implied.
    #[must_use]
    pub fn includes(self, state: UploadRootState) -> bool {
        match self {
            Self::All => true,
            Self::Active => matches!(
                state,
                UploadRootState::Reserved | UploadRootState::ExposurePossible
            ),
            Self::DeletionPending => {
                state == UploadRootState::Confirmed(LifecyclePhase::DeletionPending)
            }
            Self::Outstanding => !matches!(
                state,
                UploadRootState::Cancelled | UploadRootState::Confirmed(LifecyclePhase::Settled)
            ),
        }
    }
}
/// Convert the wire filter to its passive local-state classifier without state access.
#[must_use]
pub fn filter(input: UploadHistoryFilter) -> UploadScanFilter {
    match input {
        UploadHistoryFilter::All => UploadScanFilter::All,
        UploadHistoryFilter::Active => UploadScanFilter::Active,
        UploadHistoryFilter::DeletionPending => UploadScanFilter::DeletionPending,
        UploadHistoryFilter::Outstanding => UploadScanFilter::Outstanding,
    }
}
