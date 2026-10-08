use crate::dto::reference::ReferenceUpload;
use crate::upload::completion::reply::*;
use candid::Principal;

fn fixture() -> (CompletionAuthority, UploadAttestationReceipt) {
    let authority = CompletionAuthority::new(
        Principal::self_authenticating([1]),
        u128::MAX.try_into().unwrap(),
        Principal::self_authenticating([2]),
    )
    .unwrap();
    let receipt = UploadAttestationReceipt {
        request: UploadAttestationRequest {
            permission: UploadAdmissionRequest {
                upload: ReferenceUpload {
                    service: authority.service(),
                    namespace: u128::MAX,
                    tenant: Principal::self_authenticating([3]),
                    upload: u128::MAX - 1,
                    object: u128::MAX - 2,
                    incarnation: u128::MAX - 3,
                    first_reference: u128::MAX - 4,
                    root: [7; 32],
                    bytes: u64::MAX,
                },
                uploader: Principal::self_authenticating([4]),
                expires_at_ns: 10,
            },
            content_digest: [8; 32],
            observed_at_ns: u64::MAX - 1,
        },
        verifier: authority.verifier(),
        accepted_at_ns: u64::MAX,
    };
    (authority, receipt)
}
fn encoded<T: CandidType>(value: T) -> Vec<u8> {
    candid::encode_one(Ok::<_, UploadAttestationFailure>(value)).unwrap()
}
fn inspect(
    authority: CompletionAuthority,
    permission: UploadAdmissionRequest,
    response: &UploadAttestationResponse,
) -> Result<UploadAttestationResponse, UploadAttestationReplyError> {
    inspection(
        authority,
        permission,
        &encoded(response),
        4096.try_into().unwrap(),
    )
}

#[test]
fn historical_receipts_and_absence_preserve_exact_scope_times_and_fences() {
    let (authority, receipt) = fixture();
    let permission = receipt.request.permission;
    assert_eq!(
        candid::decode_one::<UploadAdmissionRequest>(
            &inspection_request(authority, permission).unwrap()
        )
        .unwrap(),
        permission
    );
    for fenced in [false, true] {
        for attestation in [
            UploadAttestationLookup::Absent,
            UploadAttestationLookup::Found(receipt),
        ] {
            let response = UploadAttestationResponse {
                permission,
                attestation,
                fenced,
            };
            assert_eq!(inspect(authority, permission, &response), Ok(response));
        }
    }
    for changed in [false, true] {
        let response = UploadAttestationMutation { receipt, changed };
        assert_eq!(
            mutation(
                authority,
                &receipt.request,
                &encoded(response),
                4096.try_into().unwrap()
            ),
            Ok(response)
        );
    }
}

#[test]
fn receipt_binding_and_chronology_are_checked_even_for_restored_history() {
    use UploadAttestationReplyError::{Binding, Invalid};
    let (authority, receipt) = fixture();
    let permission = receipt.request.permission;
    let response = UploadAttestationResponse {
        permission,
        attestation: UploadAttestationLookup::Found(receipt),
        fenced: true,
    };
    let mut wrong_scope = permission;
    wrong_scope.upload.namespace = 1;
    assert_eq!(inspection_request(authority, wrong_scope), Err(Binding));
    wrong_scope = permission;
    wrong_scope.upload.service = Principal::self_authenticating([9]);
    assert_eq!(inspection_request(authority, wrong_scope), Err(Binding));
    let mut invalid = permission;
    invalid.upload.object = 0;
    assert_eq!(inspection_request(authority, invalid), Err(Invalid));
    invalid = permission;
    invalid.uploader = Principal::anonymous();
    assert_eq!(inspection_request(authority, invalid), Err(Invalid));
    let mut wrong_echo = response;
    wrong_echo.permission.expires_at_ns += 1;
    assert_eq!(inspect(authority, permission, &wrong_echo), Err(Binding));
    for change in 0..5 {
        let mut wrong = receipt;
        match change {
            0 => wrong.verifier = Principal::self_authenticating([9]),
            1 => wrong.request.permission.upload.first_reference = 1,
            2 => wrong.request.permission.uploader = authority.verifier(),
            3 => wrong.request.permission.expires_at_ns += 1,
            _ => wrong.accepted_at_ns = 0,
        }
        let expected = if change == 4 { Invalid } else { Binding };
        assert_eq!(
            inspect(
                authority,
                permission,
                &UploadAttestationResponse {
                    attestation: UploadAttestationLookup::Found(wrong),
                    ..response
                }
            ),
            Err(expected)
        );
        assert_eq!(
            mutation(
                authority,
                &receipt.request,
                &encoded(UploadAttestationMutation {
                    receipt: wrong,
                    changed: false
                }),
                4096.try_into().unwrap()
            ),
            Err(expected)
        );
    }
    for change_time in [false, true] {
        let mut other = receipt;
        if change_time {
            other.request.observed_at_ns -= 1;
        } else {
            other.request.content_digest[0] ^= 1;
        }
        // Historical lookup must preserve a different statement for conflict diagnosis.
        let conflict = UploadAttestationResponse {
            attestation: UploadAttestationLookup::Found(other),
            ..response
        };
        assert_eq!(inspect(authority, permission, &conflict), Ok(conflict));
        assert_eq!(
            mutation(
                authority,
                &receipt.request,
                &encoded(UploadAttestationMutation {
                    receipt: other,
                    changed: false
                }),
                4096.try_into().unwrap()
            ),
            Err(Binding)
        );
    }
}

#[test]
fn malformed_oversized_and_remote_refusals_never_become_absence() {
    let (authority, receipt) = fixture();
    let permission = receipt.request.permission;
    for (bytes, expected) in [
        (b"DIDL".to_vec(), UploadAttestationReplyError::Invalid),
        (vec![0; 4097], UploadAttestationReplyError::Limit),
        (encoded(42_u64), UploadAttestationReplyError::Invalid),
        (
            candid::encode_args((
                Ok::<_, UploadAttestationFailure>(UploadAttestationResponse {
                    permission,
                    attestation: UploadAttestationLookup::Absent,
                    fenced: false,
                }),
                1_u64,
            ))
            .unwrap(),
            UploadAttestationReplyError::Invalid,
        ),
    ] {
        assert_eq!(
            inspection(authority, permission, &bytes, 4096.try_into().unwrap()),
            Err(expected)
        );
        assert_eq!(
            mutation(
                authority,
                &receipt.request,
                &bytes,
                4096.try_into().unwrap()
            ),
            Err(expected)
        );
    }
    for failure in [
        UploadAttestationFailure::Denied,
        UploadAttestationFailure::Conflict,
        UploadAttestationFailure::Phase,
        UploadAttestationFailure::Observation,
        UploadAttestationFailure::Permission(
            crate::dto::upload::admission::UploadAdmissionFailure::Fenced,
        ),
    ] {
        let bytes = candid::encode_one(Err::<UploadAttestationResponse, _>(failure)).unwrap();
        assert_eq!(
            inspection(authority, permission, &bytes, 4096.try_into().unwrap()),
            Err(UploadAttestationReplyError::Remote(failure))
        );
    }
}
