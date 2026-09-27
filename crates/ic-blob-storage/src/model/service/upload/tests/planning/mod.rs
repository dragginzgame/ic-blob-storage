use super::*;
use crate::model::service::upload::planning::{AdmissionCapacityLookup, AdmissionCapacityView};

fn scope(tenant: u8) -> AdmissionCapacityLookup {
    AdmissionCapacityLookup {
        tenant: p(tenant),
        namespace: id(1),
    }
}

fn view(owner: &UploadAdmissions, tenant: u8) -> AdmissionCapacityView {
    owner
        .admission_capacity(context(tenant), scope(tenant))
        .unwrap()
}

fn two_tenants() -> UploadAdmissions {
    let sample = empty_admissions();
    let mut limits = sample.config.limits();
    limits.catalog.max_objects = count(4);
    limits.catalog.max_tenant_objects = count(3);
    limits.manifests.max_tenant_chunks = count(3);
    let mut owner = UploadAdmissions::new(
        ServiceConfiguration::new(sample.config.bindings(), limits, sample.config.billing())
            .unwrap(),
    );
    for tenant in [4, 6] {
        owner
            .update_tenant(
                context(2),
                TenantUpdate {
                    tenant: p(tenant),
                    expected: None,
                    active: true,
                },
            )
            .unwrap();
    }
    owner
}

#[test]
fn capacity_authenticates_scope_and_preserves_suspended_inspection() {
    let mut owner = empty_admissions();
    assert_eq!(
        owner.admission_capacity(context(4), scope(4)),
        Err(UploadAdmissionError::Tenant(TenantError::NotEnrolled))
    );
    let enrollment = owner
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: None,
                active: true,
            },
        )
        .unwrap();
    let initial = view(&owner, 4);
    for actor in [2, 5, 6] {
        assert_eq!(
            owner.admission_capacity(context(actor), scope(4)),
            Err(UploadAdmissionError::NotProject)
        );
    }
    assert_eq!(
        owner.admission_capacity(
            UploadContext {
                service: p(9),
                ..context(4)
            },
            scope(4)
        ),
        Err(UploadAdmissionError::WrongService)
    );
    assert_eq!(
        owner.admission_capacity(
            context(4),
            AdmissionCapacityLookup {
                namespace: id(2),
                ..scope(4)
            }
        ),
        Err(UploadAdmissionError::WrongNamespace)
    );
    let suspended = owner
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: Some(enrollment),
                active: false,
            },
        )
        .unwrap();
    assert_eq!(
        view(&owner, 4),
        AdmissionCapacityView {
            enrollment: suspended,
            ..initial
        }
    );
    assert_eq!(
        owner.admit(context(4), permission(1), 10),
        Err(UploadAdmissionError::Tenant(TenantError::Suspended))
    );
    assert_eq!(owner.catalog().usage().operations, 0);
}

#[test]
fn cancellation_and_failed_or_exact_retries_preserve_lifetime_headroom() {
    let mut owner = admissions();
    let initial = view(&owner, 4);
    let first = permission(1);
    owner.admit(context(4), first, 10).unwrap();
    let reserved = view(&owner, 4);
    assert_eq!(reserved.remaining_objects, 1);
    assert_eq!(reserved.remaining_active_uploads, 1);
    assert_eq!(reserved.remaining_manifest_chunks, 3);
    assert_eq!(reserved.remaining_bytes, 10);
    let mut conflict = first;
    conflict.expires_at_ns += 1;
    assert_eq!(
        owner.admit(context(4), conflict, 10),
        Err(UploadAdmissionError::PermissionConflict)
    );
    assert_eq!(view(&owner, 4), reserved);
    owner.revoke(context(4), first.request).unwrap();
    let cancelled = view(&owner, 4);
    assert_eq!(cancelled.remaining_objects, reserved.remaining_objects);
    assert_eq!(
        cancelled.remaining_manifest_chunks,
        reserved.remaining_manifest_chunks
    );
    assert_eq!(
        cancelled.remaining_active_uploads,
        initial.remaining_active_uploads
    );
    assert_eq!(cancelled.remaining_bytes, initial.remaining_bytes);
    assert_eq!(
        owner.admit(context(4), first, 200),
        Ok(UploadAdmission::Existing(UploadPhase::Cancelled))
    );
    assert_eq!(view(&owner, 4), cancelled);
    let second = permission(2);
    owner.admit(context(4), second, 10).unwrap();
    owner.revoke(context(4), second.request).unwrap();
    assert_eq!(view(&owner, 4).remaining_objects, 0);
    assert_eq!(view(&owner, 4).remaining_bytes, initial.remaining_bytes);
    assert!(matches!(
        owner.admit(context(4), permission(3), 10),
        Err(UploadAdmissionError::Catalog(UploadError::Catalog(
            CatalogError::Capacity(CatalogCapacity::Objects)
        )))
    ));
}

#[test]
fn other_tenant_obligations_limit_headroom_until_billing_really_stops() {
    let mut owner = two_tenants();
    let first = permission(1);
    let mut other = permission(2);
    let reference = other.request.object.first;
    other.request.object.first = ReferenceKey::new(
        ObjectBinding::new(p(1), p(6), reference.object().identity()).unwrap(),
        reference.reference(),
    );
    owner.admit(context(4), first, 10).unwrap();
    owner.admit(context(6), other, 10).unwrap();
    let full = view(&owner, 4);
    assert_eq!(full.remaining_objects, 2);
    assert_eq!(full.remaining_manifest_chunks, 2);
    assert_eq!(full.remaining_active_uploads, 0);
    assert_eq!(full.remaining_bytes, 0);
    prepare(&mut owner, &first);
    owner.expose(context(5), first.request, 11).unwrap();
    assert_eq!(view(&owner, 4), full);
    owner.confirm_upload(first.request).unwrap();
    assert_eq!(view(&owner, 4).remaining_active_uploads, 1);
    assert_eq!(view(&owner, 4).remaining_bytes, 0);
    owner
        .apply_reference(
            context(4),
            first.request.object.root,
            ReferenceRequest {
                id: ReferenceRequestId::new(id(1)),
                operation: ReferenceOperation::Release(first.request.object.first),
            },
        )
        .unwrap();
    assert_eq!(owner.catalog().tenant_usage(p(4)).logical_bytes, 0);
    assert_eq!(view(&owner, 4).remaining_bytes, 0);
    assert!(matches!(
        owner.admit(context(4), permission(3), 12),
        Err(UploadAdmissionError::Catalog(UploadError::Catalog(
            CatalogError::Capacity(CatalogCapacity::PhysicalBytes)
        )))
    ));
    owner
        .confirm_provider_deleted(
            first.request.object.root,
            first.request.object.first.object(),
        )
        .unwrap();
    assert_eq!(owner.catalog().usage().physical_bytes, 10);
    assert_eq!(view(&owner, 4).remaining_bytes, 0);
    assert!(matches!(
        owner.admit(context(4), permission(3), 12),
        Err(UploadAdmissionError::Catalog(UploadError::Catalog(
            CatalogError::Capacity(CatalogCapacity::LiabilityBytes)
        )))
    ));
    owner
        .confirm_billing_stopped(
            first.request.object.root,
            first.request.object.first.object(),
        )
        .unwrap();
    let settled = view(&owner, 4);
    assert_eq!(settled.remaining_bytes, 10);
    assert_eq!(settled.remaining_objects, full.remaining_objects);
    assert_eq!(
        settled.remaining_manifest_chunks,
        full.remaining_manifest_chunks
    );
    owner.admit(context(4), permission(3), 12).unwrap();
    assert_eq!(view(&owner, 4).remaining_bytes, 0);
}

#[test]
fn byte_headroom_retains_u128_width() {
    let sample = empty_admissions();
    let mut limits = sample.config.limits();
    let maximum = u128::from(u64::MAX) + 100;
    limits.catalog.max_physical_bytes = id(maximum);
    limits.catalog.max_liability_bytes = id(maximum + 10);
    limits.catalog.max_tenant_logical_bytes = id(maximum + 20);
    let mut owner = UploadAdmissions::new(
        ServiceConfiguration::new(sample.config.bindings(), limits, sample.config.billing())
            .unwrap(),
    );
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
    assert_eq!(view(&owner, 4).remaining_bytes, maximum);
    owner.admit(context(4), permission(1), 10).unwrap();
    assert_eq!(view(&owner, 4).remaining_bytes, maximum - 10);
}

#[test]
fn manifest_exhaustion_remains_visible_when_bytes_and_object_slots_remain() {
    let sample = empty_admissions();
    let mut limits = sample.config.limits();
    limits.manifests.max_chunks = count(1);
    limits.manifests.max_tenant_chunks = count(1);
    let mut owner = UploadAdmissions::new(
        ServiceConfiguration::new(sample.config.bindings(), limits, sample.config.billing())
            .unwrap(),
    );
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
    let first = permission(1);
    owner.admit(context(4), first, 10).unwrap();
    owner.revoke(context(4), first.request).unwrap();
    let observed = view(&owner, 4);
    assert_eq!(observed.remaining_bytes, 20);
    assert_eq!(observed.remaining_objects, 1);
    assert_eq!(observed.remaining_manifest_chunks, 0);
    assert_eq!(
        owner.admit(context(4), permission(2), 11),
        Err(UploadAdmissionError::ManifestCapacity(
            UploadManifestLimit::Tenant
        ))
    );
    assert_eq!(view(&owner, 4), observed);
}
