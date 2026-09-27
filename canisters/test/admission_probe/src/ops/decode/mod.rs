//! Explicit local ingress envelope, independent of the retained-state budgets.
#![expect(
    clippy::needless_pass_by_value,
    reason = "CDK custom decoders take ownership of the argument buffer"
)]

use blob_test_protocol::admission::{Command, Request};
use candid::{CandidType, DecoderConfig, Deserialize, Principal, decode_one_with_config};

// Bounded manifests only. No file bytes enter this probe. These limits are
// fixture inputs, not a production API or a network ingress maximum.
const COMMAND_BYTES: usize = 16 * 1024;
const COMMAND_WORK: usize = 64 * 1024;
const SMALL_BYTES: usize = 4096;

pub(crate) fn command(bytes: Vec<u8>) -> Command {
    decode(&bytes, COMMAND_BYTES, COMMAND_WORK)
}

pub(crate) fn request(bytes: Vec<u8>) -> Request {
    decode(&bytes, SMALL_BYTES, 32 * 1024)
}

pub(crate) fn operator(bytes: Vec<u8>) -> Principal {
    decode(&bytes, SMALL_BYTES, 32 * 1024)
}

fn decode<T: CandidType + for<'de> Deserialize<'de>>(
    bytes: &[u8],
    maximum: usize,
    work: usize,
) -> T {
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
    decode_one_with_config(bytes, &config)
        .unwrap_or_else(|_| ic_cdk::trap("invalid or over-budget probe input"))
}
