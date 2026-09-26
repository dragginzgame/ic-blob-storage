//! Bounded immutable root claims for adapters whose callbacks carry only roots.
//!
//! Claims cover the entire service, including tenants and provider namespaces,
//! because the reviewed callback does not identify those scopes. A root is never
//! reassigned or forgotten by this model, even after deletion or billing settlement.
//! This is transient: production use requires durable intent-before-effect claims,
//! exclusive provider ownership and restore fencing. An empty new map is not recovery.

use std::{collections::BTreeMap, num::NonZeroUsize};

use candid::Principal;
use thiserror::Error;

use crate::model::identity::ProviderRootHash;

use super::binding::{ObjectBinding, ObjectBindingError};

/// One service's lifetime root-to-object claims; not callback authentication.
///
/// Retired claims consume capacity permanently. Filling the bound rejects fresh
/// allocations without erasing history or blocking lookup of previous claims.
/// No duplicate-content sharing, namespace alias or root reuse is inferred.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RootClaims {
    service: Principal,
    limit: NonZeroUsize,
    claims: BTreeMap<ProviderRootHash, ObjectBinding>,
}

impl RootClaims {
    /// Start a fresh transient claim set, not a reset or restore of an existing one.
    /// # Errors
    /// Rejects anonymous or management service principals.
    pub fn new(service: Principal, limit: NonZeroUsize) -> Result<Self, ObjectBindingError> {
        if service == Principal::anonymous() || service == Principal::management_canister() {
            return Err(ObjectBindingError::InvalidService);
        }
        Ok(Self {
            service,
            limit,
            claims: BTreeMap::new(),
        })
    }

    /// Claim a root for exactly one object lifetime before exposing upload authority.
    ///
    /// Exact replay returns `Existing`; it never authorizes repeating an upload.
    /// Each object binding can claim only one root. Reverse lookup is bounded by
    /// the configured lifetime claim limit. The future workflow must persist this
    /// claim atomically with the upload intent before issuing a certificate/effect.
    /// # Errors
    /// Rejects wrong service, root reassignment, changed root for the same object,
    /// or full capacity. Rejection leaves all claims unchanged.
    pub fn claim(
        &mut self,
        root: ProviderRootHash,
        object: ObjectBinding,
    ) -> Result<RootClaimOutcome, RootClaimError> {
        if object.service() != self.service {
            return Err(RootClaimError::WrongService);
        }
        if let Some(existing) = self.claims.get(&root) {
            return if *existing == object {
                Ok(RootClaimOutcome::Existing)
            } else {
                Err(RootClaimError::RootAlreadyClaimed)
            };
        }
        if self.claims.values().any(|existing| *existing == object) {
            return Err(RootClaimError::ObjectAlreadyClaimed);
        }
        if self.claims.len() >= self.limit.get() {
            return Err(RootClaimError::LimitReached);
        }
        self.claims.insert(root, object);
        Ok(RootClaimOutcome::Claimed)
    }

    /// Resolve a root to its original immutable claim, never the newest object.
    ///
    /// Authenticate the gateway and its provider namespace separately. Resolution
    /// supplies correlation only: it does not prove physical deletion, billing stop
    /// or that the provider had no pre-existing object under this root. The lifecycle
    /// still rejects a live object's deletion and checks the complete binding.
    /// # Errors
    /// Unknown roots cannot acquire a claim through a callback.
    pub fn resolve(&self, root: ProviderRootHash) -> Result<ObjectBinding, RootClaimError> {
        self.claims
            .get(&root)
            .copied()
            .ok_or(RootClaimError::UnknownRoot)
    }

    /// Lifetime slots consumed, including claims for physically deleted objects.
    #[must_use]
    pub fn slots(&self) -> usize {
        self.claims.len()
    }
}

/// Whether a claim was newly allocated or was an exact local replay.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RootClaimOutcome {
    /// The root and object were previously unclaimed.
    Claimed,
    /// The original root/object pair already exists; no new effect is authorized.
    Existing,
}

/// Rejected claim or lookup; no identity records are removed or changed.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum RootClaimError {
    /// The object belongs to a different service.
    #[error("root claim belongs to another service")]
    WrongService,
    /// The root belongs to another tenant, namespace, object or incarnation.
    #[error("provider root is already claimed")]
    RootAlreadyClaimed,
    /// This exact object lifetime already names a different provider root.
    #[error("object already has a provider root")]
    ObjectAlreadyClaimed,
    /// Lifetime root capacity is exhausted; retired claims are not evicted.
    #[error("root claim limit reached")]
    LimitReached,
    /// A callback or lookup supplied a root absent from the claim history.
    #[error("provider root is unknown")]
    UnknownRoot,
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU128;

    use super::*;
    use crate::model::lifecycle::binding::ObjectIdentity;

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
}
