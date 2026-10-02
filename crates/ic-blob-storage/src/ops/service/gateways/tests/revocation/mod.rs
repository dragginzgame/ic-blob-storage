use super::*;
use crate::{
    dto::{
        gateway::{
            GatewayRevocationFailure as Error, GatewayRevocationRequest, GatewayRevocationResponse,
        },
        operator::OperatorScope,
    },
    workflow::gateways::revocation::revoke,
};
fn request() -> GatewayRevocationRequest {
    GatewayRevocationRequest {
        scope: OperatorScope {
            service: p(1),
            cashier: p(3),
            payment_account: config().bindings().payment_account,
            namespace: 1,
        },
        gateway: p(4),
    }
}
#[test]
fn shared_revocation_invalidates_sync_and_reads_even_when_absent_and_survives_restore() {
    let memory = VectorMemory::default();
    let mut store = StableGatewayRegistry::install(memory.clone(), config()).unwrap();
    store.add(context(), scope(), p(4)).unwrap();
    for removed in [true, false] {
        let token = store.begin_sync(context(), scope()).unwrap();
        let generation = store.registry().unwrap().read_generation.current();
        let response = revoke(&mut store, context(), request()).unwrap();
        assert_eq!(
            response,
            GatewayRevocationResponse {
                request: request(),
                removed
            }
        );
        let bytes = candid::encode_one(response).unwrap();
        assert_eq!(
            candid::decode_one::<GatewayRevocationResponse>(&bytes).unwrap(),
            response
        );
        assert_eq!(store.inspect(context(), scope()).unwrap().principals, []);
        assert_eq!(
            store
                .inspect(context(), scope())
                .unwrap()
                .sync
                .pending_sequence,
            None
        );
        assert_ne!(
            store.registry().unwrap().read_generation.current(),
            generation
        );
        assert_eq!(
            store.registry().unwrap().check_sync(token, scope()),
            Err(GatewaySyncError::StaleSync)
        );
    }
    let before = memory.borrow().clone();
    let mut restored = StableGatewayRegistry::open(memory.clone(), config()).unwrap();
    assert_eq!(restored.inspect(context(), scope()).unwrap().principals, []);
    assert_eq!(
        revoke(&mut restored, context(), request()),
        Err(Error::Fenced)
    );
    assert_eq!(*memory.borrow(), before);
}
#[test]
fn shared_revocation_checks_full_scope_and_actual_caller_without_writes() {
    let memory = VectorMemory::default();
    let mut store = StableGatewayRegistry::install(memory.clone(), config()).unwrap();
    store.add(context(), scope(), p(4)).unwrap();
    store.begin_sync(context(), scope()).unwrap();
    let before = memory.borrow().clone();
    let original = request();
    for changed in [
        OperatorScope {
            service: p(8),
            ..original.scope
        },
        OperatorScope {
            cashier: p(8),
            ..original.scope
        },
        OperatorScope {
            payment_account: p(8),
            ..original.scope
        },
        OperatorScope {
            namespace: u128::MAX,
            ..original.scope
        },
    ] {
        assert_eq!(
            revoke(
                &mut store,
                context(),
                GatewayRevocationRequest {
                    scope: changed,
                    ..original
                }
            ),
            Err(Error::Binding)
        );
    }
    assert_eq!(
        revoke(
            &mut store,
            UploadContext {
                service: p(8),
                ..context()
            },
            original
        ),
        Err(Error::Binding)
    );
    for actor in [
        p(8),
        Principal::anonymous(),
        Principal::management_canister(),
    ] {
        assert_eq!(
            revoke(&mut store, UploadContext { actor, ..context() }, original),
            Err(Error::Denied)
        );
    }
    for gateway in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            revoke(
                &mut store,
                context(),
                GatewayRevocationRequest {
                    gateway,
                    ..original
                }
            ),
            Err(Error::Invalid)
        );
    }
    assert_eq!(
        revoke(
            &mut store,
            context(),
            GatewayRevocationRequest {
                scope: OperatorScope {
                    namespace: 0,
                    ..original.scope
                },
                ..original
            }
        ),
        Err(Error::Invalid)
    );
    assert_eq!(*memory.borrow(), before);
}
