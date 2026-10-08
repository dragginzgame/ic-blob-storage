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

impl Fixture {
    fn history(
        &self,
        actor: Principal,
        input: UploadHistoryRequest,
    ) -> Result<UploadHistoryPage, UploadHistoryFailure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, UPLOAD_HISTORY_METHOD, (input,))
            .unwrap()
    }
    fn history_request(&self) -> UploadHistoryRequest {
        UploadHistoryRequest {
            service: self.service,
            namespace: self.config.namespace,
            scope: UploadHistoryScope::Tenant(self.tenant),
            filter: UploadHistoryFilter::All,
            cursor: None,
        }
    }
}
#[test]
fn standalone_history_preserves_distinct_full_width_ids_and_fenced_suspended_inspection() {
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
    let request = f.history_request();
    let before = f.harness.pic.get_stable_memory(f.service);
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
    assert_eq!(f.history(f.tenant, request), Ok(expected.clone()));
    let replicated: Result<UploadHistoryPage, UploadHistoryFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, UPLOAD_HISTORY_METHOD, (request,))
        .unwrap();
    assert_eq!(replicated, Ok(expected.clone()));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    let revoked: Result<UploadRevocationResponse, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_revoke_upload", (permission,))
        .unwrap();
    revoked.unwrap();
    expected.entries[0].state = UploadContentState::Cancelled;
    let suspended: Result<TenantEnrollmentResponse, TenantFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.operator,
            "blob_update_tenant",
            (TenantUpdateRequest {
                scope: f.scope(),
                expected: enrollment.enrollment,
                active: false,
            },),
        )
        .unwrap();
    suspended.unwrap();
    assert_eq!(f.history(f.tenant, request), Ok(expected.clone()));
    let active = UploadHistoryRequest {
        filter: UploadHistoryFilter::Active,
        ..request
    };
    let page = f.history(f.tenant, active).unwrap();
    assert_eq!(page.request, active);
    assert_eq!(page.entries, []);
    assert_eq!(page.scanned, 1);
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    expected.fenced = true;
    let restored = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(f.history(f.tenant, request), Ok(expected.clone()));
    let operator = UploadHistoryRequest {
        scope: UploadHistoryScope::Service,
        ..request
    };
    expected.request = operator;
    assert_eq!(f.history(f.operator, operator), Ok(expected));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &restored);
}

#[test]
fn standalone_history_rejects_scope_and_cursor_errors_even_without_records() {
    let f = Fixture::new();
    let request = f.history_request();
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.uploader, f.operator, f.controller, Principal::anonymous()] {
        assert_eq!(f.history(actor, request), Err(UploadHistoryFailure::Denied));
    }
    let operator = UploadHistoryRequest {
        scope: UploadHistoryScope::Service,
        ..request
    };
    for actor in [f.tenant, f.uploader, f.controller, Principal::anonymous()] {
        assert_eq!(
            f.history(actor, operator),
            Err(UploadHistoryFailure::Denied)
        );
    }
    for invalid in [
        UploadHistoryRequest {
            namespace: 1,
            ..request
        },
        UploadHistoryRequest {
            service: f.operator,
            ..request
        },
    ] {
        assert_eq!(
            f.history(f.tenant, invalid),
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
        ),
        Err(UploadHistoryFailure::Invalid)
    );
    let cursor = UploadHistoryCursor {
        service: f.service,
        namespace: request.namespace,
        scope: request.scope,
        filter: request.filter,
        after_tenant: f.tenant,
        after_request: 0,
    };
    assert_eq!(
        f.history(
            f.tenant,
            UploadHistoryRequest {
                cursor: Some(cursor),
                ..request
            }
        ),
        Err(UploadHistoryFailure::Invalid)
    );
    assert_eq!(
        f.history(
            f.tenant,
            UploadHistoryRequest {
                cursor: Some(UploadHistoryCursor {
                    after_request: u128::MAX,
                    after_tenant: f.operator,
                    ..cursor
                }),
                ..request
            }
        ),
        Err(UploadHistoryFailure::CursorScope)
    );
    let page = f.history(f.tenant, request).unwrap();
    assert_eq!(page.entries, []);
    assert_eq!(page.scanned, 0);
    assert_eq!(page.next, None);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}
