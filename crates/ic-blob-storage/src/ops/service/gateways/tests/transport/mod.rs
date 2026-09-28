use super::*;
use crate::{
    ops::{
        caffeine::query::{
            CashierQueryRequest,
            reply::{BoundGatewayReplyError, QueryReplyBindingError},
            transport::{CashierQueryResponse, CashierQueryTransport},
        },
        service::gateways::{access::GatewayRegistryAccess, reply::GatewaySyncReplyError},
    },
    workflow::gateways::{
        begin_sync,
        transport::{GatewayQueryError, query_sync},
    },
};
use std::{
    cell::{Cell, RefCell},
    future::Future,
    num::NonZeroUsize,
    task::{Context, Poll, Waker},
};
struct Host(RefCell<StableGatewayRegistry<VectorMemory>>);
impl GatewayRegistryAccess for Host {
    type Memory = VectorMemory;
    fn with_gateway_registry<R>(
        &self,
        op: impl FnOnce(&mut StableGatewayRegistry<VectorMemory>) -> R,
    ) -> R {
        op(&mut self.0.borrow_mut())
    }
}
struct Reply {
    cashier: Principal,
    calls: Cell<usize>,
}
impl CashierQueryTransport for Reply {
    type Error = ();
    fn query(
        &self,
        request: &CashierQueryRequest,
        max_bytes: NonZeroUsize,
    ) -> impl Future<Output = Result<CashierQueryResponse, ()>> {
        assert_eq!(request.cashier(), scope().cashier());
        assert_eq!(request.method_name(), "storage_gateway_list_v1");
        candid::decode_args::<()>(request.arguments()).unwrap();
        self.calls.set(self.calls.get() + 1);
        let bytes = candid::encode_one(vec![p(4)]).unwrap();
        assert!(bytes.len() <= max_bytes.get());
        std::future::ready(Ok(CashierQueryResponse {
            cashier: self.cashier,
            bytes,
        }))
    }
}
fn ready<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let Poll::Ready(value) = future
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    else {
        panic!("immediate test transport");
    };
    value
}
#[test]
fn gateway_transport_checks_pending_state_on_poll_and_source_before_committing() {
    let memory = VectorMemory::default();
    let host = Host(RefCell::new(
        StableGatewayRegistry::install(memory.clone(), config()).unwrap(),
    ));
    let attempt = host
        .with_gateway_registry(|store| begin_sync(store, context(), scope()))
        .unwrap();
    let wrong = Reply {
        cashier: p(8),
        calls: Cell::new(0),
    };
    let before = memory.borrow().clone();
    let result = ready(query_sync(
        &host,
        context(),
        &attempt,
        &wrong,
        super::reply::limits(),
    ));
    assert!(matches!(
        result,
        Err(GatewayQueryError::Registry(GatewaySyncReplyError::Reply(
            BoundGatewayReplyError::Binding(QueryReplyBindingError::SourceMismatch)
        )))
    ));
    assert_eq!(wrong.calls.get(), 1);
    assert_eq!(*memory.borrow(), before);
    let transport = Reply {
        cashier: scope().cashier(),
        calls: Cell::new(0),
    };
    let future = query_sync(
        &host,
        context(),
        &attempt,
        &transport,
        super::reply::limits(),
    );
    host.with_gateway_registry(|store| store.remove(context(), scope(), p(4)))
        .unwrap();
    assert!(matches!(
        ready(future),
        Err(GatewayQueryError::Registry(GatewaySyncReplyError::Store(
            GatewayStoreError::Sync(GatewaySyncError::StaleSync)
        )))
    ));
    assert_eq!(transport.calls.get(), 0);
    let current = host
        .with_gateway_registry(|store| begin_sync(store, context(), scope()))
        .unwrap();
    ready(query_sync(
        &host,
        context(),
        &current,
        &transport,
        super::reply::limits(),
    ))
    .unwrap();
    assert_eq!(transport.calls.get(), 1);
    assert_eq!(
        host.with_gateway_registry(|store| store.inspect(context(), scope()))
            .unwrap()
            .principals,
        vec![p(4)]
    );
}
