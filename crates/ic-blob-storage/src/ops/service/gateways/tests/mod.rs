use super::*;
use crate::{
    model::billing::configuration::BillingConfiguration, ops::service::tenant::tests::config,
};
use ic_memory::ic_stable_structures::{Storable, VectorMemory};
use std::num::NonZeroU128;
fn p(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}
fn context() -> UploadContext {
    UploadContext {
        service: p(1),
        actor: p(2),
    }
}
fn scope() -> GatewayScope {
    GatewayScope::new(p(1), NonZeroU128::MIN, p(3)).unwrap()
}

#[test]
fn gateway_scope_operator_and_restoration_never_reset_pending_history() {
    let m = VectorMemory::default();
    let mut store = StableGatewayRegistry::install(m.clone(), config()).unwrap();
    let token = store.begin_sync(context(), scope()).unwrap();
    for candidate in [
        GatewayScope::new(p(8), NonZeroU128::MIN, p(3)).unwrap(),
        GatewayScope::new(p(1), NonZeroU128::new(2).unwrap(), p(3)).unwrap(),
        GatewayScope::new(p(1), NonZeroU128::MIN, p(8)).unwrap(),
    ] {
        assert_eq!(
            store.inspect(context(), candidate),
            Err(GatewayStoreError::Binding)
        );
    }
    assert_eq!(
        store.inspect(
            UploadContext {
                actor: p(4),
                ..context()
            },
            scope()
        ),
        Err(GatewayStoreError::NotOperator)
    );
    let before = store.inspect(context(), scope()).unwrap();
    drop(store);
    let mut restored = StableGatewayRegistry::open(m.clone(), config()).unwrap();
    assert_eq!(
        restored.inspect(context(), scope()).unwrap(),
        GatewayRegistryView {
            fenced: true,
            ..before
        }
    );
    assert_eq!(
        restored.cancel_sync(context(), scope(), token),
        Err(GatewayStoreError::Fenced)
    );
    assert_eq!(
        restored.begin_sync(context(), scope()),
        Err(GatewayStoreError::Fenced)
    );
    assert_eq!(
        restored.remove(context(), scope(), p(4)),
        Err(GatewayStoreError::Fenced)
    );
    assert!(matches!(
        StableGatewayRegistry::install(m, config()),
        Err(GatewayStoreError::AlreadyAllocated)
    ));
    assert!(matches!(
        StableGatewayRegistry::open(VectorMemory::default(), config()),
        Err(GatewayStoreError::Missing)
    ));
}
#[test]
fn maximum_gateway_record_roundtrips_with_pending_identity_and_rejects_oversized_configuration() {
    let base = config();
    let make = |bound| {
        ServiceConfiguration::new(
            base.bindings(),
            base.limits(),
            BillingConfiguration::new(
                base.billing().cashier(),
                base.billing().funding_limits(),
                bound,
                bound,
            )
            .unwrap(),
        )
        .unwrap()
    };
    let config = make(MAX_MEMBERS as u64);
    let m = VectorMemory::default();
    let mut store = StableGatewayRegistry::install(m.clone(), config).unwrap();
    let principals: Vec<_> = (0..MAX_MEMBERS)
        .map(|n| {
            let mut bytes = [0xff; 29];
            bytes[..8].copy_from_slice(&(n as u64).to_be_bytes());
            Principal::from_slice(&bytes)
        })
        .collect();
    let attempt = crate::workflow::gateways::begin_sync(&mut store, context(), scope()).unwrap();
    let bytes = candid::encode_one(&principals).unwrap();
    crate::workflow::gateways::complete_sync(
        &mut store,
        context(),
        &attempt,
        scope(),
        &bytes,
        crate::ops::caffeine::gateway::GatewayReplyLimits {
            max_bytes: 65_536.try_into().unwrap(),
            decoding_quota: 500_000.try_into().unwrap(),
            skipping_quota: 1000.try_into().unwrap(),
            max_type_entries: 32.try_into().unwrap(),
        },
    )
    .unwrap();
    store.begin_sync(context(), scope()).unwrap();
    let record = store.records.get(&0).unwrap();
    assert_eq!(GatewayRegistryRecord::from_bytes(record.to_bytes()), record);
    drop(store);
    let restored = StableGatewayRegistry::open(m, config).unwrap();
    let view = restored.inspect(context(), scope()).unwrap();
    assert_eq!(view.principals, principals);
    assert_eq!(view.sync.pending_sequence, Some(2));
    let empty = VectorMemory::default();
    assert!(matches!(
        StableGatewayRegistry::install(empty.clone(), make(MAX_MEMBERS as u64 + 1)),
        Err(GatewayStoreError::UnsupportedEnvelope)
    ));
    assert_eq!(empty.size(), 0);
}
#[test]
fn changed_gateway_bindings_and_extra_rows_reject_restore_without_repair() {
    let m = VectorMemory::default();
    let mut store = StableGatewayRegistry::install(m.clone(), config()).unwrap();
    let base = config();
    for changed in [
        ServiceConfiguration::new(
            crate::model::service::configuration::ServiceBindings {
                operator: p(9),
                ..base.bindings()
            },
            base.limits(),
            base.billing(),
        )
        .unwrap(),
        ServiceConfiguration::new(
            base.bindings(),
            base.limits(),
            BillingConfiguration::new(p(8), base.billing().funding_limits(), 8, 4).unwrap(),
        )
        .unwrap(),
        ServiceConfiguration::new(
            base.bindings(),
            base.limits(),
            BillingConfiguration::new(p(3), base.billing().funding_limits(), 9, 4).unwrap(),
        )
        .unwrap(),
    ] {
        assert!(matches!(
            StableGatewayRegistry::open(m.clone(), changed),
            Err(GatewayStoreError::Binding)
        ));
    }
    store.records.insert(1, store.records.get(&0).unwrap());
    let before = m.borrow().clone();
    assert!(matches!(
        StableGatewayRegistry::open(m.clone(), config()),
        Err(GatewayStoreError::InvalidRecord)
    ));
    assert_eq!(*m.borrow(), before);
}

mod reply;
mod transport;
