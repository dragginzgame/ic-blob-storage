use std::num::NonZeroU128;

use super::*;
use ic_blob_storage_contracts::binding::ObjectIdentity;

fn principal(id: u8) -> Principal {
    Principal::from_slice(&[id, 1])
}

fn object(tenant: u8, namespace: u128, incarnation: u128) -> ObjectBinding {
    ObjectBinding::new(
        principal(1),
        principal(tenant),
        ObjectIdentity {
            namespace: NonZeroU128::new(namespace).expect("namespace"),
            object: NonZeroU128::new(1).expect("object"),
            incarnation: NonZeroU128::new(incarnation).expect("incarnation"),
        },
    )
    .expect("binding")
}

fn root(value: u8) -> ProviderRootHash {
    ProviderRootHash::try_from([value; 32].as_slice()).expect("root")
}

fn claims(limit: usize) -> RootClaims {
    RootClaims::new(principal(1), NonZeroUsize::new(limit).expect("limit")).expect("service")
}

#[test]
fn root_is_never_reassigned_across_tenant_namespace_or_incarnation() {
    let original = object(2, 1, 1);
    let mut claims = claims(4);
    assert_eq!(
        claims.claim(root(1), original),
        Ok(RootClaimOutcome::Claimed)
    );
    let before = claims.clone();
    for other in [object(3, 1, 1), object(2, 2, 1), object(2, 1, 2)] {
        assert_eq!(
            claims.claim(root(1), other),
            Err(RootClaimError::RootAlreadyClaimed)
        );
        assert_eq!(claims, before);
    }
    assert_eq!(
        claims.claim(root(2), original),
        Err(RootClaimError::ObjectAlreadyClaimed)
    );
    assert_eq!(claims.resolve(root(1)), Ok(original));
    assert_eq!(claims.resolve(root(2)), Err(RootClaimError::UnknownRoot));
    assert_eq!(claims, before);
}

#[test]
fn capacity_preserves_exact_replay_and_existing_callback_resolution() {
    let original = object(2, 1, 1);
    let mut claims = claims(1);
    claims.claim(root(1), original).expect("claim");
    let before = claims.clone();
    assert_eq!(
        claims.claim(root(2), object(2, 1, 2)),
        Err(RootClaimError::LimitReached)
    );
    assert_eq!(
        claims.claim(root(1), original),
        Ok(RootClaimOutcome::Existing)
    );
    assert_eq!(claims.resolve(root(1)), Ok(original));
    assert_eq!(claims.slots(), 1);
    assert_eq!(claims, before);
}

#[test]
fn invalid_or_wrong_service_never_acquires_a_claim() {
    for invalid in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            RootClaims::new(invalid, NonZeroUsize::new(1).expect("limit")),
            Err(ObjectBindingError::InvalidService)
        );
    }
    let mut claims = claims(1);
    let other = ObjectBinding::new(principal(3), principal(2), object(2, 1, 1).identity())
        .expect("other service");
    assert_eq!(
        claims.claim(root(1), other),
        Err(RootClaimError::WrongService)
    );
    assert_eq!(claims.slots(), 0);
}
