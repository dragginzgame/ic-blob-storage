use super::*;
use crate::{
    ops::{
        caffeine::{
            gateway::{GatewayReplyError, GatewayReplyLimits},
            query::reply::{BoundGatewayReplyError, QueryReplyBindingError},
        },
        service::gateways::reply::GatewaySyncReplyError,
    },
    workflow::gateways::{begin_sync, cancel_sync, complete_sync},
};
use std::num::NonZeroUsize;
pub(super) fn limits() -> GatewayReplyLimits {
    GatewayReplyLimits {
        max_bytes: NonZeroUsize::new(4096).unwrap(),
        decoding_quota: NonZeroUsize::new(100_000).unwrap(),
        skipping_quota: NonZeroUsize::new(1000).unwrap(),
        max_type_entries: NonZeroUsize::new(32).unwrap(),
    }
}
fn rejected(error: GatewayReplyError) -> GatewaySyncReplyError {
    GatewaySyncReplyError::Reply(BoundGatewayReplyError::Reply(error))
}
#[test]
fn durable_gateway_query_uses_canonical_request_and_commits_only_a_valid_bounded_reply() {
    let memory = VectorMemory::default();
    let mut store = StableGatewayRegistry::install(memory.clone(), config()).unwrap();
    store.add(context(), scope(), p(9)).unwrap();
    let attempt = begin_sync(&mut store, context(), scope()).unwrap();
    assert_eq!(attempt.scope(), scope());
    assert_eq!(attempt.request().cashier(), scope().cashier());
    assert_eq!(attempt.request().method_name(), "storage_gateway_list_v1");
    candid::decode_args::<()>(attempt.request().arguments()).unwrap();
    let before = memory.borrow().clone();
    let good = candid::encode_one(vec![p(4), p(5), p(4)]).unwrap();
    for (bytes, bounded, error) in [
        (vec![], limits(), GatewayReplyError::InvalidReply),
        (
            good.clone(),
            GatewayReplyLimits {
                max_bytes: NonZeroUsize::MIN,
                ..limits()
            },
            GatewayReplyError::ReplyTooLarge,
        ),
        (
            good.clone(),
            GatewayReplyLimits {
                decoding_quota: NonZeroUsize::MIN,
                ..limits()
            },
            GatewayReplyError::InvalidReply,
        ),
        (
            candid::encode_one(Vec::<Principal>::new()).unwrap(),
            limits(),
            GatewayReplyError::Sync(GatewaySyncError::InvalidList(GatewayListError::Empty)),
        ),
        (
            candid::encode_one(vec![p(4); 9]).unwrap(),
            limits(),
            GatewayReplyError::Sync(GatewaySyncError::InvalidList(
                GatewayListError::TooManyEntries {
                    actual: 9,
                    maximum: 8,
                },
            )),
        ),
    ] {
        assert_eq!(
            complete_sync(&mut store, context(), &attempt, scope(), &bytes, bounded),
            Err(rejected(error))
        );
        assert_eq!(*memory.borrow(), before);
    }
    complete_sync(
        &mut store,
        context(),
        &attempt,
        scope(),
        &good,
        GatewayReplyLimits {
            max_bytes: NonZeroUsize::new(good.len()).unwrap(),
            ..limits()
        },
    )
    .unwrap();
    let view = store.inspect(context(), scope()).unwrap();
    assert_eq!(view.principals, vec![p(4), p(5)]);
    assert_eq!(view.sync.pending_sequence, None);
    let committed = memory.borrow().clone();
    assert_eq!(
        complete_sync(&mut store, context(), &attempt, scope(), &good, limits()),
        Err(rejected(GatewayReplyError::Sync(
            GatewaySyncError::StaleSync
        )))
    );
    assert_eq!(*memory.borrow(), committed);
}
#[test]
fn durable_gateway_reply_checks_authority_source_and_revocation_before_decode() {
    let memory = VectorMemory::default();
    let mut store = StableGatewayRegistry::install(memory.clone(), config()).unwrap();
    let old = begin_sync(&mut store, context(), scope()).unwrap();
    let before = memory.borrow().clone();
    let bad = vec![0; 4097];
    assert_eq!(
        complete_sync(
            &mut store,
            UploadContext {
                actor: p(9),
                ..context()
            },
            &old,
            scope(),
            &bad,
            limits()
        ),
        Err(GatewaySyncReplyError::Store(GatewayStoreError::NotOperator))
    );
    assert_eq!(
        complete_sync(
            &mut store,
            UploadContext {
                service: p(9),
                ..context()
            },
            &old,
            scope(),
            &bad,
            limits()
        ),
        Err(GatewaySyncReplyError::Store(GatewayStoreError::Binding))
    );
    for source in [
        GatewayScope::new(p(9), NonZeroU128::MIN, p(3)).unwrap(),
        GatewayScope::new(p(1), NonZeroU128::new(2).unwrap(), p(3)).unwrap(),
    ] {
        assert_eq!(
            complete_sync(&mut store, context(), &old, source, &bad, limits()),
            Err(rejected(GatewayReplyError::Sync(
                GatewaySyncError::WrongScope
            )))
        );
    }
    let wrong_cashier = GatewayScope::new(p(1), NonZeroU128::MIN, p(9)).unwrap();
    assert_eq!(
        complete_sync(&mut store, context(), &old, wrong_cashier, &bad, limits()),
        Err(GatewaySyncReplyError::Reply(
            BoundGatewayReplyError::Binding(QueryReplyBindingError::SourceMismatch)
        ))
    );
    assert_eq!(*memory.borrow(), before);
    store.remove(context(), scope(), p(4)).unwrap();
    let current = begin_sync(&mut store, context(), scope()).unwrap();
    let pending = memory.borrow().clone();
    assert_eq!(
        complete_sync(&mut store, context(), &old, scope(), &bad, limits()),
        Err(rejected(GatewayReplyError::Sync(
            GatewaySyncError::StaleSync
        )))
    );
    assert_eq!(
        cancel_sync(&mut store, context(), &old),
        Err(GatewaySyncReplyError::Store(GatewayStoreError::Sync(
            GatewaySyncError::StaleSync
        )))
    );
    assert_eq!(*memory.borrow(), pending);
    cancel_sync(&mut store, context(), &current).unwrap();
    assert_eq!(
        store
            .inspect(context(), scope())
            .unwrap()
            .sync
            .pending_sequence,
        None
    );
}
#[test]
fn durable_gateway_restoration_rejects_even_the_original_host_retained_request() {
    let memory = VectorMemory::default();
    let mut store = StableGatewayRegistry::install(memory.clone(), config()).unwrap();
    let attempt = begin_sync(&mut store, context(), scope()).unwrap();
    drop(store);
    let mut restored = StableGatewayRegistry::open(memory.clone(), config()).unwrap();
    let before = memory.borrow().clone();
    let expected = GatewaySyncReplyError::Store(GatewayStoreError::Fenced);
    assert_eq!(
        complete_sync(&mut restored, context(), &attempt, scope(), &[], limits()),
        Err(expected)
    );
    assert_eq!(
        cancel_sync(&mut restored, context(), &attempt),
        Err(expected)
    );
    assert!(
        matches!(begin_sync(&mut restored, context(), scope()), Err(error) if error == expected)
    );
    assert_eq!(*memory.borrow(), before);
    assert_eq!(
        restored
            .inspect(context(), scope())
            .unwrap()
            .sync
            .pending_sequence,
        Some(1)
    );
}
