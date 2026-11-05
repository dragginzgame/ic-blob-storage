//! Scoped operation recovery through the actual standalone endpoint.
use super::*;
use ic_blob_storage_contracts::dto::upload::history::UploadContentState;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryCursor;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryEntry;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryFailure;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryFilter;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryPage;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryRequest;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryScope;
use ic_blob_storage_contracts::protocol::UPLOAD_HISTORY_METHOD;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;

impl Fixture {
    async fn history(
        &self,
        actor: Principal,
        input: UploadHistoryRequest,
    ) -> Result<UploadHistoryPage, UploadHistoryFailure> {
        self.harness
            .pic
            .query_blob_as(
                self.service,
                actor,
                UPLOAD_HISTORY_METHOD,
                input,
                REPLY_BYTES,
            )
            .await
            .unwrap()
    }
    fn history_request(&self) -> UploadHistoryRequest {
        UploadHistoryRequest {
            service: self.service.into(),
            namespace: self.config.namespace,
            scope: UploadHistoryScope::Tenant(self.tenant.into()),
            filter: UploadHistoryFilter::All,
            cursor: None,
        }
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "Keep full-width histories and suspended/restored observations on the same installed owner"
)]
async fn standalone_history_preserves_distinct_full_width_ids_and_fenced_suspended_inspection() {
    let f = Fixture::new().await;
    let enrollment = f.enroll(f.operator).await.unwrap();
    let mut permission = f.manifest().permission;
    permission.upload.upload = u128::MAX;
    permission.upload.object = u128::MAX - 1;
    permission.upload.incarnation = u128::MAX - 2;
    permission.upload.first_reference = u128::MAX - 3;
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
    let request = f.history_request();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let mut expected = UploadHistoryPage {
        request,
        entries: vec![UploadHistoryEntry {
            request: permission.upload,
            state: UploadContentState::Reserved,
        }],
        next: None,
        scanned: 1,
        fenced: false,
    };
    assert_eq!(f.history(f.tenant, request).await, Ok(expected.clone()));
    let replicated: Result<UploadHistoryPage, UploadHistoryFailure> = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            f.tenant,
            UPLOAD_HISTORY_METHOD,
            request,
            REPLY_BYTES,
        )
        .await
        .unwrap();
    assert_eq!(replicated, Ok(expected.clone()));
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    let revoked: Result<UploadRevocationResponse, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            f.tenant,
            "blob_revoke_upload",
            permission,
            REPLY_BYTES,
        )
        .await
        .unwrap();
    revoked.unwrap();
    expected.entries[0].state = UploadContentState::Cancelled;
    let suspended: Result<TenantEnrollmentResponse, TenantFailure> = f
        .harness
        .pic
        .update_tenant_as(
            f.service,
            f.operator,
            TenantUpdateRequest {
                scope: f.scope(),
                expected: enrollment.enrollment,
                active: false,
            },
            REPLY_BYTES,
        )
        .await
        .unwrap();
    suspended.unwrap();
    assert_eq!(f.history(f.tenant, request).await, Ok(expected.clone()));
    let active = UploadHistoryRequest {
        filter: UploadHistoryFilter::Active,
        ..request
    };
    let page = f.history(f.tenant, active).await.unwrap();
    assert_eq!(page.request, active);
    assert_eq!(page.entries, []);
    assert_eq!(page.scanned, 1);
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    expected.fenced = true;
    let restored = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(f.history(f.tenant, request).await, Ok(expected.clone()));
    let operator = UploadHistoryRequest {
        scope: UploadHistoryScope::Service,
        ..request
    };
    expected.request = operator;
    assert_eq!(f.history(f.operator, operator).await, Ok(expected));
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &restored,
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_history_rejects_scope_and_cursor_errors_even_without_records() {
    let f = Fixture::new().await;
    let request = f.history_request();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    for actor in [f.uploader, f.operator, f.controller, Principal::anonymous()] {
        assert_eq!(
            f.history(actor, request).await,
            Err(UploadHistoryFailure::Denied)
        );
    }
    let operator = UploadHistoryRequest {
        scope: UploadHistoryScope::Service,
        ..request
    };
    for actor in [f.tenant, f.uploader, f.controller, Principal::anonymous()] {
        assert_eq!(
            f.history(actor, operator).await,
            Err(UploadHistoryFailure::Denied)
        );
    }
    for invalid in [
        UploadHistoryRequest {
            namespace: 1,
            ..request
        },
        UploadHistoryRequest {
            service: f.operator.into(),
            ..request
        },
    ] {
        assert_eq!(
            f.history(f.tenant, invalid).await,
            Err(UploadHistoryFailure::Binding)
        );
    }
    assert_eq!(
        f.history(
            f.tenant,
            UploadHistoryRequest {
                namespace: 0,
                ..request
            }
        )
        .await,
        Err(UploadHistoryFailure::Invalid)
    );
    let cursor = UploadHistoryCursor {
        service: f.service.into(),
        namespace: request.namespace,
        scope: request.scope,
        filter: request.filter,
        after_tenant: f.tenant.into(),
        after_request: 0,
    };
    assert_eq!(
        f.history(
            f.tenant,
            UploadHistoryRequest {
                cursor: Some(cursor),
                ..request
            }
        )
        .await,
        Err(UploadHistoryFailure::Invalid)
    );
    assert_eq!(
        f.history(
            f.tenant,
            UploadHistoryRequest {
                cursor: Some(UploadHistoryCursor {
                    after_request: u128::MAX,
                    after_tenant: f.operator.into(),
                    ..cursor
                }),
                ..request
            }
        )
        .await,
        Err(UploadHistoryFailure::CursorScope)
    );
    let page = f.history(f.tenant, request).await.unwrap();
    assert_eq!(page.entries, []);
    assert_eq!(page.scanned, 0);
    assert_eq!(page.next, None);
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
}
