//! Explicit small fixture envelope; these are not deployment defaults.
use candid::Principal;
use ic_blob_storage::model::{
    billing::{FundingLimits, configuration::BillingConfiguration},
    catalog::{CatalogLimits, admission::UploadLimits},
    service::configuration::{
        ServiceBindings, ServiceConfiguration, ServiceLimits, ServiceManifestLimits,
    },
};
use std::num::{NonZeroU64, NonZeroU128, NonZeroUsize};

pub(super) fn configuration(operator: Principal) -> ServiceConfiguration {
    let two = NonZeroUsize::new(2).unwrap();
    let bytes = NonZeroU128::new(20).unwrap();
    ServiceConfiguration::new(
        ServiceBindings {
            service: ic_cdk::api::canister_self(),
            operator,
            payment_account: ic_cdk::api::canister_self(),
            namespace: NonZeroU128::MIN,
        },
        ServiceLimits {
            max_tenants: two,
            max_object_bytes: NonZeroU64::new(10).unwrap(),
            max_headers: NonZeroUsize::new(8).unwrap(),
            max_header_bytes: NonZeroUsize::new(1024).unwrap(),
            manifests: ServiceManifestLimits {
                max_chunks: two,
                max_tenant_chunks: two,
            },
            catalog: CatalogLimits {
                max_objects: two,
                max_tenant_objects: two,
                max_physical_bytes: bytes,
                max_liability_bytes: bytes,
                max_tenant_logical_bytes: bytes,
                max_references_per_object: two,
                max_receipts_per_object: NonZeroUsize::new(3).unwrap(),
            },
            uploads: UploadLimits {
                max_active: two,
                max_tenant_active: two,
            },
        },
        BillingConfiguration::new(operator, FundingLimits::new(1, 10, 100).unwrap(), 8, 4).unwrap(),
    )
    .unwrap()
}
