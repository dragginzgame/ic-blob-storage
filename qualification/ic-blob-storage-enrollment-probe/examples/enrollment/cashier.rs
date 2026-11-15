//! Offline discriminator for the observed Cashier ABI, not a funding or accounting adapter.
//! Keep the complete raw reply; even an Ok tag does not establish enrollment or credit.

use ic_blob_storage_enrollment_probe::protocol::REPLY_BYTES;
use ic_platform::candid_codec::{
    Reserved,
    decoder::{Arity, Limits, decode_args},
};
use ic_platform_types::Principal;
use serde::Serialize;
use serde_json::{Value, json};

ic_platform::candid_variant! {
    #[derive(Serialize)]
    enum TopUpError {
        2102411630 => NotAuthorized(Principal),
        3638018673 => AccountBalanceOverflow,
        3758147371 => InternalError(String),
        4052296485 => TopUpWithoutCycles,
    }
}
ic_platform::candid_variant! {
    enum TopUpResult { 17724 => Ok(Reserved), 3456837 => Err(TopUpError) }
}

pub(super) fn classify(bytes: &[u8]) -> Result<Value, String> {
    let (result,): (TopUpResult,) = decode_args(
        bytes,
        Limits {
            bytes: REPLY_BYTES,
            header: 2048,
            types: 32,
            work: 65_536,
            allocation: 65_536,
        },
        Arity::Exact,
    )
    .map_err(|error| format!("invalid or oversized Cashier top-up reply: {error:?}"))?;
    Ok(match result {
        // Validate/skip the payload; this command makes no balance or credit claim.
        TopUpResult::Ok(_) => json!({ "provider_result": "Ok", "payload_interpreted": false }),
        TopUpResult::Err(error) => json!({ "provider_result": "Err", "error": error }),
    })
}
