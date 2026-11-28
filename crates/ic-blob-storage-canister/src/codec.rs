//! Private standalone-host hooks; Platform owns exports/copy/dispatch/reply, Wire owns frames.

use ic_blob_storage::protocol::RPC_LIMITS;
use ic_wire as wire;
use serde::{Serialize, de::DeserializeOwned};

pub(super) const INIT: u64 = wire::method_id("blob_init");
pub(super) const UPGRADE: u64 = wire::method_id("blob_post_upgrade");
pub(super) const UPDATE: u64 = wire::method_id("blob_update");
pub(super) const QUERY: u64 = wire::method_id("blob_query");
pub(super) const MANAGEMENT: u64 = wire::method_id("blob_management");

pub(super) fn decode<T: DeserializeOwned, const ID: u64>(bytes: &[u8]) -> Result<T, wire::Error> {
    ic_blob_storage::protocol::frame::decode(
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
    ic_blob_storage::protocol::frame::encode(
        &value,
        wire::Encoding::Frame {
            method: ID,
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
    )
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
