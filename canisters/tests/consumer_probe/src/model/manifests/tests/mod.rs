use super::*;
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestHeader;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestRequest;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHashLimits;
use ic_blob_storage_contracts::identity::caffeine::manifest::builder::CaffeineManifestBuilder;
fn p(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}
fn fixture() -> (ConsumerRecord, ManifestIntent) {
    let headers = [
        CaffeineHeader {
            name: "Content-Length",
            value: "10",
        },
        CaffeineHeader {
            name: "Content-Type",
            value: "image/png",
        },
    ];
    let limits = limits();
    let mut builder = CaffeineManifestBuilder::new(
        10,
        &headers,
        CaffeineHashLimits {
            max_content_bytes: limits.max_content_bytes,
            max_append_bytes: 10.try_into().unwrap(),
            max_headers: limits.max_headers,
            max_header_bytes: limits.max_header_bytes,
        },
        limits.max_chunks,
    )
    .unwrap();
    builder.append(0, &[7; 10]).unwrap();
    let built = builder.finish().unwrap();
    let intent = ManifestIntent {
        id: 1,
        request: UploadManifestRequest {
            permission: UploadAdmissionRequest {
                upload: ReferenceUpload {
                    service: p(1),
                    tenant: p(2),
                    namespace: 1,
                    upload: u128::MAX,
                    object: u128::MAX - 1,
                    incarnation: 1,
                    first_reference: 1,
                    root: *built.hashes().provider_root.as_bytes(),
                    bytes: 10,
                },
                uploader: p(3),
                expires_at_ns: 100,
            },
            declaration: UploadManifestDeclaration {
                chunks: built
                    .manifest()
                    .chunks()
                    .iter()
                    .map(|c| *c.as_bytes())
                    .collect(),
                headers: headers
                    .iter()
                    .map(|h| UploadManifestHeader {
                        name: h.name.into(),
                        value: h.value.into(),
                    })
                    .collect(),
            },
        },
    };
    (ConsumerRecord::new(p(4), p(1), p(3)), intent)
}
#[test]
fn uploader_manifest_recovery_preserves_original_metadata_and_rejects_stale_or_foreign_outcomes() {
    let (mut r, intent) = fixture();
    r.save_manifest(&intent).unwrap();
    r.start_manifest(1).unwrap();
    let mut original = intent.request.declaration.clone();
    original.headers.reverse();
    let response = UploadManifestResponse {
        permission: intent.request.permission,
        manifest: UploadManifestInspection::Prepared(original.clone()),
    };
    r.acknowledge_manifest(1, Ok(response.clone())).unwrap();
    assert_eq!(r.manifest_view(1).unwrap().result, Some(Ok(original)));
    let before = r.manifest_view(1).unwrap();
    let absent = UploadManifestResponse {
        manifest: UploadManifestInspection::Unprepared,
        ..response.clone()
    };
    assert_eq!(r.acknowledge_manifest(1, Ok(absent)), Err(Failure::Pending));
    let mut foreign = response;
    foreign.permission.expires_at_ns += 1;
    assert_eq!(
        r.acknowledge_manifest(1, Ok(foreign)),
        Err(Failure::Conflict)
    );
    assert_eq!(
        r.acknowledge_manifest(
            1,
            Err(UploadManifestFailure::Permission(
                UploadAdmissionFailure::Inactive
            ))
        ),
        Err(Failure::Conflict)
    );
    assert_eq!(r.manifest_view(1), Ok(before));
    r.validate().unwrap();
    assert_eq!(r.start_manifest(1), Ok(false));
}
#[test]
fn uploader_manifest_restore_validation_rejects_duplicate_history_unstarted_results_and_bad_declarations()
 {
    let (mut r, intent) = fixture();
    r.save_manifest(&intent).unwrap();
    r.start_manifest(1).unwrap();
    r.acknowledge_manifest(
        1,
        Ok(UploadManifestResponse {
            permission: intent.request.permission,
            manifest: UploadManifestInspection::Prepared(intent.request.declaration),
        }),
    )
    .unwrap();
    r.validate().unwrap();
    let mut wrong = r.clone();
    wrong.manifests[0].started = false;
    assert_eq!(wrong.validate(), Err(Failure::State));
    wrong = r.clone();
    wrong.manifests.push(wrong.manifests[0].clone());
    assert_eq!(wrong.validate(), Err(Failure::Conflict));
    wrong = r.clone();
    if let Some(Ok(declaration)) = &mut wrong.manifests[0].result {
        declaration.chunks[0][0] ^= 1;
    }
    assert_eq!(wrong.validate(), Err(Failure::Invalid));
    wrong = r.clone();
    wrong.manifests[0].intent.request.declaration.headers[0].value = "x".repeat(1024);
    assert_eq!(wrong.validate(), Err(Failure::Capacity));
    wrong = r;
    wrong.manifests[0].intent.request.permission.uploader = p(8);
    assert_eq!(wrong.validate(), Err(Failure::Invalid));
}

#[test]
fn uploader_cancellation_preserves_intent_capacity_and_late_outcomes_without_reopening_dispatch() {
    for started in [false, true] {
        for refused in [false, true] {
            let (mut r, intent) = fixture();
            r.save_manifest(&intent).unwrap();
            if started {
                r.start_manifest(1).unwrap();
            }
            r.cancel_manifest(1).unwrap();
            r.cancel_manifest(1).unwrap();
            r.save_manifest(&intent).unwrap();
            assert_eq!(r.start_manifest(1), Err(Failure::State));
            let saved = r.manifest_view(1).unwrap();
            assert!(saved.cancelled);
            assert_eq!(saved.started, started);
            assert_eq!(saved.result, None);
            let outcome = if refused {
                Err(UploadManifestFailure::Revoked)
            } else {
                Ok(UploadManifestResponse {
                    permission: intent.request.permission,
                    manifest: UploadManifestInspection::Prepared(
                        intent.request.declaration.clone(),
                    ),
                })
            };
            if started {
                r.acknowledge_manifest(1, outcome).unwrap();
                assert!(r.manifest_view(1).unwrap().cancelled);
                assert!(r.manifest_view(1).unwrap().result.is_some());
            } else {
                assert_eq!(r.acknowledge_manifest(1, outcome), Err(Failure::State));
            }
            assert_eq!(r.start_manifest(1), Err(Failure::State));
            let mut changed = intent.clone();
            changed.request.permission.expires_at_ns += 1;
            assert_eq!(r.save_manifest(&changed), Err(Failure::Conflict));
            let mut second = intent.clone();
            second.id = 2;
            second.request.permission.upload.upload -= 1;
            r.save_manifest(&second).unwrap();
            r.cancel_manifest(2).unwrap();
            second.id = 3;
            second.request.permission.upload.upload -= 1;
            assert_eq!(r.save_manifest(&second), Err(Failure::Capacity));
            r.validate().unwrap();
        }
    }
}
