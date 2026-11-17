use crate::protocol::{Error, Installation, LIMITS, Outcome, REPLY_BYTES, View};
use crate::state::State;
use ic_platform_runtime::{
    call::{Call, Outcome as CallOutcome, Wait},
    raw as ic0, system as api,
};
use ic_wire as wire;
use serde::{Serialize, de::DeserializeOwned};
use std::{
    cell::{Cell, RefCell},
    mem::DropGuard,
};

const INIT: u64 = wire::method_id("enrollment_init");
const UPGRADE: u64 = wire::method_id("enrollment_upgrade");
const ATTEMPT: u64 = wire::method_id("enrollment_attempt");
const INSPECT: u64 = wire::method_id("enrollment_inspect");

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
    static IN_FLIGHT: Cell<bool> = const { Cell::new(false) };
}

#[ic_platform_runtime::init(
    runtime = ic_platform_runtime,
    guard = unguarded,
    decode = decode::<Installation, INIT>,
    max_bytes = 16384,
    on_error = fail
)]
fn init(config: Installation) {
    assert!(
        api::is_controller(&config.operator),
        "operator must be a controller"
    );
    STATE.with_borrow_mut(|state| *state = Some(State::create(api::canister_self(), config)));
}

#[ic_platform_runtime::pre_upgrade(
    runtime = ic_platform_runtime,
    guard = unguarded,
    decode = empty,
    max_bytes = 0,
    on_error = fail
)]
fn pre_upgrade(_: ()) {
    assert!(
        !IN_FLIGHT.get(),
        "stop and drain runtime calls before upgrade"
    );
}

#[ic_platform_runtime::post_upgrade(
    runtime = ic_platform_runtime,
    guard = unguarded,
    decode = decode::<(), UPGRADE>,
    max_bytes = 16384,
    on_error = fail
)]
fn post_upgrade(_: ()) {
    STATE.with_borrow_mut(|state| *state = Some(State::open(api::canister_self())));
}

#[ic_platform_runtime::update(
    runtime = ic_platform_runtime,
    name = "enrollment_inspect",
    guard = unguarded,
    decode = decode::<(), INSPECT>,
    encode = encode::<_, INSPECT>,
    max_bytes = 16384,
    on_error = fail
)]
fn enrollment_inspect(_: ()) -> Result<View, Error> {
    inspect(api::caller())
}

#[ic_platform_runtime::update(
    runtime = ic_platform_runtime,
    name = "enrollment_attempt",
    guard = unguarded,
    decode = decode::<(), ATTEMPT>,
    encode = encode::<_, ATTEMPT>,
    max_bytes = 16384,
    on_error = fail
)]
async fn enrollment_attempt(_: ()) -> Result<View, Error> {
    let caller = api::caller();
    let view = inspect(caller)?;
    if view.attempted_at.is_some() {
        return Ok(view);
    }
    STATE.with_borrow_mut(|state| state.as_mut().unwrap().begin(api::time()));
    IN_FLIGHT.set(true);
    let _flight = DropGuard::new((), |()| IN_FLIGHT.set(false));
    // The same message commits intent and dispatch. There is exactly one immutable
    // effect, including on a provider refusal, transport error or lost ingress reply.
    let result = Call::new(
        view.configuration.cashier,
        "account_top_up_v1",
        Wait::Bounded { seconds: 30 },
    )
    .cycles(0)
    .owned_args(view.arguments.into_vec())
    .execute()
    .await
    .map(|completion| completion.into_parts().0);
    let outcome = match result {
        Ok(CallOutcome::Reply(reply)) if reply.len() <= REPLY_BYTES => {
            Outcome::Reply(ic_wire::Bytes::new(reply).unwrap())
        }
        Ok(CallOutcome::Reply(reply)) => Outcome::ReplyTooLarge(reply.len() as u64),
        Ok(CallOutcome::Rejected(error) | CallOutcome::OutcomeUnknown(error)) => {
            Outcome::Rejected(error.code())
        }
        Err(_) => Outcome::NotEnqueued,
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

fn decode<T: DeserializeOwned, const ID: u64>(bytes: &[u8]) -> Result<T, wire::Error> {
    assert!(
        ic0::msg_arg_data_size() <= LIMITS.bytes,
        "enrollment input limit"
    );
    wire::decode(
        bytes,
        wire::Encoding::Frame {
            method: ID,
            direction: wire::Direction::Request,
        },
        LIMITS,
    )
}

fn encode<T: Serialize, const ID: u64>(value: T) -> Result<Vec<u8>, wire::Error> {
    wire::encode(
        &value,
        wire::Encoding::Frame {
            method: ID,
            direction: wire::Direction::Response,
        },
        LIMITS,
    )
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "Runtime guard signature; authorization belongs to the handler"
)]
fn unguarded() -> Result<(), std::convert::Infallible> {
    Ok(())
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "Runtime lifecycle decoder signature; pre-upgrade has no arguments"
)]
fn empty(_: &[u8]) -> Result<(), std::convert::Infallible> {
    Ok(())
}

fn fail(error: impl std::fmt::Debug) -> ! {
    api::trap(&format!("entrypoint failed: {error:?}"))
}
