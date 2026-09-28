use super::*;
use crate::{
    model::{
        identity::ProviderRootHash,
        lifecycle::{
            ReferenceId,
            binding::{ObjectBinding, ObjectIdentity, ReferenceKey},
        },
        service::read::ReadTarget,
    },
    ops::service::tenant::tests::config,
};
use ic_memory::ic_stable_structures::{Storable, VectorMemory};
use std::num::{NonZeroU64, NonZeroU128};
fn p(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}
fn operator() -> UploadContext {
    UploadContext {
        service: p(1),
        actor: p(2),
    }
}
fn limits() -> ReadSessionLimits {
    ReadSessionLimits::new(
        3.try_into().unwrap(),
        2.try_into().unwrap(),
        64.try_into().unwrap(),
        192.try_into().unwrap(),
        128.try_into().unwrap(),
    )
    .unwrap()
}
fn memory() -> ReadSessionMemories<VectorMemory> {
    ReadSessionMemories {
        journal: VectorMemory::default(),
        sessions: VectorMemory::default(),
        tenants: VectorMemory::default(),
    }
}
fn cloned(m: &ReadSessionMemories<VectorMemory>) -> ReadSessionMemories<VectorMemory> {
    ReadSessionMemories {
        journal: m.journal.clone(),
        sessions: m.sessions.clone(),
        tenants: m.tenants.clone(),
    }
}
fn intent(tenant: u8) -> ReadSessionIntent {
    let object = ObjectBinding::new(
        p(1),
        p(tenant),
        ObjectIdentity {
            namespace: NonZeroU128::MIN,
            object: NonZeroU128::MIN,
            incarnation: NonZeroU128::MIN,
        },
    )
    .unwrap();
    ReadSessionIntent {
        context: UploadContext {
            service: p(1),
            actor: p(tenant),
        },
        scope: GatewayScope::new(p(1), NonZeroU128::MIN, p(3)).unwrap(),
        chunk: ReadChunkTarget {
            target: ReadTarget {
                root: ProviderRootHash::try_from([1; 32].as_slice()).unwrap(),
                reference: ReferenceKey::new(object, ReferenceId::new(NonZeroU128::MIN)),
                gateway: p(6),
            },
            index: 0,
        },
        gateway_generation: 3,
        tenant_generation: NonZeroU64::MIN,
    }
}
#[test]
fn read_session_capacity_is_tenant_and_global_and_only_exact_callbacks_release_it() {
    let m = memory();
    let mut store = StableReadSessions::install(cloned(&m), config(), limits()).unwrap();
    let first = store.reserve(&intent(4)).unwrap();
    let second = store.reserve(&intent(4)).unwrap();
    assert_eq!(store.reserve(&intent(4)), Err(ReadSessionError::Capacity));
    let third = store.reserve(&intent(5)).unwrap();
    assert_eq!(store.reserve(&intent(5)), Err(ReadSessionError::Capacity));
    assert_eq!(
        store.inspect(operator()).unwrap().usage,
        ReadSessionUsage {
            sessions: 3,
            reserved_bytes: 192
        }
    );
    assert_eq!(
        store.complete(operator(), &first),
        Err(ReadSessionError::Denied)
    );
    let mut changed = first.clone();
    changed.intent.chunk.index = 1;
    assert_eq!(
        store.complete(intent(4).context, &changed),
        Err(ReadSessionError::Stale)
    );
    store.complete(intent(4).context, &first).unwrap();
    let newer = store.reserve(&intent(4)).unwrap();
    assert!(newer.sequence > third.sequence);
    assert_eq!(
        store.complete(intent(4).context, &first),
        Err(ReadSessionError::Stale)
    );
    assert_eq!(store.inspect(operator()).unwrap().usage.sessions, 3);
    let page = store
        .inspect_active(operator(), second.sequence, NonZeroU32::MIN)
        .unwrap();
    assert_eq!(page[0].sequence, third.sequence);
    for ticket in [&second, &third, &newer] {
        store.complete(ticket.intent.context, ticket).unwrap();
    }
    assert_eq!(store.tenants.len(), 0);
    assert_eq!(store.sessions.len(), 0);
    assert_eq!(
        store.inspect(operator()).unwrap().last_sequence,
        newer.sequence
    );
    assert_eq!(store.inspect(operator()).unwrap().usage.reserved_bytes, 0);
}
#[test]
fn read_session_byte_limits_are_independent_and_codec_restore_preserves_fenced_occupancy() {
    let limits = ReadSessionLimits::new(
        8.try_into().unwrap(),
        4.try_into().unwrap(),
        64.try_into().unwrap(),
        128.try_into().unwrap(),
        64.try_into().unwrap(),
    )
    .unwrap();
    let m = memory();
    let mut store = StableReadSessions::install(cloned(&m), config(), limits).unwrap();
    let first = store.reserve(&intent(4)).unwrap();
    assert_eq!(store.reserve(&intent(4)), Err(ReadSessionError::Capacity));
    store.reserve(&intent(5)).unwrap();
    assert_eq!(store.reserve(&intent(6)), Err(ReadSessionError::Capacity));
    let record = store.sessions.get(&first.sequence).unwrap();
    assert_eq!(ReadSessionRecord::from_bytes(record.to_bytes()), record);
    let summary = store.inspect(operator()).unwrap();
    let mut restored = StableReadSessions::open(cloned(&m), config(), limits).unwrap();
    assert_eq!(
        restored.inspect(operator()).unwrap(),
        ReadSessionSummary {
            fenced: true,
            ..summary
        }
    );
    assert_eq!(restored.reserve(&intent(4)), Err(ReadSessionError::Fenced));
    assert_eq!(
        restored.complete(first.intent.context, &first),
        Err(ReadSessionError::Fenced)
    );
    assert_eq!(
        restored
            .inspect_active(operator(), 0, NonZeroU32::new(64).unwrap())
            .unwrap()
            .len(),
        2
    );
    assert!(matches!(
        StableReadSessions::open(cloned(&m), config(), self::limits()),
        Err(ReadSessionError::Binding)
    ));
    store.tenants.remove(&p(4));
    assert!(matches!(
        StableReadSessions::open(cloned(&m), config(), limits),
        Err(ReadSessionError::InvalidRecord)
    ));
    assert_eq!(store.sessions.len(), 2);
}
