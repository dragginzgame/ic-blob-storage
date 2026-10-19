//! Frozen old installation schema is local to historical input tests.
use ic_platform::Principal;
use ic_platform::candid::contract::CandidType;
use serde::{Deserialize, Serialize};
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
struct ServiceConfigurationInput {
    service: Principal,
    operator: Principal,
    payment_account: Principal,
    namespace: u128,
    resources: ServiceResourceInput,
    billing: ServiceBillingInput,
    funding: ServiceFundingInput,
    reads: ServiceReadInput,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
#[expect(
    clippy::struct_field_names,
    reason = "Frozen historical field names are the persisted schema"
)]
struct ServiceResourceInput {
    max_tenants: u32,
    max_object_bytes: u64,
    max_headers: u32,
    max_header_bytes: u32,
    max_chunks: u32,
    max_tenant_chunks: u32,
    max_objects: u32,
    max_tenant_objects: u32,
    max_physical_bytes: u128,
    max_liability_bytes: u128,
    max_tenant_logical_bytes: u128,
    max_references_per_object: u32,
    max_receipts_per_object: u32,
    max_active: u32,
    max_tenant_active: u32,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
struct ServiceBillingInput {
    cashier: Principal,
    reserve: u128,
    minimum_balance: u128,
    target_balance: u128,
    max_gateway_entries: u32,
    max_gateway_unique: u32,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
struct ServiceFundingInput {
    allocated: u128,
    renewal_ceiling: u128,
    reserve: u128,
    max_attempts: u32,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
struct ServiceReadInput {
    sessions: u32,
    tenant_sessions: u32,
    reply_bytes: u32,
    bytes: u64,
    tenant_bytes: u64,
}
pub(in crate::native) fn configuration()
-> ic_blob_storage_contracts::dto::configuration::ServiceConfigurationInput {
    let bytes = hex::decode(
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/installation/configuration.hex"
        ))
        .trim(),
    )
    .unwrap();
    let value: ServiceConfigurationInput = ic_platform::candid::decode_one(
        &bytes,
        ic_platform::candid::DecodeLimits {
            reply_bytes: 16_384,
            decoding_quota: 2_000_000,
            skipping_quota: 100_000,
            type_entries: 128,
            header_bytes: 4096,
        },
    )
    .unwrap();
    assert_eq!(
        ic_platform::candid::contract::encode_one(&value).unwrap(),
        bytes
    );
    serde_json::from_slice(&serde_json::to_vec(&value).unwrap()).unwrap()
}
