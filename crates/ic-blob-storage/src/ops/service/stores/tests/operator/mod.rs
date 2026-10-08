use super::*;
use crate::ops::service::operator::OperatorStores;
use crate::workflow::operator::inspect;
use ic_blob_storage_contracts::dto::operator::LocalStatusFailure;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_blob_storage_contracts::upload::binding::UploadContext;
fn context() -> UploadContext {
    let input = candidate();
    UploadContext {
        service: input.service,
        actor: input.operator,
    }
}
fn scope() -> OperatorScope {
    let input = candidate();
    OperatorScope {
        service: input.service,
        namespace: input.namespace,
        cashier: input.billing.cashier,
        payment_account: input.payment_account,
    }
}
#[test]
fn local_operator_snapshot_preserves_width_and_checks_every_scope_field_without_writes() {
    let input = candidate();
    let config = validate_candidate(input.service, input).unwrap();
    let memory = std::array::from_fn(|_| VectorMemory::default());
    let stores = ServiceStores::install(memories(&memory), config).unwrap();
    let before = memory.each_ref().map(|m| m.borrow().clone());
    let view = inspect((&stores).into(), context(), scope()).unwrap();
    assert_eq!(view.scope, scope());
    assert_eq!(view.funding.available_allocation, u128::MAX);
    assert_eq!(view.funding.attachment_allowance, u128::MAX - 100);
    assert_eq!(view.funding.retained_intents, 0);
    assert_eq!(view.funding.last_operation, None);
    assert_eq!(view.uploads.operations, 0);
    assert_eq!(view.reads.sessions, 0);
    assert_eq!(view.gateways.members, []);
    let roundtrip = candid::decode_one(&candid::encode_one(&view).unwrap()).unwrap();
    assert_eq!(view, roundtrip);
    for bad in [
        OperatorScope {
            service: input.operator,
            ..scope()
        },
        OperatorScope {
            namespace: 0,
            ..scope()
        },
        OperatorScope {
            namespace: 1,
            ..scope()
        },
        OperatorScope {
            cashier: input.operator,
            ..scope()
        },
        OperatorScope {
            payment_account: input.service,
            ..scope()
        },
    ] {
        assert_eq!(
            inspect((&stores).into(), context(), bad),
            Err(LocalStatusFailure::Binding)
        );
    }
    assert_eq!(
        inspect(
            (&stores).into(),
            UploadContext {
                service: input.operator,
                ..context()
            },
            scope()
        ),
        Err(LocalStatusFailure::Binding)
    );
    for actor in [input.payment_account, candid::Principal::anonymous()] {
        assert_eq!(
            inspect(
                (&stores).into(),
                UploadContext { actor, ..context() },
                scope()
            ),
            Err(LocalStatusFailure::Denied)
        );
    }
    assert!(
        memory.each_ref().map(|m| m.borrow().clone()).eq(&before),
        "inspection changed stable bytes"
    );
}
#[test]
fn operator_snapshot_rejects_mixed_owners_even_with_same_service_and_operator() {
    let input = candidate();
    let original = ServiceStores::install(
        memories(&std::array::from_fn(|_| VectorMemory::default())),
        validate_candidate(input.service, input).unwrap(),
    )
    .unwrap();
    let mut changed = input;
    changed.namespace = 1;
    let other = ServiceStores::install(
        memories(&std::array::from_fn(|_| VectorMemory::default())),
        validate_candidate(changed.service, changed).unwrap(),
    )
    .unwrap();
    for selected in [
        OperatorStores {
            funding: &other.funding,
            ..(&original).into()
        },
        OperatorStores {
            gateways: &other.gateways,
            ..(&original).into()
        },
        OperatorStores {
            reads: &other.reads,
            ..(&original).into()
        },
    ] {
        assert_eq!(
            inspect(selected, context(), scope()),
            Err(LocalStatusFailure::Binding)
        );
    }
}
#[test]
fn operator_snapshot_preserves_each_restore_fence_independently() {
    let input = candidate();
    let config = validate_candidate(input.service, input).unwrap();
    let memory = std::array::from_fn(|_| VectorMemory::default());
    let fresh = ServiceStores::install(memories(&memory), config).unwrap();
    let restored = ServiceStores::open(memories(&memory), config).unwrap();
    let before = memory.each_ref().map(|m| m.borrow().clone());
    let mixed = inspect(
        OperatorStores {
            reads: &restored.reads,
            ..(&fresh).into()
        },
        context(),
        scope(),
    )
    .unwrap();
    assert!(mixed.reads.fenced);
    assert!(!mixed.uploads.fenced);
    assert!(!mixed.funding.fenced);
    assert!(!mixed.gateways.fenced);
    let all = inspect((&restored).into(), context(), scope()).unwrap();
    assert!(all.reads.fenced && all.uploads.fenced && all.funding.fenced && all.gateways.fenced);
    assert_eq!(
        all.funding.available_allocation,
        mixed.funding.available_allocation
    );
    assert!(
        memory.each_ref().map(|m| m.borrow().clone()).eq(&before),
        "inspection changed stable bytes"
    );
}
