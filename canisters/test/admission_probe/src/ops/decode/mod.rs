//! Explicit local ingress envelope, independent of the retained-state budgets.
#![expect(
    clippy::needless_pass_by_value,
    reason = "CDK custom decoders take ownership of the argument buffer"
)]

use blob_test_protocol::admission::{ContentLookup, Installation, Request};
use ic_platform::candid::contract::CandidType;
use ic_platform::candid::contract::serde::Deserialize;
use ic_platform::candid::decoder::{DecoderConfig, IDLDeserialize, decode_one_with_config};

// Bounded manifests only. No file bytes enter this probe. These limits are
// fixture inputs, not a production API or a network ingress maximum.
const COMMAND_BYTES: usize = 16 * 1024;
const COMMAND_WORK: usize = 64 * 1024;
const SMALL_BYTES: usize = 4096;

pub(crate) fn mutation<T: CandidType + for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T {
    let before = super::resources::instructions();
    let config = config(&bytes, COMMAND_BYTES, COMMAND_WORK);
    let mut decoder = checked(IDLDeserialize::new_with_config(&bytes, &config));
    let header = super::resources::instructions();
    let command = checked(decoder.get_value());
    let value = super::resources::instructions();
    checked(decoder.done());
    drop(decoder);
    drop(bytes);
    let after = super::resources::instructions();
    super::resources::decoded(before, header, value, after);
    command
}

pub(crate) fn request(bytes: Vec<u8>) -> Request {
    decode(&bytes, SMALL_BYTES, 32 * 1024)
}

pub(crate) fn content(bytes: Vec<u8>) -> ContentLookup {
    decode(&bytes, SMALL_BYTES, 32 * 1024)
}

pub(crate) fn retained_descriptor(
    bytes: Vec<u8>,
) -> blob_test_protocol::admission::input::RetainedDescriptorInput {
    decode(&bytes, SMALL_BYTES, 32 * 1024)
}

pub(crate) fn installation(bytes: Vec<u8>) -> Installation {
    decode(&bytes, SMALL_BYTES, 32 * 1024)
}

fn decode<T: CandidType + for<'de> Deserialize<'de>>(
    bytes: &[u8],
    maximum: usize,
    work: usize,
) -> T {
    checked(decode_one_with_config(bytes, &config(bytes, maximum, work)))
}

fn checked<T>(result: Result<T, ic_platform::candid::contract::Error>) -> T {
    result.unwrap_or_else(|_| ic_platform::cdk::api::trap("invalid or over-budget probe input"))
}

fn config(bytes: &[u8], maximum: usize, work: usize) -> DecoderConfig {
    if bytes.len() > maximum {
        ic_platform::cdk::api::trap("probe input exceeds byte budget");
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(work)
        .set_skipping_quota(1024)
        .set_max_type_len(128)
        .set_max_header_len(SMALL_BYTES)
        .set_full_error_message(false);
    config
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
pub(crate) fn blob<T: serde::Serialize + serde::de::DeserializeOwned>(
    method: &str,
    bytes: &[u8],
) -> T {
    let limit = ic_blob_storage_contracts::wire::request_limit(method).expect("owned method");
    ic_blob_storage_contracts::wire::decode_request(method, bytes, limit)
        .expect("invalid or over-budget Blob probe input")
}
