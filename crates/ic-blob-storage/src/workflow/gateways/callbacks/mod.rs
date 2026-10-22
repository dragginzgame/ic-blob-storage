//! Scoped live gateway observations, not provider effect or deletion authority.
use crate::model::gateway::registry::GatewayScope;
use crate::ops::service::gateways::GatewayStoreError;
use crate::ops::service::gateways::StableGatewayRegistry;
use crate::ops::service::uploads::StableUploads;
use crate::ops::service::uploads::UploadStoreError;
use crate::ops::service::uploads::read::UploadRootObservation;
use crate::policy::catalog::upload::UploadRootStatus;
use crate::policy::gateway::GatewayAccessError;
use crate::policy::gateway::GatewayCallbackContext;
use crate::policy::gateway::assess_gateway_callback;
use crate::policy::gateway::assess_gateway_membership;
use ic_blob_storage_contracts::identity::batch::ProviderRootBatch;
use ic_memory::Memory;
use thiserror::Error;

/// A gateway observation was refused before returning any batch data.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum GatewayCallbackError {
    /// An active read owner is mismatched, fenced or inconsistent.
    #[error(transparent)]
    Reads(#[from] crate::model::service::read::session::ReadSessionError),
    /// Raw processing bounds exceeded before lookup.
    #[error(transparent)]
    Batch(#[from] ic_blob_storage_contracts::identity::batch::RootBatchError),
    /// Malformed keys cannot become deletion permission.
    #[error("malformed provider root")]
    MalformedRoot,

    /// Registry configuration, retained state or restore fence rejection.
    #[error(transparent)]
    Registry(#[from] GatewayStoreError),
    /// Object owner configuration, state or restore fence rejection.
    #[error(transparent)]
    Uploads(#[from] UploadStoreError),
    /// Actual caller/service or trusted object binding failed pure policy.
    #[error(transparent)]
    Access(#[from] GatewayAccessError),
}

/// Inspect local root phases under current scoped gateway authority.
/// The endpoint supplies actual caller/service and an explicit installed scope;
/// it bounds decoding and constructs `batch` using trusted host limits. The two
/// owners must have the same complete configuration. No operator impersonation,
/// cached authorization, await or reusable permit occurs here.
///
/// Membership is checked even for empty/malformed/unknown batches. Both restore
/// fences block these operational observations. Each known root's binding comes
/// from authoritative indexes and is checked before returning only its phase;
/// tenant identities, request IDs, manifests and reference details stay private.
/// Order, duplicates and malformed positions are preserved. Unknown or cancelled
/// roots are not permission to delete; pending exposure remains explicit. Hosts
/// must separately qualify provider liveness/deletion mapping, evidence, callback
/// correlation and read-session generations before exposing a provider protocol.
/// # Errors
/// Rejects configuration/scope, caller, either fence or invalid stored bindings.
pub fn observe_roots<G: Memory, U: Memory>(
    registry: &StableGatewayRegistry<G>,
    uploads: &StableUploads<U>,
    context: GatewayCallbackContext,
    scope: GatewayScope,
    batch: &ProviderRootBatch,
) -> Result<Vec<UploadRootStatus>, GatewayCallbackError> {
    let current = registry.callback_view(uploads.callback_configuration(), scope)?;
    assess_gateway_membership(&current.registry, context)?;
    if current.fenced {
        return Err(GatewayStoreError::Fenced.into());
    }
    uploads.check_callback_active()?;
    uploads
        .observe_roots_for_owner(batch)?
        .into_iter()
        .map(|entry| {
            Ok(match entry {
                UploadRootObservation::Unknown => UploadRootStatus::Unknown,
                UploadRootObservation::Malformed(error) => UploadRootStatus::Malformed(error),
                UploadRootObservation::Known(view) => {
                    assess_gateway_callback(
                        view.request.object.first.object(),
                        &current.registry,
                        context,
                    )?;
                    UploadRootStatus::Known(view.state)
                }
            })
        })
        .collect()
}

/// Public provider liveness projection. Unknown roots are conservatively protected;
/// only a known released/cancelled lifetime can return false. No tenant metadata
/// is disclosed. Active reservations and read sessions remain protected.
/// # Errors
/// Rejects mismatched service/owners, restore fences, malformed keys and limits.
pub fn provider_liveness<M: Memory>(
    stores: &crate::ops::service::stores::ServiceStores<M>,
    context: GatewayCallbackContext,
    scope: GatewayScope,
    roots: &[Vec<u8>],
) -> Result<Vec<bool>, GatewayCallbackError> {
    let protected = provider_authority(stores, context, scope, false)?;
    let batch = provider_batch(roots)?;
    stores
        .uploads
        .observe_roots_for_owner(&batch)?
        .into_iter()
        .map(|entry| {
            Ok(match entry {
                UploadRootObservation::Malformed(_) => {
                    return Err(GatewayCallbackError::MalformedRoot);
                }
                UploadRootObservation::Unknown => true,
                UploadRootObservation::Known(view) => {
                    use ic_blob_storage_contracts::upload::history::{
                        LifecyclePhase, UploadRootState,
                    };
                    protected.contains(&view.request.object.root)
                        || matches!(
                            view.state,
                            UploadRootState::Reserved
                                | UploadRootState::Confirmed(LifecyclePhase::Live)
                        )
                }
            })
        })
        .collect()
}
/// Enumerate at most 10,000 eligible original tree-root bytes. Acknowledged roots
/// leave this projection while all original records and liabilities remain.
/// # Errors
/// Rejects non-gateways, mismatched scope, fences or inconsistent retained state.
pub fn provider_deletions<M: Memory>(
    stores: &crate::ops::service::stores::ServiceStores<M>,
    context: GatewayCallbackContext,
    scope: GatewayScope,
) -> Result<Vec<Vec<u8>>, GatewayCallbackError> {
    let protected = provider_authority(stores, context, scope, true)?;
    Ok(stores.uploads.gateway_deletions(&protected)?)
}
/// Record gateway acknowledgment without claiming physical deletion or billing
/// settlement. Exact duplicate/late acknowledgments retain their original lifetime.
/// Validate the full bounded batch before any write; no paid effect or await.
/// # Errors
/// Rejects non-gateways, fences, malformed/unknown roots and live/read-held objects.
/// # Panics
/// Stable write traps must propagate for IC transaction rollback.
pub fn acknowledge_provider_deletions<M: Memory>(
    stores: &mut crate::ops::service::stores::ServiceStores<M>,
    context: GatewayCallbackContext,
    scope: GatewayScope,
    roots: &[Vec<u8>],
) -> Result<(), GatewayCallbackError> {
    let protected = provider_authority(stores, context, scope, true)?;
    let batch = provider_batch(roots)?;
    let roots = batch
        .entries()
        .iter()
        .map(|root| {
            root.as_ref()
                .copied()
                .map_err(|_| GatewayCallbackError::MalformedRoot)
        })
        .collect::<Result<std::collections::BTreeSet<_>, _>>()?;
    Ok(stores
        .uploads
        .acknowledge_gateway_roots(&roots, &protected)?)
}
fn provider_batch(roots: &[Vec<u8>]) -> Result<ProviderRootBatch, GatewayCallbackError> {
    use ic_blob_storage_contracts::provider::gc;
    Ok(ProviderRootBatch::from_bytes(roots, gc::ROOT_LIMITS)?)
}
fn provider_authority<M: Memory>(
    stores: &crate::ops::service::stores::ServiceStores<M>,
    context: GatewayCallbackContext,
    scope: GatewayScope,
    membership: bool,
) -> Result<
    std::collections::BTreeSet<ic_blob_storage_contracts::identity::ProviderRootHash>,
    GatewayCallbackError,
> {
    let config = stores.uploads.callback_configuration();
    let current = stores.gateways.callback_view(config, scope)?;
    if context.service != scope.service() {
        return Err(GatewayAccessError::WrongService.into());
    }
    if membership {
        assess_gateway_membership(&current.registry, context)?;
    }
    if current.fenced {
        return Err(GatewayStoreError::Fenced.into());
    }
    stores.uploads.check_callback_active()?;
    Ok(stores.reads.protected_roots(config)?)
}
