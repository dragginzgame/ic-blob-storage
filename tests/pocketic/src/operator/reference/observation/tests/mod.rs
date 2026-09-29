use super::*;
use candid::Principal;
use ic_blob_storage::dto::reference::{
    ReferenceAction, ReferenceFailure, ReferenceReceiptResponse, ReferenceTransitionFailure,
    ReferenceUpload,
};

fn request() -> ReferenceCommand {
    ReferenceCommand {
        upload: ReferenceUpload {
            service: Principal::from_slice(&[1, 1]),
            tenant: Principal::from_slice(&[2, 1]),
            namespace: u128::MAX,
            upload: u128::MAX - 1,
            object: u128::MAX - 2,
            incarnation: u128::MAX - 3,
            first_reference: u128::MAX - 4,
            root: [1; 32],
            bytes: 3,
        },
        reference: u128::MAX - 5,
        operation: u128::MAX,
        action: ReferenceAction::Retain,
    }
}

#[test]
fn distinguishes_absence_success_and_recorded_failure_without_retrying() {
    let input = request();
    for result in [
        None,
        Some(Ok(ReferenceChange::Changed)),
        Some(Ok(ReferenceChange::Unchanged)),
        Some(Err(ReferenceTransitionFailure::Released)),
    ] {
        let receipt = result.map_or(ReferenceReceiptLookup::Absent, |result| {
            ReferenceReceiptLookup::Found(ReferenceReceiptResponse {
                request: input,
                result,
            })
        });
        let bytes = candid::encode_one(Ok::<_, ReferenceFailure>(receipt)).unwrap();
        let (code, value) = decode(input, &bytes).unwrap();
        match result {
            None => {
                assert_eq!(code, 4);
                assert_eq!(value["status"], "absent");
                assert_eq!(value["retry_authority"], "not_established");
            }
            Some(Ok(change)) => {
                assert_eq!(code, 0);
                assert_eq!(value["changed"], change == ReferenceChange::Changed);
            }
            Some(Err(_)) => {
                assert_eq!(code, 4);
                assert_eq!(value["status"], "recorded_failure");
                assert_eq!(value["failure"], "Released");
            }
        }
    }
}

#[test]
fn rejects_changed_operation_oversize_and_malformed_replies() {
    let input = request();
    let changed = ReferenceReceiptLookup::Found(ReferenceReceiptResponse {
        request: ReferenceCommand {
            action: ReferenceAction::Release,
            ..input
        },
        result: Ok(ReferenceChange::Changed),
    });
    assert_eq!(
        decode(
            input,
            &candid::encode_one(Ok::<_, ReferenceFailure>(changed)).unwrap()
        ),
        Err(Failure::Binding)
    );
    assert_eq!(decode(input, &vec![0; 4097]), Err(Failure::ReplyTooLarge));
    assert_eq!(decode(input, &[0; 5]), Err(Failure::InvalidReply));
}

#[test]
fn service_refusals_are_preserved_and_never_rendered_as_absence_or_recorded_failure() {
    for failure in [
        ReferenceFailure::Invalid,
        ReferenceFailure::Denied,
        ReferenceFailure::Binding,
        ReferenceFailure::Unknown,
        ReferenceFailure::Unconfirmed,
        ReferenceFailure::Conflict,
        ReferenceFailure::Inactive,
        ReferenceFailure::Fenced,
        ReferenceFailure::Capacity,
        ReferenceFailure::Internal,
    ] {
        let bytes = candid::encode_one(Err::<ReferenceReceiptLookup, _>(failure)).unwrap();
        let (code, value) = decode(request(), &bytes).unwrap();
        assert_eq!(code, 3);
        assert_eq!(value["status"], "service_refusal");
        assert_eq!(value["failure"], format!("{failure:?}"));
    }
}
