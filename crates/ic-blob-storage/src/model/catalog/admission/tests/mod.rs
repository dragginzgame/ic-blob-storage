use super::*;
use crate::model::lifecycle::{
    LifecyclePhase, ReferenceId,
    binding::{ObjectIdentity, ReferenceKey},
    requests::{ReferenceOperation, ReferenceRequestId},
};

fn n(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).expect("positive")
}
fn bound(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("positive")
}
fn p(value: u8) -> Principal {
    Principal::from_slice(&[value, 1])
}
fn root(value: u8) -> ProviderRootHash {
    ProviderRootHash::try_from([value; 32].as_slice()).expect("root")
}
fn limits() -> CatalogLimits {
    CatalogLimits {
        max_objects: bound(8),
        max_tenant_objects: bound(6),
        max_physical_bytes: n(1000),
        max_liability_bytes: n(1000),
        max_tenant_logical_bytes: n(1000),
        max_references_per_object: bound(1),
        max_receipts_per_object: bound(1),
    }
}
fn uploads() -> UploadLimits {
    UploadLimits {
        max_active: bound(4),
        max_tenant_active: bound(3),
    }
}
fn owner() -> UploadCatalog {
    UploadCatalog::new(p(1), limits(), uploads()).expect("owner")
}
fn request(id: u8, tenant: u8, bytes: u64) -> UploadRequest {
    UploadRequest {
        id: UploadRequestId::new(n(u128::from(id))),
        object: UploadObject {
            root: root(id),
            bytes,
            first: ReferenceKey::new(
                ObjectBinding::new(
                    p(1),
                    p(tenant),
                    ObjectIdentity {
                        namespace: n(1),
                        object: n(u128::from(id)),
                        incarnation: n(1),
                    },
                )
                .expect("binding"),
                ReferenceId::new(n(1)),
            ),
        },
        content: ContentDigest::compute(&[id]),
    }
}
fn actor(request: UploadRequest) -> Principal {
    request.object.first.object().tenant()
}
fn confirm(owner: &mut UploadCatalog, request: UploadRequest) {
    owner
        .mark_exposure_possible(actor(request), request)
        .expect("exposure");
    assert_eq!(owner.confirm_upload(request), Ok(LifecycleChange::Changed));
}
fn release(owner: &mut UploadCatalog, request: UploadRequest) {
    assert_eq!(
        owner.apply_reference(
            request.object.root,
            actor(request),
            ReferenceRequest {
                id: ReferenceRequestId::new(n(1)),
                operation: ReferenceOperation::Release(request.object.first),
            }
        ),
        Ok(ReferenceRequestOutcome::Recorded {
            result: Ok(LifecycleChange::Changed)
        })
    );
}
fn snapshot(
    owner: &UploadCatalog,
) -> (BlobCatalog, Vec<(UploadRequest, UploadPhase)>, UploadUsage) {
    (
        owner.catalog.clone(),
        owner
            .operations
            .values()
            .map(|op| (op.request, op.phase))
            .collect(),
        owner.usage(),
    )
}

#[test]
fn confirmation_transfers_reserved_capacity_without_double_counting() {
    let mut owner = UploadCatalog::new(
        p(1),
        CatalogLimits {
            max_physical_bytes: n(100),
            max_liability_bytes: n(100),
            max_tenant_logical_bytes: n(100),
            ..limits()
        },
        uploads(),
    )
    .expect("owner");
    let a = request(1, 2, 60);
    let b = request(2, 3, 40);
    for request in [a, b] {
        assert_eq!(
            owner.reserve(actor(request), request),
            Ok(UploadAdmission::Reserved)
        );
    }
    let full = owner.usage();
    assert_eq!(full.physical_bytes, 100);
    assert_eq!(full.reserved_bytes, 100);
    assert_eq!(owner.confirmed().usage().objects, 0);
    confirm(&mut owner, b);
    assert_eq!(
        owner.usage(),
        UploadUsage {
            active_reservations: 1,
            reserved_bytes: 60,
            ..full
        }
    );
    assert_eq!(owner.confirmed().usage().physical_bytes, 40);
    confirm(&mut owner, a);
    assert_eq!(
        owner.usage(),
        UploadUsage {
            active_reservations: 0,
            reserved_bytes: 0,
            ..full
        }
    );
    assert_eq!(owner.confirmed().usage().physical_bytes, 100);
    assert_eq!(owner.tenant_usage(p(2)).logical_bytes, 60);
    assert_eq!(owner.tenant_usage(p(3)).logical_bytes, 40);
    assert_eq!(owner.tenant_usage(p(9)), UploadUsage::default());
}

#[test]
fn all_admission_bounds_reject_atomically_and_preserve_exact_replay() {
    let cases = [
        (
            CatalogLimits {
                max_objects: bound(1),
                ..limits()
            },
            uploads(),
            CatalogError::Capacity(CatalogCapacity::Objects).into(),
        ),
        (
            CatalogLimits {
                max_tenant_objects: bound(1),
                ..limits()
            },
            uploads(),
            CatalogError::Capacity(CatalogCapacity::TenantObjects).into(),
        ),
        (
            CatalogLimits {
                max_physical_bytes: n(100),
                ..limits()
            },
            uploads(),
            CatalogError::Capacity(CatalogCapacity::PhysicalBytes).into(),
        ),
        (
            CatalogLimits {
                max_liability_bytes: n(100),
                ..limits()
            },
            uploads(),
            CatalogError::Capacity(CatalogCapacity::LiabilityBytes).into(),
        ),
        (
            CatalogLimits {
                max_tenant_logical_bytes: n(100),
                ..limits()
            },
            uploads(),
            CatalogError::Capacity(CatalogCapacity::TenantLogicalBytes).into(),
        ),
        (
            limits(),
            UploadLimits {
                max_active: bound(1),
                ..uploads()
            },
            UploadError::ActiveLimit,
        ),
        (
            limits(),
            UploadLimits {
                max_tenant_active: bound(1),
                ..uploads()
            },
            UploadError::TenantActiveLimit,
        ),
    ];
    for (catalog, uploads, error) in cases {
        let mut owner = UploadCatalog::new(p(1), catalog, uploads).expect("owner");
        let a = request(1, 2, 100);
        let b = request(2, 2, 1);
        owner.reserve(actor(a), a).expect("admit first");
        let before = snapshot(&owner);
        assert_eq!(owner.reserve(actor(b), b), Err(error));
        assert_eq!(
            owner.reserve(actor(a), a),
            Ok(UploadAdmission::Existing(UploadPhase::Reserved))
        );
        assert_eq!(
            owner.catalog.claims.resolve(b.object.root),
            Err(RootClaimError::UnknownRoot)
        );
        assert_eq!(snapshot(&owner), before);
        // Full admission capacity cannot prevent transfer to the confirmed catalog.
        confirm(&mut owner, a);
        release(&mut owner, a);
    }
}

#[test]
fn competing_tenants_share_global_bytes_but_ids_and_logical_bounds_are_scoped() {
    let mut owner = UploadCatalog::new(
        p(1),
        CatalogLimits {
            max_tenant_logical_bytes: n(60),
            max_physical_bytes: n(100),
            ..limits()
        },
        uploads(),
    )
    .expect("owner");
    let a = request(1, 2, 60);
    let b = UploadRequest {
        id: a.id,
        ..request(2, 3, 40)
    };
    owner.reserve(actor(a), a).expect("tenant a");
    assert_eq!(
        owner.reserve(p(2), request(3, 2, 1)),
        Err(CatalogError::Capacity(CatalogCapacity::TenantLogicalBytes).into())
    );
    owner
        .reserve(actor(b), b)
        .expect("same request ID in another tenant");
    assert_eq!(
        owner.reserve(p(4), request(3, 4, 1)),
        Err(CatalogError::Capacity(CatalogCapacity::PhysicalBytes).into())
    );
    owner.cancel(actor(a), a).expect("cancel unexposed");
    owner
        .reserve(p(4), request(3, 4, 60))
        .expect("freed global capacity");
    assert_eq!(owner.usage().physical_bytes, 100);
    assert_eq!(owner.tenant_usage(p(2)).operations, 1);
    assert_eq!(owner.tenant_usage(p(2)).logical_bytes, 0);
}

#[test]
fn exposure_uncertainty_retains_capacity_and_cannot_be_cancelled_or_reset() {
    let mut owner = UploadCatalog::new(
        p(1),
        limits(),
        UploadLimits {
            max_active: bound(1),
            ..uploads()
        },
    )
    .expect("owner");
    let a = request(1, 2, 100);
    owner.reserve(actor(a), a).expect("reserve");
    assert_eq!(
        owner.confirm_upload(a),
        Err(UploadError::InvalidPhase(UploadPhase::Reserved))
    );
    assert_eq!(
        owner.mark_exposure_possible(actor(a), a),
        Ok(LifecycleChange::Changed)
    );
    let before = snapshot(&owner);
    for _ in 0..3 {
        assert_eq!(
            owner.mark_exposure_possible(actor(a), a),
            Ok(LifecycleChange::Unchanged)
        );
        assert_eq!(
            owner.reserve(actor(a), a),
            Ok(UploadAdmission::Existing(UploadPhase::ExposurePossible))
        );
        assert_eq!(
            owner.cancel(actor(a), a),
            Err(UploadError::InvalidPhase(UploadPhase::ExposurePossible))
        );
        assert_eq!(
            owner.reserve(p(3), request(2, 3, 0)),
            Err(UploadError::ActiveLimit)
        );
        assert_eq!(
            owner.confirm_provider_deleted(a.object.root, a.object.first.object()),
            Err(CatalogError::UnknownRoot)
        );
        assert_eq!(
            owner.confirm_billing_stopped(a.object.root, a.object.first.object()),
            Err(CatalogError::UnknownRoot)
        );
        assert_eq!(snapshot(&owner), before);
    }
    owner
        .confirm_upload(a)
        .expect("independently resolved completion");
    owner
        .reserve(p(3), request(2, 3, 0))
        .expect("active slot freed by resolution");
    assert_eq!(owner.usage().liability_bytes, 100);
}

#[test]
fn cancellation_retains_history_and_never_reopens_the_operation() {
    let mut owner = UploadCatalog::new(
        p(1),
        CatalogLimits {
            max_objects: bound(1),
            ..limits()
        },
        uploads(),
    )
    .expect("owner");
    let a = request(1, 2, 100);
    owner.reserve(actor(a), a).expect("reserve");
    assert_eq!(owner.cancel(actor(a), a), Ok(LifecycleChange::Changed));
    let before = snapshot(&owner);
    assert_eq!(
        owner.usage(),
        UploadUsage {
            operations: 1,
            ..UploadUsage::default()
        }
    );
    assert_eq!(owner.cancel(actor(a), a), Ok(LifecycleChange::Unchanged));
    assert_eq!(
        owner.reserve(actor(a), a),
        Ok(UploadAdmission::Existing(UploadPhase::Cancelled))
    );
    assert_eq!(
        owner.mark_exposure_possible(actor(a), a),
        Err(UploadError::InvalidPhase(UploadPhase::Cancelled))
    );
    assert_eq!(
        owner.confirm_upload(a),
        Err(UploadError::InvalidPhase(UploadPhase::Cancelled))
    );
    assert_eq!(
        owner.reserve(p(3), request(2, 3, 0)),
        Err(CatalogError::Capacity(CatalogCapacity::Objects).into())
    );
    assert_eq!(
        owner.catalog.claims.resolve(a.object.root),
        Ok(a.object.first.object())
    );
    assert_eq!(snapshot(&owner), before);
}

#[test]
fn exact_payload_actor_and_scope_are_required_even_for_retries() {
    let mut owner = owner();
    let a = request(1, 2, 100);
    owner.reserve(actor(a), a).expect("reserve");
    let before = snapshot(&owner);
    let binding = a.object.first.object();
    let changed_bindings = [
        ObjectBinding::new(p(9), p(2), binding.identity()).expect("service"),
        ObjectBinding::new(
            p(1),
            p(2),
            ObjectIdentity {
                namespace: n(2),
                ..binding.identity()
            },
        )
        .expect("namespace"),
        ObjectBinding::new(
            p(1),
            p(2),
            ObjectIdentity {
                object: n(2),
                ..binding.identity()
            },
        )
        .expect("object"),
        ObjectBinding::new(
            p(1),
            p(2),
            ObjectIdentity {
                incarnation: n(2),
                ..binding.identity()
            },
        )
        .expect("incarnation"),
    ];
    let mut changed = vec![
        UploadRequest {
            content: ContentDigest::compute(b"different"),
            ..a
        },
        UploadRequest {
            object: UploadObject {
                root: root(2),
                ..a.object
            },
            ..a
        },
        UploadRequest {
            object: UploadObject {
                bytes: 99,
                ..a.object
            },
            ..a
        },
        UploadRequest {
            object: UploadObject {
                first: ReferenceKey::new(binding, ReferenceId::new(n(2))),
                ..a.object
            },
            ..a
        },
    ];
    changed.extend(changed_bindings.map(|binding| UploadRequest {
        object: UploadObject {
            first: ReferenceKey::new(binding, a.object.first.reference()),
            ..a.object
        },
        ..a
    }));
    for candidate in changed {
        let expected = if candidate.object.first.object().service() == p(1) {
            UploadError::RequestConflict
        } else {
            UploadError::Root(RootClaimError::WrongService)
        };
        assert_eq!(owner.reserve(p(2), candidate), Err(expected));
        assert_eq!(owner.cancel(p(2), candidate), Err(expected));
        assert_eq!(owner.mark_exposure_possible(p(2), candidate), Err(expected));
        assert_eq!(owner.phase(p(2), candidate), Err(expected));
        assert_eq!(
            owner.confirm_upload(candidate),
            Err(UploadError::RequestConflict)
        );
    }
    assert_eq!(snapshot(&owner), before);
}

#[test]
fn wrong_actors_and_unknown_operations_cannot_mutate_or_read_reservations() {
    let mut owner = owner();
    let a = request(1, 2, 100);
    owner.reserve(actor(a), a).expect("reserve");
    let before = snapshot(&owner);
    for foreign in [
        p(1),
        p(3),
        Principal::anonymous(),
        Principal::management_canister(),
    ] {
        assert_eq!(owner.reserve(foreign, a), Err(UploadError::Denied));
        assert_eq!(owner.cancel(foreign, a), Err(UploadError::Denied));
        assert_eq!(
            owner.mark_exposure_possible(foreign, a),
            Err(UploadError::Denied)
        );
        assert_eq!(owner.phase(foreign, a), Err(UploadError::Denied));
    }
    assert_eq!(
        owner.confirm_upload(request(2, 2, 1)),
        Err(UploadError::UnknownRequest)
    );
    assert_eq!(snapshot(&owner), before);
}

#[test]
fn different_operations_cannot_reuse_roots_or_object_lifetimes() {
    let mut owner = owner();
    let a = request(1, 2, 100);
    owner.reserve(actor(a), a).expect("reserve");
    for cancelled in [false, true] {
        if cancelled {
            owner.cancel(actor(a), a).expect("cancel");
        }
        let before = snapshot(&owner);
        let new_id = UploadRequest {
            id: UploadRequestId::new(n(2)),
            ..a
        };
        let other_tenant = UploadRequest {
            object: UploadObject {
                root: a.object.root,
                ..request(3, 3, 100).object
            },
            ..request(3, 3, 100)
        };
        for request in [new_id, other_tenant] {
            assert_eq!(
                owner.reserve(actor(request), request),
                Err(UploadError::Root(RootClaimError::RootAlreadyClaimed))
            );
        }
        assert_eq!(
            owner.reserve(
                p(2),
                UploadRequest {
                    object: UploadObject {
                        root: root(2),
                        ..a.object
                    },
                    ..new_id
                }
            ),
            Err(UploadError::Root(RootClaimError::ObjectAlreadyClaimed))
        );
        assert_eq!(snapshot(&owner), before);
    }
}

#[test]
fn release_deletion_and_billing_free_distinct_capacities_and_replay_never_resurrects() {
    let mut owner = UploadCatalog::new(
        p(1),
        CatalogLimits {
            max_physical_bytes: n(100),
            max_liability_bytes: n(100),
            ..limits()
        },
        uploads(),
    )
    .expect("owner");
    let a = request(1, 2, 100);
    let b = request(2, 3, 100);
    owner.reserve(actor(a), a).expect("reserve");
    confirm(&mut owner, a);
    release(&mut owner, a);
    assert_eq!(owner.usage().logical_bytes, 0);
    assert_eq!(
        owner.reserve(actor(b), b),
        Err(CatalogError::Capacity(CatalogCapacity::PhysicalBytes).into())
    );
    owner
        .confirm_provider_deleted(a.object.root, a.object.first.object())
        .expect("physical deletion");
    assert_eq!(owner.usage().physical_bytes, 0);
    assert_eq!(
        owner.reserve(actor(b), b),
        Err(CatalogError::Capacity(CatalogCapacity::LiabilityBytes).into())
    );
    owner
        .confirm_billing_stopped(a.object.root, a.object.first.object())
        .expect("billing cessation");
    let before = snapshot(&owner);
    assert_eq!(
        owner.reserve(actor(a), a),
        Ok(UploadAdmission::Existing(UploadPhase::Confirmed))
    );
    assert_eq!(owner.confirm_upload(a), Ok(LifecycleChange::Unchanged));
    assert_eq!(
        owner.mark_exposure_possible(actor(a), a),
        Err(UploadError::InvalidPhase(UploadPhase::Confirmed))
    );
    assert_eq!(
        owner.cancel(actor(a), a),
        Err(UploadError::InvalidPhase(UploadPhase::Confirmed))
    );
    assert_eq!(snapshot(&owner), before);
    assert_eq!(
        owner
            .confirmed()
            .get(a.object.root)
            .expect("history")
            .lifecycle()
            .phase(),
        LifecyclePhase::Settled
    );
    owner.reserve(actor(b), b).expect("new capacity");
    assert_eq!(owner.usage().operations, 2);
    assert_eq!(owner.usage().liability_bytes, 100);
}

#[test]
fn zero_bytes_still_consume_tenant_slots_and_wide_totals_are_exact() {
    let mut owner = UploadCatalog::new(
        p(1),
        CatalogLimits {
            max_tenant_objects: bound(2),
            max_physical_bytes: n(u128::MAX),
            max_liability_bytes: n(u128::MAX),
            max_tenant_logical_bytes: n(u128::MAX),
            ..limits()
        },
        uploads(),
    )
    .expect("owner");
    for id in [1, 2] {
        let r = request(id, 2, 0);
        owner.reserve(actor(r), r).expect("zero bytes");
        owner.cancel(actor(r), r).expect("cancel");
    }
    assert_eq!(
        owner.reserve(p(2), request(3, 2, 0)),
        Err(CatalogError::Capacity(CatalogCapacity::TenantObjects).into())
    );
    for id in [3, 4] {
        owner
            .reserve(p(3), request(id, 3, u64::MAX))
            .expect("wide bytes");
    }
    assert_eq!(owner.usage().reserved_bytes, 2 * u128::from(u64::MAX));
    assert_eq!(owner.usage().physical_bytes, 2 * u128::from(u64::MAX));
    confirm(&mut owner, request(3, 3, u64::MAX));
    assert_eq!(owner.usage().physical_bytes, 2 * u128::from(u64::MAX));
    assert_eq!(owner.tenant_usage(p(2)).operations, 2);
}
