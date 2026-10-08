use super::*;
use crate::workflow::uploads::certificate;
use crate::workflow::uploads::certificate::UploadCertificateFailure as F;
use crate::workflow::uploads::exposure;
use exposure_boundary::{evidence, prepared};
use ic_blob_storage_contracts::dto::upload::UploadState;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure as A;
use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure as E;
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

#[test]
fn certificate_assessment_reports_exact_permission_and_all_missing_facts_without_mutation() {
    use ic_blob_storage_contracts::dto::upload::certificate::UploadCertificateAssessmentResponse;
    use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureBlocker as C;
    let m = memory();
    let store = prepared(clone_memory(&m));
    let mut host = evidence();
    let root = host.permission.request.object.root.to_string();
    let before = m.permissions.borrow().clone();
    host.observed_at_ns = 1;
    host.namespace_binding = false;
    host.trusted_uploader = false;
    host.current_owner = false;
    host.durable_commit = false;
    let result = certificate::inspect(&store, context(5), &root, host, 2).unwrap();
    assert_eq!(result.permission, manifest_boundary::input().permission);
    assert_eq!(result.assessed_at_ns, 2);
    assert_eq!(
        result.blockers,
        vec![
            C::StaleObservation,
            C::NamespaceBinding,
            C::TrustedUploader,
            C::CurrentOwner,
            C::Durability
        ]
    );
    assert_eq!(
        candid::decode_one::<UploadCertificateAssessmentResponse>(
            &candid::encode_one(&result).unwrap()
        )
        .unwrap(),
        result
    );
    for actor in [1, 2, 4, 6] {
        assert_eq!(
            certificate::inspect(&store, context(actor), &root, host, 2),
            Err(E::Permission(A::Denied))
        );
    }
    host.permission.expires_at_ns += 1;
    assert_eq!(
        certificate::inspect(&store, context(5), &root, host, 2),
        Err(E::EvidenceBinding)
    );
    assert_eq!(*m.permissions.borrow(), before);
}

#[test]
fn successful_certificate_assessment_never_reserves_or_survives_permission_changes() {
    let m = memory();
    let mut store = prepared(clone_memory(&m));
    let host = evidence();
    let root = host.permission.request.object.root.to_string();
    let before = m.permissions.borrow().clone();
    assert_eq!(
        certificate::inspect(&store, context(5), &root, host, 2)
            .unwrap()
            .blockers,
        []
    );
    assert_eq!(*m.permissions.borrow(), before);
    assert_eq!(
        exposure::inspect(&store, context(5), manifest_boundary::input().permission)
            .unwrap()
            .state,
        UploadState::Reserved
    );
    store.revoke(context(4), host.permission.request).unwrap();
    assert_eq!(
        certificate::inspect(&store, context(5), &root, host, 2),
        Err(E::Revoked)
    );
    assert_eq!(
        certificate::issue(&mut store, context(5), &root, host, 2),
        Err(F::Exposure(E::Revoked))
    );
}
