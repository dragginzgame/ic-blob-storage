//! Actual standalone quota observations, independent of admission and restore authority.
use super::*;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityFailure as F;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityResponse;
use ic_blob_storage_contracts::protocol::UPLOAD_CAPACITY_METHOD;
impl Fixture {
    pub(super) fn capacity(
        &self,
        actor: Principal,
        scope: TenantScope,
    ) -> Result<UploadCapacityResponse, F> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, UPLOAD_CAPACITY_METHOD, (scope,))
            .unwrap()
    }
}
#[test]
fn standalone_capacity_is_tenant_scoped_and_bounded_without_reserving_quota() {
    let f = Fixture::new();
    let scope = f.scope();
    assert_eq!(f.capacity(f.tenant, scope), Err(F::NotEnrolled));
    let enrollment = f.enroll(f.operator).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.operator, f.controller, f.uploader, Principal::anonymous()] {
        assert_eq!(f.capacity(actor, scope), Err(F::Denied));
    }
    for scope in [
        TenantScope {
            service: f.operator,
            ..scope
        },
        TenantScope {
            namespace: 1,
            ..scope
        },
    ] {
        assert_eq!(f.capacity(f.tenant, scope), Err(F::Binding));
    }
    assert_eq!(
        f.capacity(
            f.tenant,
            TenantScope {
                namespace: 0,
                ..scope
            }
        ),
        Err(F::Invalid)
    );
    let observed = f.capacity(f.tenant, scope).unwrap();
    assert_eq!(observed.scope, scope);
    assert_eq!(observed.enrollment, enrollment.enrollment.unwrap());
    assert_eq!(
        observed.max_object_bytes,
        f.config.resources.max_object_bytes
    );
    assert_eq!(
        observed.max_headers,
        u64::from(f.config.resources.max_headers)
    );
    assert_eq!(
        observed.max_header_bytes,
        u64::from(f.config.resources.max_header_bytes)
    );
    assert_eq!(
        observed.remaining_bytes,
        f.config.resources.max_physical_bytes
    );
    assert_eq!(observed.remaining_objects, 2);
    assert_eq!(
        observed.remaining_logical_bytes,
        f.config.resources.max_tenant_logical_bytes
    );
    assert_eq!(
        observed.remaining_physical_bytes,
        f.config.resources.max_physical_bytes
    );
    assert_eq!(
        observed.remaining_liability_bytes,
        f.config.resources.max_liability_bytes
    );
    assert_eq!(observed.remaining_active_uploads, 2);
    assert_eq!(observed.remaining_manifest_chunks, 20);
    assert!(!observed.fenced);
    let replicated: Result<UploadCapacityResponse, F> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, UPLOAD_CAPACITY_METHOD, (scope,))
        .unwrap();
    assert_eq!(replicated, Ok(observed));
    for bytes in [b"DIDL".to_vec(), vec![0; 4097]] {
        let rejected = f
            .harness
            .pic
            .query_call(f.service, f.tenant, UPLOAD_CAPACITY_METHOD, bytes)
            .unwrap_err();
        assert_eq!(rejected.reject_code, RejectCode::CanisterError);
    }
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}

#[test]
fn standalone_capacity_tracks_shared_contention_cleanup_and_fenced_suspended_history() {
    let f = Fixture::new();
    let enrollment = f.enroll(f.operator).unwrap();
    let scope = f.scope();
    let initial = f.capacity(f.tenant, scope).unwrap();
    let permission = f.manifest().permission;
    let other_tenant = f.uploader;
    let other_scope = TenantScope {
        tenant: other_tenant,
        ..scope
    };
    let enrolled: Result<TenantEnrollmentResponse, TenantFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.operator,
            "blob_update_tenant",
            (TenantUpdateRequest {
                scope: other_scope,
                expected: None,
                active: true,
            },),
        )
        .unwrap();
    enrolled.unwrap();
    let other_permission = UploadAdmissionRequest {
        upload: ReferenceUpload {
            tenant: other_tenant,
            root: [8; 32],
            ..permission.upload
        },
        ..permission
    };
    for permission in [permission, other_permission] {
        let admitted: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
            .harness
            .pic
            .update_candid_as(
                f.service,
                permission.upload.tenant,
                "blob_admit_upload",
                (permission,),
            )
            .unwrap();
        admitted.unwrap();
    }
    let full = f.capacity(f.tenant, scope).unwrap();
    assert_eq!(full.remaining_objects, 0);
    assert_eq!(full.remaining_active_uploads, 0);
    assert_eq!(full.remaining_bytes, 0);
    assert_eq!(full.remaining_manifest_chunks, 0);
    let suspended: Result<TenantEnrollmentResponse, TenantFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.operator,
            "blob_update_tenant",
            (TenantUpdateRequest {
                scope,
                expected: enrollment.enrollment,
                active: false,
            },),
        )
        .unwrap();
    suspended.unwrap();
    let revoked: Result<UploadRevocationResponse, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_revoke_upload", (permission,))
        .unwrap();
    revoked.unwrap();
    let cancelled = f.capacity(f.tenant, scope).unwrap();
    assert!(!cancelled.enrollment.active);
    assert_eq!(cancelled.remaining_objects, 0);
    assert_eq!(cancelled.remaining_manifest_chunks, 0);
    assert_eq!(cancelled.remaining_active_uploads, 1);
    assert_eq!(
        cancelled.remaining_bytes,
        initial.remaining_bytes - u128::from(other_permission.upload.bytes)
    );
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        f.capacity(f.tenant, scope),
        Ok(UploadCapacityResponse {
            fenced: true,
            ..cancelled
        })
    );
    assert!(f.capacity(other_tenant, other_scope).unwrap().fenced);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &restored);
}
