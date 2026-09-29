//! Render the maintained receipt decoder's result without inventing retry authority.
use crate::operator::Failure;
use ic_blob_storage::{
    dto::reference::{ReferenceChange, ReferenceCommand, ReferenceReceiptLookup},
    ops::service::references::reply::{self, ReferenceReplyError},
};
use serde_json::{Value, json};

pub(super) fn decode(input: ReferenceCommand, bytes: &[u8]) -> Result<(u8, Value), Failure> {
    let receipt = match reply::decode(input, bytes, 4096.try_into().unwrap()) {
        Ok(receipt) => receipt,
        Err(ReferenceReplyError::Remote(failure)) => {
            return Ok((
                3,
                json!({"status":"service_refusal", "failure":format!("{failure:?}")}),
            ));
        }
        Err(ReferenceReplyError::Limit) => return Err(Failure::ReplyTooLarge),
        Err(ReferenceReplyError::Binding) => return Err(Failure::Binding),
        Err(ReferenceReplyError::Invalid) => return Err(Failure::InvalidReply),
    };
    let ReferenceReceiptLookup::Found(receipt) = receipt else {
        return Ok((
            4,
            json!({"status":"absent","retry_authority":"not_established"}),
        ));
    };
    Ok(match receipt.result {
        Ok(change) => (
            0,
            json!({"status":"recorded_success","changed":change == ReferenceChange::Changed}),
        ),
        Err(error) => (
            4,
            json!({"status":"recorded_failure","failure":format!("{error:?}")}),
        ),
    })
}

#[cfg(test)]
mod tests;
