//! Deliberate reentrant schedules with captured request data across each await.

use blob_test_protocol::{SourceMode, SourceObservation, SyncFailure};
use candid::Principal;

use crate::ops;

pub(crate) fn initialize(service: Principal, gateway: Principal, driver: Principal) {
    ops::initialize(service, gateway, driver);
}

pub(crate) fn configure(caller: Principal, mode: SourceMode) -> bool {
    if !ops::read(|state| state.driver == caller) {
        return false;
    }
    ops::configure(mode);
    true
}

pub(crate) fn observation(caller: Principal) -> Option<SourceObservation> {
    ops::read(|state| (state.driver == caller).then_some(state.observation))
}

pub(crate) async fn run_sync(caller: Principal) -> Result<(), SyncFailure> {
    let service = ops::read(|state| {
        (state.driver == caller)
            .then_some(state.service)
            .ok_or(SyncFailure::Denied)
    })?;
    ops::sync(service).await
}

pub(crate) async fn reply(caller: Principal) {
    if !ops::read(|state| state.service == caller) {
        ops::reject();
        return;
    }
    let (service, gateway, mode) = ops::receive();
    match mode {
        SourceMode::Reject => {
            ops::reject();
            return;
        }
        SourceMode::Malformed => {
            ops::reply(vec![0]);
            return;
        }
        SourceMode::Oversized => {
            ops::reply(vec![0; 4097]);
            return;
        }
        SourceMode::Empty => {
            ops::reply_empty();
            return;
        }
        SourceMode::Overlap => {
            let result = ops::sync(service).await;
            ops::record_nested(result);
        }
        SourceMode::Revoke => ops::revoke(service).await,
        SourceMode::Replace(next) => {
            ops::revoke(service).await;
            ops::replace(next);
            let result = ops::sync(service).await;
            ops::record_nested(result);
        }
        SourceMode::Valid => {}
    }
    // Deliberately return the captured OLD list after the reentrant mutation.
    ops::reply_list(gateway);
}
