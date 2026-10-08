use super::*;
use crate::workflow::uploads::admission::admit;
use crate::workflow::uploads::manifests::inspect;
use crate::workflow::uploads::manifests::prepare;
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure as A;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::manifest::*;
use ic_blob_storage_contracts::upload::manifests::reply;
use ic_blob_storage_contracts::upload::manifests::reply::UploadManifestReplyError as E;
use ic_blob_storage_contracts::upload::manifests::reply::UploadManifestReplyLimits;
use std::num::NonZeroUsize;

pub(super) fn input() -> UploadManifestRequest {
    let built = built();
    UploadManifestRequest {
        permission: UploadAdmissionRequest {
            upload: ReferenceUpload {
                service: p(1),
                tenant: p(4),
                namespace: 1,
                upload: u128::MAX,
                object: u128::MAX - 1,
                incarnation: 7,
                first_reference: 9,
                root: *built.hashes().provider_root.as_bytes(),
                bytes: 10,
            },
            uploader: p(5),
            expires_at_ns: 100,
        },
        declaration: UploadManifestDeclaration {
            chunks: built
                .manifest()
                .chunks()
                .iter()
                .map(|c| *c.as_bytes())
                .collect(),
            headers: HEADERS
                .iter()
                .map(|h| UploadManifestHeader {
                    name: h.name.into(),
                    value: h.value.into(),
                })
                .collect(),
        },
    }
}
fn limits() -> UploadManifestReplyLimits {
    UploadManifestReplyLimits {
        max_reply_bytes: NonZeroUsize::new(4096).unwrap(),
        declaration: config().manifest_limits(),
    }
}
fn observation(input: &UploadManifestRequest) -> UploadManifestResponse {
    UploadManifestResponse {
        permission: input.permission,
        manifest: UploadManifestInspection::Prepared(input.declaration.clone()),
    }
}
fn encoded(response: UploadManifestResponse) -> Vec<u8> {
    candid::encode_one(Ok::<_, UploadManifestFailure>(response)).unwrap()
}
#[test]
fn manifest_boundary_preserves_original_declaration_and_exact_permission_through_retirement_and_restore()
 {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    enroll(&mut store);
    let input = input();
    admit(&mut store, context(4), input.permission, 1).unwrap();
    assert_eq!(
        inspect(&store, context(5), input.permission)
            .unwrap()
            .manifest,
        UploadManifestInspection::Unprepared
    );
    for actor in [2, 4, 6] {
        assert_eq!(
            prepare(&mut store, context(actor), &input, 2),
            Err(UploadManifestFailure::Permission(A::Denied))
        );
    }
    for actor in [2, 6] {
        assert_eq!(
            inspect(&store, context(actor), input.permission),
            Err(UploadManifestFailure::Permission(A::Denied))
        );
    }
    let mut changed = input.clone();
    changed.permission.expires_at_ns += 1;
    assert_eq!(
        prepare(&mut store, context(5), &changed, 2),
        Err(UploadManifestFailure::Permission(A::Conflict))
    );
    changed.permission = input.permission;
    changed.permission.uploader = p(6);
    assert_eq!(
        prepare(&mut store, context(6), &changed, 2),
        Err(UploadManifestFailure::Permission(A::Denied))
    );
    assert_eq!(
        inspect(&store, context(4), changed.permission),
        Err(UploadManifestFailure::Permission(A::Conflict))
    );
    let first = prepare(&mut store, context(5), &input, 2).unwrap();
    assert!(first.changed);
    assert_eq!(first.observation, observation(&input));
    let usage = store.usage().unwrap();
    let mut equivalent = input.clone();
    equivalent.declaration.headers.reverse();
    let replay = prepare(&mut store, context(5), &equivalent, 3).unwrap();
    assert!(!replay.changed);
    assert_eq!(replay.observation, first.observation);
    assert_eq!(store.usage().unwrap(), usage);
    assert_eq!(
        prepare(&mut store, context(5), &input, 100),
        Err(UploadManifestFailure::Permission(A::Expired))
    );
    let request = ic_blob_storage_contracts::upload::admission::parse(context(4), input.permission)
        .unwrap()
        .request;
    store.revoke(context(4), request).unwrap();
    assert_eq!(
        prepare(&mut store, context(5), &input, 3),
        Err(UploadManifestFailure::Revoked)
    );
    for actor in [4, 5] {
        assert_eq!(
            inspect(&store, context(actor), input.permission),
            Ok(first.observation.clone())
        );
    }
    drop(store);
    let mut restored = StableUploads::open(m, config()).unwrap();
    assert_eq!(
        inspect(&restored, context(5), input.permission),
        Ok(first.observation)
    );
    assert_eq!(
        prepare(&mut restored, context(5), &input, 3),
        Err(UploadManifestFailure::Permission(A::Fenced))
    );
}
#[test]
fn manifest_boundary_rejects_raw_overruns_and_inconsistent_declarations_without_binding() {
    let mut store = StableUploads::install(memory(), config()).unwrap();
    enroll(&mut store);
    let input = input();
    admit(&mut store, context(4), input.permission, 1).unwrap();
    let usage = store.usage().unwrap();
    let mut too_many = input.clone();
    too_many.declaration.chunks = vec![[0; 32]; 5];
    let mut too_large = input.clone();
    too_large.declaration.headers[1].value = "x".repeat(1024);
    let mut too_many_headers = input.clone();
    too_many_headers.declaration.headers = vec![input.declaration.headers[0].clone(); 9];
    for changed in [too_many, too_large, too_many_headers] {
        assert_eq!(
            prepare(&mut store, context(5), &changed, 2),
            Err(UploadManifestFailure::Limit)
        );
    }
    let mut leaf = input.clone();
    leaf.declaration.chunks[0][0] ^= 1;
    let mut length = input.clone();
    length.declaration.headers[0].value = "11".into();
    let mut duplicate = input.clone();
    duplicate
        .declaration
        .headers
        .push(input.declaration.headers[1].clone());
    let mut case = input.clone();
    case.declaration.headers[1].name = "content-type".into();
    for changed in [leaf, length, duplicate, case] {
        assert_eq!(
            prepare(&mut store, context(5), &changed, 2),
            Err(UploadManifestFailure::Declaration)
        );
    }
    assert_eq!(
        inspect(&store, context(4), input.permission)
            .unwrap()
            .manifest,
        UploadManifestInspection::Unprepared
    );
    assert_eq!(store.usage().unwrap(), usage);
}
#[test]
fn manifest_reply_bounds_identity_consistency_and_explicit_preparation() {
    let input = input();
    let response = observation(&input);
    let bytes = encoded(response.clone());
    assert_eq!(
        reply::inspection(input.permission, &bytes, limits()),
        Ok(response.clone())
    );
    assert_eq!(
        reply::inspection(
            input.permission,
            &bytes,
            UploadManifestReplyLimits {
                max_reply_bytes: NonZeroUsize::MIN,
                ..limits()
            }
        ),
        Err(E::Limit)
    );
    assert_eq!(
        reply::inspection(input.permission, b"bad", limits()),
        Err(E::Invalid)
    );
    let mut changed = response.clone();
    changed.permission.expires_at_ns += 1;
    assert_eq!(
        reply::inspection(input.permission, &encoded(changed), limits()),
        Err(E::Binding)
    );
    let mut wrong = input.clone();
    wrong.declaration.chunks[0][0] ^= 1;
    assert_eq!(
        reply::inspection(input.permission, &encoded(observation(&wrong)), limits()),
        Err(E::Invalid)
    );
    wrong = input.clone();
    wrong.declaration.headers[0].value = "11".into();
    assert_eq!(
        reply::inspection(input.permission, &encoded(observation(&wrong)), limits()),
        Err(E::Invalid)
    );
    wrong = input.clone();
    wrong.declaration.chunks.resize(5, [0; 32]);
    assert_eq!(
        reply::inspection(input.permission, &encoded(observation(&wrong)), limits()),
        Err(E::Limit)
    );
    let mutation = UploadManifestMutation {
        observation: response,
        changed: true,
    };
    let bytes = candid::encode_one(Ok::<_, UploadManifestFailure>(mutation.clone())).unwrap();
    let mut equivalent = input.clone();
    equivalent.declaration.headers.reverse();
    assert_eq!(reply::mutation(&equivalent, &bytes, limits()), Ok(mutation));
    let absent = UploadManifestResponse {
        permission: input.permission,
        manifest: UploadManifestInspection::Unprepared,
    };
    assert_eq!(
        reply::inspection(input.permission, &encoded(absent.clone()), limits()),
        Ok(absent.clone())
    );
    let bytes = candid::encode_one(Ok::<_, UploadManifestFailure>(UploadManifestMutation {
        observation: absent,
        changed: false,
    }))
    .unwrap();
    assert_eq!(reply::mutation(&input, &bytes, limits()), Err(E::Invalid));
    let bytes = candid::encode_one(Err::<UploadManifestResponse, _>(
        UploadManifestFailure::Revoked,
    ))
    .unwrap();
    assert_eq!(
        reply::inspection(input.permission, &bytes, limits()),
        Err(E::Remote(UploadManifestFailure::Revoked))
    );
}

#[test]
fn manifest_reply_rejects_incompatible_prepared_payload_instead_of_reporting_absence() {
    #[derive(candid::CandidType)]
    enum MalformedInspection {
        Prepared(u64),
    }
    #[derive(candid::CandidType)]
    struct MalformedResponse {
        permission: UploadAdmissionRequest,
        manifest: MalformedInspection,
    }
    let input = input();
    let bytes = candid::encode_one(Ok::<_, UploadManifestFailure>(MalformedResponse {
        permission: input.permission,
        manifest: MalformedInspection::Prepared(1),
    }))
    .unwrap();
    assert_eq!(
        reply::inspection(input.permission, &bytes, limits()),
        Err(E::Invalid)
    );
}
