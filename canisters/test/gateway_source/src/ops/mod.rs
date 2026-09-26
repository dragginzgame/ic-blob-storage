//! Fixture state and individual platform effects; no production provider contract.

use std::cell::RefCell;

use blob_test_protocol::{SourceMode, SourceObservation, SyncFailure};
use candid::Principal;
use ic_cdk::call::Call;

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

pub(crate) struct State {
    pub service: Principal,
    pub gateway: Principal,
    pub driver: Principal,
    pub mode: SourceMode,
    pub observation: SourceObservation,
}

pub(crate) fn initialize(service: Principal, gateway: Principal, driver: Principal) {
    STATE.with_borrow_mut(|state| {
        *state = Some(State {
            service,
            gateway,
            driver,
            mode: SourceMode::Valid,
            observation: SourceObservation {
                requests: 0,
                nested_sync: None,
            },
        });
    });
}

pub(crate) fn read<T>(f: impl FnOnce(&State) -> T) -> T {
    STATE.with_borrow(|state| f(state.as_ref().expect("initialized source")))
}

fn mutate<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with_borrow_mut(|state| f(state.as_mut().expect("initialized source")))
}

pub(crate) fn configure(mode: SourceMode) {
    mutate(|state| {
        state.mode = mode;
        state.observation.nested_sync = None;
    });
}

pub(crate) fn receive() -> (Principal, Principal, SourceMode) {
    mutate(|state| {
        state.observation.requests = state
            .observation
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
    mutate(|state| state.observation.nested_sync = Some(result));
}

pub(crate) async fn sync(service: Principal) -> Result<(), SyncFailure> {
    Call::bounded_wait(service, "sync_gateway")
        .await
        .expect("local probe sync transport")
        .candid()
        .expect("typed probe reply")
}

pub(crate) async fn revoke(service: Principal) {
    let revoked: bool = Call::bounded_wait(service, "revoke_gateway")
        .await
        .expect("local revoke transport")
        .candid()
        .expect("revoke reply");
    assert!(revoked, "source is the explicit fixture operator");
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
