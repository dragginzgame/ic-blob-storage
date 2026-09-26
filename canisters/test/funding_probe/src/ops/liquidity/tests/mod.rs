use super::*;
use blob_test_protocol::funding::{FundingReplyMode, preview::FundingPreviewRequest};

#[test]
fn preview_payload_bounds_every_valid_local_reply_control() {
    for offered in [1, 127, 128, 16_383, 16_384, 10_000_000_000_000, u128::MAX] {
        let request = preview_request(FundingPreviewRequest {
            service: Principal::from_slice(&[1]),
            peer: Principal::from_slice(&[2]),
            id: u64::MAX,
            requested_cycles: offered,
            revision: 0,
        });
        let maximum = candid::encode_one(request).unwrap().len();
        for accept in [0, offered / 2, offered] {
            for reply in [
                FundingReplyMode::Success,
                FundingReplyMode::DelayedSuccess,
                FundingReplyMode::ProviderError,
                FundingReplyMode::Malformed,
                FundingReplyMode::Reject,
                FundingReplyMode::Trap,
            ] {
                for trap_callback in [false, true] {
                    let variant = FundingRequest {
                        accept,
                        reply,
                        trap_callback,
                        ..request
                    };
                    assert!(candid::encode_one(variant).unwrap().len() <= maximum);
                }
            }
        }
    }
}
