use super::*;
use crate::model::service::upload::manifest::UploadManifest;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHeader;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineChunkHash;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineManifestError;
use ic_blob_storage_contracts::upload::metadata::UploadMetadataError;

const HEADERS: [ic_blob_storage_contracts::identity::caffeine::CaffeineHeader<'static>; 1] = [
    ic_blob_storage_contracts::identity::caffeine::CaffeineHeader {
        name: "Content-Length",
        value: "10",
    },
];

fn leaf(input: UploadPermission) -> [CaffeineChunkHash; 1] {
    let n = u8::try_from(input.request.object.first.object().identity().object.get()).unwrap();
    [CaffeineChunkHash::try_from(
        hashes_with_headers(n, &[])
            .provider_root
            .as_bytes()
            .as_slice(),
    )
    .unwrap()]
}

#[test]
fn manifest_binding_is_atomic_idempotent_and_never_provider_completion() {
    let mut owner = admissions();
    let input = permission(1);
    owner.admit(context(4), input, 10).unwrap();
    assert_eq!(
        owner.expose(context(5), input.request, 11),
        Err(UploadAdmissionError::ManifestNotPrepared)
    );
    let original = owner.lookup(context(5), input.request).unwrap();
    let chunks = leaf(input);
    let bad = [CaffeineChunkHash::try_from([0; 32].as_slice()).unwrap()];
    assert_eq!(
        owner.prepare_manifest(
            context(5),
            input.request,
            UploadManifest {
                chunks: &bad,
                headers: &HEADERS
            },
            11
        ),
        Err(UploadAdmissionError::Manifest(
            CaffeineManifestError::RootMismatch
        ))
    );
    assert_eq!(owner.lookup(context(5), input.request).unwrap(), original);
    for expected in [LifecycleChange::Changed, LifecycleChange::Unchanged] {
        assert_eq!(
            owner.prepare_manifest(
                context(5),
                input.request,
                UploadManifest {
                    chunks: &chunks,
                    headers: &HEADERS
                },
                11
            ),
            Ok(expected)
        );
    }
    assert_eq!(
        owner.lookup(context(5), input.request).unwrap().manifest,
        UploadManifestState::Bound
    );
    assert_eq!(
        owner.catalog().phase(p(4), input.request),
        Ok(UploadPhase::Reserved)
    );
    assert_eq!(
        owner.confirm_upload(input.request),
        Err(UploadError::InvalidPhase(UploadPhase::Reserved))
    );
    owner.expose(context(5), input.request, 12).unwrap();
    assert_eq!(
        owner.catalog().phase(p(4), input.request),
        Ok(UploadPhase::ExposurePossible)
    );
    assert!(
        owner
            .catalog()
            .confirmed()
            .get(input.request.object.root)
            .is_none()
    );
    owner.revoke(context(4), input.request).unwrap();
    assert_eq!(owner.catalog().usage().reserved_bytes, 10);
}

#[test]
fn manifest_calls_recheck_uploader_time_and_activation() {
    let mut owner = admissions();
    let input = permission(1);
    owner.admit(context(4), input, 10).unwrap();
    let chunks = leaf(input);
    let declaration = UploadManifest {
        chunks: &chunks,
        headers: &HEADERS,
    };
    for actor in [2, 3, 4, 6] {
        assert_eq!(
            owner.prepare_manifest(context(actor), input.request, declaration, 11),
            Err(UploadAdmissionError::NotUploader)
        );
    }
    for (time, error) in [
        (9, UploadAdmissionError::ClockReversed),
        (100, UploadAdmissionError::Expired),
    ] {
        assert_eq!(
            owner.prepare_manifest(context(5), input.request, declaration, time),
            Err(error)
        );
    }
    set_active(&mut owner, false);
    assert_eq!(
        owner.prepare_manifest(context(5), input.request, declaration, 11),
        Err(TenantError::Suspended.into())
    );
    set_active(&mut owner, true);
    assert_eq!(
        owner.prepare_manifest(context(5), input.request, declaration, 11),
        Err(TenantError::StalePermission.into())
    );
    assert_eq!(
        owner.lookup(context(5), input.request).unwrap().manifest,
        UploadManifestState::Unprepared
    );
    owner.revoke(context(4), input.request).unwrap();
    assert_eq!(
        owner.prepare_manifest(context(5), input.request, declaration, 11),
        Err(UploadAdmissionError::Revoked)
    );
}

#[test]
fn declared_length_is_not_a_verified_content_claim() {
    let mut owner = admissions();
    let false_metadata = [CaffeineHeader {
        name: "Content-Length",
        value: "1",
    }];
    let mut input = permission(1);
    // Ten actual bytes, but a root that includes the false one-byte declaration.
    input.request.object.root = hashes_with_headers(1, &false_metadata).provider_root;
    input.request.object.bytes = 1;
    owner.admit(context(4), input, 10).unwrap();
    let chunks = leaf(input);
    owner
        .prepare_manifest(
            context(5),
            input.request,
            UploadManifest {
                chunks: &chunks,
                headers: &false_metadata,
            },
            11,
        )
        .unwrap();
    // Exactly why the host must establish pre-charge provider size enforcement
    // before real issuance: consistent declarations cannot prove actual bytes.
    let view = owner.lookup(context(5), input.request).unwrap();
    assert_eq!(view.manifest, UploadManifestState::Bound);
    assert_eq!(view.phase, UploadPhase::Reserved);
    assert_eq!(owner.catalog().usage().reserved_bytes, 1);
    assert!(
        owner
            .catalog()
            .confirmed()
            .get(input.request.object.root)
            .is_none()
    );
}

fn invalid_metadata(huge: &str) -> Vec<(Vec<CaffeineHeader<'_>>, UploadMetadataError)> {
    use UploadMetadataError::DuplicateHeader;
    use UploadMetadataError::HeaderBytes;
    use UploadMetadataError::HeaderCount;
    use UploadMetadataError::HeaderName;
    use UploadMetadataError::HeaderValue;
    use UploadMetadataError::LengthMismatch;
    use UploadMetadataError::LengthRequired;
    use UploadMetadataError::NonCanonicalLength;
    let mut cases = vec![(vec![], LengthRequired)];
    for value in [
        "",
        "010",
        "+10",
        "-10",
        "10,10",
        "10.0",
        "18446744073709551616",
    ] {
        cases.push((
            vec![CaffeineHeader {
                name: "Content-Length",
                value,
            }],
            NonCanonicalLength,
        ));
    }
    cases.push((
        vec![CaffeineHeader {
            name: "content-length",
            value: "10",
        }],
        NonCanonicalLength,
    ));
    cases.push((
        vec![CaffeineHeader {
            name: "Content-Length",
            value: "9",
        }],
        LengthMismatch,
    ));
    for name in ["", "Bad Name", "Bad:Name", "Náme", "X\r\nHeader"] {
        cases.push((
            vec![HEADERS[0], CaffeineHeader { name, value: "ok" }],
            HeaderName,
        ));
    }
    for value in [
        " 10",
        "10 ",
        "10\r\nInjected: yes",
        "10\t",
        "\u{feff}10",
        "x\u{2028}y",
        "x\u{7f}y",
    ] {
        cases.push((
            vec![CaffeineHeader {
                name: "Content-Length",
                value,
            }],
            HeaderValue,
        ));
    }
    cases.push((
        vec![
            HEADERS[0],
            CaffeineHeader {
                name: "content-length",
                value: "10",
            },
        ],
        DuplicateHeader,
    ));
    cases.push((
        vec![
            HEADERS[0],
            CaffeineHeader {
                name: "X-Test",
                value: "a",
            },
            CaffeineHeader {
                name: "x-test",
                value: "b",
            },
        ],
        DuplicateHeader,
    ));
    cases.push((vec![HEADERS[0]; 9], HeaderCount));
    cases.push((
        vec![
            HEADERS[0],
            CaffeineHeader {
                name: "X",
                value: huge,
            },
        ],
        HeaderBytes,
    ));

    cases
}

#[test]
fn malformed_metadata_cannot_replace_a_bound_manifest_or_change_accounting() {
    let mut owner = admissions();
    let input = permission(1);
    owner.admit(context(4), input, 10).unwrap();
    let chunks = leaf(input);
    let huge = "x".repeat(1024);
    let cases = invalid_metadata(&huge);
    for bound in [false, true] {
        if bound {
            prepare(&mut owner, &input);
        }
        let original = owner.lookup(context(5), input.request).unwrap();
        let usage = owner.catalog().usage();
        for (headers, error) in &cases {
            let manifest = UploadManifest {
                chunks: &chunks,
                headers,
            };
            assert_eq!(
                owner.prepare_manifest(context(6), input.request, manifest, 11),
                Err(UploadAdmissionError::NotUploader)
            );
            assert_eq!(
                owner.prepare_manifest(context(5), input.request, manifest, 11),
                Err(UploadAdmissionError::Metadata(*error))
            );
            assert_eq!(owner.lookup(context(5), input.request).unwrap(), original);
            assert_eq!(owner.catalog().usage(), usage);
        }
    }
    owner.expose(context(5), input.request, 12).unwrap();
    assert_eq!(
        owner.catalog().phase(p(4), input.request),
        Ok(UploadPhase::ExposurePossible)
    );
}
