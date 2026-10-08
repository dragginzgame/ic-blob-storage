//! Production-host indexed discovery, exact independent identities and restore fencing.
use super::*;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryFailure as F;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryRequest;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryResponse;
use ic_blob_storage_contracts::protocol::UPLOAD_DISCOVERY_METHOD;
impl Fixture {
    fn discovery(
        &self,
        actor: Principal,
        request: UploadDiscoveryRequest,
    ) -> Result<UploadDiscoveryResponse, F> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, UPLOAD_DISCOVERY_METHOD, (request,))
            .unwrap()
    }
}
#[test]
fn standalone_discovery_authenticates_even_unknown_roots_and_bounds_ingress() {
    let f = Fixture::new();
    let request = UploadDiscoveryRequest {
        scope: f.scope(),
        root: [9; 32],
    };
    let before = f.harness.pic.get_stable_memory(f.service);
    let expected = UploadDiscoveryResponse {
        request,
        content: None,
        fenced: false,
    };
    assert_eq!(f.discovery(f.tenant, request), Ok(expected));
    let replicated: Result<UploadDiscoveryResponse, F> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, UPLOAD_DISCOVERY_METHOD, (request,))
        .unwrap();
    assert_eq!(replicated, Ok(expected));
    for actor in [f.operator, f.controller, f.uploader, Principal::anonymous()] {
        assert_eq!(f.discovery(actor, request), Err(F::Denied));
    }
    for scope in [
        TenantScope {
            service: f.operator,
            ..request.scope
        },
        TenantScope {
            namespace: 1,
            ..request.scope
        },
    ] {
        assert_eq!(
            f.discovery(f.tenant, UploadDiscoveryRequest { scope, ..request }),
            Err(F::Binding)
        );
    }
    assert_eq!(
        f.discovery(
            f.tenant,
            UploadDiscoveryRequest {
                scope: TenantScope {
                    namespace: 0,
                    ..request.scope
                },
                ..request
            }
        ),
        Err(F::Invalid)
    );
    for bytes in [b"DIDL".to_vec(), vec![0; 4097]] {
        let failure = f
            .harness
            .pic
            .query_call(f.service, f.tenant, UPLOAD_DISCOVERY_METHOD, bytes)
            .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
    }
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}

#[test]
fn standalone_discovery_keeps_original_identity_and_exposes_restore_fence() {
    let f = Fixture::new();
    let enrollment = f.enroll(f.operator).unwrap();
    let mut permission = f.manifest().permission;
    permission.upload.upload = u128::MAX;
    permission.upload.object = u128::MAX - 1;
    permission.upload.incarnation = u128::MAX - 2;
    permission.upload.first_reference = u128::MAX - 3;
    let admitted: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_admit_upload", (permission,))
        .unwrap();
    admitted.unwrap();
    let request = UploadDiscoveryRequest {
        scope: f.scope(),
        root: permission.upload.root,
    };
    let expected = UploadDiscoveryResponse {
        request,
        content: Some(
            ic_blob_storage_contracts::dto::upload::history::UploadHistoryEntry {
                request: permission.upload,
                state:
                    ic_blob_storage_contracts::dto::upload::history::UploadContentState::Reserved,
            },
        ),
        fenced: false,
    };
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(f.discovery(f.tenant, request), Ok(expected));
    let replicated: Result<UploadDiscoveryResponse, F> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, UPLOAD_DISCOVERY_METHOD, (request,))
        .unwrap();
    assert_eq!(replicated, Ok(expected));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    let suspended: Result<TenantEnrollmentResponse, TenantFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.operator,
            "blob_update_tenant",
            (TenantUpdateRequest {
                scope: request.scope,
                expected: enrollment.enrollment,
                active: false,
            },),
        )
        .unwrap();
    suspended.unwrap();
    assert_eq!(f.discovery(f.tenant, request), Ok(expected));
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        f.discovery(f.tenant, request),
        Ok(UploadDiscoveryResponse {
            fenced: true,
            ..expected
        })
    );
    assert_eq!(f.discovery(f.operator, request), Err(F::Denied));
    let absent = UploadDiscoveryRequest {
        root: [9; 32],
        ..request
    };
    assert_eq!(
        f.discovery(f.tenant, absent),
        Ok(UploadDiscoveryResponse {
            request: absent,
            content: None,
            fenced: true,
        })
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &restored);
}
