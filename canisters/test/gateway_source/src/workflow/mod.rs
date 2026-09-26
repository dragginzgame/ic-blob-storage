//! Deliberate reentrant schedules with captured request data across each await.

use blob_test_protocol::{SourceMode, SourceObservation, SyncFailure};
use candid::Principal;

use crate::ops;
pub(crate) mod readback;

pub(crate) fn initialize(service: Principal, gateway: Principal, driver: Principal) {
    ops::initialize(service, gateway, driver);
}

pub(crate) fn configure(caller: Principal, mode: SourceMode) -> bool {
    if !ops::read(|state| state.driver == caller && !state.fenced) {
        return false;
    }
    ops::configure(mode);
    true
}

pub(crate) fn observation(caller: Principal) -> Option<SourceObservation> {
    ops::read(|state| state.driver == caller).then(ops::observation)
}

pub(crate) async fn run_sync(caller: Principal) -> Result<(), SyncFailure> {
    let service = ops::read(|state| {
        (state.driver == caller && !state.fenced)
            .then_some(state.service)
            .ok_or(SyncFailure::Denied)
    })?;
    ops::sync(service).await
}

pub(crate) async fn run_deletion(caller: Principal, roots: Vec<Vec<u8>>) -> Result<(), u32> {
    let service = ops::read(|state| {
        assert_eq!(state.driver, caller, "fixture driver only");
        assert!(!state.fenced, "restored source is fenced");
        state.service
    });
    assert!(
        roots.len() <= 8 && roots.iter().all(|root| root.len() == 32),
        "bounded fixture roots"
    );
    ops::confirm_deletion(service, roots).await
}

pub(crate) async fn reply(caller: Principal) {
    if !ops::read(|state| state.service == caller && !state.fenced) {
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

pub(crate) fn prepare_upgrade() {
    ops::prepare_upgrade();
}
pub(crate) fn restore() {
    ops::restore();
}
pub(crate) fn recovery(
    caller: Principal,
) -> Option<blob_test_protocol::source::SourceRecoveryView> {
    ops::read(|state| state.driver == caller).then(ops::recovery)
}
