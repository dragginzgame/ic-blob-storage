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
#[test]
fn passive_request_encoders_preserve_independent_identities_and_validate_inputs() {
    let r = request();
    assert_eq!(
        candid::decode_one::<ReferenceCommand>(&receipt_request(r).unwrap()).unwrap(),
        r
    );
    let s = ReferenceStatusRequest {
        upload: r.upload,
        reference: r.reference,
    };
    assert_eq!(
        candid::decode_one::<ReferenceStatusRequest>(&status_request(s).unwrap()).unwrap(),
        s
    );
    assert_eq!(
        receipt_request(ReferenceCommand { operation: 0, ..r }),
        Err(ReferenceReplyError::Invalid)
    );
    assert_eq!(
        receipt_request(ReferenceCommand { reference: 0, ..r }),
        Err(ReferenceReplyError::Invalid)
    );
    assert_eq!(
        status_request(ReferenceStatusRequest { reference: 0, ..s }),
        Err(ReferenceReplyError::Invalid)
    );
    assert_eq!(
        status_request(ReferenceStatusRequest {
            upload: ReferenceUpload {
                bytes: 0,
                ..s.upload
            },
            ..s
        }),
        Err(ReferenceReplyError::Invalid)
    );
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

#[test]
fn reference_status_reply_preserves_fences_and_rejects_binding_size_and_wire_failures() {
    let command = request();
    let request = ReferenceStatusRequest {
        upload: command.upload,
        reference: command.reference,
    };
    let limit = 4096.try_into().unwrap();
    for (live, fenced) in [(true, false), (true, true), (false, false), (false, true)] {
        let response = ReferenceStatusResponse {
            request,
            live,
            fenced,
        };
        let bytes = encode_one(Ok::<_, ReferenceFailure>(response)).unwrap();
        assert_eq!(decode_status(request, &bytes, limit), Ok(response));
        assert_eq!(
            decode_status(request, &bytes, NonZeroUsize::MIN),
            Err(ReferenceReplyError::Limit)
        );
        assert_eq!(
            decode_status(
                ReferenceStatusRequest {
                    reference: 0,
                    ..request
                },
                &bytes,
                limit
            ),
            Err(ReferenceReplyError::Invalid)
        );
    }
    let mut altered = vec![ReferenceStatusRequest {
        reference: 8,
        ..request
    }];
    for upload in [
        ReferenceUpload {
            service: command.upload.tenant,
            ..command.upload
        },
        ReferenceUpload {
            tenant: command.upload.service,
            ..command.upload
        },
        ReferenceUpload {
            namespace: 4,
            ..command.upload
        },
        ReferenceUpload {
            upload: 1,
            ..command.upload
        },
        ReferenceUpload {
            object: 1,
            ..command.upload
        },
        ReferenceUpload {
            incarnation: 1,
            ..command.upload
        },
        ReferenceUpload {
            first_reference: 1,
            ..command.upload
        },
        ReferenceUpload {
            root: [8; 32],
            ..command.upload
        },
        ReferenceUpload {
            bytes: 11,
            ..command.upload
        },
    ] {
        altered.push(ReferenceStatusRequest { upload, ..request });
    }
    for changed in altered {
        let bytes = encode_one(Ok::<_, ReferenceFailure>(ReferenceStatusResponse {
            request: changed,
            live: true,
            fenced: false,
        }))
        .unwrap();
        assert_eq!(
            decode_status(request, &bytes, limit),
            Err(ReferenceReplyError::Binding)
        );
    }
    assert_eq!(
        decode_status(request, b"DIDL", limit),
        Err(ReferenceReplyError::Invalid)
    );
    let denied = encode_one(Err::<ReferenceStatusResponse, _>(ReferenceFailure::Denied)).unwrap();
    assert_eq!(
        decode_status(request, &denied, limit),
        Err(ReferenceReplyError::Remote(ReferenceFailure::Denied))
    );
}
