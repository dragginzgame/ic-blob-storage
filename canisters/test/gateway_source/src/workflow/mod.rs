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

pub(crate) fn inspect_gateways(caller: Principal) {
    if !ops::read(|state| state.driver == caller) {
        ops::reject();
        return;
    }
    if let Some(gateway) = ops::gateway_inspection() {
        ops::reply_list(gateway);
    } else {
        ops::reject();
    }
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
    if !passive_reply(gateway, mode).await {
        return;
    }
    match mode {
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
        _ => {}
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

pub(crate) fn inspect_relationship(caller: Principal, owner: Principal) {
    // Driver-only substitute; not a claim about Cashier account-query authorization.
    if !ops::read(|state| state.driver == caller && !state.fenced) {
        ops::reject();
        return;
    }
    if let Some(bytes) = ops::balance::inspection_reply(owner) {
        ops::reply(bytes);
    } else {
        ops::reject();
    }
}

// Shared controlled byte/hold behavior for both local scheduling experiments.
async fn passive_reply(gateway: Principal, mode: SourceMode) -> bool {
    match mode {
        SourceMode::Hold => {
            let token = ops::sync_hold::begin(gateway);
            let ready = ops::sync_hold::wait(token).await;
            ops::sync_hold::finish(token);
            if ready {
                return true;
            }
            ops::reject();
        }
        SourceMode::Reject => ops::reject(),
        SourceMode::Malformed => ops::reply(vec![0]),
        SourceMode::Oversized => ops::reply(vec![0; 4097]),
        SourceMode::Empty => ops::reply_empty(),
        _ => return true,
    }
    false
}

// Explicit update substitute for scheduling evidence; never exported under the
// provider's query method. Scripted effects are unavailable on this endpoint.
pub(crate) async fn gateway_query(caller: Principal) {
    let allowed = ops::read(|state| {
        state.service == caller
            && !state.fenced
            && matches!(
                state.mode,
                SourceMode::Valid
                    | SourceMode::Hold
                    | SourceMode::Reject
                    | SourceMode::Malformed
                    | SourceMode::Oversized
                    | SourceMode::Empty
            )
    });
    if !allowed {
        ops::reject();
        return;
    }
    let (_, gateway, mode) = ops::receive();
    if passive_reply(gateway, mode).await {
        ops::reply_list(gateway);
    }
}
