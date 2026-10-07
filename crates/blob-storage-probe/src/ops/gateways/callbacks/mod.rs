//! Bounded fixture conversions and synchronous access, not callback orchestration.
use super::{failure, scope};
use crate::ops::{ProbeMemory, STATE, conversion, read};
use blob_test_protocol::storage::{
    Failure,
    gateways::{RootView, RootsInput},
};
use ic_blob_storage::{
    model::{
        gateway::registry::GatewayScope,
        identity::{
            HashParseError,
            batch::{ProviderRootBatch, RootBatchLimits},
        },
    },
    ops::service::{gateways::StableGatewayRegistry, uploads::StableUploads},
    policy::{catalog::upload::UploadRootStatus, gateway::GatewayAccessError},
};
use std::num::NonZeroUsize;
pub(crate) fn parse(input: &RootsInput) -> Result<(GatewayScope, ProviderRootBatch), Failure> {
    let installed = scope(input.scope)?;
    let batch = ProviderRootBatch::from_bytes(
        &input.roots,
        RootBatchLimits {
            max_entries: NonZeroUsize::new(8).unwrap(),
            max_bytes: NonZeroUsize::new(256).unwrap(),
        },
    )
    .map_err(|_| Failure::Capacity)?;
    Ok((installed, batch))
}
pub(crate) fn with_owners<R>(
    operation: impl FnOnce(&StableGatewayRegistry<ProbeMemory>, &StableUploads<ProbeMemory>) -> R,
) -> R {
    STATE.with_borrow(|state| {
        let state = state.as_ref().unwrap();
        operation(&state.gateways, &state.uploads)
    })
}
pub(crate) fn access_failure(error: GatewayAccessError) -> Failure {
    match error {
        GatewayAccessError::NotGateway => Failure::Denied,
        GatewayAccessError::WrongService | GatewayAccessError::WrongNamespace => Failure::Binding,
    }
}
pub(crate) fn present(entries: Vec<UploadRootStatus>) -> Result<Vec<RootView>, Failure> {
    entries
        .into_iter()
        .map(|entry| {
            Ok(match entry {
                UploadRootStatus::Known(state) => RootView::Known(read::content_state(state)),
                UploadRootStatus::Unknown => RootView::Unknown,
                UploadRootStatus::Malformed(HashParseError::InvalidByteLength { actual }) => {
                    RootView::Malformed {
                        bytes: actual as u64,
                    }
                }
                UploadRootStatus::Malformed(_) => return Err(Failure::Invalid),
                UploadRootStatus::WrongNamespace => return Err(Failure::Binding),
            })
        })
        .collect()
}
pub(crate) fn upload_failure(
    error: ic_blob_storage::ops::service::uploads::UploadStoreError,
) -> Failure {
    conversion::failure(error)
}
pub(crate) fn registry_failure(
    error: ic_blob_storage::ops::service::gateways::GatewayStoreError,
) -> Failure {
    failure(error)
}
