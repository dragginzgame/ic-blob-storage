use super::*;
use ic_blob_storage_contracts::dto::configuration::ServiceBillingInput;
use ic_blob_storage_contracts::dto::configuration::ServiceFundingInput;
use ic_blob_storage_contracts::dto::configuration::ServiceReadInput;
use ic_blob_storage_contracts::dto::configuration::ServiceResourceInput;
fn p(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}
pub(crate) fn candidate() -> ServiceConfigurationInput {
    ServiceConfigurationInput {
        service: p(1),
        operator: p(2),
        payment_account: p(3),
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
            cashier: p(4),
            reserve: u128::MAX,
            minimum_balance: u128::MAX,
            target_balance: u128::MAX,
            max_gateway_entries: 8,
            max_gateway_unique: 4,
        },
        funding: ServiceFundingInput {
            allocated: u128::MAX,
            renewal_ceiling: u128::MAX,
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

#[test]
fn validated_fields_construct_service_accounting_and_reads() {
    let input = candidate();
    let stores = validate_candidate(p(1), input).unwrap();
    assert_eq!(
        stores.service(),
        ic_blob_storage_contracts::configuration::validate_candidate(p(1), input)
            .unwrap()
            .service()
    );
    let funding = stores.funding().reconstruct(&[]).unwrap();
    assert_eq!(
        (
            funding.allocated(),
            funding.renewal_ceiling(),
            funding.transferable()
        ),
        (
            input.funding.allocated,
            input.funding.renewal_ceiling,
            input.funding.allocated - input.funding.reserve
        )
    );
    assert_eq!(stores.reads().max_reply_bytes(), input.reads.reply_bytes);
}
