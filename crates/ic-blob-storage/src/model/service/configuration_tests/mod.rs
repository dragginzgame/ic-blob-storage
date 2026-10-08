//! Service lifecycle acceptance for the shared immutable configuration.
use candid::Principal;
use ic_blob_storage_contracts::configuration::billing::BillingConfiguration;
use ic_blob_storage_contracts::configuration::funding::FundingLimits;
use ic_blob_storage_contracts::configuration::limits::CatalogLimits;
use ic_blob_storage_contracts::configuration::limits::UploadLimits;
use ic_blob_storage_contracts::configuration::service::*;
use std::num::NonZeroU64;
use std::num::NonZeroU128;
use std::num::NonZeroUsize;
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
fn accepted_reference_budget_can_retain_and_release_every_advertised_reference() {
    use crate::model::lifecycle::BlobLifecycle;
    use crate::model::lifecycle::LifecycleChange;
    use crate::model::lifecycle::requests::ReferenceRequestOutcome;
    use crate::model::lifecycle::requests::ReferenceRequests;
    use ic_blob_storage_contracts::binding::ObjectBinding;
    use ic_blob_storage_contracts::binding::ObjectIdentity;
    use ic_blob_storage_contracts::binding::ReferenceId;
    use ic_blob_storage_contracts::binding::ReferenceKey;
    use ic_blob_storage_contracts::reference::binding::ReferenceOperation;
    use ic_blob_storage_contracts::reference::binding::ReferenceRequest;
    use ic_blob_storage_contracts::reference::binding::ReferenceRequestId;
    use ic_blob_storage_contracts::upload::history::LifecyclePhase;
    let config = ServiceConfiguration::new(bindings(), limits(), billing()).unwrap();
    let object = ObjectBinding::new(
        config.bindings().service,
        p(9),
        ObjectIdentity {
            namespace: config.bindings().namespace,
            object: bytes(1),
            incarnation: bytes(1),
        },
    )
    .unwrap();
    let first = ReferenceKey::new(object, ReferenceId::new(bytes(1)));
    let second = ReferenceKey::new(object, ReferenceId::new(bytes(2)));
    let lifecycle = BlobLifecycle::from_confirmed_upload(
        10,
        first,
        config.limits().catalog.max_references_per_object,
    );
    let mut journal =
        ReferenceRequests::new(lifecycle, config.limits().catalog.max_receipts_per_object).unwrap();
    for (id, operation) in [
        (1, ReferenceOperation::Retain(second)),
        (2, ReferenceOperation::Release(first)),
        (3, ReferenceOperation::Release(second)),
    ] {
        assert_eq!(
            journal
                .apply(
                    p(9),
                    ReferenceRequest {
                        id: ReferenceRequestId::new(bytes(id)),
                        operation
                    }
                )
                .unwrap(),
            ReferenceRequestOutcome::Recorded {
                result: Ok(LifecycleChange::Changed)
            }
        );
    }
    assert_eq!(journal.lifecycle().phase(), LifecyclePhase::DeletionPending);
}
