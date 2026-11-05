//! Explicit local ingress envelope, independent of the retained-state budgets.
#![expect(
    clippy::needless_pass_by_value,
    reason = "CDK custom decoders take ownership of the argument buffer"
)]

use blob_test_protocol::admission::{ContentLookup, Installation, Request};
use serde::de::DeserializeOwned;

// Bounded manifests only. No file bytes enter this probe. These limits are
// fixture inputs, not a production API or a network ingress maximum.
const COMMAND_BYTES: usize = 16 * 1024;
const SMALL_BYTES: usize = 4096;

pub(crate) fn mutation<T: DeserializeOwned, const METHOD: u64>(bytes: Vec<u8>) -> T {
    let before = super::resources::instructions();
    let command = decode::<T, METHOD>(&bytes, COMMAND_BYTES);
    drop(bytes);
    let after = super::resources::instructions();
    super::resources::decoded(before, after);
    command
}

pub(crate) fn request<const METHOD: u64>(bytes: Vec<u8>) -> Request {
    decode::<_, METHOD>(&bytes, SMALL_BYTES)
}

pub(crate) fn content<const METHOD: u64>(bytes: Vec<u8>) -> ContentLookup {
    decode::<_, METHOD>(&bytes, SMALL_BYTES)
}

pub(crate) fn retained_descriptor<const METHOD: u64>(
    bytes: Vec<u8>,
) -> blob_test_protocol::admission::input::RetainedDescriptorInput {
    decode::<_, METHOD>(&bytes, SMALL_BYTES)
}

pub(crate) fn installation(bytes: Vec<u8>) -> Installation {
    let (value,) = blob_test_protocol::fixture_wire::request("fixture_init", &bytes, SMALL_BYTES)
        .expect("private Wire installation");
    value
}

fn decode<T: DeserializeOwned, const METHOD: u64>(bytes: &[u8], maximum: usize) -> T {
    let (value,) = blob_test_protocol::fixture_wire::request_id(METHOD, bytes, maximum)
        .expect("invalid or over-budget probe input");
    value
}

/// Each guard binds a declared method without inspect-message-only system calls.
pub(crate) fn admit_discovery() -> Result<(), String> {
    admit_blob("blob_lookup_content")
}
pub(crate) fn admit_upload_capacity() -> Result<(), String> {
    admit_blob("blob_upload_capacity")
}
pub(crate) fn admit_reference_capacity() -> Result<(), String> {
    admit_blob("blob_reference_capacity")
}

/// Admit the service method and byte ceiling before copying its ingress buffer.
#[expect(
    clippy::unnecessary_wraps,
    reason = "Platform guards require this signature"
)]
fn admit_blob(method: &str) -> Result<(), String> {
    let limit = ic_blob_storage_contracts::wire::request_limit(method).expect("owned method");
    assert!(
        ic_platform::ic0::msg_arg_data_size() <= limit,
        "request limit"
    );
    Ok(())
}
/// Decode the exact method-bound service frame after admission.
pub(crate) fn blob<T: serde::de::DeserializeOwned>(method: &str, bytes: &[u8]) -> T {
    let limit = ic_blob_storage_contracts::wire::request_limit(method).expect("owned method");
    ic_blob_storage_contracts::wire::decode_request(method, bytes, limit)
        .expect("invalid or over-budget Blob probe input")
}
