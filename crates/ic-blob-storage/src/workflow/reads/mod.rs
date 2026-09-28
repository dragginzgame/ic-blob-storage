//! Current durable read authority and invalidation across a host-managed await.
//!
//! Capture/recheck alone reserve no slot and verify no bytes. The `sessions`
//! module composes bounded durable admission and exact one-shot completion.
//! The `chunk` module composes that journal with one host call and leaf verification.
//! Qualified provider transport and source authentication remain host duties.
pub mod chunk;
pub mod download;
pub mod sessions;
use crate::{
    model::{
        gateway::registry::GatewayScope,
        service::{read::ReadTarget, upload::UploadContext},
    },
    ops::service::{
        gateways::{GatewayStoreError, StableGatewayRegistry},
        uploads::{StableUploads, UploadStoreError},
    },
    policy::gateway::{GatewayAccessError, GatewayCallbackContext, assess_gateway_callback},
};
use ic_memory::ic_stable_structures::Memory;
use std::num::NonZeroU64;
use thiserror::Error;

/// Opaque host-retained observation, never an ingress DTO or reusable permit.
/// Capture and recheck with the original authenticated context. This value neither
/// consumes a session slot nor grants freshness after restoration. No serialization.
/// Use only with the original exclusive owners. Host installation identity and
/// stale-instance exclusion remain required; fresh constructors are not recovery.
#[derive(Debug)]
pub struct ReadAuthority {
    context: UploadContext,
    scope: GatewayScope,
    target: ReadTarget,
    gateway_generation: u64,
    tenant_generation: NonZeroU64,
}
impl ReadAuthority {
    /// Original target for later exact session/transport binding; not dispatch authority.
    #[must_use]
    pub const fn target(&self) -> ReadTarget {
        self.target
    }
}
/// A current authority observation failed. No failure changes storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum ReadAuthorityError {
    /// Registry configuration, stored state or restoration is invalid.
    #[error(transparent)]
    Registry(#[from] GatewayStoreError),
    /// Tenant/object authority, stored state or upload restoration is invalid.
    #[error(transparent)]
    Uploads(#[from] UploadStoreError),
    /// Selected gateway does not own the scoped object or is no longer a member.
    #[error(transparent)]
    Gateway(#[from] GatewayAccessError),
    /// Root or exact reference is absent, unconfirmed or no longer live.
    #[error("read target unavailable")]
    Unavailable,
    /// Original caller/context or either authority generation changed.
    #[error("stale read authority")]
    Stale,
    /// No further safe gateway read generations; administrative revocation still works.
    #[error("gateway read authority exhausted")]
    Exhausted,
}
/// Observe current tenant/reference/gateway authority synchronously from both owners.
/// Checks full owner configuration, explicit scope, both restore fences and current
/// enrollment. Reads indexes, not manifests or reference/receipt histories.
/// # Errors
/// Rejects mismatched authority, unavailable content, restored owners or exhaustion.
pub fn capture<G: Memory, U: Memory>(
    registry: &StableGatewayRegistry<G>,
    uploads: &StableUploads<U>,
    context: UploadContext,
    scope: GatewayScope,
    target: ReadTarget,
) -> Result<ReadAuthority, ReadAuthorityError> {
    let current = registry.callback_view(uploads.callback_configuration(), scope)?;
    if current.fenced {
        return Err(GatewayStoreError::Fenced.into());
    }
    let tenant_generation = uploads
        .read_authority_generation(context, target.root, target.reference)?
        .ok_or(ReadAuthorityError::Unavailable)?;
    assess_gateway_callback(
        target.reference.object(),
        &current.registry,
        GatewayCallbackContext {
            service: context.service,
            actor: target.gateway,
        },
    )?;
    let gateway_generation = current
        .registry
        .read_generation
        .current()
        .ok_or(ReadAuthorityError::Exhausted)?;
    Ok(ReadAuthority {
        context,
        scope,
        target,
        gateway_generation,
        tenant_generation,
    })
}
/// Recheck the original exact target after an await and before any byte disclosure.
/// Current membership alone is insufficient: remove/re-add, successful sync (even
/// unchanged membership), and tenant suspension/reactivation invalidate old reads.
/// Failed edits and sync begin/cancel do not change authority generations.
/// This check is read-only and repeatable; it is not one-shot callback settlement.
/// # Errors
/// Rejects changed context, either fence, lost authority/reference or stale generations.
pub fn recheck<G: Memory, U: Memory>(
    registry: &StableGatewayRegistry<G>,
    uploads: &StableUploads<U>,
    context: UploadContext,
    original: &ReadAuthority,
) -> Result<(), ReadAuthorityError> {
    if context != original.context {
        return Err(ReadAuthorityError::Stale);
    }
    let current = capture(registry, uploads, context, original.scope, original.target)?;
    if current.gateway_generation != original.gateway_generation
        || current.tenant_generation != original.tenant_generation
    {
        return Err(ReadAuthorityError::Stale);
    }
    Ok(())
}
