//! Private standalone-host hooks; Platform owns exports/copy/dispatch/reply, Wire owns frames.

use ic_blob_storage::protocol::RPC_LIMITS;
use ic_platform_runtime::{raw as ic0, system as api};
use ic_wire as wire;
use serde::{Serialize, de::DeserializeOwned};

pub(super) const INIT: u64 = wire::method_id("blob_init");
pub(super) const UPGRADE: u64 = wire::method_id("blob_post_upgrade");
pub(super) const UPDATE: u64 = wire::method_id("blob_update");
pub(super) const QUERY: u64 = wire::method_id("blob_query");
pub(super) const MANAGEMENT: u64 = wire::method_id("blob_management");

/// Check the byte limit and method header before decoding. The service authorizes the caller.
#[expect(
    clippy::unnecessary_wraps,
    reason = "Platform guard signature; malformed transport still traps"
)]
pub(super) fn admit<const ID: u64>() -> Result<(), String> {
    let length = ic0::msg_arg_data_size();
    if length > RPC_LIMITS.bytes {
        api::trap("Wire request exceeds byte allowance");
    }
    let mut header = [0; 11]; // Wire's fixed request-header argument, validated below.
    if length < header.len() {
        trap(ic_wire::Error::Malformed);
    }
    // Inspect only the fixed header before Runtime allocates/copies the payload.
    ic0::msg_arg_data_copy(&mut header, 0);
    wire::validate_request_header(ID, &header).unwrap_or_else(|error| trap(error));
    Ok(())
}

pub(super) fn decode<T: DeserializeOwned, const ID: u64>(bytes: &[u8]) -> Result<T, wire::Error> {
    wire::decode(
        bytes,
        wire::Encoding::Frame {
            method: ID,
            direction: wire::Direction::Request,
        },
        RPC_LIMITS,
    )
}

pub(super) fn encode<T: Serialize, const ID: u64>(value: T) -> Result<Vec<u8>, wire::Error> {
    // Trapping here rolls back the synchronous handler's mutation. Returning an
    // error frame/rejection instead would commit it. These endpoints never await.
    wire::encode(
        &value,
        wire::Encoding::Frame {
            method: ID,
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
    )
}

#[cold]
fn trap(error: ic_wire::Error) -> ! {
    api::trap(&error.to_string())
}

/// Accept system lifecycle invocations.
#[expect(
    clippy::unnecessary_wraps,
    reason = "Runtime guard signature; authorization belongs to the handler"
)]
pub(super) fn unguarded() -> Result<(), std::convert::Infallible> {
    Ok(())
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "Runtime lifecycle decoder signature; pre-upgrade has no arguments"
)]
pub(super) fn empty(_: &[u8]) -> Result<(), std::convert::Infallible> {
    Ok(())
}

pub(super) fn fail(error: impl std::fmt::Debug) -> ! {
    api::trap(&format!("entrypoint failed: {error:?}"))
}
