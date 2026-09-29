//! IC transaction rollback and same-release inspection of the durable upload owner.
#![cfg(not(target_family = "wasm"))]
mod authenticated_cli;
mod reference_cli;
mod storage_exposure;
mod storage_gateways;
mod storage_manifests;
mod storage_tenants;
mod support;
use blob_test_protocol::{
    admission::{Permission, Phase, Request, input::PreparationInput},
    journey::{JourneyManifest, JourneyUsage},
    storage::{Failure, FaultAdmission, FaultPreparation, Observation, Status, WriteFault},
};
use candid::Principal;
use ic_blob_storage::dto::tenant::{
    TenantEnrollment, TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest,
};
use ic_blob_storage::model::identity::caffeine::{
    CaffeineHashLimits, CaffeineHeader, manifest::builder::CaffeineManifestBuilder,
};
use ic_blob_storage::ops::service::uploads::tenants::{
    TENANT_INSPECTION_METHOD, TENANT_UPDATE_METHOD,
};
use ic_testkit::{
    Fake,
    pic::CandidCallExt,
    pocket_ic::{CanisterSettings, RejectCode},
};
use std::num::{NonZeroU64, NonZeroUsize};
use support::{Harness, fixture_path};
struct Fixture {
    harness: Harness,
    service: Principal,
    operator: Principal,
    controller: Principal,
    tenant: Principal,
    uploader: Principal,
    other: Principal,
}
impl Fixture {
    fn local_status(&self) -> ic_blob_storage::dto::operator::LocalServiceStatus {
        self.harness
            .pic
            .query_candid_as::<Result<_, ic_blob_storage::dto::operator::LocalStatusFailure>, _>(
                self.service,
                self.operator,
                ic_blob_storage::ops::service::operator::LOCAL_STATUS_METHOD,
                (ic_blob_storage::dto::operator::OperatorScope {
                    service: self.service,
                    namespace: 1,
                    cashier: self.operator,
                    payment_account: self.service,
                },),
            )
            .unwrap()
            .unwrap()
    }
    fn new() -> Self {
        Self::with_operator(Harness::new(), Fake::principal(1))
    }
    fn with_operator(harness: Harness, operator: Principal) -> Self {
        let controller = Fake::principal(2);
        let service = harness.pic.create_canister_with_settings(
            Some(controller),
            Some(CanisterSettings {
                controllers: Some(vec![controller]),
                ..CanisterSettings::default()
            }),
        );
        harness.pic.install_canister(
            service,
            Self::wasm(),
            candid::encode_one(operator).unwrap(),
            Some(controller),
        );
        Self {
            harness,
            service,
            operator,
            controller,
            tenant: Fake::principal(3),
            uploader: Fake::principal(4),
            other: Fake::principal(5),
        }
    }
    fn wasm() -> Vec<u8> {
        std::fs::read(fixture_path("BLOB_STORAGE_PROBE_WASM")).unwrap()
    }
    fn permission(&self, id: u128, content: u8) -> (Permission, PreparationInput) {
        let headers = [
            CaffeineHeader {
                name: "Content-Length",
                value: "10",
            },
            CaffeineHeader {
                name: "Content-Type",
                value: "image/png",
            },
        ];
        let mut builder = CaffeineManifestBuilder::new(
            10,
            &headers,
            CaffeineHashLimits {
                max_content_bytes: NonZeroU64::new(10).unwrap(),
                max_append_bytes: NonZeroUsize::new(10).unwrap(),
                max_headers: NonZeroUsize::new(8).unwrap(),
                max_header_bytes: NonZeroUsize::new(1024).unwrap(),
            },
            NonZeroUsize::MIN,
        )
        .unwrap();
        builder.append(0, &[content; 10]).unwrap();
        let built = builder.finish().unwrap();
        let request = Request {
            service: self.service,
            tenant: self.tenant,
            namespace: 1,
            id,
            root: *built.hashes().provider_root.as_bytes(),
            bytes: 10,
        };
        (
            Permission {
                request,
                uploader: self.uploader,
                expires_at_ns: u64::MAX,
            },
            PreparationInput {
                request,
                manifest: JourneyManifest {
                    chunks: built
                        .manifest()
                        .chunks()
                        .iter()
                        .map(|c| *c.as_bytes())
                        .collect(),
                    headers: headers
                        .iter()
                        .map(|h| (h.name.into(), h.value.into()))
                        .collect(),
                },
            },
        )
    }
    fn admit(&self, actor: Principal, input: Permission) -> Result<bool, Failure> {
        let result: Result<
            ic_blob_storage::dto::upload::admission::UploadAdmissionMutation,
            ic_blob_storage::dto::upload::admission::UploadAdmissionFailure,
        > = self
            .harness
            .pic
            .update_candid_as(
                self.service,
                actor,
                "blob_admit_upload",
                (admission_input(input),),
            )
            .unwrap();
        result.map(|r| !r.replayed).map_err(admission_failure)
    }
    fn prepare(&self, input: &PreparationInput) -> Result<bool, Failure> {
        let result: Result<
            ic_blob_storage::dto::upload::manifest::UploadManifestMutation,
            ic_blob_storage::dto::upload::manifest::UploadManifestFailure,
        > = self
            .harness
            .pic
            .update_candid_as(
                self.service,
                self.uploader,
                "blob_prepare_upload",
                (self.preparation_input(input),),
            )
            .unwrap();
        result.map(|r| r.changed).map_err(|e| {
            use ic_blob_storage::dto::upload::manifest::UploadManifestFailure as E;
            match e {
                E::Permission(e) => admission_failure(e),
                E::Revoked => Failure::Revoked,
                E::Phase => Failure::Phase,
                E::Declaration => Failure::Invalid,
                E::Limit => Failure::Capacity,
            }
        })
    }
    fn preparation_input(
        &self,
        input: &PreparationInput,
    ) -> ic_blob_storage::dto::upload::manifest::UploadManifestRequest {
        use ic_blob_storage::dto::upload::manifest::{
            UploadManifestDeclaration, UploadManifestHeader, UploadManifestRequest,
        };
        UploadManifestRequest {
            permission: admission_input(Permission {
                request: input.request,
                uploader: self.uploader,
                expires_at_ns: u64::MAX,
            }),
            declaration: UploadManifestDeclaration {
                chunks: input.manifest.chunks.clone(),
                headers: input
                    .manifest
                    .headers
                    .iter()
                    .map(|(name, value)| UploadManifestHeader {
                        name: name.clone(),
                        value: value.clone(),
                    })
                    .collect(),
            },
        }
    }
    fn expose(&self, input: Request) -> Result<(), Failure> {
        use blob_test_protocol::storage::exposure::{
            ExposureInput, ExposureOutcome, ExposureScenario,
        };
        use ic_blob_storage::dto::upload::exposure::UploadExposureFailure as E;
        let result: Result<ExposureOutcome, E> = self
            .harness
            .pic
            .update_candid_as(
                self.service,
                self.uploader,
                "expose",
                (ExposureInput {
                    permission: admission_input(Permission {
                        request: input,
                        uploader: self.uploader,
                        expires_at_ns: u64::MAX,
                    }),
                    scenario: ExposureScenario::QualifiedSubstitute,
                    trap_write: false,
                    trap_after: false,
                },),
            )
            .unwrap();
        result
            .map(|r| {
                assert!(matches!(r, ExposureOutcome::Exposed(_)));
            })
            .map_err(|e| match e {
                E::Permission(e) => admission_failure(e),
                E::Unprepared => Failure::Unprepared,
                E::Revoked => Failure::Revoked,
                E::Phase => Failure::Phase,
                E::EvidenceBinding => Failure::Binding,
            })
    }
    fn revoke(&self, input: Permission) -> Result<bool, Failure> {
        let result: Result<
            ic_blob_storage::dto::upload::admission::UploadRevocationResponse,
            ic_blob_storage::dto::upload::admission::UploadAdmissionFailure,
        > = self
            .harness
            .pic
            .update_candid_as(
                self.service,
                self.tenant,
                "blob_revoke_upload",
                (admission_input(input),),
            )
            .unwrap();
        result.map(|r| r.changed).map_err(admission_failure)
    }
    fn lookup(&self, actor: Principal, input: Request) -> Result<Observation, Failure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "lookup", (input,))
            .unwrap()
    }
    fn status(&self) -> Status {
        let result: Result<Status, Failure> = self
            .harness
            .pic
            .query_candid_as(self.service, self.operator, "status", ())
            .unwrap();
        result.unwrap()
    }
    fn tenant_scope(&self) -> TenantScope {
        TenantScope {
            service: self.service,
            namespace: 1,
            tenant: self.tenant,
        }
    }
    fn enroll(
        &self,
        expected: Option<TenantEnrollment>,
        active: bool,
    ) -> Result<TenantEnrollment, TenantFailure> {
        let response: Result<TenantEnrollmentResponse, TenantFailure> = self
            .harness
            .pic
            .update_candid_as(
                self.service,
                self.operator,
                TENANT_UPDATE_METHOD,
                (TenantUpdateRequest {
                    scope: self.tenant_scope(),
                    expected,
                    active,
                },),
            )
            .unwrap();
        response.map(|v| {
            assert_eq!(v.scope, self.tenant_scope());
            assert!(!v.fenced);
            v.enrollment.unwrap()
        })
    }
    fn tenant(&self) -> Option<TenantEnrollment> {
        let result: Result<TenantEnrollmentResponse, TenantFailure> = self
            .harness
            .pic
            .query_candid_as(
                self.service,
                self.tenant,
                TENANT_INSPECTION_METHOD,
                (self.tenant_scope(),),
            )
            .unwrap();
        let response = result.unwrap();
        assert_eq!(response.scope, self.tenant_scope());
        response.enrollment
    }
    fn restart(&self) {
        self.harness
            .pic
            .stop_canister(self.service, Some(self.controller))
            .unwrap();
        self.harness
            .pic
            .start_canister(self.service, Some(self.controller))
            .unwrap();
    }
}
#[test]
fn admission_write_traps_roll_back_identity_permission_and_both_totals() {
    for fault in [
        WriteFault::Objects,
        WriteFault::Permissions,
        WriteFault::RootRequests,
        WriteFault::Usage,
    ] {
        let f = Fixture::new();
        f.enroll(None, true).unwrap();
        let (attempted, _) = f.permission(1, 1);
        let before = f.status();
        let error = f
            .harness
            .pic
            .update_call(
                f.service,
                f.tenant,
                "admit_with_write_trap",
                candid::encode_one(FaultAdmission {
                    permission: admission_input(attempted),
                    fault,
                })
                .unwrap(),
            )
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
        assert_eq!(f.status(), before);
        assert_eq!(f.lookup(f.tenant, attempted.request), Err(Failure::Unknown));
        // The failed root is free for another object, and the failed object/operation
        // is free for another root. No partial index or cached capacity survived.
        let (same_root, _) = f.permission(2, 1);
        let (same_object, _) = f.permission(1, 2);
        assert_eq!(f.admit(f.tenant, same_root), Ok(true));
        assert_eq!(f.admit(f.tenant, same_object), Ok(true));
        assert_eq!(
            f.status(),
            Status {
                operations: 2,
                active: 2,
                bytes: 20,
                usage: JourneyUsage {
                    logical: 20,
                    physical: 20,
                    liability: 20
                },
                fenced: false
            }
        );
        assert_eq!(f.admit(f.tenant, same_root), Ok(false));
        assert_eq!(f.admit(f.tenant, attempted), Err(Failure::Conflict));
        assert_eq!(
            f.admit(f.tenant, f.permission(3, 3).0),
            Err(Failure::Capacity)
        );
    }
}
#[test]
fn failed_preparation_cannot_leave_a_manifest_or_permission_flag() {
    let f = Fixture::new();
    f.enroll(None, true).unwrap();
    let (permission, preparation) = f.permission(1, 1);
    f.admit(f.tenant, permission).unwrap();
    let before = f.lookup(f.uploader, permission.request).unwrap();
    let totals = f.status();
    for fault in [WriteFault::Manifests, WriteFault::Permissions] {
        let error = f
            .harness
            .pic
            .update_call(
                f.service,
                f.uploader,
                "prepare_with_write_trap",
                candid::encode_one(FaultPreparation {
                    preparation: f.preparation_input(&preparation),
                    fault,
                })
                .unwrap(),
            )
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
        assert_eq!(f.lookup(f.uploader, permission.request), Ok(before));
        assert_eq!(f.status(), totals);
        assert_eq!(f.expose(permission.request), Err(Failure::Unprepared));
    }
    assert_eq!(f.prepare(&preparation), Ok(true));
    assert_eq!(f.prepare(&preparation), Ok(false));
    f.expose(permission.request).unwrap();
    assert_eq!(f.revoke(permission), Ok(true));
    assert_eq!(f.revoke(permission), Ok(false));
    assert_eq!(f.status(), totals);
    assert_eq!(
        f.lookup(f.uploader, permission.request).unwrap().phase,
        Phase::ExposurePossible
    );
    assert_eq!(f.expose(permission.request), Err(Failure::Revoked));
}
#[test]
fn upgrade_preserves_complete_pending_obligations_and_fences_all_mutations() {
    let f = Fixture::new();
    let active = f.enroll(None, true).unwrap();
    let (permission, preparation) = f.permission(u128::MAX, 7);
    for actor in [f.controller, f.operator, f.other, f.uploader] {
        assert_eq!(f.admit(actor, permission), Err(Failure::Denied));
    }
    f.admit(f.tenant, permission).unwrap();
    f.prepare(&preparation).unwrap();
    f.expose(permission.request).unwrap();
    f.revoke(permission).unwrap();
    let (cancelled, second_manifest) = f.permission(2, 8);
    f.admit(f.tenant, cancelled).unwrap();
    f.prepare(&second_manifest).unwrap();
    f.revoke(cancelled).unwrap();
    let suspended = f.enroll(Some(active), false).unwrap();
    let before = f.status();
    assert_eq!(
        before,
        Status {
            operations: 2,
            active: 1,
            bytes: 10,
            usage: JourneyUsage {
                logical: 10,
                physical: 10,
                liability: 10
            },
            fenced: false
        }
    );
    let original = f.lookup(f.uploader, permission.request).unwrap();
    let cancelled_view = f.lookup(f.tenant, cancelled.request).unwrap();
    assert_eq!(cancelled_view.phase, Phase::Cancelled);
    assert_eq!(f.lookup(f.other, permission.request), Err(Failure::Denied));
    f.restart();
    assert_eq!(f.status(), before);
    let failed = f
        .harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.other).unwrap(),
            Some(f.controller),
        )
        .unwrap_err();
    assert_eq!(failed.reject_code, RejectCode::CanisterError);
    assert_eq!(f.status(), before);
    assert_eq!(f.tenant(), Some(suspended));
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(
        f.status(),
        Status {
            fenced: true,
            ..before
        }
    );
    assert_eq!(f.tenant(), Some(suspended));
    assert_eq!(f.lookup(f.uploader, permission.request), Ok(original));
    assert_eq!(f.lookup(f.tenant, cancelled.request), Ok(cancelled_view));
    assert_eq!(f.lookup(f.other, permission.request), Err(Failure::Denied));
    assert_eq!(f.enroll(Some(suspended), true), Err(TenantFailure::Fenced));
    assert_eq!(f.admit(f.tenant, permission), Err(Failure::Fenced));
    assert_eq!(f.prepare(&preparation), Err(Failure::Fenced));
    assert_eq!(f.expose(permission.request), Err(Failure::Fenced));
    assert_eq!(f.revoke(permission), Err(Failure::Fenced));
    f.restart();
    assert!(f.status().fenced);
    assert_eq!(f.lookup(f.tenant, permission.request), Ok(original));
}

#[test]
fn failed_cancellation_retains_permission_and_reservation_until_retry() {
    let f = Fixture::new();
    f.enroll(None, true).unwrap();
    let (permission, _) = f.permission(1, 1);
    f.admit(f.tenant, permission).unwrap();
    let before = f.lookup(f.tenant, permission.request).unwrap();
    let totals = f.status();
    let error = f
        .harness
        .pic
        .update_call(
            f.service,
            f.tenant,
            "revoke_with_usage_write_trap",
            candid::encode_one(admission_input(permission)).unwrap(),
        )
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert_eq!(f.lookup(f.tenant, permission.request), Ok(before));
    assert_eq!(f.status(), totals);
    assert_eq!(f.revoke(permission), Ok(true));
    assert_eq!(
        f.status(),
        Status {
            active: 0,
            bytes: 0,
            usage: JourneyUsage {
                logical: 0,
                physical: 0,
                liability: 0
            },
            ..totals
        }
    );
    assert_eq!(f.revoke(permission), Ok(false));
    assert_eq!(f.admit(f.tenant, permission), Ok(false));
    assert_eq!(
        f.lookup(f.tenant, permission.request).unwrap().phase,
        Phase::Cancelled
    );
}

mod storage_lifecycle;

mod storage_planning;
mod storage_reads;

mod storage_consumer;
mod storage_funding;

fn admission_input(
    input: Permission,
) -> ic_blob_storage::dto::upload::admission::UploadAdmissionRequest {
    ic_blob_storage::dto::upload::admission::UploadAdmissionRequest {
        upload: ic_blob_storage::dto::reference::ReferenceUpload {
            service: input.request.service,
            tenant: input.request.tenant,
            namespace: input.request.namespace,
            upload: input.request.id,
            object: input.request.id,
            incarnation: 1,
            first_reference: 1,
            root: input.request.root,
            bytes: input.request.bytes,
        },
        uploader: input.uploader,
        expires_at_ns: input.expires_at_ns,
    }
}

fn admission_failure(
    error: ic_blob_storage::dto::upload::admission::UploadAdmissionFailure,
) -> Failure {
    use ic_blob_storage::dto::upload::admission::UploadAdmissionFailure as A;
    match error {
        A::Denied => Failure::Denied,
        A::Binding => Failure::Binding,
        A::Unknown => Failure::Unknown,
        A::Conflict => Failure::Conflict,
        A::Inactive => Failure::Inactive,
        A::Fenced => Failure::Fenced,
        A::Capacity => Failure::Capacity,
        A::Expired | A::Invalid => Failure::Invalid,
        A::Internal => panic!("unexpected internal permission failure"),
    }
}
