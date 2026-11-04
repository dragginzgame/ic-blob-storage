//! Passive gateway root observations shared by the maintained callback workflow.
use ic_blob_storage_contracts::identity::HashParseError;
use ic_blob_storage_contracts::upload::history::UploadRootState;

/// Gateway root observation, never a provider liveness/deletion boolean.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadRootStatus {
    /// Current local state in the authenticated gateway's namespace.
    Known(UploadRootState),
    /// No local claim exists; this is not permission to delete a provider object.
    Unknown,
    /// Root belongs to another namespace; no phase or binding is disclosed.
    WrongNamespace,
    /// Malformed input preserved in its original batch position.
    Malformed(HashParseError),
}
