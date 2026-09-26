//! Fixture state and individual platform effects; no production provider contract.
pub(crate) mod readback;
mod storage;

use crate::model::{ActionRecord, SourceJournalRecord};
use blob_test_protocol::{
    SourceMode, SourceObservation, SyncFailure,
    source::{SourceAction, SourceEffectView, SourceRecoveryView},
};
use candid::Principal;
use ic_cdk::call::{Call, CallFailed};
use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<Option<SourceJournalRecord>> = const { RefCell::new(None) };
}

pub(crate) fn initialize(service: Principal, gateway: Principal, driver: Principal) {
    storage::open();
    let state = SourceJournalRecord::new(service, gateway, driver);
    storage::save(&state);
    STATE.with_borrow_mut(|slot| *slot = Some(state));
}

pub(crate) fn prepare_upgrade() {
    assert!(
        !read(SourceJournalRecord::busy),
        "source has unresolved work"
    );
}

pub(crate) fn restore() {
    storage::open();
    let mut state = storage::load();
    assert!(state.valid(), "bounded source journal required");
    state.fence();
    storage::save(&state);
    STATE.with_borrow_mut(|slot| *slot = Some(state));
}

pub(crate) fn read<T>(f: impl FnOnce(&SourceJournalRecord) -> T) -> T {
    STATE.with_borrow(|state| f(state.as_ref().expect("initialized source")))
}

fn mutate<T>(f: impl FnOnce(&mut SourceJournalRecord) -> T) -> T {
    STATE.with_borrow_mut(|state| {
        let state = state.as_mut().expect("initialized source");
        assert!(!state.fenced, "restored source is fenced");
        let result = f(state);
        storage::save(state);
        result
    })
}

pub(crate) fn observation() -> SourceObservation {
    read(|state| SourceObservation {
        requests: state.requests,
        nested_sync: state.nested_sync,
    })
}

pub(crate) fn recovery() -> SourceRecoveryView {
    read(|state| SourceRecoveryView {
        fenced: state.fenced,
        service: state.service,
        gateway: state.gateway,
        driver: state.driver,
        mode: state.mode,
        read: state.read.config.as_ref().map(readback::config_view),
        read_pending: state.read.pending,
        read_ready: state.read.ready,
        effects: state
            .effects
            .iter()
            .map(|effect| SourceEffectView {
                action: match &effect.action {
                    ActionRecord::Sync => SourceAction::Sync,
                    ActionRecord::Revoke => SourceAction::Revoke,
                    ActionRecord::Delete(roots) => SourceAction::Delete(roots.clone()),
                },
                succeeded: effect.succeeded,
            })
            .collect(),
    })
}

pub(crate) fn configure(mode: SourceMode) {
    mutate(|state| {
        state.mode = mode;
        state.nested_sync = None;
    });
}

pub(crate) fn receive() -> (Principal, Principal, SourceMode) {
    mutate(|state| {
        state.requests = state
            .requests
            .checked_add(1)
            .expect("fixture request bound");
        (state.service, state.gateway, state.mode)
    })
}

pub(crate) fn replace(gateway: Principal) {
    mutate(|state| {
        state.gateway = gateway;
        state.mode = SourceMode::Valid;
    });
}

pub(crate) fn record_nested(result: Result<(), SyncFailure>) {
    mutate(|state| state.nested_sync = Some(result));
}

pub(crate) async fn sync(service: Principal) -> Result<(), SyncFailure> {
    let id = mutate(|state| state.begin_effect(ActionRecord::Sync));
    let result: Result<(), SyncFailure> = Call::bounded_wait(service, "sync_gateway")
        .await
        .expect("local probe sync transport")
        .candid()
        .expect("typed probe reply");
    mutate(|state| state.finish_effect(id, result.is_ok()));
    result
}

pub(crate) async fn revoke(service: Principal) {
    let id = mutate(|state| state.begin_effect(ActionRecord::Revoke));
    let revoked: bool = Call::bounded_wait(service, "revoke_gateway")
        .await
        .expect("local revoke transport")
        .candid()
        .expect("revoke reply");
    mutate(|state| state.finish_effect(id, revoked));
    assert!(revoked, "source is the explicit fixture operator");
}

pub(crate) async fn confirm_deletion(service: Principal, roots: Vec<Vec<u8>>) -> Result<(), u32> {
    let retained = roots
        .iter()
        .map(|root| root.as_slice().try_into().expect("bounded root"))
        .collect();
    let id = mutate(|state| state.begin_effect(ActionRecord::Delete(retained)));
    let result = match Call::unbounded_wait(service, "_immutableObjectStorageConfirmBlobDeletion")
        .with_arg(roots)
        .await
    {
        Ok(reply) => reply.candid::<()>().map_err(|_| 0),
        Err(CallFailed::CallRejected(error)) => Err(error.raw_reject_code()),
        Err(CallFailed::InsufficientLiquidCycleBalance(_) | CallFailed::CallPerformFailed(_)) => {
            Err(0)
        }
    };
    mutate(|state| state.finish_effect(id, result.is_ok()));
    result
}

pub(crate) fn reply_list(gateway: Principal) {
    reply(candid::encode_one(vec![gateway]).expect("fixture principal list"));
}
pub(crate) fn reply_empty() {
    reply(candid::encode_one(Vec::<Principal>::new()).expect("empty list"));
}
pub(crate) fn reply(bytes: Vec<u8>) {
    ic_cdk::api::msg_reply(bytes);
}
pub(crate) fn reject() {
    ic_cdk::api::msg_reject("deliberate local source rejection");
}
