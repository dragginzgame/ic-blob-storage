use super::*;
use crate::protocol::REPLY_BYTES;

#[test]
fn maximum_reply_fits_persistence_and_inspection_budgets() {
    let owner = Principal::from_slice(&[3; 29]);
    let configuration = Installation {
        operator: Principal::from_slice(&[1; 29]),
        cashier: Principal::from_slice(&[2; 29]),
        target_balance: 9_900_000_000,
    };
    let record = Record {
        version: 1,
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
    let bytes = native::encode(&record, LIMITS).unwrap();
    let decoded: Record = native::decode(&bytes, LIMITS).unwrap();
    assert_eq!(decoded.view, record.view);
    let reply = Ok::<_, crate::protocol::Error>(record.view);
    let bytes = native::encode_response(crate::INSPECT, &reply, LIMITS).unwrap();
    let decoded: Result<View, crate::protocol::Error> =
        native::decode_response(crate::INSPECT, &bytes, LIMITS).unwrap();
    assert_eq!(decoded, reply);
}
