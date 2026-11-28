//! Private standalone-host hooks; Platform owns exports/copy/dispatch/reply, Wire owns frames.

use ic_blob_storage_wire::{Contract, RPC_LIMITS};
use ic_wire as wire;
use serde::de::DeserializeOwned;

pub(super) const INIT: u64 = wire::method_id("blob_init");
pub(super) const UPGRADE: u64 = wire::method_id("blob_post_upgrade");
pub(super) fn decode<T: DeserializeOwned, const ID: u64>(bytes: &[u8]) -> Result<T, wire::Error> {
    ic_blob_storage_wire::frame::decode(
        bytes,
        wire::Encoding::Frame {
            method: ID,
            direction: wire::Direction::Request,
        },
        RPC_LIMITS,
    )
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "Platform passes the owned handler result"
)]
pub(super) fn encode_response<C: Contract>(value: C::Response) -> Result<Vec<u8>, wire::Error> {
    // A trap here rolls back the synchronous handler's mutation.
    C::encode_response(&value)
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
