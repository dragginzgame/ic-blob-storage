//! Installed synchronous reply-failure probe for the selected Platform/Wire hooks.
//! Owns one raw stable page solely to observe IC rollback; never deployed as Blob.

#![cfg(target_arch = "wasm32")]

use ic_platform_runtime::system as api;
use ic_storage::{DefaultMemoryImpl, Memory};
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

/// Allocate the stable page used to observe committed writes.
#[ic_platform_runtime::init(
    runtime = ic_platform_runtime,
    guard = unguarded,
    decode = empty,
    max_bytes = 32,
    on_error = fail
)]
fn init(_: ()) {
    assert_eq!(DefaultMemoryImpl::default().grow(1), 0);
}

/// Increment the probe byte and optionally trigger a reply-encoding failure.
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
    DefaultMemoryImpl::default().write(0, &[value]);
    // No manual reply or simulated error: the actual encoder must trap after this write.
    vec![value; if fail_reply { LIMITS.bytes } else { 1 }]
}

/// Read the last committed probe byte.
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
    let mut value = [0];
    DefaultMemoryImpl::default().read(0, &mut value);
    value[0]
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

/// Allow all callers to exercise the disposable reply-failure probe.
#[expect(
    clippy::unnecessary_wraps,
    reason = "Runtime guard signature; authorization belongs to the handler"
)]
fn unguarded() -> Result<(), std::convert::Infallible> {
    Ok(())
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "Runtime lifecycle decoder signature; probe installation has no arguments"
)]
fn empty(_: &[u8]) -> Result<(), std::convert::Infallible> {
    Ok(())
}

fn fail(error: impl std::fmt::Debug) -> ! {
    api::trap(&format!("entrypoint failed: {error:?}"))
}
