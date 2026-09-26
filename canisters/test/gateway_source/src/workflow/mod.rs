//! Deliberate reentrant schedules with captured request data across each await.

use blob_test_protocol::{SourceMode, SourceObservation, SyncFailure};
use candid::Principal;

use crate::ops;
pub(crate) mod readback;

pub(crate) fn initialize(service: Principal, gateway: Principal, driver: Principal) {
    ops::initialize(service, gateway, driver);
}

pub(crate) fn configure(caller: Principal, mode: SourceMode) -> bool {
    if !ops::read(|state| state.driver == caller && !state.fenced && state.sync.held.is_none()) {
        return false;
    }
    ops::configure(mode);
    true
}

pub(crate) fn resume_sync(caller: Principal) -> bool {
    if !ops::read(|state| state.driver == caller && !state.fenced) {
        return false;
    }
    ops::sync_hold::resume()
}

pub(crate) fn observation(caller: Principal) -> Option<SourceObservation> {
    ops::read(|state| state.driver == caller).then(ops::observation)
}

pub(crate) async fn run_sync(
    caller: Principal,
    input: blob_test_protocol::GatewaySyncRequest,
) -> Result<(), SyncFailure> {
    let service = ops::read(|state| {
        (state.driver == caller && !state.fenced)
            .then_some(state.service)
            .ok_or(SyncFailure::Denied)
    })?;
    if input.service != service {
        return Err(SyncFailure::Binding);
    }
    ops::sync(service, input).await
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

pub(crate) async fn reply(caller: Principal, input: blob_test_protocol::GatewaySyncRequest) {
    if !ops::read(|state| state.service == caller && !state.fenced) {
        ops::reject();
        return;
    }
    let (service, gateway, mode) = ops::receive();
    match mode {
        SourceMode::Hold => {
            let token = ops::sync_hold::begin(gateway);
            let ready = ops::sync_hold::wait(token).await;
            ops::sync_hold::finish(token);
            if !ready {
                ops::reject();
                return;
            }
        }
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
            let result = ops::sync(service, input).await;
            ops::record_nested(result);
        }
        SourceMode::Revoke => ops::revoke(service).await,
        SourceMode::Replace(next) => {
            ops::revoke(service).await;
            ops::replace(next);
            // This deliberate scenario explicitly authorizes a new sync after
            // its own revocation; it is not a retry of the original request.
            let next_request = ops::next_sync_request(input);
            let result = ops::sync(service, next_request).await;
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

pub(crate) fn configure_balance(
    caller: Principal,
    config: blob_test_protocol::balance::BalanceSourceConfig,
) -> bool {
    if !ops::read(|s| s.driver == caller && !s.fenced) {
        return false;
    }
    ops::balance::configure(config)
}
pub(crate) fn resume_balance(caller: Principal) -> bool {
    if !ops::read(|s| s.driver == caller && !s.fenced) {
        return false;
    }
    ops::balance::resume()
}
pub(crate) fn balance_observation(
    caller: Principal,
) -> Option<blob_test_protocol::balance::BalanceSourceView> {
    ops::read(|s| s.driver == caller).then(ops::balance::view)
}
pub(crate) async fn balance_reply(caller: Principal, account: Principal) {
    if !ops::read(|s| s.service == caller && !s.fenced) {
        ops::reject();
        return;
    }
    let Some(reply) = ops::balance::begin(account) else {
        ops::reject();
        return;
    };
    let ready = ops::balance::wait().await;
    ops::balance::finish();
    if !ready || reply.reject {
        ops::reject();
    } else {
        ops::reply(reply.bytes);
    }
}
