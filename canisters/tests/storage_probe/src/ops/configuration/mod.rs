//! Explicit small fixture envelope through the shared host boundary, not deployment defaults.
use ic_blob_storage::ops::service::configuration::validate_candidate;
use ic_blob_storage::ops::service::stores::ServiceStoreConfiguration;
use ic_blob_storage_contracts::dto::configuration::ServiceBillingInput;
use ic_blob_storage_contracts::dto::configuration::ServiceConfigurationInput;
use ic_blob_storage_contracts::dto::configuration::ServiceFundingInput;

pub(super) fn configuration(
    input: blob_test_protocol::storage::resources::StorageProbeInstallation,
) -> ServiceStoreConfiguration {
    assert!(
        (1..=10_000).contains(&input.resources.max_objects)
            && (1..=32).contains(&input.resources.max_tenants)
            && (10..=67_108_864).contains(&input.resources.max_object_bytes)
            && (1..=10_000).contains(&input.max_funding_attempts),
        "bounded fixture capacity"
    );
    let operator = input.operator;
    let service = ic_platform::cdk::api::canister_self();
    validate_candidate(
        service,
        ServiceConfigurationInput {
            service,
            operator: operator.into(),
            payment_account: service,
            namespace: 1,
            resources: input.resources,
            billing: ServiceBillingInput {
                cashier: operator.into(),
                reserve: 1,
                minimum_balance: 10,
                target_balance: 100,
                max_gateway_entries: 8,
                max_gateway_unique: 4,
            },
            funding: ServiceFundingInput {
                allocated: 1000_u128.max(u128::from(input.max_funding_attempts) + 100),
                renewal_ceiling: 2000_u128.max(u128::from(input.max_funding_attempts) + 100),
                reserve: 100,
                max_attempts: input.max_funding_attempts,
            },
        },
    )
    .unwrap()
}
