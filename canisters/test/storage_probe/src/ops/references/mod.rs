//! Borrow the durable owner with fixture-only write interruption controls.
use super::{ProbeMemory, STATE, TRAP_WRITE};
use blob_test_protocol::storage::WriteFault;
use ic_blob_storage::ops::service::uploads::StableUploads;
pub(crate) fn with_uploads_mut<R>(
    fault: Option<WriteFault>,
    operation: impl FnOnce(&mut StableUploads<ProbeMemory>) -> R,
) -> R {
    TRAP_WRITE.set(fault);
    let result = STATE.with_borrow_mut(|state| operation(&mut state.as_mut().unwrap().uploads));
    TRAP_WRITE.set(None);
    result
}
