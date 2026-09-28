//! Internal configuration and live-owner checks for gateway workflows.
use super::{Memory, ServiceConfiguration, StableUploads, UploadStoreError};
impl<M: Memory> StableUploads<M> {
    pub(crate) const fn callback_configuration(&self) -> &ServiceConfiguration {
        &self.config
    }
    pub(crate) fn check_callback_active(&self) -> Result<(), UploadStoreError> {
        self.mutable()
    }
}
