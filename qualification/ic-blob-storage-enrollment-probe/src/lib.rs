//! Single-use owner-origin Cashier enrollment probe, not a Blob service or funding engine.
//! No uploads, certificates, arbitrary calls, attached cycles, timers or retries.

pub mod protocol;
mod state;

use ic_platform_runtime::{
    cdk::{
        self, api,
        call::{Call, CallFailed, WaitPolicy},
    },
    ic0,
};
use ic_wire::native;
use protocol::{Error, Installation, LIMITS, Outcome, REPLY_BYTES, View};
use serde::{Serialize, de::DeserializeOwned};
use state::State;
use std::{
    cell::{Cell, RefCell},
    mem::DropGuard,
};

const INIT: u64 = native::method_id("enrollment_init");
const UPGRADE: u64 = native::method_id("enrollment_upgrade");
const ATTEMPT: u64 = native::method_id("enrollment_attempt");
const INSPECT: u64 = native::method_id("enrollment_inspect");

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
    static IN_FLIGHT: Cell<bool> = const { Cell::new(false) };
}

#[cdk::init(decode_with = "decode::<Installation, INIT>")]
fn init(config: Installation) {
    assert!(
        api::is_controller(&config.operator),
        "operator must be a controller"
    );
    STATE.with_borrow_mut(|state| *state = Some(State::create(api::canister_self(), config)));
}

#[cdk::pre_upgrade]
fn pre_upgrade() {
    assert!(
        !IN_FLIGHT.get(),
        "stop and drain runtime calls before upgrade"
    );
}

#[cdk::post_upgrade(decode_with = "decode::<(), UPGRADE>")]
fn post_upgrade() {
    STATE.with_borrow_mut(|state| *state = Some(State::open(api::canister_self())));
}

#[cdk::update(
    decode_with = "decode::<(), INSPECT>",
    encode_with = "encode::<_, INSPECT>"
)]
fn enrollment_inspect() -> Result<View, Error> {
    inspect(api::msg_caller())
}

#[cdk::update(
    decode_with = "decode::<(), ATTEMPT>",
    encode_with = "encode::<_, ATTEMPT>"
)]
async fn enrollment_attempt() -> Result<View, Error> {
    let caller = api::msg_caller();
    let view = inspect(caller)?;
    if view.attempted_at.is_some() {
        return Ok(view);
    }
    STATE.with_borrow_mut(|state| state.as_mut().unwrap().begin(api::time()));
    IN_FLIGHT.set(true);
    let _flight = DropGuard::new((), |()| IN_FLIGHT.set(false));
    // The same message commits intent and dispatch. There is exactly one immutable
    // effect, including on a provider refusal, transport error or lost ingress reply.
    let result = Call::bounded_wait(view.configuration.cashier, "account_top_up_v1")
        .with_wait_policy(WaitPolicy::Bounded {
            timeout_seconds: 30,
        })
        .with_cycles(0)
        .take_raw_args(view.arguments.into_vec())
        .await;
    let outcome = match result {
        Ok(reply) if reply.len() <= REPLY_BYTES => {
            Outcome::Reply(ic_wire::Bytes::new(reply.into_bytes()).unwrap())
        }
        Ok(reply) => Outcome::ReplyTooLarge(reply.len() as u64),
        Err(CallFailed::CallRejected(error)) => Outcome::Rejected(error.raw_reject_code()),
        Err(CallFailed::CallPerformFailed(_) | CallFailed::InsufficientLiquidCycleBalance(_)) => {
            Outcome::NotEnqueued
        }
    };
    // Revocation cannot erase an already incurred result. Record first, then
    // decide whether this caller may receive the retained observation.
    STATE.with_borrow_mut(|state| state.as_mut().unwrap().finish(outcome));
    inspect(caller)
}

fn inspect(caller: ic_platform_types::Principal) -> Result<View, Error> {
    let operator = STATE.with_borrow(|state| state.as_ref().unwrap().view().configuration.operator);
    if caller != operator || !api::is_controller(&caller) {
        return Err(Error::Denied);
    }
    Ok(STATE.with_borrow(|state| state.as_ref().unwrap().view().clone()))
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "Runtime codec hook owns ingress"
)]
fn decode<T: DeserializeOwned, const ID: u64>(bytes: Vec<u8>) -> T {
    assert!(
        ic0::msg_arg_data_size() <= LIMITS.bytes,
        "enrollment input limit"
    );
    native::decode_request(ID, &bytes, LIMITS).unwrap()
}

fn encode<T: Serialize, const ID: u64>(value: T) -> Vec<u8> {
    native::encode_response(ID, &value, LIMITS).unwrap()
}
