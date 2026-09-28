//! Scoped live gateway observations, not provider effect or deletion authority.
use crate::{
    model::{gateway::registry::GatewayScope, identity::batch::ProviderRootBatch},
    ops::service::{
        gateways::{GatewayStoreError, StableGatewayRegistry},
        uploads::{StableUploads, UploadStoreError, read::UploadRootObservation},
    },
    policy::{
        catalog::upload::UploadRootStatus,
        gateway::{
            GatewayAccessError, GatewayCallbackContext, assess_gateway_callback,
            assess_gateway_membership,
        },
    },
};
use ic_memory::ic_stable_structures::Memory;
use thiserror::Error;

/// A gateway observation was refused before returning any batch data.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum GatewayCallbackError {
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
