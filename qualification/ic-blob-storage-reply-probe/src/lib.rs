//! Installed synchronous reply-failure probe for the selected Platform/Wire hooks.
//! Owns one raw stable page solely to observe IC rollback; never deployed as Blob.

use ic_platform_runtime::{cdk, cdk::api, ic0};
use ic_wire::native;
use serde::{Serialize, de::DeserializeOwned};

const WRITE: u64 = native::method_id("probe_write");
const READ: u64 = native::method_id("probe_read");
const LIMITS: ic_wire::Limits = ic_wire::Limits {
    bytes: 32,
    allocation: 1024,
    nodes: 128,
    depth: 8,
};

#[cdk::init]
fn init() {
    assert_eq!(ic0::stable64_grow(1), 0);
}

#[cdk::update(
    decode_with = "decode::<bool, WRITE>",
    encode_with = "encode::<_, WRITE>"
)]
fn probe_write(fail_reply: bool) -> Vec<u8> {
    let value = probe_read() + 1;
    ic0::stable64_write(&[value], 0);
    // No manual reply or simulated error: the actual encoder must trap after this write.
    vec![value; if fail_reply { LIMITS.bytes } else { 1 }]
}

#[cdk::query(decode_with = "decode::<(), READ>", encode_with = "encode::<_, READ>")]
fn probe_read() -> u8 {
    let mut value = [0];
    ic0::stable64_read(&mut value, 0);
    value[0]
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "Runtime codec hook owns ingress"
)]
fn decode<T: DeserializeOwned, const ID: u64>(bytes: Vec<u8>) -> T {
    native::decode_request(ID, &bytes, LIMITS).unwrap_or_else(|error| api::trap(error.to_string()))
}

fn encode<T: Serialize, const ID: u64>(value: T) -> Vec<u8> {
    native::encode_response(ID, &value, LIMITS).unwrap_or_else(|error| api::trap(error.to_string()))
}
