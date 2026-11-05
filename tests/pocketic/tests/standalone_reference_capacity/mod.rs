//! Production-host absence and authority; confirmed lifecycle coverage uses labelled fixtures.
use super::*;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityFailure as F;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityRequest;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityResponse;
use ic_blob_storage_contracts::protocol::REFERENCE_CAPACITY_METHOD;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
impl Fixture {
    async fn reference_capacity(
        &self,
        actor: Principal,
        request: ReferenceCapacityRequest,
    ) -> Result<ReferenceCapacityResponse, F> {
        self.harness
            .pic
            .query_blob_as(
                self.service,
                actor,
                REFERENCE_CAPACITY_METHOD,
                request,
                REPLY_BYTES,
            )
            .await
            .unwrap()
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_reference_capacity_authenticates_even_unknown_roots_and_bounds_ingress() {
    let f = Fixture::new().await;
    let request = ReferenceCapacityRequest {
        scope: f.scope(),
        root: [9; 32],
    };
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let expected = ReferenceCapacityResponse {
        request,
        headroom: None,
        fenced: false,
    };
    assert_eq!(f.reference_capacity(f.tenant, request).await, Ok(expected));
    let replicated: Result<ReferenceCapacityResponse, F> = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            f.tenant,
            REFERENCE_CAPACITY_METHOD,
            request,
            REPLY_BYTES,
        )
        .await
        .unwrap();
    assert_eq!(replicated, Ok(expected));
    for actor in [f.operator, f.controller, f.uploader, Principal::anonymous()] {
        assert_eq!(f.reference_capacity(actor, request).await, Err(F::Denied));
    }
    for scope in [
        TenantScope {
            service: f.operator.into(),
            ..request.scope
        },
        TenantScope {
            namespace: 1,
            ..request.scope
        },
    ] {
        assert_eq!(
            f.reference_capacity(f.tenant, ReferenceCapacityRequest { scope, ..request })
                .await,
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
        )
        .await,
        Err(F::Invalid)
    );
    for bytes in [b"DIDL".to_vec(), vec![0; 4097]] {
        let failure = f
            .harness
            .pic
            .query_call(f.service, f.tenant, REFERENCE_CAPACITY_METHOD, bytes)
            .await
            .unwrap_err();
        assert_eq!(
            crate::canister_rejection(&failure).reject_code,
            RejectCode::CanisterError
        );
    }
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_reference_capacity_keeps_unconfirmed_absence_and_exposes_restore_fence() {
    let f = Fixture::new().await;
    let enrollment = f.enroll(f.operator).await.unwrap();
    let permission = f.manifest().permission;
    let admitted: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            f.tenant,
            "blob_admit_upload",
            permission,
            REPLY_BYTES,
        )
        .await
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
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(f.reference_capacity(f.tenant, request).await, Ok(expected));
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    let suspended: Result<TenantEnrollmentResponse, TenantFailure> = f
        .harness
        .pic
        .update_tenant_as(
            f.service,
            f.operator,
            TenantUpdateRequest {
                scope: request.scope,
                expected: enrollment.enrollment,
                active: false,
            },
            REPLY_BYTES,
        )
        .await
        .unwrap();
    suspended.unwrap();
    assert_eq!(f.reference_capacity(f.tenant, request).await, Ok(expected));
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        f.reference_capacity(f.tenant, request).await,
        Ok(ReferenceCapacityResponse {
            fenced: true,
            ..expected
        })
    );
    assert_eq!(
        f.reference_capacity(f.operator, request).await,
        Err(F::Denied)
    );
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &restored,
    );
}
