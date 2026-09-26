//! Immutable local ownership and object-incarnation bindings.
//!
//! Namespace IDs identify a future service-owned provider configuration; they
//! are not provider credentials or a deployed provider namespace contract.
//! Nonzero IDs do not establish allocation freshness or backup/reuse safety.

use std::num::NonZeroU128;

use candid::Principal;
use thiserror::Error;

use super::ReferenceId;

/// Independently allocated identities within a service and tenant.
///
/// A provider root or content digest must not be substituted for these IDs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObjectIdentity {
    /// Service-owned provider configuration/namespace identity.
    pub namespace: NonZeroU128,
    /// Logical object identity within the tenant and namespace.
    pub object: NonZeroU128,
    /// Particular lifetime of this object, distinct from previous lifetimes.
    pub incarnation: NonZeroU128,
}

/// Validated, immutable object ownership binding; not proof of caller authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObjectBinding {
    service: Principal,
    tenant: Principal,
    identity: ObjectIdentity,
}

impl ObjectBinding {
    /// Bind a local object identity to its service and tenant principals.
    ///
    /// Other principal classes are accepted without claiming they are deployed
    /// canisters. No provider resolution, identity allocation or tenant enrollment
    /// happens here.
    /// # Errors
    /// Rejects anonymous and management principals in either role.
    pub fn new(
        service: Principal,
        tenant: Principal,
        identity: ObjectIdentity,
    ) -> Result<Self, ObjectBindingError> {
        if !is_concrete(service) {
            return Err(ObjectBindingError::InvalidService);
        }
        if !is_concrete(tenant) {
            return Err(ObjectBindingError::InvalidTenant);
        }
        Ok(Self {
            service,
            tenant,
            identity,
        })
    }

    /// Service instance that owns the object.
    #[must_use]
    pub const fn service(self) -> Principal {
        self.service
    }

    /// Tenant that owns the object; not inferred from controllers or a digest.
    #[must_use]
    pub const fn tenant(self) -> Principal {
        self.tenant
    }

    /// Namespace, logical object and incarnation identities.
    #[must_use]
    pub const fn identity(self) -> ObjectIdentity {
        self.identity
    }

    /// Compare a supplied binding with this expected binding before mutation.
    ///
    /// # Errors
    /// Returns the first mismatch in service/tenant/namespace/object/incarnation
    /// order, without exposing the expected identifiers in the error.
    pub fn check(self, supplied: Self) -> Result<(), ObjectBindingMismatch> {
        if self.service != supplied.service {
            return Err(ObjectBindingMismatch::Service);
        }
        if self.tenant != supplied.tenant {
            return Err(ObjectBindingMismatch::Tenant);
        }
        if self.identity.namespace != supplied.identity.namespace {
            return Err(ObjectBindingMismatch::Namespace);
        }
        if self.identity.object != supplied.identity.object {
            return Err(ObjectBindingMismatch::Object);
        }
        if self.identity.incarnation != supplied.identity.incarnation {
            return Err(ObjectBindingMismatch::Incarnation);
        }
        Ok(())
    }
}

fn is_concrete(principal: Principal) -> bool {
    principal != Principal::anonymous() && principal != Principal::management_canister()
}

/// Invalid principal role in an object binding.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ObjectBindingError {
    /// The service principal is anonymous or management.
    #[error("invalid service principal")]
    InvalidService,
    /// The tenant principal is anonymous or management.
    #[error("invalid tenant principal")]
    InvalidTenant,
}

/// The supplied reference or observation belongs to another object scope.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ObjectBindingMismatch {
    /// Wrong service instance.
    #[error("service binding mismatch")]
    Service,
    /// Wrong tenant owner.
    #[error("tenant binding mismatch")]
    Tenant,
    /// Wrong provider configuration/namespace.
    #[error("namespace binding mismatch")]
    Namespace,
    /// Wrong logical object.
    #[error("object binding mismatch")]
    Object,
    /// Wrong object lifetime.
    #[error("object incarnation mismatch")]
    Incarnation,
}

/// A reference ID qualified by its complete object binding.
///
/// This key conveys identity only. A caller can construct a matching key without
/// being authorized; the workflow must independently authenticate the actor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReferenceKey {
    object: ObjectBinding,
    reference: ReferenceId,
}

impl ReferenceKey {
    /// Qualify one reference ID without granting access to its object.
    #[must_use]
    pub const fn new(object: ObjectBinding, reference: ReferenceId) -> Self {
        Self { object, reference }
    }

    /// The bound object incarnation.
    #[must_use]
    pub const fn object(self) -> ObjectBinding {
        self.object
    }

    /// Reference ID local to that object incarnation.
    #[must_use]
    pub const fn reference(self) -> ReferenceId {
        self.reference
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn special_principals_are_rejected_in_each_role() {
        let valid = Principal::from_slice(&[1, 1]);
        let one = NonZeroU128::new(1).expect("positive ID");
        let identity = ObjectIdentity {
            namespace: one,
            object: one,
            incarnation: one,
        };
        for special in [Principal::anonymous(), Principal::management_canister()] {
            assert_eq!(
                ObjectBinding::new(special, valid, identity),
                Err(ObjectBindingError::InvalidService)
            );
            assert_eq!(
                ObjectBinding::new(valid, special, identity),
                Err(ObjectBindingError::InvalidTenant)
            );
        }
        assert!(ObjectBinding::new(valid, valid, identity).is_ok());
    }
}
