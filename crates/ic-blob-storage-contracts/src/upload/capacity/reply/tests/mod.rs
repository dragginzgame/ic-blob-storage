use super::*;
use crate::dto::tenant::TenantEnrollment;
use ic_platform::Principal;

fn response() -> UploadCapacityResponse {
    UploadCapacityResponse {
        scope: TenantScope {
            service: Principal::from_slice(&[1]),
            namespace: 7,
            tenant: Principal::from_slice(&[2]),
        },
        enrollment: TenantEnrollment {
            generation: 1,
            active: false,
        },
        max_object_bytes: 10,
        max_headers: 16,
        max_header_bytes: 4096,
        remaining_objects: 0,
        remaining_active_uploads: 0,
        remaining_manifest_chunks: 0,
        remaining_logical_bytes: u128::MAX,
        remaining_physical_bytes: u128::MAX - 1,
        remaining_liability_bytes: u128::MAX - 2,
        remaining_bytes: u128::MAX - 2,
        fenced: true,
    }
}

#[test]
fn preserves_wide_headroom_suspension_and_fence_with_exact_input_ceiling() {
    let expected = response();
    let bytes = crate::wire::encode_response(
        "blob_upload_capacity",
        Ok::<_, UploadCapacityFailure>(expected),
    )
    .unwrap();
    assert_eq!(
        decode(
            expected.scope,
            &bytes,
            NonZeroUsize::new(bytes.len()).unwrap()
        ),
        Ok(expected)
    );
    assert_eq!(
        decode(
            expected.scope,
            &bytes,
            NonZeroUsize::new(bytes.len() - 1).unwrap()
        ),
        Err(UploadCapacityReplyError::Limit)
    );
}

#[test]
fn rejects_inconsistent_counters_and_bindings_without_hiding_remote_refusals() {
    let expected = response();
    let ceiling = NonZeroUsize::new(4096).unwrap();
    for field in 0..5 {
        let mut invalid = expected;
        match field {
            0 => invalid.enrollment.generation = 0,
            1 => invalid.max_object_bytes = 0,
            2 => invalid.max_headers = 0,
            3 => invalid.max_header_bytes = 0,
            _ => invalid.remaining_bytes -= 1,
        }
        let bytes = crate::wire::encode_response(
            "blob_upload_capacity",
            Ok::<_, UploadCapacityFailure>(invalid),
        )
        .unwrap();
        assert_eq!(
            decode(expected.scope, &bytes, ceiling),
            Err(UploadCapacityReplyError::Invalid)
        );
    }
    let mut changed = expected;
    changed.scope.namespace += 1;
    let bytes = crate::wire::encode_response(
        "blob_upload_capacity",
        Ok::<_, UploadCapacityFailure>(changed),
    )
    .unwrap();
    assert_eq!(
        decode(expected.scope, &bytes, ceiling),
        Err(UploadCapacityReplyError::Binding)
    );
    assert_eq!(
        decode(expected.scope, b"invalid", ceiling),
        Err(UploadCapacityReplyError::Invalid)
    );
    for refusal in [
        UploadCapacityFailure::Denied,
        UploadCapacityFailure::Binding,
        UploadCapacityFailure::NotEnrolled,
        UploadCapacityFailure::Invalid,
        UploadCapacityFailure::Internal,
    ] {
        let bytes = crate::wire::encode_response(
            "blob_upload_capacity",
            Err::<UploadCapacityResponse, _>(refusal),
        )
        .unwrap();
        assert_eq!(
            decode(expected.scope, &bytes, ceiling),
            Err(UploadCapacityReplyError::Remote(refusal))
        );
    }
}
