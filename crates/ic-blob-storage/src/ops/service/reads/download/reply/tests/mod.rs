use super::*;
use crate::dto::download::DownloadHeader;
use candid::Principal;
fn setup() -> (
    DownloadRequest,
    CaffeineDownloadScope,
    DownloadResponse,
    DownloadReplyLimits,
) {
    let request = DownloadRequest {
        service: Principal::from_slice(&[1, 1]),
        tenant: Principal::from_slice(&[2, 1]),
        namespace: u128::MAX,
        root: [3; 32],
        object: u128::MAX,
        incarnation: u128::MAX,
        reference: u128::MAX,
    };
    let scope = CaffeineDownloadScope::new(
        request.service,
        request.namespace.try_into().unwrap(),
        "project/β",
    )
    .unwrap();
    let response = DownloadResponse {
        request,
        owner: request.service,
        project: scope.project().into(),
        bytes: 10,
        headers: vec![
            DownloadHeader {
                name: "Content-Length".into(),
                value: "10".into(),
            },
            DownloadHeader {
                name: "Content-Type".into(),
                value: "image/png".into(),
            },
        ],
    };
    let limits = DownloadReplyLimits {
        max_reply_bytes: 4096.try_into().unwrap(),
        max_content_bytes: 10.try_into().unwrap(),
        max_headers: 8.try_into().unwrap(),
        max_header_bytes: 1024.try_into().unwrap(),
    };
    (request, scope, response, limits)
}
fn encode(response: DownloadResponse) -> Vec<u8> {
    candid::encode_one(Ok::<_, DownloadFailure>(response)).unwrap()
}
#[test]
fn descriptor_reply_requires_every_original_identity_and_preserves_metadata() {
    let (request, scope, response, limits) = setup();
    assert_eq!(
        decode(request, &scope, &encode(response.clone()), limits),
        Ok(response.clone())
    );
    for field in 0..9 {
        let mut changed = response.clone();
        match field {
            0 => changed.request.service = request.tenant,
            1 => changed.request.tenant = request.service,
            2 => changed.request.namespace = 1,
            3 => changed.request.root[0] ^= 1,
            4 => changed.request.object = 1,
            5 => changed.request.incarnation = 1,
            6 => changed.request.reference = 1,
            7 => changed.owner = request.tenant,
            _ => changed.project.push('x'),
        }
        assert_eq!(
            decode(request, &scope, &encode(changed), limits),
            Err(DownloadReplyError::Binding)
        );
    }
    let mut bad_request = request;
    bad_request.reference = 0;
    assert_eq!(
        decode(bad_request, &scope, &encode(response), limits),
        Err(DownloadReplyError::Invalid)
    );
}
#[test]
fn descriptor_reply_bounds_decoding_and_rejects_inconsistent_hash_metadata() {
    let (request, scope, response, limits) = setup();
    let encoded = encode(response.clone());
    for bytes in [
        &b"invalid"[..],
        &encoded[..encoded.len() - 1],
        &candid::encode_one("wrong type").unwrap(),
    ] {
        assert_eq!(
            decode(request, &scope, bytes, limits),
            Err(DownloadReplyError::Invalid)
        );
    }
    assert_eq!(
        decode(
            request,
            &scope,
            &encoded,
            DownloadReplyLimits {
                max_reply_bytes: (encoded.len() - 1).try_into().unwrap(),
                ..limits
            }
        ),
        Err(DownloadReplyError::Limit)
    );
    for field in 0..5 {
        let mut changed = response.clone();
        match field {
            0 => changed.bytes = 11,
            1 => changed.bytes = 0,
            2 => changed.headers[0].value = "9".into(),
            3 => changed.headers.push(changed.headers[0].clone()),
            _ => changed.headers[1].value = "image/png\r\nX: y".into(),
        }
        let expected = if field < 2 {
            DownloadReplyError::Limit
        } else {
            DownloadReplyError::Invalid
        };
        assert_eq!(
            decode(request, &scope, &encode(changed), limits),
            Err(expected)
        );
    }
    assert_eq!(
        decode(
            request,
            &scope,
            &encoded,
            DownloadReplyLimits {
                max_header_bytes: 1.try_into().unwrap(),
                ..limits
            }
        ),
        Err(DownloadReplyError::Limit)
    );
    let refusal = candid::encode_one(Err::<DownloadResponse, _>(DownloadFailure::Fenced)).unwrap();
    assert_eq!(
        decode(request, &scope, &refusal, limits),
        Err(DownloadReplyError::Remote(DownloadFailure::Fenced))
    );
}
