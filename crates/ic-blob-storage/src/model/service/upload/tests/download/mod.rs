use super::*;
use crate::model::{
    catalog::admission::read::UploadRootState,
    identity::caffeine::{CaffeineHeader, manifest::CaffeineChunkHash},
    lifecycle::LifecyclePhase,
    service::upload::{content::ContentLookup, manifest::UploadManifest},
};

mod retained;

fn query(input: UploadPermission) -> ContentLookup {
    ContentLookup {
        tenant: p(4),
        namespace: id(1),
        root: input.request.object.root,
    }
}

#[test]
fn descriptor_is_tenant_scoped_and_survives_suspension_and_cancellation() {
    let mut owner = admissions();
    let input = permission(1);
    assert_eq!(owner.content_descriptor(context(4), query(input)), Ok(None));
    owner.admit(context(4), input, 1).unwrap();
    assert_eq!(owner.content_descriptor(context(4), query(input)), Ok(None));
    prepare(&mut owner, &input);
    let original = owner
        .content_descriptor(context(4), query(input))
        .unwrap()
        .unwrap();
    let headers = original.headers.to_vec();
    assert_eq!(original.content.request, input.request);
    assert_eq!(original.content.state, UploadRootState::Reserved);
    let usage = owner.catalog().usage();
    for actor in [2, 3, 5, 6] {
        assert_eq!(
            owner.content_descriptor(context(actor), query(input)),
            Err(UploadAdmissionError::NotProject)
        );
    }
    assert_eq!(
        owner.content_descriptor(
            context(6),
            ContentLookup {
                tenant: p(6),
                ..query(input)
            }
        ),
        Ok(None)
    );
    assert_eq!(
        owner.content_descriptor(
            UploadContext {
                service: p(9),
                ..context(4)
            },
            query(input)
        ),
        Err(UploadAdmissionError::WrongService)
    );
    assert_eq!(
        owner.content_descriptor(
            context(4),
            ContentLookup {
                namespace: id(9),
                ..query(input)
            }
        ),
        Err(UploadAdmissionError::WrongNamespace)
    );
    assert_eq!(owner.catalog().usage(), usage);
    set_active(&mut owner, false);
    assert_eq!(
        owner
            .content_descriptor(context(4), query(input))
            .unwrap()
            .unwrap()
            .headers,
        headers
    );
    owner.revoke(context(4), input.request).unwrap();
    let cancelled = owner
        .content_descriptor(context(4), query(input))
        .unwrap()
        .unwrap();
    assert_eq!(cancelled.content.state, UploadRootState::Cancelled);
    assert_eq!(cancelled.headers, headers);
}

#[test]
fn preparation_owns_original_metadata_and_retries_cannot_replace_it() {
    let mut owner = admissions();
    let mut input = permission(1);
    let mut value = "application/octet-stream".to_owned();
    let headers = [
        CaffeineHeader {
            name: "Content-Type",
            value: &value,
        },
        CaffeineHeader {
            name: "Content-Length",
            value: "10",
        },
    ];
    input.request.object.root = hashes_with_headers(1, &headers).provider_root;
    let chunks = [CaffeineChunkHash::try_from(
        hashes_with_headers(1, &[])
            .provider_root
            .as_bytes()
            .as_slice(),
    )
    .unwrap()];
    owner.admit(context(4), input, 1).unwrap();
    owner
        .prepare_manifest(
            context(5),
            input.request,
            UploadManifest {
                chunks: &chunks,
                headers: &headers,
            },
            2,
        )
        .unwrap();
    let first = owner
        .content_descriptor(context(4), query(input))
        .unwrap()
        .unwrap()
        .headers
        .to_vec();
    let reverse = [headers[1], headers[0]];
    assert_eq!(
        owner.prepare_manifest(
            context(5),
            input.request,
            UploadManifest {
                chunks: &chunks,
                headers: &reverse
            },
            2
        ),
        Ok(LifecycleChange::Unchanged)
    );
    value.replace_range(.., "text/plain");
    let changed = [
        CaffeineHeader {
            name: "Content-Type",
            value: &value,
        },
        CaffeineHeader {
            name: "Content-Length",
            value: "10",
        },
    ];
    assert!(matches!(
        owner.prepare_manifest(
            context(5),
            input.request,
            UploadManifest {
                chunks: &chunks,
                headers: &changed
            },
            2
        ),
        Err(UploadAdmissionError::Manifest(_))
    ));
    let retained = owner
        .content_descriptor(context(4), query(input))
        .unwrap()
        .unwrap();
    assert_eq!(retained.headers, first);
    assert_eq!(retained.headers[0].value, "application/octet-stream");
    assert_eq!(owner.catalog().usage().reserved_bytes, 10);
}

#[test]
fn descriptor_preserves_metadata_through_separate_deletion_and_billing_states() {
    let mut owner = admissions();
    let input = permission(1);
    owner.admit(context(4), input, 1).unwrap();
    prepare(&mut owner, &input);
    let headers = owner
        .content_descriptor(context(4), query(input))
        .unwrap()
        .unwrap()
        .headers
        .to_vec();
    owner.expose(context(5), input.request, 2).unwrap();
    assert_eq!(
        owner
            .content_descriptor(context(4), query(input))
            .unwrap()
            .unwrap()
            .content
            .state,
        UploadRootState::ExposurePossible
    );
    owner.confirm_upload(input.request).unwrap();
    let root = input.request.object.root;
    for expected in [
        LifecyclePhase::Live,
        LifecyclePhase::DeletionPending,
        LifecyclePhase::ProviderDeleted,
        LifecyclePhase::Settled,
    ] {
        let view = owner
            .content_descriptor(context(4), query(input))
            .unwrap()
            .unwrap();
        assert_eq!(view.content.state, UploadRootState::Confirmed(expected));
        assert_eq!(view.headers, headers);
        match expected {
            LifecyclePhase::Live => {
                owner
                    .apply_reference(
                        context(4),
                        root,
                        ReferenceRequest {
                            id: ReferenceRequestId::new(id(1)),
                            operation: ReferenceOperation::Release(input.request.object.first),
                        },
                    )
                    .unwrap();
            }
            LifecyclePhase::DeletionPending => {
                owner
                    .confirm_provider_deleted(root, input.request.object.first.object())
                    .unwrap();
            }
            LifecyclePhase::ProviderDeleted => {
                owner
                    .confirm_billing_stopped(root, input.request.object.first.object())
                    .unwrap();
            }
            LifecyclePhase::Settled => {}
        }
    }
}
