//! IC transaction rollback and same-release inspection of the durable upload owner.
#![cfg(not(target_family = "wasm"))]
use ic_blob_storage_pocketic_tests::control_calls::ControlCallExt as _;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
mod authenticated_cli;
mod browser_driver;
mod install_code;
mod reference_cli;
mod snapshots;
mod storage_exposure;
mod storage_gateways;
mod storage_growth;
mod storage_manifests;
mod storage_reference_native_cli;
mod storage_resources;
mod storage_tenants;
use ic_blob_storage_pocketic_tests::installation;
mod support;
use blob_test_protocol::{
    admission::{Permission, Phase, Request, input::PreparationInput},
    journey::{JourneyManifest, JourneyUsage},
    storage::{Failure, FaultAdmission, FaultPreparation, Observation, Status, WriteFault},
};
use ic_blob_storage_contracts::dto::tenant::TenantEnrollment;
use ic_blob_storage_contracts::dto::tenant::TenantEnrollmentResponse;
use ic_blob_storage_contracts::dto::tenant::TenantFailure;
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::dto::tenant::TenantUpdateRequest;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHashLimits;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHeader;
use ic_blob_storage_contracts::identity::caffeine::manifest::builder::CaffeineManifestBuilder;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::canister_rejection;
use ic_platform::principal::ExternalPrincipal as Principal;
use ic_testkit::fake_principal;
use ic_testkit::pocket_ic::{CanisterSettings, RejectCode};
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
    fn installation(method: &str, operator: Principal) -> Vec<u8> {
        blob_test_protocol::fixture_wire::encode_one(
            method,
            blob_test_protocol::storage::resources::StorageProbeInstallation {
                operator,
                max_objects: 2,
                max_tenants: 2,
                max_object_bytes: 10,
                max_funding_attempts: 4,
            },
        )
        .unwrap()
    }
    async fn local_status(&self) -> ic_blob_storage_contracts::dto::operator::LocalServiceStatus {
        self.harness
            .pic
            .query_blob_as::<Result<_, ic_blob_storage_contracts::dto::operator::LocalStatusFailure>, _>(self.service, self.operator, ic_blob_storage_contracts::protocol::LOCAL_STATUS_METHOD, ic_blob_storage_contracts::dto::operator::OperatorScope {
                    service: self.service.into(),
                    namespace: 1,
                    cashier: self.operator.into(),
                    payment_account: self.service.into(),
                }, REPLY_BYTES)
            .await.unwrap()
            .unwrap()
    }
    async fn new() -> Self {
        Self::with_operator(Harness::new().await, fake_principal(1)).await
    }
    async fn with_operator(harness: Harness, operator: Principal) -> Self {
        let controller = fake_principal(2);
        let service = installation::controlled_canister(&harness.pic, controller).await;
        harness
            .pic
            .install_canister(
                service,
                Self::wasm(),
                Self::installation("fixture_init", operator),
                Some(controller),
            )
            .await
            .unwrap();
        Self {
            harness,
            service,
            operator,
            controller,
            tenant: fake_principal(3),
            uploader: fake_principal(4),
            other: fake_principal(5),
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
    async fn admit(&self, actor: Principal, input: Permission) -> Result<bool, Failure> {
        let result: Result<
            ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionMutation,
            ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure,
        > = self
            .harness
            .pic
            .update_blob_as(
                self.service,
                actor,
                "blob_admit_upload",
                admission_input(input),
                REPLY_BYTES,
            )
            .await
            .unwrap();
        result.map(|r| !r.replayed).map_err(admission_failure)
    }
    async fn prepare(&self, input: &PreparationInput) -> Result<bool, Failure> {
        use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure as E;
        let result: Result<
            ic_blob_storage_contracts::dto::upload::manifest::UploadManifestMutation,
            ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure,
        > = self
            .harness
            .pic
            .update_blob_as(
                self.service,
                self.uploader,
                "blob_prepare_upload",
                self.preparation_input(input),
                REPLY_BYTES,
            )
            .await
            .unwrap();
        result.map(|r| r.changed).map_err(|e| match e {
            E::Permission(e) => admission_failure(e),
            E::Revoked => Failure::Revoked,
            E::Phase => Failure::Phase,
            E::Declaration => Failure::Invalid,
            E::Limit => Failure::Capacity,
        })
    }
    fn preparation_input(
        &self,
        input: &PreparationInput,
    ) -> ic_blob_storage_contracts::dto::upload::manifest::UploadManifestRequest {
        use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestDeclaration;
        use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestHeader;
        use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestRequest;
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
    async fn expose(&self, input: Request) -> Result<(), Failure> {
        use blob_test_protocol::storage::exposure::ExposureInput;
        use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure as E;
        use ic_blob_storage_contracts::dto::upload::{
            UploadState, admission::UploadAdmissionResponse,
        };
        let result: Result<UploadAdmissionResponse, E> = self
            .harness
            .pic
            .update_control_as(
                self.service,
                self.uploader,
                "expose",
                (ExposureInput {
                    permission: admission_input(Permission {
                        request: input,
                        uploader: self.uploader,
                        expires_at_ns: u64::MAX,
                    }),
                    trap_write: false,
                    trap_after: false,
                },),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap();
        result
            .map(|r| {
                assert_eq!(r.state, UploadState::Registered);
            })
            .map_err(|e| match e {
                E::Permission(e) => admission_failure(e),
                E::Unprepared => Failure::Unprepared,
                E::Revoked => Failure::Revoked,
                E::Phase => Failure::Phase,
            })
    }
    async fn revoke(&self, input: Permission) -> Result<bool, Failure> {
        let result: Result<
            ic_blob_storage_contracts::dto::upload::admission::UploadRevocationResponse,
            ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure,
        > = self
            .harness
            .pic
            .update_blob_as(
                self.service,
                self.tenant,
                "blob_revoke_upload",
                admission_input(input),
                REPLY_BYTES,
            )
            .await
            .unwrap();
        result.map(|r| r.changed).map_err(admission_failure)
    }
    async fn lookup(&self, actor: Principal, input: Request) -> Result<Observation, Failure> {
        self.harness
            .pic
            .query_control_as(self.service, actor, "lookup", (input,), REPLY_BYTES)
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
    async fn status(&self) -> Status {
        let result: Result<Status, Failure> = self
            .harness
            .pic
            .query_control_as(self.service, self.operator, "status", (), REPLY_BYTES)
            .await
            .map(|(reply,)| reply)
            .unwrap();
        result.unwrap()
    }
    fn tenant_scope(&self) -> TenantScope {
        TenantScope {
            service: self.service.into(),
            namespace: 1,
            tenant: self.tenant.into(),
        }
    }
    async fn enroll(
        &self,
        expected: Option<TenantEnrollment>,
        active: bool,
    ) -> Result<TenantEnrollment, TenantFailure> {
        let response: Result<TenantEnrollmentResponse, TenantFailure> = self
            .harness
            .pic
            .update_tenant_as(
                self.service,
                self.operator,
                TenantUpdateRequest {
                    scope: self.tenant_scope(),
                    expected,
                    active,
                },
                REPLY_BYTES,
            )
            .await
            .unwrap();
        response.map(|v| {
            assert_eq!(v.scope, self.tenant_scope());
            assert!(!v.fenced);
            v.enrollment.unwrap()
        })
    }
    async fn tenant(&self) -> Option<TenantEnrollment> {
        let result: Result<TenantEnrollmentResponse, TenantFailure> = self
            .harness
            .pic
            .query_tenant_as(self.service, self.tenant, self.tenant_scope(), REPLY_BYTES)
            .await
            .unwrap();
        let response = result.unwrap();
        assert_eq!(response.scope, self.tenant_scope());
        response.enrollment
    }
    async fn restart(&self) {
        self.harness
            .pic
            .stop_canister(self.service, Some(self.controller))
            .await
            .unwrap();
        self.harness
            .pic
            .start_canister(self.service, Some(self.controller))
            .await
            .unwrap();
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn admission_write_traps_roll_back_identity_permission_and_both_totals() {
    for fault in [
        WriteFault::Objects,
        WriteFault::Permissions,
        WriteFault::Usage,
    ] {
        let f = Fixture::new().await;
        f.enroll(None, true).await.unwrap();
        let (attempted, _) = f.permission(1, 1);
        let before = f.status().await;
        let error = f
            .harness
            .pic
            .update_call(
                f.service,
                f.tenant,
                "admit_with_write_trap",
                ic_blob_storage_pocketic_tests::control_calls::encode_control_one(FaultAdmission {
                    permission: admission_input(attempted),
                    fault,
                })
                .unwrap(),
            )
            .await
            .unwrap_err();
        assert_eq!(
            crate::canister_rejection(&error).reject_code,
            RejectCode::CanisterError
        );
        assert_eq!(f.status().await, before);
        assert_eq!(
            f.lookup(f.tenant, attempted.request).await,
            Err(Failure::Unknown)
        );
        // The failed root is free for another object, and the failed object/operation
        // is free for another root. No partial index or cached capacity survived.
        let (same_root, _) = f.permission(2, 1);
        let (same_object, _) = f.permission(1, 2);
        assert_eq!(f.admit(f.tenant, same_root).await, Ok(true));
        assert_eq!(f.admit(f.tenant, same_object).await, Ok(true));
        assert_eq!(
            f.status().await,
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
        assert_eq!(f.admit(f.tenant, same_root).await, Ok(false));
        assert_eq!(f.admit(f.tenant, attempted).await, Err(Failure::Conflict));
        assert_eq!(
            f.admit(f.tenant, f.permission(3, 3).0).await,
            Err(Failure::Capacity)
        );
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn failed_preparation_cannot_leave_a_manifest_or_permission_flag() {
    let f = Fixture::new().await;
    f.enroll(None, true).await.unwrap();
    let (permission, preparation) = f.permission(1, 1);
    f.admit(f.tenant, permission).await.unwrap();
    let before = f.lookup(f.uploader, permission.request).await.unwrap();
    let totals = f.status().await;
    for fault in [WriteFault::Manifests, WriteFault::Permissions] {
        let error = f
            .harness
            .pic
            .update_call(
                f.service,
                f.uploader,
                "prepare_with_write_trap",
                ic_blob_storage_pocketic_tests::control_calls::encode_control_one(
                    FaultPreparation {
                        preparation: f.preparation_input(&preparation),
                        fault,
                    },
                )
                .unwrap(),
            )
            .await
            .unwrap_err();
        assert_eq!(
            crate::canister_rejection(&error).reject_code,
            RejectCode::CanisterError
        );
        assert_eq!(f.lookup(f.uploader, permission.request).await, Ok(before));
        assert_eq!(f.status().await, totals);
        assert_eq!(f.expose(permission.request).await, Err(Failure::Unprepared));
    }
    assert_eq!(f.prepare(&preparation).await, Ok(true));
    assert_eq!(f.prepare(&preparation).await, Ok(false));
    f.expose(permission.request).await.unwrap();
    assert_eq!(f.revoke(permission).await, Ok(true));
    assert_eq!(f.revoke(permission).await, Ok(false));
    assert_eq!(
        f.status().await,
        Status {
            active: 0,
            bytes: 0,
            ..totals
        }
    );
    assert_eq!(
        f.lookup(f.uploader, permission.request)
            .await
            .unwrap()
            .phase,
        Phase::Registered
    );
    assert_eq!(f.expose(permission.request).await, Err(Failure::Revoked));
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn upgrade_preserves_complete_pending_obligations_and_fences_all_mutations() {
    let f = Fixture::new().await;
    let active = f.enroll(None, true).await.unwrap();
    let (permission, preparation) = f.permission(u128::MAX, 7);
    for actor in [f.controller, f.operator, f.other, f.uploader] {
        assert_eq!(f.admit(actor, permission).await, Err(Failure::Denied));
    }
    f.admit(f.tenant, permission).await.unwrap();
    f.prepare(&preparation).await.unwrap();
    f.expose(permission.request).await.unwrap();
    f.revoke(permission).await.unwrap();
    let (cancelled, second_manifest) = f.permission(2, 8);
    f.admit(f.tenant, cancelled).await.unwrap();
    f.prepare(&second_manifest).await.unwrap();
    f.revoke(cancelled).await.unwrap();
    let suspended = f.enroll(Some(active), false).await.unwrap();
    let before = f.status().await;
    assert_eq!(
        before,
        Status {
            operations: 2,
            active: 0,
            bytes: 0,
            usage: JourneyUsage {
                logical: 10,
                physical: 10,
                liability: 10
            },
            fenced: false
        }
    );
    let original = f.lookup(f.uploader, permission.request).await.unwrap();
    let cancelled_view = f.lookup(f.tenant, cancelled.request).await.unwrap();
    assert_eq!(cancelled_view.phase, Phase::Cancelled);
    assert_eq!(
        f.lookup(f.other, permission.request).await,
        Err(Failure::Denied)
    );
    f.restart().await;
    assert_eq!(f.status().await, before);
    let failed = f
        .harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation("fixture_post_upgrade", f.other),
            Some(f.controller),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&failed).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(f.status().await, before);
    assert_eq!(f.tenant().await, Some(suspended));
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation("fixture_post_upgrade", f.operator),
            Some(f.controller),
        )
        .await
        .unwrap();
    assert_eq!(
        f.status().await,
        Status {
            fenced: true,
            ..before
        }
    );
    assert_eq!(f.tenant().await, Some(suspended));
    assert_eq!(f.lookup(f.uploader, permission.request).await, Ok(original));
    assert_eq!(
        f.lookup(f.tenant, cancelled.request).await,
        Ok(cancelled_view)
    );
    assert_eq!(
        f.lookup(f.other, permission.request).await,
        Err(Failure::Denied)
    );
    assert_eq!(
        f.enroll(Some(suspended), true).await,
        Err(TenantFailure::Fenced)
    );
    assert_eq!(f.admit(f.tenant, permission).await, Err(Failure::Fenced));
    assert_eq!(f.prepare(&preparation).await, Err(Failure::Fenced));
    assert_eq!(f.expose(permission.request).await, Err(Failure::Fenced));
    assert_eq!(f.revoke(permission).await, Err(Failure::Fenced));
    f.restart().await;
    assert!(f.status().await.fenced);
    assert_eq!(f.lookup(f.tenant, permission.request).await, Ok(original));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn failed_cancellation_retains_permission_and_reservation_until_retry() {
    let f = Fixture::new().await;
    f.enroll(None, true).await.unwrap();
    let (permission, _) = f.permission(1, 1);
    f.admit(f.tenant, permission).await.unwrap();
    let before = f.lookup(f.tenant, permission.request).await.unwrap();
    let totals = f.status().await;
    let error = f
        .harness
        .pic
        .update_call(
            f.service,
            f.tenant,
            "revoke_with_usage_write_trap",
            ic_blob_storage_pocketic_tests::control_calls::encode_control_one(admission_input(
                permission,
            ))
            .unwrap(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(f.lookup(f.tenant, permission.request).await, Ok(before));
    assert_eq!(f.status().await, totals);
    assert_eq!(f.revoke(permission).await, Ok(true));
    assert_eq!(
        f.status().await,
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
    assert_eq!(f.revoke(permission).await, Ok(false));
    assert_eq!(f.admit(f.tenant, permission).await, Ok(false));
    assert_eq!(
        f.lookup(f.tenant, permission.request).await.unwrap().phase,
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
) -> ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest {
    ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest {
        upload: ic_blob_storage_contracts::dto::reference::ReferenceUpload {
            service: input.request.service.into(),
            tenant: input.request.tenant.into(),
            namespace: input.request.namespace,
            upload: input.request.id,
            object: input.request.id,
            incarnation: 1,
            first_reference: 1,
            root: input.request.root,
            bytes: input.request.bytes,
        },
        uploader: input.uploader.into(),
        expires_at_ns: input.expires_at_ns,
    }
}

fn admission_failure(
    error: ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure,
) -> Failure {
    use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure as A;
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
