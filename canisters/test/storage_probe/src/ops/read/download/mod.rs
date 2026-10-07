//! Explicit fixture project and owner access; no provider effects.
use crate::ops::{ProbeMemory, STATE};
use ic_blob_storage::{
    model::service::{read::download::CaffeineDownloadScope, upload::UploadContext},
    ops::service::uploads::StableUploads,
};
pub(crate) fn scope(context: UploadContext) -> CaffeineDownloadScope {
    CaffeineDownloadScope::new(
        context.service,
        1.try_into().unwrap(),
        "fixture project/β?&=",
    )
    .unwrap()
}
pub(crate) fn with_uploads<R>(operation: impl FnOnce(&StableUploads<ProbeMemory>) -> R) -> R {
    STATE.with_borrow(|state| operation(&state.as_ref().unwrap().uploads))
}
pub(crate) fn operator(context: UploadContext) -> bool {
    STATE.with_borrow(|state| state.as_ref().unwrap().operator == context.actor)
}
