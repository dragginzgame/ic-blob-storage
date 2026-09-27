use super::*;
use crate::{
    model::lifecycle::{binding::ObjectIdentity, roots::RootClaims},
    ops::service::tenant::tests::config,
};
use candid::Principal;
use ic_memory::ic_stable_structures::VectorMemory;
use std::num::NonZeroU128;

fn p(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}
fn context(tenant: u8) -> UploadContext {
    UploadContext {
        service: p(1),
        actor: p(tenant),
    }
}
fn root(n: u8) -> ProviderRootHash {
    ProviderRootHash::try_from([n; 32].as_slice()).unwrap()
}
fn object(tenant: u8, id: u128) -> ObjectBinding {
    ObjectBinding::new(
        p(1),
        p(tenant),
        ObjectIdentity {
            namespace: NonZeroU128::MIN,
            object: NonZeroU128::new(id).unwrap(),
            incarnation: NonZeroU128::MIN,
        },
    )
    .unwrap()
}
fn memories() -> (VectorMemory, VectorMemory) {
    (VectorMemory::default(), VectorMemory::default())
}

#[test]
fn heap_and_stable_claims_agree_and_full_history_keeps_exact_replays() {
    let (a, b) = memories();
    let mut stable = StableRootClaims::install(a.clone(), b.clone(), config()).unwrap();
    let mut heap = RootClaims::new(p(1), config().limits().catalog.max_objects).unwrap();
    for (r, o) in [
        (root(1), object(4, 1)),
        (root(1), object(4, 1)),
        (root(1), object(5, 1)),
        (root(2), object(4, 1)),
        (root(2), object(4, 2)),
        (root(3), object(4, 3)),
        (root(1), object(4, 1)),
    ] {
        let before = (a.borrow().clone(), b.borrow().clone());
        let expected = heap.claim(r, o);
        let actual = stable.claim(
            UploadContext {
                service: p(1),
                actor: o.tenant(),
            },
            r,
            o,
        );
        assert_eq!(actual, expected.map_err(RootStoreError::Claim));
        if actual != Ok(RootClaimOutcome::Claimed) {
            assert_eq!((&*a.borrow(), &*b.borrow()), (&before.0, &before.1));
        }
        assert_eq!(stable.slots(), heap.slots() as u64);
    }
    drop(stable);
    let mut reopened = StableRootClaims::open(a, b, config()).unwrap();
    assert!(reopened.is_fenced());
    assert_eq!(reopened.lookup(context(4), root(1)), Ok(Some(object(4, 1))));
    assert_eq!(reopened.lookup(context(5), root(1)), Ok(None));
    assert_eq!(
        reopened.claim(context(4), root(1), object(4, 1)),
        Err(RootStoreError::Fenced)
    );
}

#[test]
fn caller_service_namespace_and_full_object_identity_are_checked() {
    let (a, b) = memories();
    let mut store = StableRootClaims::install(a.clone(), b.clone(), config()).unwrap();
    let before = (a.borrow().clone(), b.borrow().clone());
    assert_eq!(
        store.claim(context(2), root(1), object(4, 1)),
        Err(RootStoreError::Denied)
    );
    let wrong = UploadContext {
        service: p(9),
        ..context(4)
    };
    assert_eq!(
        store.claim(wrong, root(1), object(4, 1)),
        Err(RootClaimError::WrongService.into())
    );
    let other_namespace = ObjectBinding::new(
        p(1),
        p(4),
        ObjectIdentity {
            namespace: NonZeroU128::new(2).unwrap(),
            ..object(4, 1).identity()
        },
    )
    .unwrap();
    assert_eq!(
        store.claim(context(4), root(1), other_namespace),
        Err(RootStoreError::WrongNamespace)
    );
    assert_eq!((&*a.borrow(), &*b.borrow()), (&before.0, &before.1));
    store.claim(context(4), root(1), object(4, 1)).unwrap();
    let next_lifetime = ObjectBinding::new(
        p(1),
        p(4),
        ObjectIdentity {
            incarnation: NonZeroU128::new(2).unwrap(),
            ..object(4, 1).identity()
        },
    )
    .unwrap();
    assert_eq!(
        store.claim(context(4), root(1), next_lifetime),
        Err(RootClaimError::RootAlreadyClaimed.into())
    );
}

#[test]
fn missing_indexes_or_configuration_changes_never_reset_retained_claims() {
    let (a, b) = memories();
    assert!(matches!(
        StableRootClaims::open(a.clone(), b.clone(), config()),
        Err(RootStoreError::Missing)
    ));
    let mut store = StableRootClaims::install(a.clone(), b.clone(), config()).unwrap();
    store.claim(context(4), root(1), object(4, 1)).unwrap();
    drop(store);
    let before = (a.borrow().clone(), b.borrow().clone());
    assert!(matches!(
        StableRootClaims::install(a.clone(), b.clone(), config()),
        Err(RootStoreError::AlreadyAllocated)
    ));
    assert!(matches!(
        StableRootClaims::open(a.clone(), VectorMemory::default(), config()),
        Err(RootStoreError::Missing)
    ));
    for field in 0..3 {
        let old = config();
        let mut bindings = old.bindings();
        let mut limits = old.limits();
        match field {
            0 => bindings.service = p(9),
            1 => bindings.namespace = NonZeroU128::new(2).unwrap(),
            _ => limits.catalog.max_objects = std::num::NonZeroUsize::new(3).unwrap(),
        }
        let changed = ServiceConfiguration::new(bindings, limits, old.billing()).unwrap();
        assert!(matches!(
            StableRootClaims::open(a.clone(), b.clone(), changed),
            Err(RootStoreError::Binding)
        ));
        assert_eq!((&*a.borrow(), &*b.borrow()), (&before.0, &before.1));
    }
}

#[test]
fn every_incomplete_or_conflicting_index_pair_is_rejected_without_repair() {
    for damage in 0..5 {
        let (a, b) = memories();
        let mut store = StableRootClaims::install(a.clone(), b.clone(), config()).unwrap();
        let key = RootObjectKeyRecord::new(object(4, 1));
        store.claim(context(4), root(1), object(4, 1)).unwrap();
        match damage {
            0 => {
                store.objects.remove(&key);
            }
            1 => {
                store.objects.insert(key, *root(2).as_bytes());
            }
            2 => {
                store.objects.remove(&key);
                store
                    .objects
                    .insert(RootObjectKeyRecord::new(object(4, 2)), *root(1).as_bytes());
            }
            3 => {
                store.roots.remove(&root_key(root(1)));
            }
            _ => {
                store
                    .roots
                    .insert(root_key(root(2)), RootClaimRecord::Claim(key));
            }
        }
        drop(store);
        let before = (a.borrow().clone(), b.borrow().clone());
        assert!(matches!(
            StableRootClaims::open(a.clone(), b.clone(), config()),
            Err(RootStoreError::InvalidRecord)
        ));
        assert_eq!((&*a.borrow(), &*b.borrow()), (&before.0, &before.1));
    }
}
