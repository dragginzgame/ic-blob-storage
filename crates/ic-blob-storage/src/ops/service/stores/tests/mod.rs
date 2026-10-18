use super::*;
mod account;
mod operator;
use crate::ops::service::configuration::{tests::candidate, validate_candidate};
use crate::test_support::memory::FixtureMemory;

fn memories(
    m: &[FixtureMemory; 16],
) -> ServiceMemories<ic_memory::RuntimeMemory<ic_memory::VectorMemory>> {
    let requests = grants::requests("store-test").unwrap();
    grants::open::<_, std::convert::Infallible>(|key| {
        let index = requests
            .iter()
            .position(|request| request.stable_key().as_str() == key)
            .unwrap();
        Ok(m[index].lease())
    })
    .unwrap()
}

#[test]
fn allocated_grant_anywhere_rejects_before_other_memories_are_written() {
    let input = candidate();
    let config = validate_candidate(input.service, input).unwrap();
    for occupied in 0..16 {
        let m: [FixtureMemory; 16] = std::array::from_fn(|_| FixtureMemory::default());
        assert_eq!(ic_memory::Memory::grow(&m[occupied].lease(), 1), 0);
        m[occupied].lease().write(0, b"retained obligation");
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
        let m: [FixtureMemory; 16] = std::array::from_fn(|i| {
            let memory = FixtureMemory::default();
            if i != missing {
                assert_eq!(ic_memory::Memory::grow(&memory.lease(), 1), 0);
                // Deliberately not a collection: preflight must precede loading.
                memory.lease().write(0, b"retained obligation");
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
    let m: [FixtureMemory; 16] = std::array::from_fn(|_| FixtureMemory::default());
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
    let context = ic_blob_storage_contracts::upload::binding::UploadContext {
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
