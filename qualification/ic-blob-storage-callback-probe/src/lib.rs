//! Local provider candidate: callbacks and a certified-ingress reply, with no provider calls.
//! Never deploy this qualification artifact on a live funded owner.

#![cfg(target_arch = "wasm32")]

mod external;

use ic_blob_storage::local::persistence::access::Access;
use ic_blob_storage::{
    local::{
        Error,
        persistence::{
            self, Service,
            callbacks::{self, Callbacks},
        },
    },
    protocol::{Command, Inspection, Installation, Observation, Outcome, RPC_LIMITS},
};
use ic_platform_runtime::{raw as ic0, system as api};
use ic_platform_types::Principal;
use ic_storage::retained::{Bank, Canister, Geometry};
use ic_wire as wire;
use serde::{Serialize, de::DeserializeOwned};
use std::cell::RefCell;

const INIT: u64 = wire::method_id("callback_init");
const UPGRADE: u64 = wire::method_id("callback_upgrade");
const UPDATE: u64 = wire::method_id("blob_update");
const QUERY: u64 = wire::method_id("blob_query");

thread_local! {
    static STATE: RefCell<Option<Callbacks>> = const { RefCell::new(None) };
}

#[ic_platform_runtime::init(
    runtime = ic_platform_runtime,
    guard = unguarded,
    decode = decode::<(Installation, Principal), INIT>,
    max_bytes = 65536,
    on_error = fail
)]
fn init((input, gateway): (Installation, Principal)) {
    memory(true);
    let service = Service::create(
        Access::canister(Canister::grant(1).unwrap(), 1),
        api::canister_self(),
        input,
    )
    .unwrap();
    let callbacks = Callbacks::create(
        service,
        Access::canister(Canister::grant(2).unwrap(), 2),
        gateway,
    )
    .unwrap();
    STATE.with_borrow_mut(|state| *state = Some(callbacks));
}

#[ic_platform_runtime::pre_upgrade(
    runtime = ic_platform_runtime,
    guard = unguarded,
    decode = empty,
    max_bytes = 0,
    on_error = fail
)]
fn pre_upgrade(_: ()) {
    STATE.with_borrow(|state| {
        assert!(
            state.as_ref().unwrap().service().is_drained(),
            "drain before upgrade"
        )
    });
    Canister::close().unwrap();
}

#[ic_platform_runtime::post_upgrade(
    runtime = ic_platform_runtime,
    guard = unguarded,
    decode = decode::<(), UPGRADE>,
    max_bytes = 65536,
    on_error = fail
)]
fn post_upgrade(_: ()) {
    memory(false);
    let service = Service::open(
        Access::canister(Canister::grant(1).unwrap(), 1),
        api::canister_self(),
    )
    .unwrap();
    assert!(service.is_drained(), "retain closed admission");
    let callbacks =
        Callbacks::open(service, Access::canister(Canister::grant(2).unwrap(), 2)).unwrap();
    STATE.with_borrow_mut(|state| *state = Some(callbacks));
}

#[ic_platform_runtime::update(
    runtime = ic_platform_runtime,
    name = "blob_update",
    guard = admit,
    decode = decode::<Command, UPDATE>,
    encode = encode::<_, UPDATE>,
    max_bytes = 65536,
    on_error = fail
)]
fn blob_update(command: Command) -> Result<Outcome, Error> {
    STATE.with_borrow_mut(|state| {
        state
            .as_mut()
            .unwrap()
            .execute(api::caller(), command, api::time())
    })
}

#[ic_platform_runtime::query(
    runtime = ic_platform_runtime,
    name = "blob_query",
    guard = admit,
    decode = decode::<Inspection, QUERY>,
    encode = encode::<_, QUERY>,
    max_bytes = 65536,
    on_error = fail
)]
fn blob_query(input: Inspection) -> Result<Observation, Error> {
    STATE.with_borrow(|state| {
        state
            .as_ref()
            .unwrap()
            .service()
            .inspect(api::caller(), input)
    })
}

#[ic_platform_runtime::update(
    runtime = ic_platform_runtime,
    name = "_immutableObjectStorageCreateCertificate",
    guard = external::admit,
    decode = external::certificate_root,
    encode = external::certificate_reply,
    max_bytes = 4096,
    on_error = fail
)]
fn issue(root: [u8; 32]) -> [u8; 32] {
    // No await: actual caller, original grant, first reference and exposure are
    // committed by Service in the same execution that returns the provider record.
    // A reply-encoding trap must roll back this execution, not return an error record.
    STATE.with_borrow_mut(|state| {
        state
            .as_mut()
            .unwrap()
            .execute(api::caller(), Command::Register(root), api::time())
            .unwrap()
    });
    root
}

#[ic_platform_runtime::query(
    runtime = ic_platform_runtime,
    name = "_immutableObjectStorageBlobsAreLive",
    guard = external::admit,
    decode = external::roots,
    encode = external::liveness,
    max_bytes = 4096,
    on_error = fail
)]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Runtime transfers decoded callback arguments"
)]
fn liveness(roots: Vec<[u8; 32]>) -> Vec<bool> {
    STATE.with_borrow(|state| state.as_ref().unwrap().liveness(&roots).unwrap())
}

#[ic_platform_runtime::query(
    runtime = ic_platform_runtime,
    name = "_immutableObjectStorageBlobsToDelete",
    guard = external::admit,
    decode = external::empty,
    encode = external::candidates,
    max_bytes = 4096,
    on_error = fail
)]
fn candidates(_: ()) -> Vec<[u8; 32]> {
    STATE.with_borrow(|state| state.as_ref().unwrap().candidates(api::caller()).unwrap())
}

#[ic_platform_runtime::update(
    runtime = ic_platform_runtime,
    name = "_immutableObjectStorageConfirmBlobDeletion",
    guard = external::admit,
    decode = external::roots,
    encode = external::done,
    max_bytes = 4096,
    on_error = fail
)]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Runtime transfers decoded callback arguments"
)]
fn acknowledge(roots: Vec<[u8; 32]>) {
    STATE.with_borrow_mut(|state| {
        state
            .as_mut()
            .unwrap()
            .acknowledge(api::caller(), &roots)
            .unwrap()
    });
}

fn memory(fresh: bool) {
    let geometry = Geometry {
        bank: Bank::A,
        nodes: 1024,
        bytes: 524288,
    };
    let roots = [
        persistence::storage_root(1, 1),
        callbacks::storage_root(2, 2),
    ];
    if fresh {
        Canister::create(geometry, &roots)
    } else {
        Canister::reopen(geometry, &roots)
    }
    .expect("native callback admission");
}

fn admit() -> Result<(), String> {
    if ic0::msg_arg_data_size() > RPC_LIMITS.bytes {
        Err("Wire input limit".into())
    } else {
        Ok(())
    }
}

fn decode<T: DeserializeOwned, const ID: u64>(bytes: &[u8]) -> Result<T, wire::Error> {
    admit().unwrap();
    wire::decode(
        bytes,
        wire::Encoding::Frame {
            method: ID,
            direction: wire::Direction::Request,
        },
        RPC_LIMITS,
    )
}

fn encode<T: Serialize, const ID: u64>(value: T) -> Result<Vec<u8>, wire::Error> {
    wire::encode(
        &value,
        wire::Encoding::Frame {
            method: ID,
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
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
