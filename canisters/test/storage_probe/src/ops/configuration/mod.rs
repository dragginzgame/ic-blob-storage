//! Explicit small fixture envelope through the shared host boundary, not deployment defaults.
use ic_blob_storage::{
    dto::configuration::{
        ServiceBillingInput, ServiceConfigurationInput, ServiceFundingInput, ServiceReadInput,
        ServiceResourceInput,
    },
    ops::service::configuration::validate_candidate,
    ops::service::stores::ServiceStoreConfiguration,
};

pub(super) fn configuration(
    input: blob_test_protocol::storage::resources::StorageProbeInstallation,
) -> ServiceStoreConfiguration {
    assert!(
        (1..=10_000).contains(&input.max_objects)
            && (1..=32).contains(&input.max_tenants)
            && (10..=67_108_864).contains(&input.max_object_bytes)
            && (1..=10_000).contains(&input.max_funding_attempts)
            && (1..=1024).contains(&input.max_read_sessions)
            && (1..=input.max_read_sessions).contains(&input.max_tenant_read_sessions),
        "bounded fixture capacity"
    );
    let objects = input.max_objects;
    let operator = input.operator;
    let bytes = input.max_object_bytes;
    let chunks = u32::try_from(
        bytes.div_ceil(ic_blob_storage::model::identity::caffeine::CAFFEINE_CHUNK_BYTES as u64),
    )
    .unwrap()
        * objects;
    let service = ic_cdk::api::canister_self();
    validate_candidate(
        service,
        ServiceConfigurationInput {
            service,
            operator,
            payment_account: service,
            namespace: 1,
            resources: ServiceResourceInput {
                max_tenants: input.max_tenants,
                max_object_bytes: bytes,
                max_headers: 8,
                max_header_bytes: 1024,
                max_chunks: chunks,
                max_tenant_chunks: chunks,
                max_objects: objects,
                max_tenant_objects: objects,
                max_physical_bytes: u128::from(objects) * u128::from(bytes),
                max_liability_bytes: u128::from(objects) * u128::from(bytes),
                max_tenant_logical_bytes: u128::from(objects) * u128::from(bytes),
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
                max_attempts: input.max_funding_attempts,
            },
            reads: ServiceReadInput {
                sessions: input.max_read_sessions,
                tenant_sessions: input.max_tenant_read_sessions,
                reply_bytes: 2048,
                bytes: 2048 * u64::from(input.max_read_sessions),
                tenant_bytes: 2048 * u64::from(input.max_tenant_read_sessions),
            },
        },
    )
    .unwrap()
}
