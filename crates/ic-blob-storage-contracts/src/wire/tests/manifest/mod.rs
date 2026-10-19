//! Representative maximum CLI declaration with full-width identities and fixed ordering.
use super::*;
use crate::dto::reference::ReferenceUpload;
use crate::dto::upload::admission::UploadAdmissionRequest;
use crate::dto::upload::manifest::{
    UploadManifestDeclaration, UploadManifestHeader, UploadManifestInspection,
    UploadManifestRequest, UploadManifestResponse,
};

fn manifest() -> UploadManifestRequest {
    UploadManifestRequest {
        permission: UploadAdmissionRequest {
            upload: ReferenceUpload {
                service: Principal::self_authenticating(b"service"),
                tenant: Principal::self_authenticating(b"tenant"),
                namespace: u128::MAX,
                upload: u128::MAX - 1,
                object: u128::MAX - 2,
                incarnation: u128::MAX - 3,
                first_reference: u128::MAX - 4,
                root: [251; 32],
                bytes: u64::MAX,
            },
            uploader: Principal::self_authenticating(b"uploader"),
            expires_at_ns: u64::MAX,
        },
        declaration: UploadManifestDeclaration {
            chunks: (0..1024)
                .map(|index| [u8::try_from(index % 251).unwrap(); 32])
                .collect(),
            headers: (0..16)
                .map(|index| UploadManifestHeader {
                    name: format!("x-fixture-{index}"),
                    value: "v".repeat(200),
                })
                .collect(),
        },
    }
}
#[test]
fn full_width_manifest_requests_and_historical_replies_keep_order_and_exact_identity() {
    let input = manifest();
    let request = encode_request("blob_prepare_upload", &input).unwrap();
    assert!(request.len() < 128 * 1024);
    assert_eq!(
        &request[..11],
        &[b'I', b'W', 8, 0, 0x36, 0, 0x42, 0x4f, 0x4c, 0x42, 0]
    );
    assert_eq!(
        decode_request::<UploadManifestRequest>("blob_prepare_upload", &request, 128 * 1024)
            .unwrap(),
        input
    );
    // Independent tuple projection freezes every positional component in this native schema.
    let u = input.permission.upload;
    let ordered = (
        (
            u.service,
            u.tenant,
            u.namespace,
            u.upload,
            u.object,
            u.incarnation,
            u.first_reference,
            u.root,
            u.bytes,
        ),
        input.permission.uploader,
        input.permission.expires_at_ns,
    );
    let headers: Vec<_> = input
        .declaration
        .headers
        .iter()
        .map(|h| (&h.name, &h.value))
        .collect();
    assert_eq!(
        encode_request(
            "blob_prepare_upload",
            (ordered, (&input.declaration.chunks, headers))
        )
        .unwrap(),
        request
    );
    for method in ["blob_upload_manifest", "blob_verification_manifest"] {
        let response =
            Ok::<_, crate::dto::upload::manifest::UploadManifestFailure>(UploadManifestResponse {
                permission: input.permission,
                manifest: UploadManifestInspection::Prepared(input.declaration.clone()),
            });
        let bytes = encode_response(method, &response).unwrap();
        assert_eq!(
            decode_response::<
                Result<UploadManifestResponse, crate::dto::upload::manifest::UploadManifestFailure>,
            >(method, &bytes, 128 * 1024)
            .unwrap(),
            response
        );
        let other = if method == "blob_upload_manifest" {
            "blob_verification_manifest"
        } else {
            "blob_upload_manifest"
        };
        assert_eq!(
            decode_response::<
                Result<UploadManifestResponse, crate::dto::upload::manifest::UploadManifestFailure>,
            >(other, &bytes, 128 * 1024),
            Err(Error::Identity)
        );
    }
    for end in [0, 10, 11, request.len() / 2, request.len() - 1] {
        assert!(
            decode_request::<UploadManifestRequest>(
                "blob_prepare_upload",
                &request[..end],
                128 * 1024
            )
            .is_err()
        );
    }
    assert_eq!(
        decode_request::<UploadManifestRequest>("blob_prepare_upload", &request, request.len() - 1),
        Err(Error::Limit)
    );
    let old = crate::document::encode(&input).unwrap();
    assert!(
        decode_request::<UploadManifestRequest>("blob_prepare_upload", &old, 128 * 1024).is_err()
    );
    let mut too_large = input;
    too_large.declaration.headers[0].value = "v".repeat(128 * 1024);
    assert_eq!(
        encode_request("blob_prepare_upload", too_large),
        Err(Error::Limit)
    );
}
