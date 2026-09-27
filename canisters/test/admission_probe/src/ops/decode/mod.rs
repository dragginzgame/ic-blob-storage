//! Explicit local ingress envelope, independent of the retained-state budgets.
#![expect(
    clippy::needless_pass_by_value,
    reason = "CDK custom decoders take ownership of the argument buffer"
)]

use blob_test_protocol::admission::{ContentLookup, Installation, Request};
use candid::{CandidType, DecoderConfig, Deserialize, de::IDLDeserialize, decode_one_with_config};

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

pub(crate) fn admission_capacity(
    bytes: Vec<u8>,
) -> blob_test_protocol::admission::planning::AdmissionCapacityInput {
    decode(&bytes, SMALL_BYTES, 32 * 1024)
}

fn decode<T: CandidType + for<'de> Deserialize<'de>>(
    bytes: &[u8],
    maximum: usize,
    work: usize,
) -> T {
    checked(decode_one_with_config(bytes, &config(bytes, maximum, work)))
}

fn checked<T>(result: candid::Result<T>) -> T {
    result.unwrap_or_else(|_| ic_cdk::trap("invalid or over-budget probe input"))
}

fn config(bytes: &[u8], maximum: usize, work: usize) -> DecoderConfig {
    if bytes.len() > maximum {
        ic_cdk::trap("probe input exceeds byte budget");
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
