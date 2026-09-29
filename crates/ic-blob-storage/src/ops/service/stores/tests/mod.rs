use super::*;
mod account;
mod operator;
use crate::ops::service::configuration::{tests::candidate, validate_candidate};
use ic_memory::ic_stable_structures::VectorMemory;

fn memories(m: &[VectorMemory; 16]) -> ServiceMemories<VectorMemory> {
    let [
        tenants,
        roots,
        objects,
        permissions,
        usage,
        manifests,
        confirmed,
        references,
        receipts,
        root_requests,
        accounting,
        intents,
        gateways,
        journal,
        sessions,
        read_tenants,
    ] = m.clone();
    ServiceMemories {
        uploads: UploadMemories {
            tenants,
            roots,
            objects,
            permissions,
            usage,
            manifests,
            confirmed,
            references,
            receipts,
            root_requests,
        },
        funding: FundingMemories {
            accounting,
            intents,
        },
        gateways,
        reads: ReadSessionMemories {
            journal,
            sessions,
            tenants: read_tenants,
        },
    }
}

#[test]
fn allocated_grant_anywhere_rejects_before_other_memories_are_written() {
    let input = candidate();
    let config = validate_candidate(input.service, input).unwrap();
    for occupied in 0..16 {
        let m: [VectorMemory; 16] = std::array::from_fn(|_| VectorMemory::default());
        assert_eq!(m[occupied].grow(1), 0);
        m[occupied].write(0, b"retained obligation");
        let before = m.each_ref().map(|m| m.borrow().clone());
        assert!(matches!(
            ServiceStores::install(memories(&m), config),
            Err(ServiceStoreError::AlreadyAllocated)
        ));
        assert_eq!(m.each_ref().map(|m| m.borrow().clone()), before);
    }
}

#[test]
fn missing_grant_anywhere_rejects_without_loading_or_initializing_any_store() {
    let input = candidate();
    let config = validate_candidate(input.service, input).unwrap();
    for missing in 0..16 {
        let m: [VectorMemory; 16] = std::array::from_fn(|i| {
            let memory = VectorMemory::default();
            if i != missing {
                assert_eq!(memory.grow(1), 0);
                // Deliberately not a collection: preflight must precede loading.
                memory.write(0, b"retained obligation");
            }
            memory
        });
        let before = m.each_ref().map(|m| m.borrow().clone());
        assert!(matches!(
            ServiceStores::open(memories(&m), config),
            Err(ServiceStoreError::Missing)
        ));
        assert_eq!(m.each_ref().map(|m| m.borrow().clone()), before);
    }
}

#[test]
fn changed_funding_or_read_limits_reject_without_repairing_other_stores() {
    let input = candidate();
    let config = validate_candidate(input.service, input).unwrap();
    let m: [VectorMemory; 16] = std::array::from_fn(|_| VectorMemory::default());
    drop(ServiceStores::install(memories(&m), config).unwrap());
    let before = m.each_ref().map(|m| m.borrow().clone());

    let mut changed = input;
    changed.funding.max_attempts += 1;
    let changed = validate_candidate(changed.service, changed).unwrap();
    assert!(matches!(
        ServiceStores::open(memories(&m), changed),
        Err(ServiceStoreError::Funding(FundingJournalError::Binding))
    ));
    assert_eq!(m.each_ref().map(|m| m.borrow().clone()), before);

    let mut changed = input;
    changed.reads.reply_bytes /= 2;
    let changed = validate_candidate(changed.service, changed).unwrap();
    assert!(matches!(
        ServiceStores::open(memories(&m), changed),
        Err(ServiceStoreError::Reads(ReadSessionError::Binding))
    ));
    assert_eq!(m.each_ref().map(|m| m.borrow().clone()), before);

    // Rejection does not prevent inspection with the original exact configuration.
    let restored = ServiceStores::open(memories(&m), config).unwrap();
    assert!(restored.uploads.is_fenced());
    assert!(restored.funding.is_fenced());
    let context = crate::model::service::upload::UploadContext {
        service: input.service,
        actor: input.operator,
    };
    let scope = crate::model::gateway::registry::GatewayScope::new(
        input.service,
        config.service().bindings().namespace,
        input.billing.cashier,
    )
    .unwrap();
    assert!(restored.gateways.inspect(context, scope).unwrap().fenced);
    assert!(restored.reads.inspect(context).unwrap().fenced);
    assert_eq!(m.each_ref().map(|m| m.borrow().clone()), before);
}
