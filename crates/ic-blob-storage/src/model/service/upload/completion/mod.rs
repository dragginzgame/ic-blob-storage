//! Explicit host-installed trust in an external content verifier.
use candid::Principal;
use std::num::NonZeroU128;

/// Immutable authority for one service and provider namespace. Never supplied by ingress.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompletionAuthority {
    service: Principal,
    namespace: NonZeroU128,
    verifier: Principal,
}
impl CompletionAuthority {
    /// Validate explicit principals; configuring this role grants trust, not provider proof.
    /// # Errors
    /// Rejects anonymous or management service/verifier identities.
    pub fn new(
        service: Principal,
        namespace: NonZeroU128,
        verifier: Principal,
    ) -> Result<Self, InvalidCompletionAuthority> {
        if [service, verifier]
            .iter()
            .any(|p| *p == Principal::anonymous() || *p == Principal::management_canister())
        {
            return Err(InvalidCompletionAuthority);
        }
        Ok(Self {
            service,
            namespace,
            verifier,
        })
    }
    /// Bound service instance.
    #[must_use]
    pub const fn service(self) -> Principal {
        self.service
    }
    /// Bound local provider namespace.
    #[must_use]
    pub const fn namespace(self) -> NonZeroU128 {
        self.namespace
    }
    /// Only this authenticated caller may attest content availability.
    #[must_use]
    pub const fn verifier(self) -> Principal {
        self.verifier
    }
}
/// Invalid explicit authority identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("invalid completion authority")]
pub struct InvalidCompletionAuthority;
