use super::*;
use crate::dto::reference::{
    ReferenceAction, ReferenceChange, ReferenceReceiptResponse, ReferenceTransitionFailure,
    ReferenceUpload,
};
use candid::{Principal, encode_one};
fn request() -> ReferenceCommand {
    ReferenceCommand {
        upload: ReferenceUpload {
            service: Principal::from_slice(&[1, 1]),
            tenant: Principal::from_slice(&[2, 1]),
            namespace: 3,
            upload: u128::MAX,
            object: u128::MAX - 1,
            incarnation: 5,
            first_reference: 6,
            root: [9; 32],
            bytes: 10,
        },
        reference: 7,
        operation: u128::MAX - 2,
        action: ReferenceAction::Retain,
    }
}
fn encoded(value: Result<ReferenceReceiptLookup, ReferenceFailure>) -> Vec<u8> {
    encode_one(value).unwrap()
}
fn decode_reply(bytes: &[u8]) -> Result<ReferenceReceiptLookup, ReferenceReplyError> {
    decode(request(), bytes, 4096.try_into().unwrap())
}
#[test]
fn receipt_reply_distinguishes_absence_success_recorded_failure_and_lookup_refusal() {
    assert_eq!(
        decode_reply(&encoded(Ok(ReferenceReceiptLookup::Absent))),
        Ok(ReferenceReceiptLookup::Absent)
    );
    for result in [
        Ok(ReferenceChange::Changed),
        Ok(ReferenceChange::Unchanged),
        Err(ReferenceTransitionFailure::UnknownReference),
        Err(ReferenceTransitionFailure::Released),
        Err(ReferenceTransitionFailure::Limit),
        Err(ReferenceTransitionFailure::DeletionQueued),
    ] {
        let response = ReferenceReceiptResponse {
            request: request(),
            result,
        };
        assert_eq!(
            decode_reply(&encoded(Ok(ReferenceReceiptLookup::Found(response)))),
            Ok(ReferenceReceiptLookup::Found(response))
        );
    }
    assert_eq!(
        decode_reply(&encoded(Err(ReferenceFailure::Conflict))),
        Err(ReferenceReplyError::Remote(ReferenceFailure::Conflict))
    );
}
#[test]
fn receipt_reply_binds_every_original_argument() {
    let r = request();
    let mut changed = vec![
        ReferenceCommand { reference: 8, ..r },
        ReferenceCommand { operation: 8, ..r },
        ReferenceCommand {
            action: ReferenceAction::Release,
            ..r
        },
    ];
    for upload in [
        ReferenceUpload {
            service: r.upload.tenant,
            ..r.upload
        },
        ReferenceUpload {
            tenant: r.upload.service,
            ..r.upload
        },
        ReferenceUpload {
            namespace: 4,
            ..r.upload
        },
        ReferenceUpload {
            upload: 4,
            ..r.upload
        },
        ReferenceUpload {
            object: 4,
            ..r.upload
        },
        ReferenceUpload {
            incarnation: 4,
            ..r.upload
        },
        ReferenceUpload {
            first_reference: 4,
            ..r.upload
        },
        ReferenceUpload {
            root: [8; 32],
            ..r.upload
        },
        ReferenceUpload {
            bytes: 11,
            ..r.upload
        },
    ] {
        changed.push(ReferenceCommand { upload, ..r });
    }
    for request in changed {
        assert_eq!(
            decode_reply(&encoded(Ok(ReferenceReceiptLookup::Found(
                ReferenceReceiptResponse {
                    request,
                    result: Ok(ReferenceChange::Changed),
                }
            )))),
            Err(ReferenceReplyError::Binding)
        );
    }
}
#[test]
fn receipt_reply_bounds_and_rejects_malformed_inputs() {
    let bytes = encoded(Ok(ReferenceReceiptLookup::Absent));
    assert_eq!(
        decode(request(), &bytes, 1.try_into().unwrap()),
        Err(ReferenceReplyError::Limit)
    );
    assert_eq!(
        decode_reply(b"not candid"),
        Err(ReferenceReplyError::Invalid)
    );
    assert_eq!(
        decode_reply(&encode_one(17_u64).unwrap()),
        Err(ReferenceReplyError::Invalid)
    );
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert_eq!(decode_reply(&trailing), Err(ReferenceReplyError::Invalid));
    let r = request();
    for invalid in [
        ReferenceCommand { operation: 0, ..r },
        ReferenceCommand { reference: 0, ..r },
        ReferenceCommand {
            upload: ReferenceUpload {
                bytes: 0,
                ..r.upload
            },
            ..r
        },
    ] {
        assert_eq!(
            decode(invalid, &bytes, 4096.try_into().unwrap()),
            Err(ReferenceReplyError::Invalid)
        );
    }
}

#[test]
fn receipt_reply_rejects_wrong_found_payload_instead_of_reporting_absence() {
    #[derive(candid::CandidType)]
    enum WrongLookup {
        Found(u64),
    }
    let value: Result<WrongLookup, ReferenceFailure> = Ok(WrongLookup::Found(17));
    assert_eq!(
        decode_reply(&encode_one(value).unwrap()),
        Err(ReferenceReplyError::Invalid)
    );
}

#[test]
fn mutation_reply_keeps_inner_failure_distinct_and_rejects_foreign_or_incomplete_results() {
    use crate::dto::reference::ReferenceMutationResponse;
    let request = request();
    let max = 4096.try_into().unwrap();
    for replayed in [false, true] {
        for result in [
            Ok(ReferenceChange::Changed),
            Ok(ReferenceChange::Unchanged),
            Err(ReferenceTransitionFailure::UnknownReference),
        ] {
            let response = ReferenceMutationResponse {
                replayed,
                receipt: ReferenceReceiptResponse { request, result },
            };
            let bytes = encode_one(Ok::<_, ReferenceFailure>(response)).unwrap();
            assert_eq!(decode_mutation(request, &bytes, max), Ok(response));
            assert_eq!(
                decode_mutation(request, &bytes, 1.try_into().unwrap()),
                Err(ReferenceReplyError::Limit)
            );
            assert_eq!(
                decode_mutation(
                    ReferenceCommand {
                        operation: 1,
                        ..request
                    },
                    &bytes,
                    max
                ),
                Err(ReferenceReplyError::Binding)
            );
        }
    }
    let refusal: Result<ReferenceMutationResponse, ReferenceFailure> =
        Err(ReferenceFailure::Capacity);
    assert_eq!(
        decode_mutation(request, &encode_one(refusal).unwrap(), max),
        Err(ReferenceReplyError::Remote(ReferenceFailure::Capacity))
    );
    for bytes in [
        b"not candid".to_vec(),
        encoded(Ok(ReferenceReceiptLookup::Absent)),
        encode_one(17_u64).unwrap(),
    ] {
        assert_eq!(
            decode_mutation(request, &bytes, max),
            Err(ReferenceReplyError::Invalid)
        );
    }
}
