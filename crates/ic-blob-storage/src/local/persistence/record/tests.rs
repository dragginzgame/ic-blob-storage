use super::*;
use crate::{
    caffeine::ManifestLimits,
    local::{Limits, UploadRequest},
};

fn actor(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}

fn record() -> Record {
    let input = Installation {
        application: actor(1),
        manager: actor(9),
        limits: Limits {
            manifest: ManifestLimits {
                max_content_bytes: 100.try_into().unwrap(),
                max_chunks: 1.try_into().unwrap(),
                max_headers: 1.try_into().unwrap(),
                max_header_bytes: 100.try_into().unwrap(),
            },
            uploads: 64.try_into().unwrap(),
            references: 64.try_into().unwrap(),
            logical_bytes: 100.try_into().unwrap(),
            exposure_bytes: 100.try_into().unwrap(),
        },
    };
    let mut model = LocalModel::new(input.application, input.limits).unwrap();
    let hex = "b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7";
    let root = std::array::from_fn(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap());
    let request = UploadRequest {
        id: 1.try_into().unwrap(),
        first_reference: 1.try_into().unwrap(),
        uploader: actor(2),
        root,
        bytes: 3,
        expires_at: 100,
    };
    model
        .authorize(actor(1), request, &[root], &[], 10)
        .unwrap();
    model.register(actor(2), root, 10).unwrap();
    Record::capture(&model, input.manager, actor(8), 0)
}

#[test]
fn corrupted_domain_records_never_publish_authority() {
    let mut bad = record();
    bad.version = 2;
    assert!(matches!(bad.restore(actor(8)), Err(OpenError::Record)));
    let mut bad = record();
    bad.references.clear();
    assert!(matches!(bad.restore(actor(8)), Err(OpenError::Record)));
    let mut bad = record();
    bad.references.push(bad.references[0]);
    assert!(matches!(bad.restore(actor(8)), Err(OpenError::Record)));
    let mut bad = record();
    bad.uploads.push(bad.uploads[0]);
    assert!(matches!(bad.restore(actor(8)), Err(OpenError::Record)));
    let mut bad = record();
    bad.uploads[0].0.state = UploadState::Attached;
    assert!(matches!(bad.restore(actor(8)), Err(OpenError::Record)));
    let mut bad = record();
    bad.references[0].2 = ReferenceStatus::Reserved;
    assert!(matches!(bad.restore(actor(8)), Err(OpenError::Record)));
    let mut bad = record();
    bad.uploads[0].0.request.expires_at = 9;
    assert!(matches!(bad.restore(actor(8)), Err(OpenError::Record)));
    let mut bad = record();
    bad.input.limits.exposure_bytes = 2.try_into().unwrap();
    assert!(matches!(bad.restore(actor(8)), Err(OpenError::Record)));
}

#[test]
fn codec_rejects_truncation_trailing_and_oversized_input() {
    let bytes = record().encode().unwrap();
    assert!(
        ic_wire::decode::<Record>(
            &bytes[..bytes.len() - 1],
            ic_wire::Encoding::Payload,
            STATE_LIMITS
        )
        .is_err()
    );
    let mut trailing = bytes;
    trailing.push(0);
    assert!(matches!(
        ic_wire::decode::<Record>(&trailing, ic_wire::Encoding::Payload, STATE_LIMITS),
        Err(ic_wire::Error::Trailing)
    ));
    assert!(matches!(
        ic_wire::decode::<Record>(
            &vec![0; STATE_LIMITS.bytes + 1],
            ic_wire::Encoding::Payload,
            STATE_LIMITS
        ),
        Err(ic_wire::Error::Limit)
    ));
}
