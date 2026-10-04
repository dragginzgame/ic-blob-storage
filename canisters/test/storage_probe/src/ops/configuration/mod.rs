//! Explicit small fixture envelope through the shared host boundary, not deployment defaults.
use candid::Principal;
use ic_blob_storage::{
    dto::configuration::{
        ServiceBillingInput, ServiceConfigurationInput, ServiceFundingInput, ServiceReadInput,
        ServiceResourceInput,
    },
    ops::service::configuration::validate_candidate,
    ops::service::stores::ServiceStoreConfiguration,
};

pub(super) fn configuration(operator: Principal, max_objects: u32) -> ServiceStoreConfiguration {
    assert!(
        (1..=10_000).contains(&max_objects),
        "bounded fixture capacity"
    );
    let objects = max_objects;
    let service = ic_cdk::api::canister_self();
    validate_candidate(
        service,
        ServiceConfigurationInput {
            service,
            operator,
            payment_account: service,
            namespace: 1,
            resources: ServiceResourceInput {
                max_tenants: 2,
                max_object_bytes: 10,
                max_headers: 8,
                max_header_bytes: 1024,
                max_chunks: objects,
                max_tenant_chunks: objects,
                max_objects: objects,
                max_tenant_objects: objects,
                max_physical_bytes: u128::from(objects) * 10,
                max_liability_bytes: u128::from(objects) * 10,
                max_tenant_logical_bytes: u128::from(objects) * 10,
                max_references_per_object: 2,
                max_receipts_per_object: 3,
                max_active: objects,
                max_tenant_active: objects,
            },
            billing: ServiceBillingInput {
                cashier: operator,
                reserve: 1,
                minimum_balance: 10,
                target_balance: 100,
                max_gateway_entries: 8,
                max_gateway_unique: 4,
            },
            funding: ServiceFundingInput {
                allocated: 1000,
                reserve: 100,
                max_attempts: 4,
            },
            reads: ServiceReadInput {
                sessions: 1,
                tenant_sessions: 1,
                reply_bytes: 2048,
                bytes: 2048,
                tenant_bytes: 2048,
            },
        },
    )
    .unwrap()
}
