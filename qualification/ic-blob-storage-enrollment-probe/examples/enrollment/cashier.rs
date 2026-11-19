//! Offline discriminator for the observed Cashier ABI, not a funding or accounting adapter.
//! Keep the complete raw reply; even an Ok tag does not establish enrollment or credit.

use ic_blob_storage_enrollment_probe::protocol::REPLY_BYTES;
use ic_platform_protocol::{
    caffeine::{TopUpResult, decode_top_up_result},
    candid::Limits,
};
use serde_json::{Value, json};

pub(super) fn classify(bytes: &[u8]) -> Result<Value, String> {
    let result = decode_top_up_result(
        bytes,
        Limits {
            bytes: REPLY_BYTES,
            header: 2048,
            types: 32,
            work: 65_536,
            allocation: 65_536,
            vector: REPLY_BYTES,
            skipping: 65_536,
        },
    )
    .map_err(|error| format!("invalid or oversized Cashier top-up reply: {error:?}"))?;

    Ok(match result {
        // Validate/skip the payload; this command makes no balance or credit claim.
        TopUpResult::Ok(_) => json!({ "provider_result": "Ok", "payload_interpreted": false }),
        TopUpResult::Err(error) => json!({ "provider_result": "Err", "error": error }),
    })
}
