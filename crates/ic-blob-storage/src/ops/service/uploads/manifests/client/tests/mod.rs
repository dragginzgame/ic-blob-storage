use super::*;
use crate::dto::reference::ReferenceUpload;
fn p(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}
#[test]
fn manifest_client_separates_actual_uploader_from_tenant_observer() {
    let input = UploadAdmissionRequest {
        upload: ReferenceUpload {
            service: p(1),
            tenant: p(2),
            namespace: 1,
            upload: 1,
            object: 2,
            incarnation: 1,
            first_reference: 1,
            root: [1; 32],
            bytes: 10,
        },
        uploader: p(3),
        expires_at_ns: 100,
    };
    let client = ReplicatedUploadManifestClient::new(p(3), p(2), p(1), NonZeroU32::MIN).unwrap();
    assert_eq!(client.binding(p(3), input, true), Ok(()));
    assert_eq!(client.binding(p(3), input, false), Ok(()));
    assert_eq!(
        client.binding(p(2), input, true),
        Err(UploadManifestClientError::Binding)
    );
    let observer = ReplicatedUploadManifestClient::new(p(2), p(2), p(1), NonZeroU32::MIN).unwrap();
    assert_eq!(observer.binding(p(2), input, false), Ok(()));
    assert_eq!(
        observer.binding(p(2), input, true),
        Err(UploadManifestClientError::Binding)
    );
    for changed in [
        UploadAdmissionRequest {
            uploader: p(4),
            ..input
        },
        UploadAdmissionRequest {
            upload: ReferenceUpload {
                service: p(4),
                ..input.upload
            },
            ..input
        },
        UploadAdmissionRequest {
            upload: ReferenceUpload {
                tenant: p(4),
                ..input.upload
            },
            ..input
        },
    ] {
        assert_eq!(
            client.binding(p(3), changed, true),
            Err(UploadManifestClientError::Binding)
        );
    }
    let outsider = ReplicatedUploadManifestClient::new(p(4), p(2), p(1), NonZeroU32::MIN).unwrap();
    assert_eq!(
        outsider.binding(p(4), input, false),
        Err(UploadManifestClientError::Binding)
    );
}
#[test]
fn manifest_client_configuration_rejects_nonidentities_and_unbounded_wait() {
    for invalid in [Principal::anonymous(), Principal::management_canister()] {
        for principals in [
            [invalid, p(2), p(3)],
            [p(1), invalid, p(3)],
            [p(1), p(2), invalid],
        ] {
            assert_eq!(
                ReplicatedUploadManifestClient::new(
                    principals[0],
                    principals[1],
                    principals[2],
                    NonZeroU32::MIN
                ),
                Err(UploadManifestClientError::Configuration)
            );
        }
    }
    assert_eq!(
        ReplicatedUploadManifestClient::new(p(1), p(2), p(3), 301.try_into().unwrap()),
        Err(UploadManifestClientError::Configuration)
    );
}
