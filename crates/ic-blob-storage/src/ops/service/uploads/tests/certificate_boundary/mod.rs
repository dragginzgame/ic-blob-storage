use super::*;
use crate::{
    dto::upload::{
        UploadState, admission::UploadAdmissionFailure as A, exposure::UploadExposureFailure as E,
    },
    policy::upload::exposure::UploadExposureBlocker as B,
    workflow::uploads::{
        certificate::{self, UploadCertificateFailure as F},
        exposure,
    },
};
use exposure_boundary::{evidence, prepared};
#[derive(candid::CandidType, candid::Deserialize)]
struct ProviderReply {
    method: String,
    blob_hash: String,
}

#[test]
fn certificate_root_resolves_exact_original_permission_and_commits_before_plain_reply() {
    let m = memory();
    let mut store = prepared(clone_memory(&m));
    let yes = evidence();
    let root = yes.permission.request.object.root.to_string();
    assert_eq!(
        certificate::resolve(&store, context(5), &root, 2),
        Ok(yes.permission)
    );
    let before = store.usage().unwrap();
    let reply = certificate::issue(&mut store, context(5), &root, yes, 2).unwrap();
    assert_eq!(store.usage().unwrap(), before);
    let input = manifest_boundary::input().permission;
    assert_eq!(
        exposure::inspect(&store, context(5), input).unwrap().state,
        UploadState::ExposurePossible
    );
    // Decode independently as the reviewed provider record, without a Result envelope.
    let decoded: ProviderReply = candid::decode_one(&candid::encode_one(&reply).unwrap()).unwrap();
    assert_eq!(decoded.method, "upload");
    assert_eq!(decoded.blob_hash, root);
    assert_eq!(
        certificate::issue(&mut store, context(5), &root, yes, 2),
        Err(F::Exposure(E::Phase))
    );
    drop(store);
    let mut restored = StableUploads::open(m, config()).unwrap();
    assert_eq!(
        certificate::issue(&mut restored, context(5), &root, yes, 2),
        Err(F::Exposure(E::Permission(A::Fenced)))
    );
    assert_eq!(
        exposure::inspect(&restored, context(5), input)
            .unwrap()
            .state,
        UploadState::ExposurePossible
    );
}

#[test]
fn certificate_root_is_a_locator_and_never_caller_or_evidence_authority() {
    let m = memory();
    let mut store = prepared(clone_memory(&m));
    let yes = evidence();
    let root = yes.permission.request.object.root.to_string();
    let before = m.permissions.borrow().clone();
    for actor in [1, 2, 4, 6] {
        assert_eq!(
            certificate::resolve(&store, context(actor), &root, 2),
            Err(E::Permission(A::Denied))
        );
    }
    assert_eq!(
        certificate::resolve(
            &store,
            context(5),
            &format!("sha256:{}", "ff".repeat(32)),
            2
        ),
        Err(E::Permission(A::Denied))
    );
    for root in [String::new(), "sha256:00".to_owned(), "x".repeat(1 << 20)] {
        assert_eq!(
            certificate::resolve(&store, context(5), &root, 2),
            Err(E::Permission(A::Invalid))
        );
    }
    let wrong_service = UploadContext {
        service: p(9),
        ..context(5)
    };
    assert_eq!(
        certificate::resolve(&store, wrong_service, &root, 2),
        Err(E::Permission(A::Binding))
    );
    let mut foreign = yes;
    foreign.permission.expires_at_ns += 1;
    assert_eq!(
        certificate::issue(&mut store, context(5), &root, foreign, 2),
        Err(F::Exposure(E::EvidenceBinding))
    );
    let blocked = certificate::issue(
        &mut store,
        context(5),
        &root,
        crate::policy::upload::exposure::UploadExposureHostEvidence {
            precharge_limits: false,
            ..yes
        },
        2,
    )
    .unwrap_err();
    assert!(matches!(blocked, F::Blocked(a) if a.blockers == [B::PrechargeLimits]));
    assert_eq!(*m.permissions.borrow(), before);
    assert_eq!(
        certificate::resolve(&store, context(5), &root, 100),
        Err(E::Permission(A::Expired))
    );
    store.revoke(context(4), yes.permission.request).unwrap();
    assert_eq!(
        certificate::issue(&mut store, context(5), &root, yes, 2),
        Err(F::Exposure(E::Revoked))
    );
}

#[test]
fn certificate_resolution_rejects_corrupt_root_request_indexes_without_writes() {
    for replacement in [None, Some(7)] {
        let m = memory();
        let mut store = prepared(clone_memory(&m));
        let root = evidence().permission.request.object.root;
        if let Some(id) = replacement {
            store.root_requests.insert(*root.as_bytes(), id);
        } else {
            store.root_requests.remove(root.as_bytes());
        }
        let before = m.permissions.borrow().clone();
        assert_eq!(
            certificate::resolve(&store, context(5), &root.to_string(), 2),
            Err(E::Permission(A::Internal))
        );
        assert_eq!(*m.permissions.borrow(), before);
    }
}
