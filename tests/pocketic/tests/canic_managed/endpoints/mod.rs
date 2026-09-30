//! Managed local admission, bounded declarations and passive occupied restoration.
use super::{Fixture, installation::enroll};
use blob_canic_probe::PreparationForwardFailure;
use candid::Principal;
use ic_blob_storage::{
    dto::{
        configuration::{HostConfigurationView, HostFailure},
        reference::ReferenceUpload,
        upload::{
            admission::{
                UploadAdmissionFailure, UploadAdmissionMutation, UploadAdmissionRequest,
                UploadAdmissionResponse,
            },
            capacity::{UploadCapacityFailure, UploadCapacityResponse},
            manifest::{
                UploadManifestDeclaration, UploadManifestFailure, UploadManifestHeader,
                UploadManifestMutation, UploadManifestRequest, UploadManifestResponse,
            },
        },
    },
    model::identity::caffeine::{
        CaffeineHashLimits, CaffeineHeader, manifest::builder::CaffeineManifestBuilder,
    },
};
use ic_testkit::pic::CandidCallExt;
use std::time::Duration;

fn configuration(f: &Fixture, actor: Principal) -> Result<HostConfigurationView, HostFailure> {
    f.pic()
        .query_candid_as(f.app(), actor, "blob_configuration", ())
        .unwrap()
}
pub(super) fn manifest(
    f: &Fixture,
    tenant: Principal,
    uploader: Principal,
) -> UploadManifestRequest {
    let headers = [CaffeineHeader {
        name: "Content-Length",
        value: "10",
    }];
    let mut builder = CaffeineManifestBuilder::new(
        10,
        &headers,
        CaffeineHashLimits {
            max_content_bytes: 10.try_into().unwrap(),
            max_append_bytes: 10.try_into().unwrap(),
            max_headers: 8.try_into().unwrap(),
            max_header_bytes: 1024.try_into().unwrap(),
        },
        2.try_into().unwrap(),
    )
    .unwrap();
    builder.append(0, &[42; 10]).unwrap();
    let prepared = builder.finish().unwrap();
    UploadManifestRequest {
        permission: UploadAdmissionRequest {
            upload: ReferenceUpload {
                service: f.app(),
                tenant,
                namespace: u128::MAX,
                upload: u128::MAX,
                object: u128::MAX - 1,
                incarnation: u128::MAX - 2,
                first_reference: u128::MAX - 3,
                root: *prepared.hashes().provider_root.as_bytes(),
                bytes: 10,
            },
            uploader,
            expires_at_ns: u64::MAX,
        },
        declaration: UploadManifestDeclaration {
            chunks: prepared
                .manifest()
                .chunks()
                .iter()
                .map(|chunk| *chunk.as_bytes())
                .collect(),
            headers: vec![UploadManifestHeader {
                name: "Content-Length".into(),
                value: "10".into(),
            }],
        },
    }
}
fn admission(
    f: &Fixture,
    actor: Principal,
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, UploadAdmissionFailure> {
    f.pic()
        .query_candid_as(f.app(), actor, "blob_upload_admission", (input,))
        .unwrap()
}
fn prepare(
    f: &Fixture,
    actor: Principal,
    input: &UploadManifestRequest,
) -> Result<UploadManifestMutation, UploadManifestFailure> {
    f.pic()
        .update_candid_as(f.app(), actor, "blob_prepare_upload", (input,))
        .unwrap()
}
fn retained_manifest(
    f: &Fixture,
    actor: Principal,
    input: UploadAdmissionRequest,
) -> Result<UploadManifestResponse, UploadManifestFailure> {
    f.pic()
        .query_candid_as(f.app(), actor, "blob_upload_manifest", (input,))
        .unwrap()
}

fn restored_owner_fences(f: &Fixture) {
    let snapshot: blob_canic_probe::CompositionSnapshot =
        f.pic().query_candid(f.app(), "probe_snapshot", ()).unwrap();
    assert_eq!(snapshot.fences, [true; 4]);
}

fn authority(
    f: &Fixture,
    operator: Principal,
    tenant: Principal,
    uploader: Principal,
    outsider: Principal,
) -> (HostConfigurationView, UploadManifestRequest) {
    let installed = configuration(f, operator).unwrap();
    assert_eq!(installed.configuration.service, f.app());
    assert_eq!(installed.release, "01".repeat(32));
    assert!(!installed.fenced);
    let before = f.pic().get_stable_memory(f.app());
    let mut denied = vec![tenant, uploader, outsider, f.root(), Principal::anonymous()];
    denied.extend(
        f.pic()
            .canister_status(f.app(), Some(f.root()))
            .unwrap()
            .settings
            .controllers,
    );
    denied.sort_unstable();
    denied.dedup();
    for actor in denied {
        assert_eq!(configuration(f, actor), Err(HostFailure::Denied));
    }
    let input = manifest(f, tenant, uploader);
    for actor in [operator, uploader, outsider, f.root()] {
        let refusal: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
            .pic()
            .update_candid_as(f.app(), actor, "blob_admit_upload", (input.permission,))
            .unwrap();
        assert_eq!(refusal, Err(UploadAdmissionFailure::Denied));
    }
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
    (installed, input)
}

#[test]
fn managed_blob_admission_and_manifest_match_shared_authority_and_survive_fenced_upgrade() {
    let f = Fixture::new();
    let operator = Principal::from_slice(&[2, 1]);
    let uploader = Principal::from_slice(&[6, 1]);
    let outsider = Principal::from_slice(&[7, 1]);
    let (scope, _) = enroll(&f);
    let (installed, input) = authority(&f, operator, scope.tenant, uploader, outsider);
    let admitted: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .pic()
        .update_candid_as(
            f.app(),
            scope.tenant,
            "blob_admit_upload",
            (input.permission,),
        )
        .unwrap();
    let admitted = admitted.unwrap();
    assert!(!admitted.replayed);
    assert_eq!(
        admission(&f, scope.tenant, input.permission),
        Ok(admitted.admission)
    );
    assert_eq!(
        prepare(&f, scope.tenant, &input),
        Err(UploadManifestFailure::Permission(
            UploadAdmissionFailure::Denied
        ))
    );
    let prepared = prepare(&f, uploader, &input).unwrap();
    assert!(prepared.changed);
    assert!(!prepare(&f, uploader, &input).unwrap().changed);
    assert_eq!(
        retained_manifest(&f, scope.tenant, input.permission),
        Ok(prepared.observation.clone())
    );
    assert_eq!(
        retained_manifest(&f, uploader, input.permission),
        Ok(prepared.observation.clone())
    );
    assert_eq!(
        retained_manifest(&f, outsider, input.permission),
        Err(UploadManifestFailure::Permission(
            UploadAdmissionFailure::Denied
        ))
    );
    let capacity: Result<UploadCapacityResponse, UploadCapacityFailure> = f
        .pic()
        .query_candid_as(f.app(), scope.tenant, "blob_upload_capacity", (scope,))
        .unwrap();
    let capacity = capacity.unwrap();
    assert_eq!(capacity.remaining_objects, 1);
    assert_eq!(capacity.remaining_active_uploads, 1);
    assert_eq!(capacity.remaining_manifest_chunks, 1);
    assert!(!capacity.fenced);
    bounded_manifest(&f, operator, uploader, &input);
    f.upgrade_same_release(Duration::from_secs(5));
    restored_owner_fences(&f);
    assert_eq!(
        configuration(&f, operator),
        Ok(HostConfigurationView {
            fenced: true,
            ..installed
        })
    );
    let before = f.pic().get_stable_memory(f.app());
    assert_eq!(
        admission(&f, scope.tenant, input.permission),
        Ok(admitted.admission)
    );
    assert_eq!(
        retained_manifest(&f, uploader, input.permission),
        Ok(prepared.observation)
    );
    assert_eq!(
        prepare(&f, uploader, &input),
        Err(UploadManifestFailure::Permission(
            UploadAdmissionFailure::Fenced
        ))
    );
    let replay: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .pic()
        .update_candid_as(
            f.app(),
            scope.tenant,
            "blob_admit_upload",
            (input.permission,),
        )
        .unwrap();
    assert_eq!(replay, Err(UploadAdmissionFailure::Fenced));
    let restored_capacity: Result<UploadCapacityResponse, UploadCapacityFailure> = f
        .pic()
        .query_candid_as(f.app(), scope.tenant, "blob_upload_capacity", (scope,))
        .unwrap();
    assert_eq!(
        restored_capacity,
        Ok(UploadCapacityResponse {
            fenced: true,
            ..capacity
        })
    );
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
}

fn bounded_manifest(
    f: &Fixture,
    operator: Principal,
    uploader: Principal,
    input: &UploadManifestRequest,
) {
    let before = f.pic().get_stable_memory(f.app());
    let mut invalid = input.clone();
    invalid.declaration.headers.push(UploadManifestHeader {
        name: "Content-Type".into(),
        value: "v".repeat(32 * 1024),
    });
    let bytes = candid::encode_one(&invalid).unwrap();
    assert!(bytes.len() > 16 * 1024 && bytes.len() < 131_072);
    // This passes the explicit managed transport bound and reaches the shared semantic limit.
    assert_eq!(
        prepare(f, uploader, &invalid),
        Err(UploadManifestFailure::Limit)
    );
    let forwarded: Result<UploadManifestMutation, PreparationForwardFailure> = f
        .pic()
        .update_candid_as(f.app(), operator, "probe_forward_prepare", (bytes,))
        .unwrap();
    assert_eq!(
        forwarded,
        Err(PreparationForwardFailure::Manifest(
            UploadManifestFailure::Permission(UploadAdmissionFailure::Denied)
        ))
    );
    // Keep valid Candid at the exact transport boundary so refusal cannot be
    // explained by malformed bytes or Canic's smaller default ingress limit.
    let current = candid::encode_one(&invalid).unwrap().len();
    let length = invalid.declaration.headers.last().unwrap().value.len() + 131_072 - current;
    invalid.declaration.headers.last_mut().unwrap().value = "v".repeat(length);
    let bytes = candid::encode_one(&invalid).unwrap();
    assert_eq!(bytes.len(), 131_072);
    assert_eq!(
        prepare(f, uploader, &invalid),
        Err(UploadManifestFailure::Limit)
    );
    invalid
        .declaration
        .headers
        .last_mut()
        .unwrap()
        .value
        .push('v');
    let bytes = candid::encode_one(&invalid).unwrap();
    assert_eq!(bytes.len(), 131_073);
    let rejected = f
        .pic()
        .update_call(f.app(), uploader, "blob_prepare_upload", bytes.clone())
        .unwrap_err();
    assert_eq!(
        rejected.reject_code,
        ic_testkit::pocket_ic::RejectCode::CanisterReject
    );
    let forwarded: Result<UploadManifestMutation, PreparationForwardFailure> = f
        .pic()
        .update_candid_as(f.app(), operator, "probe_forward_prepare", (bytes,))
        .unwrap();
    assert_eq!(forwarded, Err(PreparationForwardFailure::Transport));
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
}
