use super::*;
use blob_test_protocol::admission::{Request, release::ReferenceFailure};
use candid::Principal;

fn request() -> ReferenceInput {
    ReferenceInput {
        object: Request {
            service: Principal::from_slice(&[1, 1]),
            tenant: Principal::from_slice(&[2, 1]),
            namespace: 1,
            id: 1,
            root: [1; 32],
            bytes: 3,
        },
        reference: 2,
        operation: u128::MAX,
        retain: true,
    }
}

#[test]
fn distinguishes_absence_success_and_recorded_failure_without_retrying() {
    let input = request();
    for result in [
        None,
        Some(Ok(true)),
        Some(Ok(false)),
        Some(Err(ReferenceFailure::Released)),
    ] {
        let receipt = result.map(|result| ReferenceReceipt {
            request: input,
            result,
        });
        let bytes = candid::encode_one(Ok::<_, RemoteFailure>(receipt)).unwrap();
        let (code, value) = decode(input, &bytes).unwrap();
        match result {
            None => {
                assert_eq!(code, 4);
                assert_eq!(value["status"], "absent");
            }
            Some(Ok(changed)) => {
                assert_eq!(code, 0);
                assert_eq!(value["changed"], changed);
            }
            Some(Err(_)) => {
                assert_eq!(code, 4);
                assert_eq!(value["status"], "recorded_failure");
            }
        }
    }
}

#[test]
fn rejects_changed_operation_oversize_malformed_and_conflicting_replies() {
    let input = request();
    let changed = ReferenceReceipt {
        request: ReferenceInput {
            retain: false,
            ..input
        },
        result: Ok(true),
    };
    assert_eq!(
        decode(
            input,
            &candid::encode_one(Ok::<_, RemoteFailure>(Some(changed))).unwrap()
        ),
        Err(Failure::Binding)
    );
    assert_eq!(decode(input, &vec![0; 4097]), Err(Failure::ReplyTooLarge));
    assert_eq!(decode(input, &[0; 5]), Err(Failure::InvalidReply));
    assert_eq!(
        decode(
            input,
            &candid::encode_one(Err::<Option<ReferenceReceipt>, _>(RemoteFailure::Conflict))
                .unwrap()
        ),
        Err(Failure::Conflict)
    );
}
