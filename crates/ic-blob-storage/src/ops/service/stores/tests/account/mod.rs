//! Pure host composition: an independently restored owner cannot publish a late report.
use super::*;
use crate::ops::caffeine::query::CashierQueryRequest;
use crate::ops::caffeine::query::transport::CashierQueryResponse;
use crate::ops::caffeine::query::transport::CashierQueryTransport;
use crate::ops::caffeine::query::transport::replicated::ReplicatedQueryError;
use crate::ops::service::account::AccountInspectionAccess;
use crate::ops::service::account::AccountInspectionLimits;
use crate::ops::service::operator::OperatorStores;
use ic_blob_storage_contracts::dto::account::*;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use std::{
    cell::{Cell, RefCell},
    future::Future,
    num::NonZeroUsize,
    task::{Context, Poll, Waker},
};

struct Host {
    stores: RefCell<ServiceStores<VectorMemory>>,
    memory: [VectorMemory; 16],
    config: ServiceStoreConfiguration,
}
impl AccountInspectionAccess for Host {
    type Memory = VectorMemory;
    fn with_account_stores<R>(&self, f: impl FnOnce(OperatorStores<'_, Self::Memory>) -> R) -> R {
        f(OperatorStores::from(&*self.stores.borrow()))
    }
}
struct Source<'a> {
    host: &'a Host,
    calls: Cell<u32>,
    restore: bool,
}
impl CashierQueryTransport for Source<'_> {
    type Error = ReplicatedQueryError;
    fn query(
        &self,
        request: &CashierQueryRequest,
        _: NonZeroUsize,
    ) -> impl Future<Output = Result<CashierQueryResponse, Self::Error>> {
        self.calls.set(self.calls.get() + 1);
        if self.restore {
            // No store borrow may survive dispatch; publish an independently reopened owner.
            *self.host.stores.borrow_mut() =
                ServiceStores::open(memories(&self.host.memory), self.host.config).unwrap();
        }
        std::future::ready(Ok(CashierQueryResponse {
            cashier: request.cashier(),
            bytes: vec![0],
        }))
    }
}
fn ready<F: Future>(future: F) -> F::Output {
    match std::pin::pin!(future)
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("local source completes immediately"),
    }
}
#[test]
fn account_inspection_checks_authority_before_dispatch_and_restore_before_decoding() {
    let input = candidate();
    let config = validate_candidate(input.service, input).unwrap();
    let memory = std::array::from_fn(|_| VectorMemory::default());
    let host = Host {
        stores: RefCell::new(ServiceStores::install(memories(&memory), config).unwrap()),
        memory,
        config,
    };
    let source = Source {
        host: &host,
        calls: Cell::new(0),
        restore: true,
    };
    let context = UploadContext {
        service: input.service,
        actor: input.operator,
    };
    let request = AccountInspectionRequest {
        scope: OperatorScope {
            service: input.service,
            namespace: input.namespace,
            cashier: input.billing.cashier,
            payment_account: input.payment_account,
        },
        kind: AccountInspectionKind::Balance,
    };
    let limits = AccountInspectionLimits {
        max_bytes: 4096.try_into().unwrap(),
        decoding_quota: 500_000.try_into().unwrap(),
        skipping_quota: 1000.try_into().unwrap(),
        max_type_entries: 64.try_into().unwrap(),
    };
    assert_eq!(
        ready(crate::workflow::account::inspect(
            &host,
            &source,
            UploadContext {
                actor: input.service,
                ..context
            },
            request,
            limits
        )),
        Err(AccountInspectionFailure::Denied)
    );
    assert_eq!(source.calls.get(), 0);
    let before = host.memory.each_ref().map(|m| m.borrow().clone());
    assert_eq!(
        ready(crate::workflow::account::inspect(
            &host, &source, context, request, limits
        )),
        Err(AccountInspectionFailure::Fenced)
    );
    assert_eq!(source.calls.get(), 1);
    assert_eq!(host.memory.each_ref().map(|m| m.borrow().clone()), before);
    assert_eq!(
        ready(crate::workflow::account::inspect(
            &host, &source, context, request, limits
        )),
        Err(AccountInspectionFailure::Fenced)
    );
    assert_eq!(source.calls.get(), 1);
}
