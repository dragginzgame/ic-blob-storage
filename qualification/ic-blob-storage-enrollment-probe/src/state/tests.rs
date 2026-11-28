use super::*;
use crate::protocol::{Installation, LIMITS, Outcome, REPLY_BYTES};
use ic_wire as wire;

#[test]
fn maximum_reply_fits_persistence_and_inspection_budgets() {
    let owner = Principal::from_slice(&[3; 29]);
    let configuration = Installation {
        operator: Principal::from_slice(&[1; 29]),
        cashier: Principal::from_slice(&[2; 29]),
        target_balance: 9_900_000_000,
    };
    let record = Record {
        version: 3,
        view: View {
            owner,
            arguments: arguments(owner, configuration.target_balance),
            configuration,
            attempted_at: Some(u64::MAX),
            outcome: Some(Outcome::Reply(
                ic_wire::Bytes::new(vec![255; REPLY_BYTES]).unwrap(),
            )),
        },
    };
    let bytes = ic_blob_storage::storage_codec::encode(&record, LIMITS.bytes).unwrap();
    let decoded: Record = ic_blob_storage::storage_codec::decode(&bytes, LIMITS.bytes, 3).unwrap();
    assert_eq!(decoded.view, record.view);
    let reply = Ok::<_, crate::protocol::Error>(record.view);
    let bytes = ic_blob_storage_wire::frame::encode(
        &reply,
        wire::Encoding::Frame {
            method: ic_wire::method_id("enrollment_inspect"),
            direction: wire::Direction::Response,
        },
        LIMITS,
    )
    .unwrap();
    let decoded: Result<View, crate::protocol::Error> = ic_blob_storage_wire::frame::decode(
        &bytes,
        wire::Encoding::Frame {
            method: ic_wire::method_id("enrollment_inspect"),
            direction: wire::Direction::Response,
        },
        LIMITS,
    )
    .unwrap();
    assert_eq!(decoded, reply);
}
