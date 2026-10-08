use super::*;
mod client;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure as A;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure as F;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestInspection;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestMutation;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestRequest;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestResponse;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineManifestLimits;
use ic_blob_storage_contracts::protocol::UPLOAD_MANIFEST_INSPECT_METHOD;
use ic_blob_storage_contracts::protocol::UPLOAD_MANIFEST_PREPARE_METHOD;
use ic_blob_storage_contracts::upload::manifests::reply;
use ic_blob_storage_contracts::upload::manifests::reply::UploadManifestReplyError;
use ic_blob_storage_contracts::upload::manifests::reply::UploadManifestReplyLimits;
fn inspect(
    f: &Fixture,
    actor: Principal,
    permission: UploadAdmissionRequest,
) -> Result<UploadManifestResponse, F> {
    f.harness
        .pic
        .query_candid_as(
            f.service,
            actor,
            UPLOAD_MANIFEST_INSPECT_METHOD,
            (permission,),
        )
        .unwrap()
}
fn prepare(
    f: &Fixture,
    actor: Principal,
    input: &UploadManifestRequest,
) -> Result<UploadManifestMutation, F> {
    f.harness
        .pic
        .update_candid_as(f.service, actor, UPLOAD_MANIFEST_PREPARE_METHOD, (input,))
        .unwrap()
}
fn limits() -> UploadManifestReplyLimits {
    UploadManifestReplyLimits {
        max_reply_bytes: NonZeroUsize::new(4096).unwrap(),
        declaration: CaffeineManifestLimits {
            max_content_bytes: NonZeroU64::new(10).unwrap(),
            max_chunks: NonZeroUsize::MIN,
            max_headers: NonZeroUsize::new(8).unwrap(),
            max_header_bytes: NonZeroUsize::new(1024).unwrap(),
        },
    }
}
#[test]
fn manifest_lost_ack_recovers_original_declaration_after_exposure_revocation_and_upgrade() {
    let f = Fixture::new();
    let enrollment = f.enroll(None, true).unwrap();
    let (permission, declaration) = f.permission(u128::MAX, 7);
    let input = f.preparation_input(&declaration);
    f.admit(f.tenant, permission).unwrap();
    assert_eq!(
        inspect(&f, f.uploader, input.permission).unwrap().manifest,
        UploadManifestInspection::Unprepared
    );
    let bytes = f
        .harness
        .pic
        .update_call(
            f.service,
            f.uploader,
            UPLOAD_MANIFEST_PREPARE_METHOD,
            candid::encode_one(&input).unwrap(),
        )
        .unwrap();
    assert_eq!(
        reply::mutation(
            &input,
            &bytes,
            UploadManifestReplyLimits {
                max_reply_bytes: NonZeroUsize::MIN,
                ..limits()
            }
        ),
        Err(UploadManifestReplyError::Limit)
    );
    let expected = UploadManifestResponse {
        permission: input.permission,
        manifest: UploadManifestInspection::Prepared(input.declaration.clone()),
    };
    assert_eq!(
        inspect(&f, f.tenant, input.permission),
        Ok(expected.clone())
    );
    let accepted = reply::mutation(&input, &bytes, limits()).unwrap();
    assert!(accepted.changed);
    assert_eq!(accepted.observation, expected);
    let mut reordered = input.clone();
    reordered.declaration.headers.reverse();
    let before = f.harness.pic.get_stable_memory(f.service);
    let replay = prepare(&f, f.uploader, &reordered).unwrap();
    assert!(!replay.changed);
    assert_eq!(replay.observation, expected);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.expose(permission.request).unwrap();
    assert_eq!(prepare(&f, f.uploader, &input), Err(F::Phase));
    f.revoke(permission).unwrap();
    f.enroll(Some(enrollment), false).unwrap();
    for actor in [f.tenant, f.uploader] {
        assert_eq!(inspect(&f, actor, input.permission), Ok(expected.clone()));
    }
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.controller, f.operator, f.other] {
        assert_eq!(
            inspect(&f, actor, input.permission),
            Err(F::Permission(A::Denied))
        );
    }
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(inspect(&f, f.uploader, input.permission), Ok(expected));
    assert_eq!(
        prepare(&f, f.uploader, &input),
        Err(F::Permission(A::Fenced))
    );
}
#[test]
fn manifest_ingress_binds_actual_uploader_full_permission_and_declared_root() {
    let f = Fixture::new();
    f.enroll(None, true).unwrap();
    let (permission, declaration) = f.permission(1, 9);
    let input = f.preparation_input(&declaration);
    f.admit(f.tenant, permission).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.tenant, f.controller, f.operator, f.other] {
        assert_eq!(prepare(&f, actor, &input), Err(F::Permission(A::Denied)));
    }
    let mut changed = input.clone();
    changed.permission.expires_at_ns -= 1;
    assert_eq!(
        prepare(&f, f.uploader, &changed),
        Err(F::Permission(A::Conflict))
    );
    assert_eq!(
        inspect(&f, f.tenant, changed.permission),
        Err(F::Permission(A::Conflict))
    );
    changed = input.clone();
    changed.permission.uploader = f.other;
    assert_eq!(
        prepare(&f, f.other, &changed),
        Err(F::Permission(A::Denied))
    );
    changed = input.clone();
    changed.declaration.chunks[0][0] ^= 1;
    assert_eq!(prepare(&f, f.uploader, &changed), Err(F::Declaration));
    changed = input.clone();
    changed.declaration.headers[0].value = "11".into();
    assert_eq!(prepare(&f, f.uploader, &changed), Err(F::Declaration));
    changed = input.clone();
    changed.declaration.headers[1].value = "x".repeat(1024);
    assert_eq!(prepare(&f, f.uploader, &changed), Err(F::Limit));
    assert_eq!(
        inspect(&f, f.uploader, input.permission).unwrap().manifest,
        UploadManifestInspection::Unprepared
    );
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert!(prepare(&f, f.uploader, &input).unwrap().changed);
}
