//! Explicit local fixture configuration; no defaults exist in the adapter.
use candid::Principal;
use ic_blob_storage::dto::configuration::{
    ServiceBillingInput, ServiceFundingInput, ServiceReadInput, ServiceResourceInput,
};
use ic_blob_storage_canic::dto::ManagedInstallationInput;
/// Explicit test policy encoded by the host; installation binds the actual service.
#[must_use]
pub fn input() -> ManagedInstallationInput {
    ManagedInstallationInput {
        project: "fixture/β?&=".to_owned(),
        completion_verifier: Principal::from_slice(&[5, 1]),
        operator: Principal::from_slice(&[2, 1]),
        payment_account: Principal::from_slice(&[3, 1]),
        namespace: u128::MAX,
        resources: ServiceResourceInput {
            max_tenants: 2,
            max_object_bytes: 10,
            max_headers: 8,
            max_header_bytes: 1024,
            max_chunks: 4,
            max_tenant_chunks: 2,
            max_objects: 4,
            max_tenant_objects: 2,
            max_physical_bytes: u128::MAX,
            max_liability_bytes: u128::MAX,
            max_tenant_logical_bytes: u128::MAX,
            max_references_per_object: 2,
            max_receipts_per_object: 3,
            max_active: 4,
            max_tenant_active: 2,
        },
        billing: ServiceBillingInput {
            cashier: Principal::from_slice(&[4, 1]),
            reserve: u128::MAX,
            minimum_balance: u128::MAX,
            target_balance: u128::MAX,
            max_gateway_entries: 8,
            max_gateway_unique: 4,
        },
        funding: ServiceFundingInput {
            allocated: u128::MAX,
            reserve: 100,
            max_attempts: 4,
        },
        reads: ServiceReadInput {
            sessions: 2,
            tenant_sessions: 1,
            reply_bytes: 2048,
            bytes: 4096,
            tenant_bytes: 2048,
        },
    }
}
