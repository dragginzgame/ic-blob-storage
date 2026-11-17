//! Installed synchronous reply-failure probe for the selected Platform/Wire hooks.
//! Owns one native retained sentinel to observe IC rollback; never deployed as Blob.

#![cfg(target_arch = "wasm32")]

use ic_platform_runtime::system as api;
use ic_storage::retained::{Bank, Canister, Geometry, Grant, Root};
use std::cell::RefCell;
thread_local! {static GRANT: RefCell<Option<Grant>>=const {RefCell::new(None)};}
use ic_wire as wire;
use serde::{Serialize, de::DeserializeOwned};

const WRITE: u64 = wire::method_id("probe_write");
const READ: u64 = wire::method_id("probe_read");
const LIMITS: ic_wire::Limits = ic_wire::Limits {
    bytes: 32,
    allocation: 1024,
    nodes: 128,
    depth: 8,
};

#[ic_platform_runtime::init(
    runtime = ic_platform_runtime,
    guard = unguarded,
    decode = empty,
    max_bytes = 32,
    on_error = fail
)]
fn init(_: ()) {
    open(true);
    GRANT.with_borrow(|grant| {
        Canister::update(&[grant.as_ref().unwrap()], |mut view| {
            view.insert(1, b"count", &[0]).unwrap()
        })
        .unwrap()
    });
}

#[ic_platform_runtime::update(
    runtime = ic_platform_runtime,
    name = "probe_write",
    guard = unguarded,
    decode = decode::<bool, WRITE>,
    encode = encode::<_, WRITE>,
    max_bytes = 32,
    on_error = fail
)]
fn probe_write(fail_reply: bool) -> Vec<u8> {
    let value = probe_read(()) + 1;
    GRANT.with_borrow(|grant| {
        Canister::update(&[grant.as_ref().unwrap()], |mut view| {
            view.insert(1, b"count", &[value]).unwrap()
        })
        .unwrap()
    });
    // No manual reply or simulated error: the actual encoder must trap after this write.
    vec![value; if fail_reply { LIMITS.bytes } else { 1 }]
}

#[ic_platform_runtime::query(
    runtime = ic_platform_runtime,
    name = "probe_read",
    guard = unguarded,
    decode = decode::<(), READ>,
    encode = encode::<_, READ>,
    max_bytes = 32,
    on_error = fail
)]
fn probe_read(_: ()) -> u8 {
    GRANT.with_borrow(|grant| {
        Canister::read(&[grant.as_ref().unwrap()], |view| {
            view.get(1, b"count").unwrap().unwrap().as_slice()[0]
        })
        .unwrap()
    })
}

fn decode<T: DeserializeOwned, const ID: u64>(bytes: &[u8]) -> Result<T, wire::Error> {
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

fn open(fresh: bool) {
    let geometry = Geometry {
        bank: Bank::A,
        nodes: 16,
        bytes: 4096,
    };
    let roots = [Root {
        id: 1,
        participant: 1,
        schema: 0x5245504c59505231,
        entries: 1,
        key_bytes: 5,
        value_bytes: 1,
    }];
    if fresh {
        Canister::create(geometry, &roots)
    } else {
        Canister::reopen(geometry, &roots)
    }
    .unwrap();
    GRANT.with_borrow_mut(|grant| *grant = Some(Canister::grant(1).unwrap()));
}
#[ic_platform_runtime::pre_upgrade(runtime=ic_platform_runtime,guard=unguarded,decode=empty,max_bytes=0,on_error=fail)]
fn pre_upgrade(_: ()) {
    Canister::close().unwrap();
}
#[ic_platform_runtime::post_upgrade(runtime=ic_platform_runtime,guard=unguarded,decode=empty,max_bytes=0,on_error=fail)]
fn post_upgrade(_: ()) {
    open(false);
}
