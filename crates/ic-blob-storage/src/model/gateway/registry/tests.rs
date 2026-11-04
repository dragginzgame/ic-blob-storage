use std::num::NonZeroUsize;

use super::*;
use ic_blob_storage_contracts::configuration::limits::GatewayListLimits;

fn p(value: u8) -> Principal {
    Principal::from_slice(&[value, 1])
}
fn scope(service: u8, namespace: u128, cashier: u8) -> GatewayScope {
    GatewayScope::new(
        p(service),
        NonZeroU128::new(namespace).expect("namespace"),
        p(cashier),
    )
    .expect("scope")
}
fn registry() -> GatewayRegistry {
    let mut gateways = GatewayMembership::new(GatewayListLimits {
        max_entries: NonZeroUsize::new(3).expect("entry bound"),
        max_unique: NonZeroUsize::new(2).expect("member bound"),
    });
    gateways.add(p(3)).expect("initial gateway");
    GatewayRegistry::new(scope(1, 1, 2), gateways)
}

#[test]
fn explicit_refresh_supersedes_pending_read_without_accepting_its_reply_or_cancel() {
    let mut registry = registry();
    let old = registry.begin_sync().unwrap();
    let current = registry.begin_sync().unwrap();
    let before = registry.clone();
    assert_eq!(
        registry.apply_sync(old, scope(1, 1, 2), &[p(4)]),
        Err(GatewaySyncError::StaleSync)
    );
    assert_eq!(registry.cancel_sync(old), Err(GatewaySyncError::StaleSync));
    assert_eq!(registry, before);
    registry
        .apply_sync(current, scope(1, 1, 2), &[p(5)])
        .unwrap();
    assert_eq!(registry.gateways().principals(), &[p(5)]);
    assert_eq!(registry.sync_view().pending_sequence, None);
}

#[test]
fn cancelled_and_replayed_responses_cannot_replace_newer_membership() {
    let mut registry = registry();
    let old = registry.begin_sync().expect("old attempt");
    assert_eq!(
        registry.sync_view(),
        GatewaySyncView {
            last_sequence: 1,
            pending_sequence: Some(1),
        }
    );
    registry.cancel_sync(old).expect("cancel old read");
    assert_eq!(
        registry.sync_view(),
        GatewaySyncView {
            last_sequence: 1,
            pending_sequence: None,
        }
    );
    let current = registry.begin_sync().expect("new attempt");
    assert_ne!(old, current);
    let before = registry.clone();
    assert_eq!(
        registry.apply_sync(old, registry.scope(), &[p(4)]),
        Err(GatewaySyncError::StaleSync)
    );
    assert_eq!(registry.cancel_sync(old), Err(GatewaySyncError::StaleSync));
    assert_eq!(registry, before);
    registry
        .apply_sync(current, registry.scope(), &[p(5), p(4), p(5)])
        .expect("current valid reply");
    assert_eq!(registry.gateways().principals(), &[p(5), p(4)]);
    let completed = registry.clone();
    assert_eq!(
        registry.sync_view(),
        GatewaySyncView {
            last_sequence: 2,
            pending_sequence: None,
        }
    );
    assert_eq!(
        registry.apply_sync(current, registry.scope(), &[p(3)]),
        Err(GatewaySyncError::StaleSync)
    );
    assert_eq!(registry, completed);
}

#[test]
fn operator_edits_invalidate_old_sync_even_when_membership_does_not_change() {
    for principal in [p(3), p(4)] {
        let mut registry = registry();
        let token = registry.begin_sync().expect("sync");
        assert_eq!(registry.remove(principal), principal == p(3));
        let edited = registry.clone();
        assert_eq!(
            registry.apply_sync(token, registry.scope(), &[p(3), p(4)]),
            Err(GatewaySyncError::StaleSync)
        );
        assert_eq!(registry, edited);
    }
    for principal in [p(3), p(4)] {
        let mut registry = registry();
        let token = registry.begin_sync().expect("sync");
        registry.add(principal).expect("operator add");
        let edited = registry.clone();
        assert_eq!(
            registry.apply_sync(token, registry.scope(), &[p(5)]),
            Err(GatewaySyncError::StaleSync)
        );
        assert_eq!(registry, edited);
    }
}

#[test]
fn wrong_scope_and_invalid_candidates_preserve_the_pending_attempt() {
    let mut registry = registry();
    let token = registry.begin_sync().expect("sync");
    let before = registry.clone();
    for wrong in [scope(4, 1, 2), scope(1, 2, 2), scope(1, 1, 4)] {
        assert_eq!(
            registry.apply_sync(token, wrong, &[p(5)]),
            Err(GatewaySyncError::WrongScope)
        );
        let foreign = GatewaySyncToken {
            scope: wrong,
            sequence: token.sequence,
        };
        assert_eq!(
            registry.apply_sync(foreign, registry.scope(), &[p(5)]),
            Err(GatewaySyncError::WrongScope)
        );
        assert_eq!(
            registry.cancel_sync(foreign),
            Err(GatewaySyncError::WrongScope)
        );
        assert_eq!(registry, before);
    }
    assert_eq!(
        registry.apply_sync(token, registry.scope(), &[]),
        Err(GatewaySyncError::InvalidList(GatewayListError::Empty))
    );
    assert_eq!(
        registry.apply_sync(token, registry.scope(), &[p(4), Principal::anonymous()]),
        Err(GatewaySyncError::InvalidList(
            GatewayListError::InvalidPrincipal {
                index: 1,
                principal: Principal::anonymous()
            }
        ))
    );
    assert_eq!(
        registry.add(Principal::anonymous()),
        Err(GatewayListError::InvalidPrincipal {
            index: 0,
            principal: Principal::anonymous()
        })
    );
    assert_eq!(registry, before);
    registry
        .apply_sync(token, registry.scope(), &[p(4)])
        .expect("unchanged attempt remains usable");
}

#[test]
fn sequence_exhaustion_never_prevents_revocation_or_revalidates_a_token() {
    let mut registry = registry();
    registry.last_sequence = u64::MAX - 1;
    let last = registry.begin_sync().expect("last available identity");
    assert!(registry.remove(p(3)));
    let revoked = registry.clone();
    assert_eq!(
        registry.begin_sync(),
        Err(GatewaySyncError::SequenceExhausted)
    );
    assert_eq!(
        registry.apply_sync(last, registry.scope(), &[p(3)]),
        Err(GatewaySyncError::StaleSync)
    );
    assert_eq!(registry, revoked);
    registry
        .add(p(4))
        .expect("explicit membership edits need no sync identity");
    assert!(registry.remove(p(4)));
    assert_eq!(registry.gateways().principals(), []);
}

#[test]
fn special_principals_cannot_own_a_registry_scope() {
    let namespace = NonZeroU128::new(1).expect("namespace");
    for invalid in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            GatewayScope::new(invalid, namespace, p(2)),
            Err(GatewayScopeError::InvalidService)
        );
        assert_eq!(
            GatewayScope::new(p(1), namespace, invalid),
            Err(GatewayScopeError::InvalidCashier)
        );
    }
}
