//! Production-host absence and authority; confirmed lifecycle coverage uses labelled fixtures.
use super::*;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityFailure as F;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityRequest;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityResponse;
use ic_blob_storage_contracts::protocol::REFERENCE_CAPACITY_METHOD;
impl Fixture {
    fn reference_capacity(
        &self,
        actor: Principal,
        request: ReferenceCapacityRequest,
    ) -> Result<ReferenceCapacityResponse, F> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, REFERENCE_CAPACITY_METHOD, (request,))
            .unwrap()
    }
}
#[test]
fn standalone_reference_capacity_authenticates_even_unknown_roots_and_bounds_ingress() {
    let f = Fixture::new();
    let request = ReferenceCapacityRequest {
        scope: f.scope(),
        root: [9; 32],
    };
    let before = f.harness.pic.get_stable_memory(f.service);
    let expected = ReferenceCapacityResponse {
        request,
        headroom: None,
        fenced: false,
    };
    assert_eq!(f.reference_capacity(f.tenant, request), Ok(expected));
    let replicated: Result<ReferenceCapacityResponse, F> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, REFERENCE_CAPACITY_METHOD, (request,))
        .unwrap();
    assert_eq!(replicated, Ok(expected));
    for actor in [f.operator, f.controller, f.uploader, Principal::anonymous()] {
        assert_eq!(f.reference_capacity(actor, request), Err(F::Denied));
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
            f.reference_capacity(f.tenant, ReferenceCapacityRequest { scope, ..request }),
            Err(F::Binding)
        );
    }
    assert_eq!(
        f.reference_capacity(
            f.tenant,
            ReferenceCapacityRequest {
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
            .query_call(f.service, f.tenant, REFERENCE_CAPACITY_METHOD, bytes)
            .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
    }
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}

#[test]
fn standalone_reference_capacity_keeps_unconfirmed_absence_and_exposes_restore_fence() {
    let f = Fixture::new();
    let enrollment = f.enroll(f.operator).unwrap();
    let permission = f.manifest().permission;
    let admitted: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_admit_upload", (permission,))
        .unwrap();
    admitted.unwrap();
    let request = ReferenceCapacityRequest {
        scope: f.scope(),
        root: permission.upload.root,
    };
    let expected = ReferenceCapacityResponse {
        request,
        headroom: None,
        fenced: false,
    };
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(f.reference_capacity(f.tenant, request), Ok(expected));
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
    assert_eq!(f.reference_capacity(f.tenant, request), Ok(expected));
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        f.reference_capacity(f.tenant, request),
        Ok(ReferenceCapacityResponse {
            fenced: true,
            ..expected
        })
    );
    assert_eq!(f.reference_capacity(f.operator, request), Err(F::Denied));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &restored);
}
