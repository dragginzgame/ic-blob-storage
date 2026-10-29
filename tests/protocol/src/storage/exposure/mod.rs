//! Exact-operation fault controls for private atomic-commit fixtures.
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use serde::Deserialize;
/// Private exposure control, also used for operator-only certificate test setup.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct ExposureInput {
    /// Exact original permission.
    pub permission: UploadAdmissionRequest,
    /// Trap during the permission write.
    pub trap_write: bool,
    /// Trap after the shared handler returns; the whole IC update must roll back.
    pub trap_after: bool,
}
