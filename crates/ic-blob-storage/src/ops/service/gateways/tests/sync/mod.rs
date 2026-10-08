use super::*;
use crate::workflow::gateways::sync::cancel;
use ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncCancellation;
use ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncFailure as Error;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
fn input(sequence: u64) -> GatewaySyncCancellation {
    GatewaySyncCancellation {
        scope: OperatorScope {
            service: p(1),
            namespace: 1,
            cashier: p(3),
            payment_account: config().bindings().payment_account,
        },
        sequence,
    }
}
#[test]
fn cancellation_uses_retained_identity_and_preserves_membership_generations_and_history() {
    let memory = VectorMemory::default();
    let mut store = StableGatewayRegistry::install(memory.clone(), config()).unwrap();
    store.add(context(), scope(), p(4)).unwrap();
    let original = store.begin_sync(context(), scope()).unwrap();
    let generation = store.registry().unwrap().read_generation;
    assert_eq!(cancel(&mut store, context(), input(1)), Ok(()));
    assert_eq!(store.registry().unwrap().read_generation, generation);
    assert_eq!(
        store.inspect(context(), scope()).unwrap().principals,
        vec![p(4)]
    );
    assert_eq!(
        store.inspect(context(), scope()).unwrap().sync,
        GatewaySyncView {
            last_sequence: 1,
            pending_sequence: None
        }
    );
    assert_eq!(
        store.registry().unwrap().check_sync(original, scope()),
        Err(GatewaySyncError::StaleSync)
    );
    store.begin_sync(context(), scope()).unwrap();
    let bytes = memory.borrow().clone();
    assert_eq!(
        cancel(&mut store, context(), input(1)),
        Err(Error::Conflict)
    );
    assert_eq!(
        cancel(&mut store, context(), input(3)),
        Err(Error::Conflict)
    );
    assert!(memory.borrow().eq(&bytes));
    let mut restored = StableGatewayRegistry::open(memory.clone(), config()).unwrap();
    assert_eq!(
        cancel(&mut restored, context(), input(2)),
        Err(Error::Fenced)
    );
    assert_eq!(
        restored
            .inspect(context(), scope())
            .unwrap()
            .sync
            .pending_sequence,
        Some(2)
    );
    assert!(memory.borrow().eq(&bytes));
}
#[test]
fn cancellation_rejects_invalid_scope_and_actual_caller_before_mutation() {
    let memory = VectorMemory::default();
    let mut store = StableGatewayRegistry::install(memory.clone(), config()).unwrap();
    store.begin_sync(context(), scope()).unwrap();
    let bytes = memory.borrow().clone();
    let input = input(1);
    for scope in [
        OperatorScope {
            service: p(8),
            ..input.scope
        },
        OperatorScope {
            namespace: u128::MAX,
            ..input.scope
        },
        OperatorScope {
            cashier: p(8),
            ..input.scope
        },
        OperatorScope {
            payment_account: p(8),
            ..input.scope
        },
    ] {
        assert_eq!(
            cancel(
                &mut store,
                context(),
                GatewaySyncCancellation { scope, ..input }
            ),
            Err(Error::Binding)
        );
    }
    assert_eq!(
        cancel(
            &mut store,
            context(),
            GatewaySyncCancellation {
                sequence: 0,
                ..input
            }
        ),
        Err(Error::Invalid)
    );
    assert_eq!(
        cancel(
            &mut store,
            UploadContext {
                actor: p(8),
                ..context()
            },
            input
        ),
        Err(Error::Denied)
    );
    assert_eq!(
        cancel(
            &mut store,
            UploadContext {
                service: p(8),
                ..context()
            },
            input
        ),
        Err(Error::Binding)
    );
    assert!(memory.borrow().eq(&bytes));
}
