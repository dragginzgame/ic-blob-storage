use crate::configuration::funding::FundingLimits;
use crate::configuration::service::*;
use std::num::NonZeroUsize;

type LimitChange = fn(&mut ServiceLimits);
type CountChange = fn(&mut ServiceLimits, NonZeroUsize);

fn p(value: u8) -> Principal {
    Principal::from_slice(&[value, 1])
}
fn n(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).unwrap()
}
fn bytes(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).unwrap()
}
fn bindings() -> ServiceBindings {
    ServiceBindings {
        service: p(1),
        operator: p(2),
        payment_account: p(1),
        namespace: bytes(1),
    }
}
fn limits() -> ServiceLimits {
    // A small test envelope including Toko's observed 10 MiB media-file size.
    // These are test inputs, not installed production defaults.
    ServiceLimits {
        max_tenants: n(2),
        max_object_bytes: NonZeroU64::new(10 * 1024 * 1024).unwrap(),
        max_headers: n(8),
        max_header_bytes: n(1024),
        manifests: ServiceManifestLimits {
            max_chunks: n(30),
            max_tenant_chunks: n(20),
        },
        catalog: CatalogLimits {
            max_objects: n(8),
            max_tenant_objects: n(4),
            max_physical_bytes: bytes(20 * 1024 * 1024),
            max_liability_bytes: bytes(30 * 1024 * 1024),
            max_tenant_logical_bytes: bytes(10 * 1024 * 1024),
            max_references_per_object: n(2),
            max_receipts_per_object: n(3),
        },
        uploads: UploadLimits {
            max_active: n(4),
            max_tenant_active: n(2),
        },
    }
}
fn billing() -> BillingConfiguration {
    BillingConfiguration::new(p(3), FundingLimits::new(1, 10, 100).unwrap(), 8, 4).unwrap()
}

#[test]
fn metadata_budget_can_represent_the_largest_admitted_length() {
    let mut candidate = limits();
    candidate.max_headers = n(1);
    candidate.max_header_bytes = n("Content-Length".len() + "10485760".len() + 3);
    assert!(ServiceConfiguration::new(bindings(), candidate, billing()).is_ok());
    candidate.max_header_bytes = n(candidate.max_header_bytes.get() - 1);
    assert_eq!(
        ServiceConfiguration::new(bindings(), candidate, billing()),
        Err(ServiceLimitError::InsufficientLengthMetadata.into())
    );
}

#[test]
fn roles_and_independent_budgets_remain_explicit() {
    for payment_account in [p(1), p(4)] {
        let bindings = ServiceBindings {
            payment_account,
            ..bindings()
        };
        let config = ServiceConfiguration::new(bindings, limits(), billing()).unwrap();
        assert_eq!(config.bindings(), bindings);
        assert_eq!(config.limits(), limits());
        assert_eq!(config.billing(), billing());
    }
    for invalid in [Principal::anonymous(), Principal::management_canister()] {
        for (bindings, field) in [
            (
                ServiceBindings {
                    service: invalid,
                    ..bindings()
                },
                ServicePrincipal::Service,
            ),
            (
                ServiceBindings {
                    operator: invalid,
                    ..bindings()
                },
                ServicePrincipal::Operator,
            ),
            (
                ServiceBindings {
                    payment_account: invalid,
                    ..bindings()
                },
                ServicePrincipal::PaymentAccount,
            ),
        ] {
            assert_eq!(
                ServiceConfiguration::new(bindings, limits(), billing()),
                Err(ServiceConfigurationError::InvalidPrincipal { field })
            );
        }
    }
}

#[test]
fn contradictory_object_upload_and_release_capacities_reject_without_replacing_valid_config() {
    let original = ServiceConfiguration::new(bindings(), limits(), billing()).unwrap();
    let cases: [(LimitChange, ServiceLimitError); 10] = [
        (
            |l| l.manifests.max_tenant_chunks = n(31),
            ServiceLimitError::TenantManifestExceedsGlobal,
        ),
        (
            |l| l.manifests.max_tenant_chunks = n(9),
            ServiceLimitError::ObjectExceedsManifest,
        ),
        (
            |l| l.catalog.max_tenant_objects = n(9),
            ServiceLimitError::TenantObjectsExceedGlobal,
        ),
        (
            |l| l.uploads.max_active = n(9),
            ServiceLimitError::UploadsExceedObjects,
        ),
        (
            |l| l.uploads.max_tenant_active = n(5),
            ServiceLimitError::TenantUploadsExceedGlobal,
        ),
        (
            |l| l.catalog.max_tenant_objects = n(1),
            ServiceLimitError::TenantUploadsExceedObjects,
        ),
        (
            |l| l.catalog.max_physical_bytes = bytes(10 * 1024 * 1024 - 1),
            ServiceLimitError::ObjectExceedsPhysical,
        ),
        (
            |l| l.catalog.max_liability_bytes = bytes(10 * 1024 * 1024 - 1),
            ServiceLimitError::ObjectExceedsLiability,
        ),
        (
            |l| l.catalog.max_tenant_logical_bytes = bytes(10 * 1024 * 1024 - 1),
            ServiceLimitError::ObjectExceedsTenantLogical,
        ),
        (
            |l| l.catalog.max_receipts_per_object = n(2),
            ServiceLimitError::InsufficientReferenceReceipts,
        ),
    ];
    for (change, error) in cases {
        let mut candidate = original.limits();
        change(&mut candidate);
        assert_eq!(
            ServiceConfiguration::new(original.bindings(), candidate, original.billing()),
            Err(error.into())
        );
        assert_eq!(original.limits(), limits());
    }
    let wrong = BillingConfiguration::new(p(3), billing().funding_limits(), 4, 5).unwrap();
    assert_eq!(
        ServiceConfiguration::new(bindings(), limits(), wrong),
        Err(ServiceLimitError::GatewayUniqueExceedsEntries.into())
    );
}

#[test]
fn counts_are_portable_without_allocating_the_advertised_collections() {
    let Ok(oversized) = usize::try_from(u64::from(u32::MAX) + 1) else {
        return;
    };
    let setters: [(CountChange, ServiceCount); 11] = [
        (
            |l, n| l.manifests.max_chunks = n,
            ServiceCount::ManifestChunksRetained,
        ),
        (
            |l, n| l.manifests.max_tenant_chunks = n,
            ServiceCount::TenantManifestChunks,
        ),
        (|l, n| l.max_headers = n, ServiceCount::Headers),
        (|l, n| l.max_header_bytes = n, ServiceCount::HeaderBytes),
        (|l, n| l.max_tenants = n, ServiceCount::Tenants),
        (|l, n| l.catalog.max_objects = n, ServiceCount::Objects),
        (
            |l, n| l.catalog.max_tenant_objects = n,
            ServiceCount::TenantObjects,
        ),
        (
            |l, n| l.catalog.max_references_per_object = n,
            ServiceCount::References,
        ),
        (
            |l, n| l.catalog.max_receipts_per_object = n,
            ServiceCount::Receipts,
        ),
        (|l, n| l.uploads.max_active = n, ServiceCount::Uploads),
        (
            |l, n| l.uploads.max_tenant_active = n,
            ServiceCount::TenantUploads,
        ),
    ];
    for (set, field) in setters {
        let mut candidate = limits();
        set(&mut candidate, n(oversized));
        assert_eq!(
            ServiceConfiguration::new(bindings(), candidate, billing()),
            Err(ServiceConfigurationError::CountOutOfRange { field })
        );
    }
}

#[test]
fn largest_portable_reference_budget_uses_wide_arithmetic() {
    let max = usize::try_from(u32::MAX).unwrap();
    let mut candidate = limits();
    candidate.catalog.max_references_per_object = n(max / 2 + 1);
    candidate.catalog.max_receipts_per_object = n(max);
    let config = ServiceConfiguration::new(bindings(), candidate, billing()).unwrap();
    assert_eq!(config.limits(), candidate);
    candidate.catalog.max_references_per_object = n(max / 2 + 2);
    assert_eq!(
        ServiceConfiguration::new(bindings(), candidate, billing()),
        Err(ServiceLimitError::InsufficientReferenceReceipts.into())
    );
}

#[test]
fn retained_leaf_arrays_must_fit_the_portable_address_space() {
    let mut candidate = limits();
    let maximum = usize::try_from(i32::MAX / 32).unwrap();
    candidate.manifests.max_chunks = n(maximum);
    candidate.manifests.max_tenant_chunks = n(maximum);
    assert!(ServiceConfiguration::new(bindings(), candidate, billing()).is_ok());
    candidate.manifests.max_chunks = n(maximum + 1);
    assert_eq!(
        ServiceConfiguration::new(bindings(), candidate, billing()),
        Err(ServiceLimitError::ManifestExceedsAddressSpace.into())
    );
}

#[test]
fn manifest_envelope_uses_the_admitted_length_and_rejects_unrepresentable_hash_work() {
    for (bytes, chunks) in [(1, 1), (1_048_576, 1), (1_048_577, 2), (10_485_760, 10)] {
        let mut candidate = limits();
        candidate.max_object_bytes = NonZeroU64::new(bytes).unwrap();
        let config = ServiceConfiguration::new(bindings(), candidate, billing()).unwrap();
        let manifest = config.manifest_limits();
        assert_eq!(manifest.max_content_bytes, candidate.max_object_bytes);
        assert_eq!(manifest.max_chunks.get(), chunks);
        assert_eq!(manifest.max_headers, candidate.max_headers);
        assert_eq!(manifest.max_header_bytes, candidate.max_header_bytes);
    }
    let mut candidate = limits();
    candidate.max_object_bytes = NonZeroU64::new(ContentVerifier::MAX_BYTES + 1).unwrap();
    assert_eq!(
        ServiceConfiguration::new(bindings(), candidate, billing()),
        Err(ServiceLimitError::ObjectExceedsHashLength.into())
    );
    // Portable leaf-count rejection precedes allocation even on a 64-bit host.
    candidate.max_object_bytes =
        NonZeroU64::new(u64::from(u32::MAX) * CAFFEINE_CHUNK_BYTES as u64 + 1).unwrap();
    assert_eq!(
        ServiceConfiguration::new(bindings(), candidate, billing()),
        Err(ServiceConfigurationError::CountOutOfRange {
            field: ServiceCount::ManifestChunks
        })
    );
}
