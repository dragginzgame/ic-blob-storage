//! Read-only projection of retained content into history entries.
use crate::model::service::upload::content::TenantContentView;
use ic_blob_storage_contracts::dto::upload::history::{UploadContentState, UploadHistoryEntry};
use ic_blob_storage_contracts::upload::history::{LifecyclePhase, UploadRootState};
pub(crate) fn present(view: TenantContentView) -> UploadHistoryEntry {
    UploadHistoryEntry {
        request: super::admission::upload(view.request),
        state: match view.state {
            UploadRootState::Reserved => UploadContentState::Reserved,
            UploadRootState::Cancelled => UploadContentState::Cancelled,
            UploadRootState::Confirmed(phase) => match phase {
                LifecyclePhase::Live => UploadContentState::Live,
                LifecyclePhase::DeletionPending => UploadContentState::DeletionPending,
                LifecyclePhase::ProviderDeleted => UploadContentState::ProviderDeleted,
                LifecyclePhase::Settled => UploadContentState::Settled,
            },
        },
    }
}
