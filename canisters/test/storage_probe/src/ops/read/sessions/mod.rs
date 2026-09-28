//! Explicit memory grants, local fault selection and passive session observations.
use crate::ops::{ProbeMemory, STATE, TRAP_WRITE};
use blob_test_protocol::storage::{Failure, WriteFault, gateways::ReadSessionsView};
use ic_blob_storage::{
    model::service::{
        configuration::ServiceConfiguration,
        read::session::{ReadSessionError, ReadSessionLimits},
        upload::UploadContext,
    },
    ops::service::{
        gateways::StableGatewayRegistry,
        reads::{ReadSessionMemories, StableReadSessions},
        uploads::StableUploads,
    },
};
pub(crate) fn initialize(
    memory: ReadSessionMemories<ProbeMemory>,
    config: &ServiceConfiguration,
    restored: bool,
) -> StableReadSessions<ProbeMemory> {
    let limits = ReadSessionLimits::new(
        1.try_into().unwrap(),
        1.try_into().unwrap(),
        2048.try_into().unwrap(),
        2048.try_into().unwrap(),
        2048.try_into().unwrap(),
    )
    .unwrap();
    if restored {
        StableReadSessions::open(memory, *config, limits)
    } else {
        StableReadSessions::install(memory, *config, limits)
    }
    .unwrap()
}
pub(crate) fn with_owners<R>(
    fault: Option<WriteFault>,
    operation: impl FnOnce(
        &StableGatewayRegistry<ProbeMemory>,
        &StableUploads<ProbeMemory>,
        &mut StableReadSessions<ProbeMemory>,
    ) -> R,
) -> R {
    STATE.with_borrow_mut(|state| {
        let state = state.as_mut().unwrap();
        TRAP_WRITE.set(fault);
        let result = operation(&state.gateways, &state.uploads, &mut state.read_sessions);
        TRAP_WRITE.set(None);
        result
    })
}
pub(crate) fn failure(error: ReadSessionError) -> Failure {
    match error {
        ReadSessionError::Binding => Failure::Binding,
        ReadSessionError::Denied => Failure::Denied,
        ReadSessionError::Capacity | ReadSessionError::Exhausted => Failure::Capacity,
        ReadSessionError::Stale => Failure::Conflict,
        ReadSessionError::Fenced => Failure::Fenced,
        _ => Failure::Invalid,
    }
}
pub(crate) fn inspect(context: UploadContext) -> Result<ReadSessionsView, Failure> {
    STATE.with_borrow(|state| {
        let summary = state
            .as_ref()
            .unwrap()
            .read_sessions
            .inspect(context)
            .map_err(failure)?;
        Ok(ReadSessionsView {
            last_sequence: summary.last_sequence,
            sessions: summary.usage.sessions,
            reserved_bytes: summary.usage.reserved_bytes,
            fenced: summary.fenced,
        })
    })
}

/// The shared handler borrows owners once at admission and once at callback.
/// Fault selection is local fixture state, never part of the production workflow.
pub(crate) struct Host {
    pub admit_fault: Option<WriteFault>,
    pub callback_fault: Option<WriteFault>,
    pub admitted: std::cell::Cell<bool>,
}
impl ic_blob_storage::ops::service::reads::access::ReadSessionAccess for Host {
    type Memory = ProbeMemory;
    fn with_read_sessions<R>(
        &self,
        operation: impl FnOnce(
            &StableGatewayRegistry<ProbeMemory>,
            &StableUploads<ProbeMemory>,
            &mut StableReadSessions<ProbeMemory>,
        ) -> R,
    ) -> R {
        let fault = if self.admitted.replace(true) {
            self.callback_fault
        } else {
            self.admit_fault
        };
        with_owners(fault, operation)
    }
}
