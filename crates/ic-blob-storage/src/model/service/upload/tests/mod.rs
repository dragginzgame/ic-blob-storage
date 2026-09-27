use super::*;
mod capacity;
mod content;
mod download;
mod manifest;
mod planning;
use crate::model::{
    billing::{FundingLimits, configuration::BillingConfiguration},
    catalog::{
        CatalogCapacity, CatalogLimits,
        admission::{UploadLimits, UploadObject},
    },
    lifecycle::{
        ReferenceId,
        binding::{ObjectIdentity, ReferenceKey},
        requests::{ReferenceOperation, ReferenceRequestId},
    },
    service::configuration::{ServiceBindings, ServiceLimits, ServiceManifestLimits},
};
use std::num::{NonZeroU64, NonZeroU128, NonZeroUsize};

fn p(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}
fn id(n: u128) -> NonZeroU128 {
    NonZeroU128::new(n).unwrap()
}
fn count(n: usize) -> NonZeroUsize {
    NonZeroUsize::new(n).unwrap()
}
fn context(actor: u8) -> UploadContext {
    UploadContext {
        service: p(1),
        actor: p(actor),
    }
}
fn admissions() -> UploadAdmissions {
    let mut owner = empty_admissions();
    owner
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: None,
                active: true,
            },
        )
        .unwrap();
    owner
}
fn empty_admissions() -> UploadAdmissions {
    UploadAdmissions::new(
        ServiceConfiguration::new(
            ServiceBindings {
                service: p(1),
                operator: p(2),
                payment_account: p(1),
                namespace: id(1),
            },
            ServiceLimits {
                max_tenants: count(2),
                max_object_bytes: NonZeroU64::new(10).unwrap(),
                max_headers: count(8),
                max_header_bytes: count(1024),
                manifests: ServiceManifestLimits {
                    max_chunks: count(4),
                    max_tenant_chunks: count(4),
                },
                catalog: CatalogLimits {
                    max_objects: count(2),
                    max_tenant_objects: count(2),
                    max_physical_bytes: id(20),
                    max_liability_bytes: id(20),
                    max_tenant_logical_bytes: id(20),
                    max_references_per_object: count(2),
                    max_receipts_per_object: count(3),
                },
                uploads: UploadLimits {
                    max_active: count(2),
                    max_tenant_active: count(2),
                },
            },
            BillingConfiguration::new(p(3), FundingLimits::new(1, 10, 100).unwrap(), 8, 4).unwrap(),
        )
        .unwrap(),
    )
}
fn permission(n: u8) -> UploadPermission {
    UploadPermission {
        request: UploadRequest {
            id: UploadRequestId::new(id(u128::from(n))),
            object: UploadObject {
                root: hashes(n).provider_root,
                bytes: 10,
                first: ReferenceKey::new(
                    ObjectBinding::new(
                        p(1),
                        p(4),
                        ObjectIdentity {
                            namespace: id(1),
                            object: id(u128::from(n)),
                            incarnation: id(1),
                        },
                    )
                    .unwrap(),
                    ReferenceId::new(id(1)),
                ),
            },
        },
        uploader: p(5),
        expires_at_ns: 100,
    }
}

fn hashes(n: u8) -> crate::model::identity::caffeine::CaffeineContentHashes {
    let headers = [crate::model::identity::caffeine::CaffeineHeader {
        name: "Content-Length",
        value: "10",
    }];
    hashes_with_headers(n, &headers)
}

fn hashes_with_headers(
    n: u8,
    headers: &[crate::model::identity::caffeine::CaffeineHeader<'_>],
) -> crate::model::identity::caffeine::CaffeineContentHashes {
    use crate::model::identity::caffeine::{CaffeineContentHasher, CaffeineHashLimits};
    let mut hasher = CaffeineContentHasher::new(
        10,
        headers,
        CaffeineHashLimits {
            max_content_bytes: NonZeroU64::new(10).unwrap(),
            max_append_bytes: count(10),
            max_headers: count(8),
            max_header_bytes: count(1024),
        },
    )
    .unwrap();
    hasher.append(0, &[n; 10]).unwrap();
    hasher.finish().unwrap()
}

fn prepare(owner: &mut UploadAdmissions, input: &UploadPermission) {
    use super::manifest::UploadManifest;
    use crate::model::identity::caffeine::manifest::CaffeineChunkHash;
    let now = owner
        .lookup(context(5), input.request)
        .unwrap()
        .admitted_at_ns;
    let n = u8::try_from(input.request.object.first.object().identity().object.get()).unwrap();
    let chunks = [CaffeineChunkHash::try_from(
        hashes_with_headers(n, &[])
            .provider_root
            .as_bytes()
            .as_slice(),
    )
    .unwrap()];
    let headers = [crate::model::identity::caffeine::CaffeineHeader {
        name: "Content-Length",
        value: "10",
    }];
    owner
        .prepare_manifest(
            context(5),
            input.request,
            UploadManifest {
                chunks: &chunks,
                headers: &headers,
            },
            now,
        )
        .unwrap();
}

fn set_active(owner: &mut UploadAdmissions, active: bool) -> TenantEnrollmentView {
    let expected = owner.tenant(context(2), p(4)).unwrap();
    owner
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected,
                active,
            },
        )
        .unwrap()
}

#[test]
fn operator_enrollment_is_required_and_bound_to_actual_service() {
    let mut owner = empty_admissions();
    let input = permission(1);
    let update = TenantUpdate {
        tenant: p(4),
        expected: None,
        active: true,
    };
    assert_eq!(
        owner.admit(context(4), input, 10),
        Err(TenantError::NotEnrolled.into())
    );
    for caller in [3, 4, 5, 6] {
        assert_eq!(
            owner.update_tenant(context(caller), update),
            Err(UploadAdmissionError::NotOperator)
        );
    }
    assert_eq!(
        owner.update_tenant(
            UploadContext {
                service: p(9),
                ..context(2)
            },
            update
        ),
        Err(UploadAdmissionError::WrongService)
    );
    assert_eq!(owner.tenant(context(4), p(4)), Ok(None));
    assert_eq!(owner.catalog().usage().operations, 0);
    for tenant in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            owner.update_tenant(context(2), TenantUpdate { tenant, ..update }),
            Err(TenantError::InvalidTenant.into())
        );
    }
    let enrolled = owner.update_tenant(context(2), update).unwrap();
    assert_eq!(owner.tenant(context(4), p(4)), Ok(Some(enrolled)));
    assert_eq!(
        owner.tenant(context(5), p(4)),
        Err(UploadAdmissionError::NotObserver)
    );
    assert_eq!(
        owner.tenant(
            UploadContext {
                service: p(9),
                ..context(2)
            },
            p(4)
        ),
        Err(UploadAdmissionError::WrongService)
    );
    assert_eq!(
        owner.admit(context(4), input, 10),
        Ok(UploadAdmission::Reserved)
    );
}

#[test]
fn enrollment_capacity_and_compare_and_set_preserve_history_and_reject_stale_control() {
    let mut owner = admissions();
    let initial = owner.tenant(context(2), p(4)).unwrap().unwrap();
    let stale_suspend = TenantUpdate {
        tenant: p(4),
        expected: Some(initial),
        active: false,
    };
    let suspended = owner.update_tenant(context(2), stale_suspend).unwrap();
    assert_eq!(
        owner.update_tenant(context(2), stale_suspend),
        Err(TenantError::Conflict.into())
    );
    owner
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(6),
                expected: None,
                active: true,
            },
        )
        .unwrap();
    assert_eq!(
        owner.update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(7),
                expected: None,
                active: true
            }
        ),
        Err(TenantError::Capacity.into())
    );
    assert_eq!(
        owner.update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: None,
                active: true
            }
        ),
        Err(TenantError::Conflict.into())
    );
    let resumed = owner
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: Some(suspended),
                active: true,
            },
        )
        .unwrap();
    assert!(resumed.generation > initial.generation);
    assert_eq!(
        owner.update_tenant(context(2), stale_suspend),
        Err(TenantError::Conflict.into())
    );
    assert_eq!(owner.tenant(context(2), p(4)), Ok(Some(resumed)));
    assert_eq!(
        owner.update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: Some(resumed),
                active: true
            }
        ),
        Ok(resumed)
    );
}

#[test]
fn suspension_blocks_new_authority_and_reactivation_cannot_renew_old_permission() {
    let mut owner = admissions();
    let input = permission(1);
    owner.admit(context(4), input, 10).unwrap();
    prepare(&mut owner, &input);
    let before = owner.catalog().usage();
    set_active(&mut owner, false);
    assert_eq!(
        owner.admit(context(4), permission(2), 11),
        Err(TenantError::Suspended.into())
    );
    assert_eq!(
        owner.expose_root(context(5), input.request.object.root, 11),
        Err(TenantError::Suspended.into())
    );
    assert_eq!(
        owner.admit(context(4), input, 11),
        Ok(UploadAdmission::Existing(UploadPhase::Reserved))
    );
    let retained = owner.lookup(context(5), input.request).unwrap();
    assert_eq!(retained.phase, UploadPhase::Reserved);
    assert_eq!(owner.catalog().usage(), before);
    let resumed = set_active(&mut owner, true);
    assert!(resumed.generation > retained.tenant_generation);
    assert_eq!(
        owner.expose(context(5), input.request, 12),
        Err(TenantError::StalePermission.into())
    );
    assert_eq!(
        owner.admit(context(4), input, 12),
        Ok(UploadAdmission::Existing(UploadPhase::Reserved))
    );
    owner.revoke(context(4), input.request).unwrap();
    let fresh = permission(2);
    owner.admit(context(4), fresh, 12).unwrap();
    prepare(&mut owner, &fresh);
    assert_eq!(
        owner
            .lookup(context(4), fresh.request)
            .unwrap()
            .tenant_generation,
        resumed.generation
    );
    owner.expose(context(5), fresh.request, 13).unwrap();
    assert_eq!(owner.catalog().usage().operations, 2);
    assert_eq!(owner.catalog().usage().reserved_bytes, 10);
}

#[test]
fn suspended_project_can_cancel_unexposed_and_reconcile_exposed_uploads() {
    let mut owner = admissions();
    let pending = permission(1);
    let exposed = permission(2);
    owner.admit(context(4), pending, 10).unwrap();
    owner.admit(context(4), exposed, 10).unwrap();
    prepare(&mut owner, &exposed);
    owner.expose(context(5), exposed.request, 11).unwrap();
    let before = owner.catalog().usage();
    set_active(&mut owner, false);
    assert_eq!(owner.catalog().usage(), before);
    owner.revoke(context(4), pending.request).unwrap();
    owner.revoke(context(4), exposed.request).unwrap();
    assert_eq!(owner.catalog().usage().reserved_bytes, 10);
    owner.confirm_upload(exposed.request).unwrap();
    assert_eq!(owner.catalog().usage().physical_bytes, 10);
    assert_eq!(owner.catalog().usage().liability_bytes, 10);
    let release = ReferenceRequest {
        id: ReferenceRequestId::new(id(1)),
        operation: ReferenceOperation::Release(exposed.request.object.first),
    };
    let root = exposed.request.object.root;
    owner.apply_reference(context(4), root, release).unwrap();
    assert_eq!(owner.catalog().usage().logical_bytes, 0);
    owner
        .confirm_provider_deleted(root, exposed.request.object.first.object())
        .unwrap();
    assert_eq!(owner.catalog().usage().liability_bytes, 10);
    owner
        .confirm_billing_stopped(root, exposed.request.object.first.object())
        .unwrap();
    assert_eq!(owner.catalog().usage().liability_bytes, 0);
    assert_eq!(owner.catalog().usage().operations, 2);
}

#[test]
fn reference_boundary_checks_actual_project_service_namespace_and_catalog_binding() {
    let mut owner = admissions();
    let input = permission(1);
    let root = input.request.object.root;
    let first = input.request.object.first;
    let second = ReferenceKey::new(first.object(), ReferenceId::new(id(2)));
    owner.admit(context(4), input, 10).unwrap();
    prepare(&mut owner, &input);
    owner.expose(context(5), input.request, 11).unwrap();
    owner.confirm_upload(input.request).unwrap();
    let retain = ReferenceRequest {
        id: ReferenceRequestId::new(id(1)),
        operation: ReferenceOperation::Retain(second),
    };
    for actor in [2, 3, 5, 6] {
        assert_eq!(
            owner.apply_reference(context(actor), root, retain),
            Err(UploadAdmissionError::NotProject)
        );
        assert_eq!(
            owner.apply_reference(
                context(actor),
                root,
                ReferenceRequest {
                    operation: ReferenceOperation::Release(first),
                    ..retain
                }
            ),
            Err(UploadAdmissionError::NotProject)
        );
    }
    assert_eq!(
        owner.apply_reference(
            UploadContext {
                service: p(9),
                ..context(4)
            },
            root,
            retain
        ),
        Err(UploadAdmissionError::WrongService)
    );
    let wrong_object = ObjectBinding::new(
        p(1),
        p(4),
        ObjectIdentity {
            namespace: id(2),
            ..first.object().identity()
        },
    )
    .unwrap();
    assert_eq!(
        owner.apply_reference(
            context(4),
            root,
            ReferenceRequest {
                operation: ReferenceOperation::Retain(ReferenceKey::new(
                    wrong_object,
                    second.reference()
                )),
                ..retain
            }
        ),
        Err(UploadAdmissionError::WrongNamespace)
    );
    let other_tenant = ObjectBinding::new(p(1), p(6), first.object().identity()).unwrap();
    assert_eq!(
        owner.apply_reference(
            context(6),
            root,
            ReferenceRequest {
                operation: ReferenceOperation::Release(ReferenceKey::new(
                    other_tenant,
                    first.reference()
                )),
                ..retain
            }
        ),
        Err(UploadAdmissionError::Reference(CatalogError::Request(
            crate::model::lifecycle::requests::ReferenceRequestError::BindingMismatch(
                crate::model::lifecycle::binding::ObjectBindingMismatch::Tenant
            )
        )))
    );
    assert_eq!(
        owner
            .catalog()
            .confirmed()
            .get(root)
            .unwrap()
            .receipt_count(),
        0
    );
}

#[test]
fn suspended_reference_operations_allow_exact_replay_and_release_without_fresh_retains() {
    let mut owner = admissions();
    let input = permission(1);
    let root = input.request.object.root;
    let first = input.request.object.first;
    let second = ReferenceKey::new(first.object(), ReferenceId::new(id(2)));
    owner.admit(context(4), input, 10).unwrap();
    prepare(&mut owner, &input);
    owner.expose(context(5), input.request, 11).unwrap();
    owner.confirm_upload(input.request).unwrap();
    let retain = ReferenceRequest {
        id: ReferenceRequestId::new(id(1)),
        operation: ReferenceOperation::Retain(second),
    };
    owner.apply_reference(context(4), root, retain).unwrap();
    set_active(&mut owner, false);
    let fresh = ReferenceRequest {
        id: ReferenceRequestId::new(id(2)),
        operation: ReferenceOperation::Retain(first),
    };
    assert_eq!(
        owner.apply_reference(context(4), root, fresh),
        Err(TenantError::Suspended.into())
    );
    assert_eq!(
        owner.apply_reference(context(4), root, retain),
        Ok(ReferenceRequestOutcome::Replayed {
            result: Ok(LifecycleChange::Changed)
        })
    );
    // Authorization applies to replay too, and changing the old payload conflicts.
    assert_eq!(
        owner.apply_reference(context(2), root, retain),
        Err(UploadAdmissionError::NotProject)
    );
    assert_eq!(
        owner.apply_reference(
            context(4),
            root,
            ReferenceRequest {
                operation: ReferenceOperation::Release(second),
                ..retain
            }
        ),
        Err(UploadAdmissionError::Reference(CatalogError::Request(
            crate::model::lifecycle::requests::ReferenceRequestError::RequestConflict
        )))
    );
    for (n, key) in [(2, first), (3, second)] {
        let release = ReferenceRequest {
            id: ReferenceRequestId::new(id(n)),
            operation: ReferenceOperation::Release(key),
        };
        assert_eq!(
            owner.apply_reference(context(4), root, release),
            Ok(ReferenceRequestOutcome::Recorded {
                result: Ok(LifecycleChange::Changed)
            })
        );
        assert_eq!(
            owner.apply_reference(context(4), root, release),
            Ok(ReferenceRequestOutcome::Replayed {
                result: Ok(LifecycleChange::Changed)
            })
        );
    }
    assert_eq!(owner.catalog().usage().logical_bytes, 0);
    assert_eq!(owner.catalog().usage().physical_bytes, 10);
    assert_eq!(owner.catalog().usage().liability_bytes, 10);
}

#[test]
fn only_project_admits_and_only_its_exact_uploader_exposes() {
    let mut owner = admissions();
    let input = permission(1);
    for caller in [2, 3, 5, 6] {
        // operator, cashier, uploader and unrelated project
        assert_eq!(
            owner.admit(context(caller), input, 10),
            Err(UploadAdmissionError::NotProject)
        );
        assert_eq!(owner.catalog().usage().operations, 0);
    }
    assert_eq!(
        owner.admit(context(4), input, 10),
        Ok(UploadAdmission::Reserved)
    );
    prepare(&mut owner, &input);
    let before = owner.catalog().usage();
    for caller in [2, 3, 4, 6] {
        assert_eq!(
            owner.expose(context(caller), input.request, 11),
            Err(UploadAdmissionError::NotUploader)
        );
        assert_eq!(owner.catalog().usage(), before);
    }
    assert_eq!(owner.expose(context(5), input.request, 11), Ok(()));
    assert_eq!(
        owner.expose(context(5), input.request, 12),
        Err(UploadAdmissionError::NotReserved(
            UploadPhase::ExposurePossible
        ))
    );
    assert_eq!(owner.catalog().usage(), before);
}

#[test]
fn bindings_payload_uploader_and_deadline_cannot_change_on_retry() {
    let mut owner = admissions();
    let input = permission(1);
    owner.admit(context(4), input, 10).unwrap();
    prepare(&mut owner, &input);
    let before = owner.catalog().usage();
    for changed in [
        UploadPermission {
            uploader: p(6),
            ..input
        },
        UploadPermission {
            expires_at_ns: 101,
            ..input
        },
        UploadPermission {
            request: UploadRequest {
                object: UploadObject {
                    bytes: 9,
                    ..input.request.object
                },
                ..input.request
            },
            ..input
        },
        UploadPermission {
            request: UploadRequest {
                object: UploadObject {
                    root: permission(2).request.object.root,
                    ..input.request.object
                },
                ..input.request
            },
            ..input
        },
    ] {
        assert_eq!(
            owner.admit(context(4), changed, 11),
            Err(UploadAdmissionError::PermissionConflict)
        );
        if changed.request != input.request {
            assert_eq!(
                owner.expose(context(5), changed.request, 11),
                Err(UploadAdmissionError::PermissionConflict)
            );
        }
        assert_eq!(owner.catalog().usage(), before);
    }
    assert_eq!(
        owner.admit(
            UploadContext {
                service: p(9),
                ..context(4)
            },
            input,
            11
        ),
        Err(UploadAdmissionError::WrongService)
    );
    let mut wrong = input;
    let old = input.request.object.first.object();
    wrong.request.object.first = ReferenceKey::new(
        ObjectBinding::new(
            p(1),
            p(4),
            ObjectIdentity {
                namespace: id(2),
                ..old.identity()
            },
        )
        .unwrap(),
        ReferenceId::new(id(1)),
    );
    assert_eq!(
        owner.admit(context(4), wrong, 11),
        Err(UploadAdmissionError::WrongNamespace)
    );
    assert_eq!(
        owner.expose(context(5), wrong.request, 11),
        Err(UploadAdmissionError::WrongNamespace)
    );
    assert_eq!(owner.catalog().usage(), before);
}

#[test]
fn expiration_and_backwards_time_neither_renew_permissions_nor_release_reservations() {
    let mut owner = admissions();
    let input = permission(1);
    assert_eq!(
        owner.admit(context(4), input, 100),
        Err(UploadAdmissionError::Expired)
    );
    owner.admit(context(4), input, 10).unwrap();
    prepare(&mut owner, &input);
    let before = owner.catalog().usage();
    assert_eq!(
        owner.expose(context(5), input.request, 9),
        Err(UploadAdmissionError::ClockReversed)
    );
    assert_eq!(
        owner.expose(context(5), input.request, 100),
        Err(UploadAdmissionError::Expired)
    );
    assert_eq!(
        owner.admit(context(4), input, 101),
        Ok(UploadAdmission::Existing(UploadPhase::Reserved))
    );
    assert_eq!(owner.catalog().usage(), before);
    // Separate explicit project revocation can cancel a never-exposed reservation.
    assert_eq!(
        owner.revoke(context(4), input.request),
        Ok(LifecycleChange::Changed)
    );
    assert_eq!(owner.catalog().usage().reserved_bytes, 0);
    assert_eq!(owner.catalog().usage().operations, 1);
    assert_eq!(
        owner.admit(context(4), input, 102),
        Ok(UploadAdmission::Existing(UploadPhase::Cancelled))
    );
    assert_eq!(
        owner.expose(context(5), input.request, 99),
        Err(UploadAdmissionError::Revoked)
    );
    assert_eq!(
        owner.revoke(context(4), input.request),
        Ok(LifecycleChange::Unchanged)
    );
}

#[test]
fn exposed_revocation_keeps_uncertainty_and_accepts_late_completion() {
    let mut owner = admissions();
    let input = permission(1);
    owner.admit(context(4), input, 10).unwrap();
    prepare(&mut owner, &input);
    owner.expose(context(5), input.request, 99).unwrap();
    let before = owner.catalog().usage();
    assert_eq!(
        owner.revoke(context(5), input.request),
        Err(UploadAdmissionError::NotProject)
    );
    owner.revoke(context(4), input.request).unwrap();
    assert_eq!(owner.catalog().usage(), before);
    assert_eq!(
        owner.expose(context(5), input.request, 99),
        Err(UploadAdmissionError::Revoked)
    );
    assert_eq!(
        owner.admit(context(4), input, 200),
        Ok(UploadAdmission::Existing(UploadPhase::ExposurePossible))
    );
    assert_eq!(
        owner.confirm_upload(input.request),
        Ok(LifecycleChange::Changed)
    );
    assert_eq!(owner.catalog().usage().reserved_bytes, 0);
    assert_eq!(owner.catalog().usage().physical_bytes, 10);
    assert_eq!(owner.catalog().usage().liability_bytes, 10);
}

#[test]
fn exact_lookup_preserves_original_permission_and_exposed_uncertainty_without_renewal() {
    let mut owner = admissions();
    let input = permission(1);
    owner.admit(context(4), input, 10).unwrap();
    prepare(&mut owner, &input);
    owner
        .expose_root(context(5), input.request.object.root, 11)
        .unwrap();
    owner.revoke(context(4), input.request).unwrap();
    let before = owner.catalog().usage();
    for actor in [2, 3, 6] {
        assert_eq!(
            owner.lookup(context(actor), input.request),
            Err(UploadAdmissionError::NotObserver)
        );
    }
    for actor in [4, 5] {
        assert_eq!(
            owner.lookup(context(actor), input.request),
            Ok(UploadPermissionView {
                permission: input,
                admitted_at_ns: 10,
                tenant_generation: NonZeroU64::MIN,
                revoked: true,
                phase: UploadPhase::ExposurePossible,
                manifest: UploadManifestState::Bound,
            })
        );
    }
    assert_eq!(
        owner.lookup(
            context(5),
            UploadRequest {
                id: UploadRequestId::new(id(9)),
                ..input.request
            }
        ),
        Err(UploadAdmissionError::UnknownPermission)
    );
    assert_eq!(
        owner.lookup(
            UploadContext {
                service: p(8),
                ..context(5)
            },
            input.request
        ),
        Err(UploadAdmissionError::WrongService)
    );
    assert_eq!(owner.catalog().usage(), before);
    assert_eq!(
        owner.expose(context(5), input.request, 11),
        Err(UploadAdmissionError::Revoked)
    );
}

#[test]
fn failed_consumer_registration_and_replayed_completion_keep_one_reference_and_its_obligations() {
    let mut owner = admissions();
    let input = permission(1);
    owner.admit(context(4), input, 10).unwrap();
    prepare(&mut owner, &input);
    owner.expose(context(5), input.request, 11).unwrap();
    owner.confirm_upload(input.request).unwrap(); // independently supplied provider fact
    // Consumer registration may fail or lose its reply: no service transition occurs.
    let confirmed = owner.catalog().usage();
    assert_eq!(
        owner.admit(context(4), input, 200),
        Ok(UploadAdmission::Existing(UploadPhase::Confirmed))
    );
    assert_eq!(
        owner.confirm_upload(input.request),
        Ok(LifecycleChange::Unchanged)
    );
    assert_eq!(owner.catalog().usage(), confirmed);
    let release = ReferenceRequest {
        id: ReferenceRequestId::new(id(1)),
        operation: ReferenceOperation::Release(input.request.object.first),
    };
    owner
        .apply_reference(context(4), input.request.object.root, release)
        .unwrap();
    let released = owner.catalog().usage();
    assert_eq!(released.logical_bytes, 0);
    assert_eq!(released.physical_bytes, 10);
    assert_eq!(released.liability_bytes, 10);
    owner.confirm_upload(input.request).unwrap();
    assert_eq!(owner.catalog().usage(), released);
    owner
        .confirm_provider_deleted(
            input.request.object.root,
            input.request.object.first.object(),
        )
        .unwrap();
    assert_eq!(owner.catalog().usage().physical_bytes, 0);
    assert_eq!(owner.catalog().usage().liability_bytes, 10);
    owner
        .confirm_billing_stopped(
            input.request.object.root,
            input.request.object.first.object(),
        )
        .unwrap();
    assert_eq!(owner.catalog().usage().liability_bytes, 0);
    assert_eq!(owner.catalog().usage().operations, 1);
}

#[test]
fn invalid_and_over_capacity_grants_cannot_allocate_permission_or_lose_lifetime_history() {
    let mut owner = admissions();
    let mut empty = permission(1);
    empty.request.object.bytes = 0;
    assert_eq!(
        owner.admit(context(4), empty, 10),
        Err(UploadAdmissionError::EmptyObject)
    );
    assert_eq!(
        owner.lookup(context(4), empty.request),
        Err(UploadAdmissionError::UnknownPermission)
    );
    assert_eq!(owner.catalog().usage().operations, 0);
    for uploader in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            owner.admit(
                context(4),
                UploadPermission {
                    uploader,
                    ..permission(1)
                },
                10
            ),
            Err(UploadAdmissionError::InvalidUploader)
        );
    }
    let mut large = permission(1);
    large.request.object.bytes = 11;
    assert_eq!(
        owner.admit(context(4), large, 10),
        Err(UploadAdmissionError::ObjectTooLarge)
    );
    assert_eq!(
        owner.expose(context(5), large.request, 11),
        Err(UploadAdmissionError::UnknownPermission)
    );
    for n in [1, 2] {
        let input = permission(n);
        owner.admit(context(4), input, 10).unwrap();
        prepare(&mut owner, &input);
        owner.revoke(context(4), input.request).unwrap();
    }
    let before = owner.catalog().usage();
    assert_eq!(
        owner.admit(context(4), permission(3), 11),
        Err(UploadAdmissionError::Catalog(UploadError::Catalog(
            CatalogError::Capacity(CatalogCapacity::Objects)
        )))
    );
    assert_eq!(
        owner.expose(context(5), permission(3).request, 12),
        Err(UploadAdmissionError::UnknownPermission)
    );
    assert_eq!(owner.catalog().usage(), before);
}

#[test]
fn root_only_certificate_lookup_uses_retained_project_operation_and_cannot_rebind_cancelled_roots()
{
    use crate::model::lifecycle::roots::RootClaimError;
    let mut owner = admissions();
    let input = permission(1);
    let root = input.request.object.root;
    assert_eq!(
        owner.expose_root(context(5), root, 11),
        Err(UploadAdmissionError::UnknownPermission)
    );
    owner.admit(context(4), input, 10).unwrap();
    prepare(&mut owner, &input);
    assert_eq!(
        owner.expose_root(context(6), root, 11),
        Err(UploadAdmissionError::NotUploader)
    );
    assert_eq!(
        owner.expose_root(
            UploadContext {
                service: p(9),
                ..context(5)
            },
            root,
            11
        ),
        Err(UploadAdmissionError::WrongService)
    );
    assert_eq!(owner.expose_root(context(5), root, 11), Ok(input.request));
    assert_eq!(
        owner.expose_root(context(5), root, 12),
        Err(UploadAdmissionError::NotReserved(
            UploadPhase::ExposurePossible
        ))
    );

    let second = permission(2);
    owner.admit(context(4), second, 12).unwrap();
    owner.revoke(context(4), second.request).unwrap();
    assert_eq!(
        owner.expose_root(context(5), second.request.object.root, 13),
        Err(UploadAdmissionError::Revoked)
    );
    // Use another fresh owner with available lifetime capacity to isolate the
    // immutable root check from global capacity exhaustion.
    let mut other = admissions();
    other.admit(context(4), input, 10).unwrap();
    other.revoke(context(4), input.request).unwrap();
    let conflicting = UploadPermission {
        request: UploadRequest {
            object: UploadObject {
                root,
                ..second.request.object
            },
            ..second.request
        },
        ..second
    };
    assert_eq!(
        other.admit(context(4), conflicting, 12),
        Err(UploadAdmissionError::Catalog(UploadError::Root(
            RootClaimError::RootAlreadyClaimed
        )))
    );
    assert_eq!(
        other.expose_root(context(5), root, 13),
        Err(UploadAdmissionError::Revoked)
    );
}
