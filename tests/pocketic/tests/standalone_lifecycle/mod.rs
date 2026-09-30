//! Repeated current-release host restoration preserves all owners and never unfences.
use super::*;
use ic_blob_storage::dto::{
    account::{
        AccountInspectionFailure, AccountInspectionKind, AccountInspectionRequest,
        AccountInspectionResponse,
    },
    gateway::{GatewayRevocationFailure, GatewayRevocationRequest, GatewayRevocationResponse},
    operator::LocalServiceStatus,
};
use ic_blob_storage::ops::service::gateways::revocation::GATEWAY_REVOCATION_METHOD;

fn fences(status: &LocalServiceStatus, expected: bool) {
    assert_eq!(status.uploads.fenced, expected);
    assert_eq!(status.funding.fenced, expected);
    assert_eq!(status.gateways.fenced, expected);
    assert_eq!(status.reads.fenced, expected);
}
fn normalized(mut status: LocalServiceStatus) -> LocalServiceStatus {
    status.uploads.fenced = false;
    status.funding.fenced = false;
    status.gateways.fenced = false;
    status.reads.fenced = false;
    status
}
fn refuses(f: &Fixture, manifest: &UploadManifestRequest) {
    assert_eq!(
        f.prepare(f.uploader, manifest),
        Err(UploadManifestFailure::Permission(
            UploadAdmissionFailure::Fenced
        ))
    );
    let removed: Result<GatewayRevocationResponse, GatewayRevocationFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.operator,
            GATEWAY_REVOCATION_METHOD,
            (GatewayRevocationRequest {
                scope: f.operator_scope(),
                gateway: Fake::principal(9),
            },),
        )
        .unwrap();
    assert_eq!(removed, Err(GatewayRevocationFailure::Fenced));
    let account: Result<AccountInspectionResponse, AccountInspectionFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.operator,
            "blob_inspect_account",
            (AccountInspectionRequest {
                scope: f.operator_scope(),
                kind: AccountInspectionKind::Balance,
            },),
        )
        .unwrap();
    assert_eq!(account, Err(AccountInspectionFailure::Fenced));
}

#[test]
fn stop_start_and_repeated_upgrade_preserve_installation_and_all_owner_fences() {
    let f = Fixture::new();
    f.enroll(f.operator).unwrap();
    let manifest = f.manifest();
    f.harness
        .pic
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (manifest.permission,),
        )
        .unwrap()
        .unwrap();
    let prepared = f.prepare(f.uploader, &manifest).unwrap().observation;
    let installation = f.configuration(f.operator).unwrap();
    let admission = f.admission(manifest.permission);
    let status = f.local_status(f.operator, f.operator_scope()).unwrap();
    fences(&status, false);
    assert_eq!(
        status.uploads.reserved_bytes,
        u128::from(manifest.permission.upload.bytes)
    );
    let before = f.harness.pic.get_stable_memory(f.service);
    f.harness
        .pic
        .stop_canister(f.service, Some(f.controller))
        .unwrap();
    f.harness
        .pic
        .start_canister(f.service, Some(f.controller))
        .unwrap();
    assert_eq!(f.configuration(f.operator).unwrap(), installation);
    assert_eq!(
        f.local_status(f.operator, f.operator_scope()).unwrap(),
        status
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    // Supplied replacement settings must not publish a partially restored host.
    assert_eq!(
        f.upgrade(super::installation(&f.config))
            .unwrap_err()
            .reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(f.configuration(f.operator).unwrap(), installation);
    assert_eq!(
        f.local_status(f.operator, f.operator_scope()).unwrap(),
        status
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    for _ in 0..2 {
        f.upgrade(candid::encode_args(()).unwrap()).unwrap();
        let before = f.harness.pic.get_stable_memory(f.service);
        let mut expected = installation.clone();
        expected.fenced = true;
        assert_eq!(f.configuration(f.operator).unwrap(), expected);
        let restored = f.local_status(f.operator, f.operator_scope()).unwrap();
        fences(&restored, true);
        assert_eq!(normalized(restored), status);
        assert_eq!(f.admission(manifest.permission), admission);
        let retained: Result<UploadManifestResponse, UploadManifestFailure> = f
            .harness
            .pic
            .query_candid_as(
                f.service,
                f.uploader,
                "blob_upload_manifest",
                (manifest.permission,),
            )
            .unwrap();
        assert_eq!(retained.unwrap(), prepared);
        refuses(&f, &manifest);
        unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    }
}
