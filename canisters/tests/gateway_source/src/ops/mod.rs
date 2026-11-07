//! Fixture state and individual platform effects; no production provider contract.
pub(crate) mod balance;
mod scheduling;
mod storage;
pub(crate) mod sync_hold;

pub(crate) fn unfunded_replicated_query() -> bool {
    ic_platform::cdk::api::in_replicated_execution()
        && ic_platform::cdk::api::msg_cycles_available() == 0
}

use crate::model::{ActionRecord, SourceJournalRecord};
use blob_test_protocol::{
    SourceMode, SourceObservation,
    source::{SourceAction, SourceEffectView, SourceRecoveryView},
};
use ic_platform::Principal;
use ic_platform::cdk::call::{Call, CallFailed};
use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<Option<SourceJournalRecord>> = const { RefCell::new(None) };
}

pub(crate) fn initialize(service: Principal, gateway: Principal, driver: Principal) {
    storage::create();
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
    })
}

pub(crate) fn gateway_inspection() -> Option<crate::model::GatewayInspection> {
    read(SourceJournalRecord::gateway_inspection)
}
pub(crate) fn reply_inspection(view: crate::model::GatewayInspection) {
    match view.mode {
        SourceMode::Valid => reply_list(view.gateway),
        SourceMode::Empty => reply_empty(),
        SourceMode::Malformed => reply(vec![0]),
        SourceMode::Oversized => reply(vec![0; 65_537]),
        _ => reject(), // Queries never run scheduling or mutation modes.
    }
}

pub(crate) fn recovery() -> SourceRecoveryView {
    read(|state| SourceRecoveryView {
        fenced: state.fenced,
        service: state.service,
        gateway: state.gateway,
        driver: state.driver,
        mode: state.mode,
        held_sync: state.sync.held.as_ref().map(|held| {
            blob_test_protocol::source::SourceHeldSyncView {
                sequence: held.sequence,
                gateway: held.gateway,
                ready: held.ready,
            }
        }),
        effects: state
            .effects
            .iter()
            .map(|effect| SourceEffectView {
                action: match &effect.action {
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
    });
}

pub(crate) fn receive() -> (Principal, Principal, SourceMode) {
    mutate(|state| {
        assert!(state.sync.held.is_none(), "one held list reply");
        state.requests = state
            .requests
            .checked_add(1)
            .expect("fixture request bound");
        (state.service.into(), state.gateway.into(), state.mode)
    })
}

pub(crate) async fn confirm_deletion(service: Principal, roots: Vec<Vec<u8>>) -> Result<(), u32> {
    let retained = roots
        .iter()
        .map(|root| root.as_slice().try_into().expect("bounded root"))
        .collect();
    let id = mutate(|state| state.begin_effect(ActionRecord::Delete(retained)));
    let result = match Call::unbounded_wait(service, "_immutableObjectStorageConfirmBlobDeletion")
        .take_raw_args(
            ic_platform::candid::contract::encode_one(roots).expect("fixture call arguments"),
        )
        .await
    {
        Ok(reply) => ic_platform::candid::decoder::decode_one_with_config::<()>(
            reply.as_ref(),
            // Preserve this fixture's upstream defaults, including unit/extra-value admission.
            &ic_platform::candid::decoder::DecoderConfig::new(),
        )
        .map_err(|_| 0),
        Err(CallFailed::CallRejected(error)) => Err(error.raw_reject_code()),
        Err(CallFailed::InsufficientLiquidCycleBalance(_) | CallFailed::CallPerformFailed(_)) => {
            Err(0)
        }
    };
    mutate(|state| state.finish_effect(id, result.is_ok()));
    result
}

pub(crate) fn reply_list(gateway: Principal) {
    reply(
        ic_platform::candid::contract::encode_one(vec![
            ic_platform::principal::ExternalPrincipal::from(gateway),
        ])
        .expect("fixture principal list"),
    );
}
pub(crate) fn reply_empty() {
    reply(
        ic_platform::candid::contract::encode_one(
            Vec::<ic_platform::principal::ExternalPrincipal>::new(),
        )
        .expect("empty list"),
    );
}
pub(crate) fn reply(bytes: Vec<u8>) {
    ic_platform::cdk::api::msg_reply(bytes);
}
pub(crate) fn reject() {
    ic_platform::cdk::api::msg_reject("deliberate local source rejection");
}
