//! Actual managed suspension/cleanup, passive operator inspection and fenced history.
use super::{Fixture, endpoints::manifest, installation::enroll};
use candid::Principal;
use ic_blob_storage::dto::{
    funding::{
        FundingHistoryFailure, FundingHistoryPage, FundingHistoryRequest,
        outcome::{FundingOutcomeFailure, FundingOutcomeRequest, FundingOutcomeResponse},
    },
    operator::{LocalServiceStatus, LocalStatusFailure, OperatorScope},
    reference::{
        ReferenceAction, ReferenceCommand, ReferenceFailure, ReferenceMutationResponse,
        ReferenceReceiptLookup,
        capacity::{ReferenceCapacityFailure, ReferenceCapacityRequest, ReferenceCapacityResponse},
        status::{ReferenceStatusRequest, ReferenceStatusResponse},
    },
    tenant::{TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest},
    upload::{
        UploadState, UploadStatusFailure, UploadStatusResponse,
        admission::{
            UploadAdmissionFailure, UploadAdmissionMutation, UploadAdmissionRequest,
            UploadRevocationResponse,
        },
        discovery::{UploadDiscoveryFailure, UploadDiscoveryRequest, UploadDiscoveryResponse},
        history::{
            UploadContentState, UploadHistoryCursor, UploadHistoryEntry, UploadHistoryFailure,
            UploadHistoryFilter, UploadHistoryPage, UploadHistoryRequest, UploadHistoryScope,
        },
    },
};
use ic_testkit::pic::CandidCallExt;
use std::time::Duration;

fn operator_scope(f: &Fixture) -> OperatorScope {
    let input = blob_canic_probe::configuration::input();
    OperatorScope {
        service: f.app(),
        namespace: input.namespace,
        cashier: input.billing.cashier,
        payment_account: input.payment_account,
    }
}
fn status(
    f: &Fixture,
    actor: Principal,
    scope: OperatorScope,
) -> Result<LocalServiceStatus, LocalStatusFailure> {
    f.pic()
        .query_candid_as(f.app(), actor, "blob_local_status", (scope,))
        .unwrap()
}
fn upload_status(
    f: &Fixture,
    actor: Principal,
    permission: UploadAdmissionRequest,
) -> Result<UploadStatusResponse, UploadStatusFailure> {
    f.pic()
        .query_candid_as(f.app(), actor, "blob_upload_status", (permission.upload,))
        .unwrap()
}
fn history(
    f: &Fixture,
    actor: Principal,
    request: UploadHistoryRequest,
) -> Result<UploadHistoryPage, UploadHistoryFailure> {
    f.pic()
        .query_candid_as(f.app(), actor, "blob_upload_history", (request,))
        .unwrap()
}
fn discovery(
    f: &Fixture,
    actor: Principal,
    request: UploadDiscoveryRequest,
) -> Result<UploadDiscoveryResponse, UploadDiscoveryFailure> {
    f.pic()
        .query_candid_as(f.app(), actor, "blob_lookup_content", (request,))
        .unwrap()
}

fn history_scope(f: &Fixture, tenant: Principal) -> UploadHistoryRequest {
    UploadHistoryRequest {
        service: f.app(),
        namespace: u128::MAX,
        scope: UploadHistoryScope::Tenant(tenant),
        filter: UploadHistoryFilter::All,
        cursor: None,
    }
}

fn history_and_reference_checks(
    f: &Fixture,
    tenant_scope: TenantScope,
    permission: UploadAdmissionRequest,
    state: UploadContentState,
    fenced: bool,
) {
    let before = f.pic().get_stable_memory(f.app());
    let request = history_scope(f, tenant_scope.tenant);
    let page = history(f, tenant_scope.tenant, request).unwrap();
    let entry = UploadHistoryEntry {
        request: permission.upload,
        state,
    };
    assert_eq!(page.entries, vec![entry]);
    assert_eq!(page.request, request);
    assert_eq!(page.scanned, 1);
    assert_eq!(page.fenced, fenced);
    let lookup = UploadDiscoveryRequest {
        scope: tenant_scope,
        root: permission.upload.root,
    };
    assert_eq!(
        discovery(f, tenant_scope.tenant, lookup),
        Ok(UploadDiscoveryResponse {
            request: lookup,
            content: Some(entry),
            fenced
        })
    );
    let reference = ReferenceCapacityRequest {
        scope: tenant_scope,
        root: permission.upload.root,
    };
    let capacity: Result<ReferenceCapacityResponse, ReferenceCapacityFailure> = f
        .pic()
        .query_candid_as(
            f.app(),
            tenant_scope.tenant,
            "blob_reference_capacity",
            (reference,),
        )
        .unwrap();
    assert_eq!(
        capacity,
        Ok(ReferenceCapacityResponse {
            request: reference,
            headroom: None,
            fenced
        })
    );
    unconfirmed_references(f, permission, fenced);
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
}

fn unconfirmed_references(f: &Fixture, permission: UploadAdmissionRequest, fenced: bool) {
    let command = ReferenceCommand {
        upload: permission.upload,
        reference: u128::MAX - 4,
        operation: u128::MAX - 5,
        action: ReferenceAction::Retain,
    };
    let receipt: Result<ReferenceReceiptLookup, ReferenceFailure> = f
        .pic()
        .query_candid_as(
            f.app(),
            permission.upload.tenant,
            "blob_reference_receipt",
            (command,),
        )
        .unwrap();
    assert_eq!(receipt, Err(ReferenceFailure::Unconfirmed));
    let request = ReferenceStatusRequest {
        upload: permission.upload,
        reference: command.reference,
    };
    let reference: Result<ReferenceStatusResponse, ReferenceFailure> = f
        .pic()
        .query_candid_as(
            f.app(),
            permission.upload.tenant,
            "blob_reference_status",
            (request,),
        )
        .unwrap();
    assert_eq!(reference, Err(ReferenceFailure::Unconfirmed));
    let mutation: Result<ReferenceMutationResponse, ReferenceFailure> = f
        .pic()
        .update_candid_as(
            f.app(),
            permission.upload.tenant,
            "blob_apply_reference",
            (command,),
        )
        .unwrap();
    assert_eq!(
        mutation,
        Err(if fenced {
            ReferenceFailure::Fenced
        } else {
            ReferenceFailure::Unconfirmed
        })
    );
}

fn private_observations(f: &Fixture, scope: TenantScope, permission: UploadAdmissionRequest) {
    let before = f.pic().get_stable_memory(f.app());
    let operator = Principal::from_slice(&[2, 1]);
    for actor in [
        operator,
        permission.uploader,
        Principal::from_slice(&[7, 1]),
        f.root(),
    ] {
        let revoked: Result<UploadRevocationResponse, UploadAdmissionFailure> = f
            .pic()
            .update_candid_as(f.app(), actor, "blob_revoke_upload", (permission,))
            .unwrap();
        assert_eq!(revoked, Err(UploadAdmissionFailure::Denied));
        private_references(f, actor, scope, permission);
        assert_eq!(
            upload_status(f, actor, permission),
            Err(UploadStatusFailure::Denied)
        );
        assert_eq!(
            history(f, actor, history_scope(f, scope.tenant)),
            Err(UploadHistoryFailure::Denied)
        );
        assert_eq!(
            discovery(
                f,
                actor,
                UploadDiscoveryRequest {
                    scope,
                    root: permission.upload.root
                }
            ),
            Err(UploadDiscoveryFailure::Denied)
        );
    }
    let request = history_scope(f, scope.tenant);
    let changed = UploadHistoryRequest {
        filter: UploadHistoryFilter::Active,
        cursor: Some(UploadHistoryCursor {
            service: request.service,
            namespace: request.namespace,
            scope: request.scope,
            filter: request.filter,
            after_tenant: scope.tenant,
            after_request: permission.upload.upload,
        }),
        ..request
    };
    assert_eq!(
        history(f, scope.tenant, changed),
        Err(UploadHistoryFailure::CursorScope)
    );
    assert_eq!(
        upload_status(
            f,
            scope.tenant,
            UploadAdmissionRequest {
                upload: ic_blob_storage::dto::reference::ReferenceUpload {
                    namespace: 1,
                    ..permission.upload
                },
                ..permission
            }
        ),
        Err(UploadStatusFailure::Binding)
    );
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
}

fn private_references(
    f: &Fixture,
    actor: Principal,
    scope: TenantScope,
    permission: UploadAdmissionRequest,
) {
    let command = ReferenceCommand {
        upload: permission.upload,
        reference: u128::MAX - 4,
        operation: u128::MAX - 5,
        action: ReferenceAction::Retain,
    };
    let receipt: Result<ReferenceReceiptLookup, ReferenceFailure> = f
        .pic()
        .query_candid_as(f.app(), actor, "blob_reference_receipt", (command,))
        .unwrap();
    assert_eq!(receipt, Err(ReferenceFailure::Denied));
    let request = ReferenceStatusRequest {
        upload: permission.upload,
        reference: command.reference,
    };
    let reference: Result<ReferenceStatusResponse, ReferenceFailure> = f
        .pic()
        .query_candid_as(f.app(), actor, "blob_reference_status", (request,))
        .unwrap();
    assert_eq!(reference, Err(ReferenceFailure::Denied));
    let mutation: Result<ReferenceMutationResponse, ReferenceFailure> = f
        .pic()
        .update_candid_as(f.app(), actor, "blob_apply_reference", (command,))
        .unwrap();
    assert_eq!(mutation, Err(ReferenceFailure::Denied));
    let request = ReferenceCapacityRequest {
        scope,
        root: permission.upload.root,
    };
    let capacity: Result<ReferenceCapacityResponse, ReferenceCapacityFailure> = f
        .pic()
        .query_candid_as(f.app(), actor, "blob_reference_capacity", (request,))
        .unwrap();
    assert_eq!(capacity, Err(ReferenceCapacityFailure::Denied));
}

fn passive_operator(
    f: &Fixture,
    operator: Principal,
    scope: OperatorScope,
    tenant: Principal,
    fenced: bool,
) {
    let before = f.pic().get_stable_memory(f.app());
    let current = status(f, operator, scope).unwrap();
    assert_eq!(
        [
            current.uploads.fenced,
            current.funding.fenced,
            current.gateways.fenced,
            current.reads.fenced
        ],
        [fenced; 4]
    );
    let request = FundingHistoryRequest {
        scope,
        cursor: None,
    };
    let funding: Result<FundingHistoryPage, FundingHistoryFailure> = f
        .pic()
        .query_candid_as(f.app(), operator, "blob_funding_history", (request,))
        .unwrap();
    assert_eq!(
        funding,
        Ok(FundingHistoryPage {
            request,
            entries: vec![],
            next: None,
            fenced
        })
    );
    let exact = FundingOutcomeRequest {
        scope,
        operation: u128::MAX,
        offered: u128::MAX - 1,
        target_balance: Some(u128::MAX - 2),
    };
    let outcome: Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure> = f
        .pic()
        .query_candid_as(f.app(), operator, "blob_funding_outcome", (exact,))
        .unwrap();
    assert_eq!(outcome, Ok(None));
    for actor in [tenant, Principal::from_slice(&[7, 1]), f.root()] {
        assert_eq!(status(f, actor, scope), Err(LocalStatusFailure::Denied));
        let funding: Result<FundingHistoryPage, FundingHistoryFailure> = f
            .pic()
            .query_candid_as(f.app(), actor, "blob_funding_history", (request,))
            .unwrap();
        assert_eq!(funding, Err(FundingHistoryFailure::Denied));
        let outcome: Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure> = f
            .pic()
            .query_candid_as(f.app(), actor, "blob_funding_outcome", (exact,))
            .unwrap();
        assert_eq!(outcome, Err(FundingOutcomeFailure::Denied));
    }
    assert_eq!(
        status(
            f,
            operator,
            OperatorScope {
                namespace: 1,
                ..scope
            }
        ),
        Err(LocalStatusFailure::Binding)
    );
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
}

#[test]
fn managed_suspension_cleanup_and_local_history_preserve_authority_accounting_and_restore_fences() {
    let f = Fixture::new();
    let operator = Principal::from_slice(&[2, 1]);
    let (scope, enrolled) = enroll(&f);
    let input = manifest(&f, scope.tenant, Principal::from_slice(&[6, 1]));
    let admitted: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .pic()
        .update_candid_as(
            f.app(),
            scope.tenant,
            "blob_admit_upload",
            (input.permission,),
        )
        .unwrap();
    admitted.unwrap();
    let prepared: Result<
        ic_blob_storage::dto::upload::manifest::UploadManifestMutation,
        ic_blob_storage::dto::upload::manifest::UploadManifestFailure,
    > = f
        .pic()
        .update_candid_as(
            f.app(),
            input.permission.uploader,
            "blob_prepare_upload",
            (&input,),
        )
        .unwrap();
    prepared.unwrap();
    let own = operator_scope(&f);
    let reserved = status(&f, operator, own).unwrap();
    assert_eq!(reserved.uploads.reserved_bytes, 10);
    assert_eq!(reserved.uploads.active_reservations, 1);
    private_observations(&f, scope, input.permission);
    passive_operator(&f, operator, own, scope.tenant, false);
    history_and_reference_checks(
        &f,
        scope,
        input.permission,
        UploadContentState::Reserved,
        false,
    );
    let suspended: Result<TenantEnrollmentResponse, TenantFailure> = f
        .pic()
        .update_candid_as(
            f.app(),
            operator,
            "blob_update_tenant",
            (TenantUpdateRequest {
                scope,
                expected: enrolled.enrollment,
                active: false,
            },),
        )
        .unwrap();
    let suspended = suspended.unwrap();
    let revoked: Result<UploadRevocationResponse, UploadAdmissionFailure> = f
        .pic()
        .update_candid_as(
            f.app(),
            scope.tenant,
            "blob_revoke_upload",
            (input.permission,),
        )
        .unwrap();
    assert!(revoked.unwrap().changed);
    assert_eq!(
        upload_status(&f, scope.tenant, input.permission),
        Ok(UploadStatusResponse {
            upload: input.permission.upload,
            state: UploadState::Cancelled,
            revoked: true
        })
    );
    let cleared = status(&f, operator, own).unwrap();
    assert_eq!(cleared.uploads.operations, 1);
    assert_eq!(cleared.uploads.active_reservations, 0);
    assert_eq!(
        [
            cleared.uploads.reserved_bytes,
            cleared.uploads.logical_bytes,
            cleared.uploads.physical_bytes,
            cleared.uploads.liability_bytes
        ],
        [0; 4]
    );
    history_and_reference_checks(
        &f,
        scope,
        input.permission,
        UploadContentState::Cancelled,
        false,
    );
    assert_operator_history(&f, operator, input.permission);
    restored_cleanup(&f, operator, own, suspended, input.permission);
}

fn restored_cleanup(
    f: &Fixture,
    operator: Principal,
    own: OperatorScope,
    suspended: TenantEnrollmentResponse,
    permission: UploadAdmissionRequest,
) {
    let scope = TenantScope {
        service: permission.upload.service,
        namespace: permission.upload.namespace,
        tenant: permission.upload.tenant,
    };
    f.upgrade_same_release(Duration::from_secs(5));
    let before = f.pic().get_stable_memory(f.app());
    passive_operator(f, operator, own, scope.tenant, true);
    history_and_reference_checks(f, scope, permission, UploadContentState::Cancelled, true);
    let tenant: Result<TenantEnrollmentResponse, TenantFailure> = f
        .pic()
        .query_candid_as(f.app(), scope.tenant, "blob_tenant", (scope,))
        .unwrap();
    assert_eq!(
        tenant,
        Ok(TenantEnrollmentResponse {
            fenced: true,
            ..suspended
        })
    );
    let revoked: Result<UploadRevocationResponse, UploadAdmissionFailure> = f
        .pic()
        .update_candid_as(f.app(), scope.tenant, "blob_revoke_upload", (permission,))
        .unwrap();
    assert_eq!(revoked, Err(UploadAdmissionFailure::Fenced));
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
}

fn assert_operator_history(f: &Fixture, operator: Principal, permission: UploadAdmissionRequest) {
    let request = UploadHistoryRequest {
        scope: UploadHistoryScope::Service,
        ..history_scope(f, permission.upload.tenant)
    };
    assert_eq!(
        history(f, operator, request).unwrap().entries,
        vec![UploadHistoryEntry {
            request: permission.upload,
            state: UploadContentState::Cancelled
        }]
    );
    let active = history(
        f,
        permission.upload.tenant,
        UploadHistoryRequest {
            filter: UploadHistoryFilter::Active,
            ..history_scope(f, permission.upload.tenant)
        },
    )
    .unwrap();
    assert!(active.entries.is_empty());
    assert_eq!(active.scanned, 1);
}
