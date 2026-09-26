//! Native composition over trusted values, not endpoint/IC authentication proof.

use std::num::{NonZeroU128, NonZeroUsize};

use candid::Principal;
use ic_blob_storage::{
    model::{
        gateway::{
            GatewayListLimits,
            membership::GatewayMembership,
            registry::{GatewayRegistry, GatewayScope, GatewaySyncError},
        },
        lifecycle::binding::{ObjectBinding, ObjectIdentity},
    },
    ops::caffeine::gateway::{GatewayReplyError, GatewayReplyLimits, apply_gateway_sync_reply},
    policy::gateway::{GatewayAccessError, GatewayCallbackContext, assess_gateway_callback},
};

fn p(value: u8) -> Principal {
    Principal::from_slice(&[value, 1])
}
fn number(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).expect("identity")
}
fn registry(service: u8, namespace: u128) -> GatewayRegistry {
    let mut gateways = GatewayMembership::new(GatewayListLimits {
        max_entries: NonZeroUsize::new(2).expect("entry bound"),
        max_unique: NonZeroUsize::new(2).expect("member bound"),
    });
    gateways.add(p(3)).expect("initial gateway");
    GatewayRegistry::new(
        GatewayScope::new(p(service), number(namespace), p(2)).expect("scope"),
        gateways,
    )
}
fn object() -> ObjectBinding {
    ObjectBinding::new(
        p(1),
        p(4),
        ObjectIdentity {
            namespace: number(1),
            object: number(1),
            incarnation: number(1),
        },
    )
    .expect("object")
}

#[test]
fn revoked_gateway_stays_denied_after_a_stale_sync_reply() {
    let mut registry = registry(1, 1);
    let context = GatewayCallbackContext {
        service: p(1),
        actor: p(3),
    };
    assert_eq!(
        assess_gateway_callback(object(), &registry, context),
        Ok(())
    );
    let old = registry.begin_sync().expect("read-only sync begun");
    let scope = registry.scope();
    let limits = GatewayReplyLimits {
        max_bytes: NonZeroUsize::new(4096).expect("byte bound"),
        decoding_quota: NonZeroUsize::new(100_000).expect("work bound"),
        skipping_quota: NonZeroUsize::new(1000).expect("skip bound"),
        max_type_entries: NonZeroUsize::new(32).expect("type bound"),
    };
    let old_reply = candid::encode_one(vec![p(3)]).expect("old gateway list");
    assert!(registry.remove(p(3)));
    assert_eq!(
        apply_gateway_sync_reply(&mut registry, old, scope, &old_reply, limits),
        Err(GatewayReplyError::Sync(GatewaySyncError::StaleSync))
    );
    assert_eq!(
        assess_gateway_callback(object(), &registry, context),
        Err(GatewayAccessError::NotGateway)
    );
    // A new explicit sync is a new authority decision, not a delayed old reply.
    let fresh = registry.begin_sync().expect("new authorized sync");
    let new_reply = candid::encode_one(vec![p(5)]).expect("new gateway list");
    apply_gateway_sync_reply(&mut registry, fresh, scope, &new_reply, limits)
        .expect("replace gateway from decoded reply");
    assert_eq!(
        assess_gateway_callback(object(), &registry, context),
        Err(GatewayAccessError::NotGateway)
    );
    assert_eq!(
        assess_gateway_callback(
            object(),
            &registry,
            GatewayCallbackContext {
                actor: p(5),
                ..context
            }
        ),
        Ok(())
    );
}

#[test]
fn membership_cannot_override_service_namespace_or_actor_binding() {
    let registry = registry(1, 1);
    let context = GatewayCallbackContext {
        service: p(1),
        actor: p(3),
    };
    assert_eq!(
        assess_gateway_callback(
            object(),
            &registry,
            GatewayCallbackContext {
                service: p(7),
                ..context
            }
        ),
        Err(GatewayAccessError::WrongService)
    );
    for actor in [
        Principal::anonymous(),
        Principal::management_canister(),
        p(1),
        p(2),
        p(4),
    ] {
        assert_eq!(
            assess_gateway_callback(
                object(),
                &registry,
                GatewayCallbackContext { actor, ..context }
            ),
            Err(GatewayAccessError::NotGateway)
        );
    }
    let other_service = GatewayScope::new(p(7), number(1), p(2)).expect("other service");
    let other_namespace = GatewayScope::new(p(1), number(2), p(2)).expect("other namespace");
    for (scope, error) in [
        (other_service, GatewayAccessError::WrongService),
        (other_namespace, GatewayAccessError::WrongNamespace),
    ] {
        let foreign = GatewayRegistry::new(scope, registry.gateways().clone());
        assert_eq!(
            assess_gateway_callback(object(), &foreign, context),
            Err(error)
        );
    }
}
