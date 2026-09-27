use crate::operator::Failure;
use blob_test_protocol::admission::{
    Failure as RemoteFailure,
    input::{ReferenceInput, ReferenceReceipt},
};
use candid::{DecoderConfig, decode_one_with_config};
use serde_json::{Value, json};

pub(super) fn decode(input: ReferenceInput, bytes: &[u8]) -> Result<(u8, Value), Failure> {
    if bytes.len() > 4096 {
        return Err(Failure::ReplyTooLarge);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(100_000)
        .set_skipping_quota(1000)
        .set_max_type_len(64)
        .set_max_header_len(4096)
        .set_full_error_message(false);
    let reply: Result<Option<ReferenceReceipt>, RemoteFailure> =
        decode_one_with_config(bytes, &config).map_err(|_| Failure::InvalidReply)?;
    let receipt = reply.map_err(|error| match error {
        RemoteFailure::NotProject | RemoteFailure::NotObserver => Failure::Denied,
        RemoteFailure::Conflict => Failure::Conflict,
        RemoteFailure::WrongService | RemoteFailure::WrongNamespace => Failure::Binding,
        _ => Failure::InvalidReply,
    })?;
    let Some(receipt) = receipt else {
        return Ok((
            4,
            json!({"status":"absent","retry_authority":"not_established"}),
        ));
    };
    if receipt.request != input {
        return Err(Failure::Binding);
    }
    Ok(match receipt.result {
        Ok(changed) => (0, json!({"status":"recorded_success","changed":changed})),
        Err(error) => (
            4,
            json!({"status":"recorded_failure","failure":format!("{error:?}")}),
        ),
    })
}

#[cfg(test)]
mod tests;
