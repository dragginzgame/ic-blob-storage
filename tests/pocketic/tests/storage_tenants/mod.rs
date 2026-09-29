//! Maintained enrollment API through actual callers and IC rollback.
use super::*;
mod client;

impl Fixture {
    fn update_enrollment(
        &self,
        actor: Principal,
        input: TenantUpdateRequest,
    ) -> Result<TenantEnrollmentResponse, TenantFailure> {
        self.harness
            .pic
            .update_candid_as(self.service, actor, TENANT_UPDATE_METHOD, (input,))
            .unwrap()
    }
    fn inspect_enrollment(
        &self,
        actor: Principal,
        scope: TenantScope,
    ) -> Result<TenantEnrollmentResponse, TenantFailure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, TENANT_INSPECTION_METHOD, (scope,))
            .unwrap()
    }
}

#[test]
fn tenant_boundary_checks_scope_authority_and_preconditions_without_writes() {
    let f = Fixture::new();
    let scope = f.tenant_scope();
    let input = TenantUpdateRequest {
        scope,
        expected: None,
        active: true,
    };
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [
        f.controller,
        f.tenant,
        f.uploader,
        f.other,
        Principal::anonymous(),
    ] {
        assert_eq!(
            f.update_enrollment(actor, input),
            Err(TenantFailure::Denied)
        );
    }
    for actor in [f.operator, f.tenant] {
        assert_eq!(
            f.inspect_enrollment(actor, scope),
            Ok(TenantEnrollmentResponse {
                scope,
                enrollment: None,
                fenced: false
            })
        );
    }
    for actor in [f.controller, f.uploader, f.other, Principal::anonymous()] {
        assert_eq!(
            f.inspect_enrollment(actor, scope),
            Err(TenantFailure::Denied)
        );
    }
    for scope in [
        TenantScope {
            service: f.other,
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
            f.update_enrollment(f.operator, TenantUpdateRequest { scope, ..input }),
            Err(TenantFailure::Binding)
        );
        assert_eq!(
            f.inspect_enrollment(f.operator, scope),
            Err(TenantFailure::Binding)
        );
    }
    for tenant in [Principal::anonymous(), Principal::management_canister()] {
        let scope = TenantScope { tenant, ..scope };
        assert_eq!(
            f.update_enrollment(f.operator, TenantUpdateRequest { scope, ..input }),
            Err(TenantFailure::Invalid)
        );
        assert_eq!(
            f.inspect_enrollment(f.operator, scope),
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
        ),
        Err(TenantFailure::Invalid)
    );
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
}

#[test]
fn tenant_compare_and_set_recovers_by_inspection_and_suspension_retains_capacity() {
    let f = Fixture::new();
    let scope = f.tenant_scope();
    let original = TenantUpdateRequest {
        scope,
        expected: None,
        active: true,
    };
    f.update_enrollment(f.operator, original).unwrap();
    // A lost enrollment reply is recovered by inspection, not blind replay.
    assert_eq!(
        f.update_enrollment(f.operator, original),
        Err(TenantFailure::Conflict)
    );
    let active = f
        .inspect_enrollment(f.operator, scope)
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
    f.admit(f.tenant, permission).unwrap();
    let obligations = f.status();
    let suspended = f.enroll(Some(active), false).unwrap();
    assert_eq!(
        suspended,
        TenantEnrollment {
            active: false,
            ..active
        }
    );
    assert_eq!(f.status(), obligations);
    assert_eq!(f.enroll(Some(active), true), Err(TenantFailure::Conflict));
    let reactivated = f.enroll(Some(suspended), true).unwrap();
    assert_eq!(
        reactivated,
        TenantEnrollment {
            generation: 2,
            active: true
        }
    );
    // Fresh-generation permission cannot replace the retained original operation.
    assert_eq!(
        f.lookup(f.tenant, permission.request).unwrap().generation,
        1
    );
    assert_eq!(f.status(), obligations);

    let second = TenantScope {
        tenant: f.other,
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
    .unwrap();
    let third = TenantScope {
        tenant: f.uploader,
        ..scope
    };
    assert_eq!(
        f.update_enrollment(
            f.operator,
            TenantUpdateRequest {
                scope: third,
                ..original
            }
        ),
        Err(TenantFailure::Capacity)
    );
    // Suspension of an existing tenant still succeeds at lifetime capacity.
    f.enroll(Some(reactivated), false).unwrap();
    assert_eq!(
        f.inspect_enrollment(f.uploader, third).unwrap().enrollment,
        None
    );
    assert_eq!(
        f.inspect_enrollment(f.tenant, second),
        Err(TenantFailure::Denied)
    );
    assert_eq!(f.status(), obligations);
}

#[test]
fn tenant_write_traps_preserve_generation_then_restore_keeps_inspection_only() {
    let f = Fixture::new();
    let scope = f.tenant_scope();
    let trap = |expected, active| {
        let before = f.harness.pic.get_stable_memory(f.service);
        let error = f
            .harness
            .pic
            .update_call(
                f.service,
                f.operator,
                "fixture_update_tenant_with_write_trap",
                candid::encode_one(TenantUpdateRequest {
                    scope,
                    expected,
                    active,
                })
                .unwrap(),
            )
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    };
    trap(None, true);
    assert_eq!(f.tenant(), None);
    let active = f.enroll(None, true).unwrap();
    trap(Some(active), false);
    assert_eq!(f.tenant(), Some(active));
    let suspended = f.enroll(Some(active), false).unwrap();
    trap(Some(suspended), true);
    assert_eq!(f.tenant(), Some(suspended));
    let reactivated = f.enroll(Some(suspended), true).unwrap();
    assert_eq!(reactivated.generation, 2);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    for actor in [f.tenant, f.operator] {
        assert_eq!(
            f.inspect_enrollment(actor, scope),
            Ok(TenantEnrollmentResponse {
                scope,
                enrollment: Some(reactivated),
                fenced: true
            })
        );
    }
    assert_eq!(
        f.enroll(Some(reactivated), true),
        Err(TenantFailure::Fenced)
    );
    assert_eq!(
        f.enroll(Some(reactivated), false),
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
        ),
        Err(TenantFailure::Denied)
    );
}
