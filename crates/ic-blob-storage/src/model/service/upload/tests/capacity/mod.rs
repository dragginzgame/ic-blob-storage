use super::*;
use crate::model::identity::caffeine::CAFFEINE_CHUNK_BYTES;

fn owner(global: usize, tenant: usize) -> UploadAdmissions {
    let sample = empty_admissions();
    let mut limits = sample.config.limits();
    limits.max_object_bytes = NonZeroU64::new(CAFFEINE_CHUNK_BYTES as u64 + 1).unwrap();
    limits.manifests = ServiceManifestLimits {
        max_chunks: count(global),
        max_tenant_chunks: count(tenant),
    };
    limits.catalog.max_objects = count(8);
    limits.catalog.max_tenant_objects = count(4);
    limits.catalog.max_physical_bytes = id(10 * CAFFEINE_CHUNK_BYTES as u128);
    limits.catalog.max_liability_bytes = limits.catalog.max_physical_bytes;
    limits.catalog.max_tenant_logical_bytes = limits.catalog.max_physical_bytes;
    let mut owner = UploadAdmissions::new(
        ServiceConfiguration::new(sample.config.bindings(), limits, sample.config.billing())
            .unwrap(),
    );
    for tenant in [p(4), p(6)] {
        owner
            .update_tenant(
                context(2),
                TenantUpdate {
                    tenant,
                    expected: None,
                    active: true,
                },
            )
            .unwrap();
    }
    owner
}

fn sized(n: u8, tenant: u8, bytes: u64) -> UploadPermission {
    let mut input = permission(n);
    let binding = input.request.object.first.object();
    input.request.object.bytes = bytes;
    input.request.object.first = ReferenceKey::new(
        ObjectBinding::new(binding.service(), p(tenant), binding.identity()).unwrap(),
        ReferenceId::new(id(1)),
    );
    input
}

#[test]
fn cancellation_releases_bytes_but_retains_leaf_capacity_and_exact_retries() {
    let mut owner = owner(3, 2);
    // One byte beyond a leaf requires two slots, before any manifest is supplied.
    let input = sized(1, 4, CAFFEINE_CHUNK_BYTES as u64 + 1);
    owner.admit(context(4), input, 10).unwrap();
    owner.revoke(context(4), input.request).unwrap();
    assert_eq!(owner.catalog().usage().reserved_bytes, 0);
    let before = owner.catalog().usage();
    assert_eq!(
        owner.admit(context(4), input, 101),
        Ok(UploadAdmission::Existing(UploadPhase::Cancelled))
    );
    let next = sized(2, 4, 1);
    assert_eq!(
        owner.admit(context(4), next, 11),
        Err(UploadAdmissionError::ManifestCapacity(
            UploadManifestLimit::Tenant
        ))
    );
    assert_eq!(owner.catalog().usage(), before);
    assert_eq!(
        owner.lookup(context(4), next.request),
        Err(UploadAdmissionError::UnknownPermission)
    );

    let other = sized(3, 6, CAFFEINE_CHUNK_BYTES as u64);
    owner.admit(context(6), other, 11).unwrap();
    owner.revoke(context(6), other.request).unwrap();
    let before = owner.catalog().usage();
    assert_eq!(
        owner.admit(context(6), sized(4, 6, 1), 12),
        Err(UploadAdmissionError::ManifestCapacity(
            UploadManifestLimit::Global
        ))
    );
    assert_eq!(owner.catalog().usage(), before);
}

#[test]
fn catalog_rejection_cannot_consume_leaf_capacity_or_claim_the_rejected_root() {
    let mut owner = owner(2, 2);
    let first = permission(1);
    owner.admit(context(4), first, 10).unwrap();
    let mut conflict = permission(2);
    conflict.request.object.root = first.request.object.root;
    let before = owner.catalog().usage();
    assert!(matches!(
        owner.admit(context(4), conflict, 10),
        Err(UploadAdmissionError::Catalog(UploadError::Root(_)))
    ));
    assert_eq!(owner.catalog().usage(), before);
    // The remaining slot must still admit this different root under the same ID.
    let second = permission(2);
    owner.admit(context(4), second, 10).unwrap();
    assert_eq!(
        owner.admit(context(4), second, 99),
        Ok(UploadAdmission::Existing(UploadPhase::Reserved))
    );
    prepare(&mut owner, &first);
    prepare(&mut owner, &second);
    for input in [first, second] {
        owner.expose(context(5), input.request, 11).unwrap();
        owner.confirm_upload(input.request).unwrap();
        let reference = input.request.object.first;
        owner
            .apply_reference(
                context(4),
                input.request.object.root,
                ReferenceRequest {
                    id: ReferenceRequestId::new(id(1)),
                    operation: ReferenceOperation::Release(reference),
                },
            )
            .unwrap();
        owner
            .confirm_provider_deleted(input.request.object.root, reference.object())
            .unwrap();
        owner
            .confirm_billing_stopped(input.request.object.root, reference.object())
            .unwrap();
    }
    assert_eq!(owner.catalog().usage().physical_bytes, 0);
    assert_eq!(owner.catalog().usage().liability_bytes, 0);
    assert_eq!(
        owner.admit(context(4), permission(3), 12),
        Err(UploadAdmissionError::ManifestCapacity(
            UploadManifestLimit::Tenant
        ))
    );
}

#[test]
fn root_index_uses_operation_identity_and_never_indexes_rejected_admission() {
    let mut owner = admissions();
    let first = permission(1);
    owner.admit(context(4), first, 10).unwrap();
    let mut second = permission(2);
    second.request.id = UploadRequestId::new(id(99));
    let mut conflicting = second;
    conflicting.request.object.first = first.request.object.first;
    assert_eq!(
        owner.admit(context(4), conflicting, 10),
        Err(UploadAdmissionError::Catalog(UploadError::Root(
            crate::model::lifecycle::roots::RootClaimError::ObjectAlreadyClaimed
        )))
    );
    assert_eq!(
        owner.expose_root(context(5), second.request.object.root, 11),
        Err(UploadAdmissionError::UnknownPermission)
    );
    owner.admit(context(4), second, 10).unwrap();
    prepare(&mut owner, &second);
    assert_eq!(
        owner.expose_root(context(5), second.request.object.root, 11),
        Ok(second.request)
    );
    let rejected = permission(3);
    assert_eq!(
        owner.admit(context(4), rejected, 11),
        Err(UploadAdmissionError::Catalog(UploadError::Catalog(
            CatalogError::Capacity(CatalogCapacity::Objects)
        )))
    );
    assert_eq!(
        owner.expose_root(context(5), rejected.request.object.root, 11),
        Err(UploadAdmissionError::UnknownPermission)
    );
    owner.revoke(context(4), first.request).unwrap();
    assert_eq!(
        owner.expose_root(context(5), first.request.object.root, 11),
        Err(UploadAdmissionError::Revoked)
    );
}
