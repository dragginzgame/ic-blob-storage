//! Maintained enrollment API through actual callers and IC rollback.
use super::*;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
mod client;

impl Fixture {
    async fn update_enrollment(
        &self,
        actor: Principal,
        input: TenantUpdateRequest,
    ) -> Result<TenantEnrollmentResponse, TenantFailure> {
        self.harness
            .pic
            .update_tenant_as(self.service, actor, input, REPLY_BYTES)
            .await
            .unwrap()
    }
    async fn inspect_enrollment(
        &self,
        actor: Principal,
        scope: TenantScope,
    ) -> Result<TenantEnrollmentResponse, TenantFailure> {
        self.harness
            .pic
            .query_tenant_as(self.service, actor, scope, REPLY_BYTES)
            .await
            .unwrap()
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn tenant_boundary_checks_scope_authority_and_preconditions_without_writes() {
    let f = Fixture::new().await;
    let scope = f.tenant_scope();
    let input = TenantUpdateRequest {
        scope,
        expected: None,
        active: true,
    };
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    for actor in [
        f.controller,
        f.tenant,
        f.uploader,
        f.other,
        Principal::anonymous(),
    ] {
        assert_eq!(
            f.update_enrollment(actor, input).await,
            Err(TenantFailure::Denied)
        );
    }
    for actor in [f.operator, f.tenant] {
        assert_eq!(
            f.inspect_enrollment(actor, scope).await,
            Ok(TenantEnrollmentResponse {
                scope,
                enrollment: None,
                fenced: false
            })
        );
    }
    for actor in [f.controller, f.uploader, f.other, Principal::anonymous()] {
        assert_eq!(
            f.inspect_enrollment(actor, scope).await,
            Err(TenantFailure::Denied)
        );
    }
    for scope in [
        TenantScope {
            service: f.other.into(),
            ..scope
        },
        TenantScope {
            namespace: 2,
            ..scope
        },
        TenantScope {
            namespace: 0,
            ..scope
        },
    ] {
        assert_eq!(
            f.update_enrollment(f.operator, TenantUpdateRequest { scope, ..input })
                .await,
            Err(TenantFailure::Binding)
        );
        assert_eq!(
            f.inspect_enrollment(f.operator, scope).await,
            Err(TenantFailure::Binding)
        );
    }
    for tenant in [Principal::anonymous(), Principal::management_canister()] {
        let scope = TenantScope {
            tenant: tenant.into(),
            ..scope
        };
        assert_eq!(
            f.update_enrollment(f.operator, TenantUpdateRequest { scope, ..input })
                .await,
            Err(TenantFailure::Invalid)
        );
        assert_eq!(
            f.inspect_enrollment(f.operator, scope).await,
            Err(TenantFailure::Invalid)
        );
    }
    assert_eq!(
        f.update_enrollment(
            f.operator,
            TenantUpdateRequest {
                expected: Some(TenantEnrollment {
                    generation: 0,
                    active: false
                }),
                ..input
            }
        )
        .await,
        Err(TenantFailure::Invalid)
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn tenant_compare_and_set_recovers_by_inspection_and_suspension_retains_capacity() {
    let f = Fixture::new().await;
    let scope = f.tenant_scope();
    let original = TenantUpdateRequest {
        scope,
        expected: None,
        active: true,
    };
    f.update_enrollment(f.operator, original).await.unwrap();
    // A lost enrollment reply is recovered by inspection, not blind replay.
    assert_eq!(
        f.update_enrollment(f.operator, original).await,
        Err(TenantFailure::Conflict)
    );
    let active = f
        .inspect_enrollment(f.operator, scope)
        .await
        .unwrap()
        .enrollment
        .unwrap();
    assert_eq!(
        active,
        TenantEnrollment {
            generation: 1,
            active: true
        }
    );
    let (permission, _) = f.permission(1, 1);
    f.admit(f.tenant, permission).await.unwrap();
    let obligations = f.status().await;
    let suspended = f.enroll(Some(active), false).await.unwrap();
    assert_eq!(
        suspended,
        TenantEnrollment {
            active: false,
            ..active
        }
    );
    assert_eq!(f.status().await, obligations);
    assert_eq!(
        f.enroll(Some(active), true).await,
        Err(TenantFailure::Conflict)
    );
    let reactivated = f.enroll(Some(suspended), true).await.unwrap();
    assert_eq!(
        reactivated,
        TenantEnrollment {
            generation: 2,
            active: true
        }
    );
    // Fresh-generation permission cannot replace the retained original operation.
    assert_eq!(
        f.lookup(f.tenant, permission.request)
            .await
            .unwrap()
            .generation,
        1
    );
    assert_eq!(f.status().await, obligations);

    let second = TenantScope {
        tenant: f.other.into(),
        ..scope
    };
    f.update_enrollment(
        f.operator,
        TenantUpdateRequest {
            scope: second,
            expected: None,
            active: false,
        },
    )
    .await
    .unwrap();
    let third = TenantScope {
        tenant: f.uploader.into(),
        ..scope
    };
    assert_eq!(
        f.update_enrollment(
            f.operator,
            TenantUpdateRequest {
                scope: third,
                ..original
            }
        )
        .await,
        Err(TenantFailure::Capacity)
    );
    // Suspension of an existing tenant still succeeds at lifetime capacity.
    f.enroll(Some(reactivated), false).await.unwrap();
    assert_eq!(
        f.inspect_enrollment(f.uploader, third)
            .await
            .unwrap()
            .enrollment,
        None
    );
    assert_eq!(
        f.inspect_enrollment(f.tenant, second).await,
        Err(TenantFailure::Denied)
    );
    assert_eq!(f.status().await, obligations);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn tenant_write_traps_preserve_generation_then_restore_keeps_inspection_only() {
    let f = Fixture::new().await;
    let scope = f.tenant_scope();
    let trap = async |expected, active| {
        let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
        let error = f
            .harness
            .pic
            .update_call(
                f.service,
                f.operator,
                "fixture_update_tenant_with_write_trap",
                ic_blob_storage_pocketic_tests::control_calls::encode_control_one(
                    TenantUpdateRequest {
                        scope,
                        expected,
                        active,
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
        assert_eq!(
            f.harness.pic.get_stable_memory(f.service).await.unwrap(),
            before
        );
    };
    trap(None, true).await;
    assert_eq!(f.tenant().await, None);
    let active = f.enroll(None, true).await.unwrap();
    trap(Some(active), false).await;
    assert_eq!(f.tenant().await, Some(active));
    let suspended = f.enroll(Some(active), false).await.unwrap();
    trap(Some(suspended), true).await;
    assert_eq!(f.tenant().await, Some(suspended));
    let reactivated = f.enroll(Some(suspended), true).await.unwrap();
    assert_eq!(reactivated.generation, 2);
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
    for actor in [f.tenant, f.operator] {
        assert_eq!(
            f.inspect_enrollment(actor, scope).await,
            Ok(TenantEnrollmentResponse {
                scope,
                enrollment: Some(reactivated),
                fenced: true
            })
        );
    }
    assert_eq!(
        f.enroll(Some(reactivated), true).await,
        Err(TenantFailure::Fenced)
    );
    assert_eq!(
        f.enroll(Some(reactivated), false).await,
        Err(TenantFailure::Fenced)
    );
    assert_eq!(
        f.update_enrollment(
            f.controller,
            TenantUpdateRequest {
                scope,
                expected: Some(reactivated),
                active: false
            }
        )
        .await,
        Err(TenantFailure::Denied)
    );
}
