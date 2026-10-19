use super::*;
use crate::dto::{
    reference::ReferenceUpload,
    tenant::TenantScope,
    upload::history::{UploadContentState, UploadHistoryEntry},
};
use ic_platform::Principal;

fn response() -> UploadDiscoveryResponse {
    let scope = TenantScope {
        service: Principal::from_slice(&[1]),
        namespace: 7,
        tenant: Principal::from_slice(&[2]),
    };
    UploadDiscoveryResponse {
        request: UploadDiscoveryRequest {
            scope,
            root: [3; 32],
        },
        content: Some(UploadHistoryEntry {
            request: ReferenceUpload {
                service: scope.service,
                namespace: scope.namespace,
                tenant: scope.tenant,
                root: [3; 32],
                upload: u128::MAX,
                object: u128::MAX - 1,
                incarnation: u128::MAX - 2,
                first_reference: u128::MAX - 3,
                bytes: u64::MAX,
            },
            state: UploadContentState::Reserved,
        }),
        fenced: true,
    }
}

#[test]
fn preserves_absence_every_lifecycle_and_restore_fence() {
    let mut expected = response();
    let ceiling = NonZeroUsize::new(4096).unwrap();
    for state in [
        UploadContentState::Reserved,
        UploadContentState::ExposurePossible,
        UploadContentState::Cancelled,
        UploadContentState::Live,
        UploadContentState::DeletionPending,
        UploadContentState::ProviderDeleted,
        UploadContentState::Settled,
    ] {
        expected.content.as_mut().unwrap().state = state;
        let bytes = crate::wire::encode_response(
            "blob_lookup_content",
            Ok::<_, UploadDiscoveryFailure>(expected),
        )
        .unwrap();
        assert_eq!(decode(expected.request, &bytes, ceiling), Ok(expected));
    }
    expected.content = None;
    let bytes = crate::wire::encode_response(
        "blob_lookup_content",
        Ok::<_, UploadDiscoveryFailure>(expected),
    )
    .unwrap();
    assert_eq!(
        decode(
            expected.request,
            &bytes,
            NonZeroUsize::new(bytes.len()).unwrap()
        ),
        Ok(expected)
    );
    assert_eq!(
        decode(
            expected.request,
            &bytes,
            NonZeroUsize::new(bytes.len() - 1).unwrap()
        ),
        Err(UploadDiscoveryReplyError::Limit)
    );
    let mut changed = expected.request;
    changed.root = [4; 32];
    assert_eq!(
        decode(changed, &bytes, ceiling),
        Err(UploadDiscoveryReplyError::Binding)
    );
}

#[test]
fn rejects_changed_original_bindings_and_zero_identities() {
    let expected = response();
    let ceiling = NonZeroUsize::new(4096).unwrap();
    for field in 0..9 {
        let mut invalid = expected;
        let original = &mut invalid.content.as_mut().unwrap().request;
        match field {
            0 => original.service = Principal::anonymous(),
            1 => original.tenant = Principal::anonymous(),
            2 => original.namespace += 1,
            3 => original.root = [4; 32],
            4 => original.upload = 0,
            5 => original.object = 0,
            6 => original.incarnation = 0,
            7 => original.first_reference = 0,
            _ => original.bytes = 0,
        }
        let bytes = crate::wire::encode_response(
            "blob_lookup_content",
            Ok::<_, UploadDiscoveryFailure>(invalid),
        )
        .unwrap();
        let error = if field < 4 {
            UploadDiscoveryReplyError::Binding
        } else {
            UploadDiscoveryReplyError::Content
        };
        assert_eq!(decode(expected.request, &bytes, ceiling), Err(error));
    }
    assert_eq!(
        decode(expected.request, b"invalid", ceiling),
        Err(UploadDiscoveryReplyError::Invalid)
    );
    for refusal in [
        UploadDiscoveryFailure::Denied,
        UploadDiscoveryFailure::Binding,
        UploadDiscoveryFailure::Invalid,
        UploadDiscoveryFailure::Internal,
    ] {
        let bytes = crate::wire::encode_response(
            "blob_lookup_content",
            Err::<UploadDiscoveryResponse, _>(refusal),
        )
        .unwrap();
        assert_eq!(
            decode(expected.request, &bytes, ceiling),
            Err(UploadDiscoveryReplyError::Remote(refusal))
        );
    }
}
