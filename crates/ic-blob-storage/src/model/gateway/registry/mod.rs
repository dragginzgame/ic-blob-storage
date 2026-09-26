//! Transient, scoped gateway membership with revocation-safe sync correlation.
//!
//! One authoritative instance must own a service/namespace's membership. Tokens
//! correlate local read-only sync attempts, not paid effects or caller authority.
//! Persistence, restore fencing and exclusion of stale instances remain external;
//! reconstructing or cloning this value does not establish a fresh identity.

use std::num::NonZeroU128;

use candid::Principal;
use thiserror::Error;

use super::{
    GatewayListError,
    membership::{GatewayAddOutcome, GatewayMembership},
};

/// Trusted service configuration identifying one gateway registry owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GatewayScope {
    service: Principal,
    namespace: NonZeroU128,
    cashier: Principal,
}

impl GatewayScope {
    /// Bind a registry to an actual service and its configured provider namespace.
    ///
    /// This validates representation only, not deployment or account ownership.
    /// # Errors
    /// Anonymous and management principals cannot fill either service role.
    pub fn new(
        service: Principal,
        namespace: NonZeroU128,
        cashier: Principal,
    ) -> Result<Self, GatewayScopeError> {
        if !concrete(service) {
            return Err(GatewayScopeError::InvalidService);
        }
        if !concrete(cashier) {
            return Err(GatewayScopeError::InvalidCashier);
        }
        Ok(Self {
            service,
            namespace,
            cashier,
        })
    }

    /// Service that owns this registry.
    #[must_use]
    pub const fn service(self) -> Principal {
        self.service
    }

    /// Opaque namespace resolved through trusted provider configuration.
    #[must_use]
    pub const fn namespace(self) -> NonZeroU128 {
        self.namespace
    }

    /// Configured Cashier from which this registry accepts synchronization data.
    #[must_use]
    pub const fn cashier(self) -> Principal {
        self.cashier
    }
}

fn concrete(principal: Principal) -> bool {
    principal != Principal::anonymous() && principal != Principal::management_canister()
}

/// Invalid principal in a registry's trusted scope.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum GatewayScopeError {
    /// Service cannot be anonymous or management.
    #[error("invalid gateway registry service")]
    InvalidService,
    /// Cashier cannot be anonymous or management.
    #[error("invalid gateway registry Cashier")]
    InvalidCashier,
}

/// Exact local sync attempt; minted only by its registry.
///
/// A token does not authorize calls, verify a response's source or survive an old
/// backup safely. It must not be accepted as an untrusted endpoint request DTO.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GatewaySyncToken {
    scope: GatewayScope,
    sequence: u64,
}

/// Membership and at most one outstanding sync, owned together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GatewayRegistry {
    scope: GatewayScope,
    gateways: GatewayMembership,
    last_sequence: u64,
    pending: Option<GatewaySyncToken>,
}

impl GatewayRegistry {
    /// Start a transient registry from validated membership and trusted scope.
    ///
    /// This is not restoration: old in-flight tokens/history cannot be reset by
    /// calling this constructor again for an existing installation.
    #[must_use]
    pub const fn new(scope: GatewayScope, gateways: GatewayMembership) -> Self {
        Self {
            scope,
            gateways,
            last_sequence: 0,
            pending: None,
        }
    }

    /// Reserve one uniquely sequenced local synchronization attempt.
    /// # Errors
    /// Rejects a second outstanding attempt or exhausted sequence space; no wrap.
    pub fn begin_sync(&mut self) -> Result<GatewaySyncToken, GatewaySyncError> {
        if self.pending.is_some() {
            return Err(GatewaySyncError::SyncInProgress);
        }
        let sequence = self
            .last_sequence
            .checked_add(1)
            .ok_or(GatewaySyncError::SequenceExhausted)?;
        let token = GatewaySyncToken {
            scope: self.scope,
            sequence,
        };
        self.last_sequence = sequence;
        self.pending = Some(token);
        Ok(token)
    }

    /// Apply an exact current sync result after validating its complete list.
    ///
    /// `response_scope` must come from trusted transport/configuration context,
    /// not a provider-supplied assertion. A later explicitly authorized sync may
    /// re-add a removed member; only attempts begun before the edit are invalidated.
    /// # Errors
    /// Scope mismatch, stale/cancelled/replayed attempts and malformed lists leave
    /// membership and pending state unchanged. Cancel an unusable attempt explicitly.
    pub fn apply_sync(
        &mut self,
        token: GatewaySyncToken,
        response_scope: GatewayScope,
        principals: &[Principal],
    ) -> Result<(), GatewaySyncError> {
        self.check_sync(token, response_scope)?;
        self.gateways.replace_from_sync(principals)?;
        self.pending = None;
        Ok(())
    }

    /// Check reply correlation before spending resources on its payload.
    ///
    /// This neither authenticates transport nor grants a reusable permission;
    /// `apply_sync` checks again before changing membership.
    /// # Errors
    /// Rejects wrong scope or an attempt that is no longer pending.
    pub fn check_sync(
        &self,
        token: GatewaySyncToken,
        response_scope: GatewayScope,
    ) -> Result<(), GatewaySyncError> {
        if response_scope != self.scope {
            return Err(GatewaySyncError::WrongScope);
        }
        self.check_token(token)
    }

    /// Abandon only the exact pending read-only sync; it cannot later apply.
    /// # Errors
    /// Wrong-scope or stale tokens cannot cancel a newer attempt.
    pub fn cancel_sync(&mut self, token: GatewaySyncToken) -> Result<(), GatewaySyncError> {
        self.check_token(token)?;
        self.pending = None;
        Ok(())
    }

    /// Apply an authorized add and invalidate any earlier pending sync.
    ///
    /// Even an already-present add is a new operator decision. Failed additions
    /// leave both membership and the pending sync unchanged.
    /// # Errors
    /// Invalid principal or membership capacity errors are returned unchanged.
    pub fn add(&mut self, principal: Principal) -> Result<GatewayAddOutcome, GatewayListError> {
        let outcome = self.gateways.add(principal)?;
        self.pending = None;
        Ok(outcome)
    }

    /// Apply an authorized revocation, invalidating an older sync even if absent.
    ///
    /// Revocation remains possible at sequence exhaustion. This cannot revoke
    /// provider-side credentials or prevent a separately authorized future sync.
    pub fn remove(&mut self, principal: Principal) -> bool {
        let removed = self.gateways.remove(principal);
        self.pending = None;
        removed
    }

    /// Trusted provider scope of this registry.
    #[must_use]
    pub const fn scope(&self) -> GatewayScope {
        self.scope
    }

    /// Read current membership; no mutable reference bypasses sync invalidation.
    #[must_use]
    pub const fn gateways(&self) -> &GatewayMembership {
        &self.gateways
    }

    fn check_token(&self, token: GatewaySyncToken) -> Result<(), GatewaySyncError> {
        if token.scope != self.scope {
            return Err(GatewaySyncError::WrongScope);
        }
        if self.pending != Some(token) {
            return Err(GatewaySyncError::StaleSync);
        }
        Ok(())
    }
}

/// Local sync rejection; no rejected reply changes membership or pending state.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum GatewaySyncError {
    /// Token or response context belongs to another service/namespace/Cashier.
    #[error("wrong gateway sync scope")]
    WrongScope,
    /// Token was cancelled, invalidated, consumed or replaced by a later attempt.
    #[error("stale gateway sync")]
    StaleSync,
    /// Another attempt must complete or be explicitly cancelled first.
    #[error("gateway sync already in progress")]
    SyncInProgress,
    /// No more local attempt identities can be allocated safely.
    #[error("gateway sync sequence exhausted")]
    SequenceExhausted,
    /// Entire membership candidate failed validation.
    #[error(transparent)]
    InvalidList(#[from] GatewayListError),
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    use super::*;
    use crate::model::gateway::GatewayListLimits;

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
    fn cancelled_and_replayed_responses_cannot_replace_newer_membership() {
        let mut registry = registry();
        let old = registry.begin_sync().expect("old attempt");
        let before = registry.clone();
        assert_eq!(registry.begin_sync(), Err(GatewaySyncError::SyncInProgress));
        assert_eq!(registry, before);
        registry.cancel_sync(old).expect("cancel old read");
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
        assert!(registry.gateways().principals().is_empty());
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
}
