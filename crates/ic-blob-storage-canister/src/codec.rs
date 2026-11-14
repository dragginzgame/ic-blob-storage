//! Private standalone-host hooks; Platform owns exports/copy/dispatch/reply, Wire owns frames.

use ic_blob_storage::protocol::RPC_LIMITS;
use ic_platform_runtime::{cdk::api, ic0};
use ic_wire::native;
use serde::{Serialize, de::DeserializeOwned};

pub(super) const INIT: u64 = native::method_id("blob_init");
pub(super) const UPGRADE: u64 = native::method_id("blob_post_upgrade");
pub(super) const UPDATE: u64 = native::method_id("blob_update");
pub(super) const QUERY: u64 = native::method_id("blob_query");
pub(super) const MANAGEMENT: u64 = native::method_id("blob_management");

#[expect(
    clippy::unnecessary_wraps,
    reason = "Platform guard signature; malformed transport still traps"
)]
pub(super) fn admit<const ID: u64>() -> Result<(), String> {
    let length = ic0::msg_arg_data_size();
    if length > RPC_LIMITS.bytes {
        api::trap("Wire request exceeds byte allowance");
    }
    if length < native::REQUEST_HEADER_BYTES {
        trap(ic_wire::Error::Malformed);
    }
    // Inspect only the fixed header before Runtime allocates/copies the payload.
    let mut header = [0; native::REQUEST_HEADER_BYTES];
    ic0::msg_arg_data_copy(&mut header, 0);
    native::validate_request_header(ID, &header).unwrap_or_else(|error| trap(error));
    Ok(())
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "Runtime transfers its argument buffer to the codec hook"
)]
pub(super) fn decode<T: DeserializeOwned, const ID: u64>(bytes: Vec<u8>) -> T {
    native::decode_request(ID, &bytes, RPC_LIMITS).unwrap_or_else(|error| trap(error))
}

pub(super) fn encode<T: Serialize, const ID: u64>(value: T) -> Vec<u8> {
    // Trapping here rolls back the synchronous handler's mutation. Returning an
    // error frame/rejection instead would commit it. These endpoints never await.
    native::encode_response(ID, &value, RPC_LIMITS).unwrap_or_else(|error| trap(error))
}

#[cold]
fn trap(error: ic_wire::Error) -> ! {
    api::trap(error.to_string())
}
