//! Independent durable-owner fences and binding checks without platform substitutes.
use super::*;
use crate::{
    model::{
        catalog::admission::read::UploadRootState,
        gateway::registry::GatewayScope,
        identity::batch::{ProviderRootBatch, RootBatchLimits},
    },
    ops::service::gateways::{GatewayStoreError, StableGatewayRegistry},
    policy::{
        catalog::upload::UploadRootStatus,
        gateway::{GatewayAccessError, GatewayCallbackContext},
    },
    workflow::gateways::callbacks::{GatewayCallbackError, observe_roots},
};
use std::num::NonZeroUsize;

fn scope() -> GatewayScope {
    GatewayScope::new(p(1), NonZeroU128::MIN, p(3)).unwrap()
}
fn caller(actor: Principal) -> GatewayCallbackContext {
    GatewayCallbackContext {
        service: p(1),
        actor,
    }
}
fn batch(roots: &[Vec<u8>]) -> ProviderRootBatch {
    ProviderRootBatch::from_bytes(
        roots,
        RootBatchLimits {
            max_entries: NonZeroUsize::new(8).unwrap(),
            max_bytes: NonZeroUsize::new(256).unwrap(),
        },
    )
    .unwrap()
}
fn registry(memory: VectorMemory) -> StableGatewayRegistry<VectorMemory> {
    let mut registry = StableGatewayRegistry::install(memory, config()).unwrap();
    registry.add(context(2), scope(), p(6)).unwrap();
    registry
}
#[test]
fn gateway_callbacks_reject_each_restored_owner_independently() {
    let gm = VectorMemory::default();
    let um = memory();
    let live_registry = registry(gm.clone());
    let live_uploads = StableUploads::install(clone_memory(&um), config()).unwrap();
    let empty = batch(&[]);
    assert_eq!(
        observe_roots(&live_registry, &live_uploads, caller(p(6)), scope(), &empty),
        Ok(vec![])
    );
    let restored_registry = StableGatewayRegistry::open(gm, config()).unwrap();
    let restored_uploads = StableUploads::open(clone_memory(&um), config()).unwrap();
    assert_eq!(
        observe_roots(
            &restored_registry,
            &live_uploads,
            caller(p(6)),
            scope(),
            &empty
        ),
        Err(GatewayCallbackError::Registry(GatewayStoreError::Fenced))
    );
    assert_eq!(
        observe_roots(
            &live_registry,
            &restored_uploads,
            caller(p(6)),
            scope(),
            &empty
        ),
        Err(GatewayCallbackError::Uploads(UploadStoreError::Fenced))
    );
}
#[test]
fn gateway_callbacks_bind_actual_service_and_complete_owner_configuration() {
    let registry = registry(VectorMemory::default());
    let uploads = StableUploads::install(memory(), config()).unwrap();
    let empty = batch(&[]);
    assert_eq!(
        observe_roots(
            &registry,
            &uploads,
            GatewayCallbackContext {
                service: p(9),
                actor: p(6)
            },
            scope(),
            &empty
        ),
        Err(GatewayCallbackError::Access(
            GatewayAccessError::WrongService
        ))
    );
    for actor in [p(2), p(3), p(4), p(5), Principal::anonymous()] {
        assert_eq!(
            observe_roots(&registry, &uploads, caller(actor), scope(), &empty),
            Err(GatewayCallbackError::Access(GatewayAccessError::NotGateway))
        );
    }
    let original = config();
    let mut changed_operator = original.bindings();
    changed_operator.operator = p(9);
    let mut changed_payer = original.bindings();
    changed_payer.payment_account = p(9);
    let mut changed_limits = original.limits();
    changed_limits.max_object_bytes = std::num::NonZeroU64::new(9).unwrap();
    for (bindings, limits) in [
        (changed_operator, original.limits()),
        (changed_payer, original.limits()),
        (original.bindings(), changed_limits),
    ] {
        let changed = ServiceConfiguration::new(bindings, limits, original.billing()).unwrap();
        let uploads = StableUploads::install(memory(), changed).unwrap();
        assert_eq!(
            observe_roots(&registry, &uploads, caller(p(6)), scope(), &empty),
            Err(GatewayCallbackError::Registry(GatewayStoreError::Binding))
        );
    }
}
#[test]
fn gateway_callbacks_fail_closed_on_missing_root_request_index() {
    let registry = registry(VectorMemory::default());
    let mut uploads = StableUploads::install(memory(), config()).unwrap();
    enroll(&mut uploads);
    let input = permission(1);
    uploads.admit(context(4), input, 1).unwrap();
    let roots = batch(&[vec![9; 32], vec![1; 32]]);
    assert_eq!(
        observe_roots(&registry, &uploads, caller(p(6)), scope(), &roots),
        Ok(vec![
            UploadRootStatus::Unknown,
            UploadRootStatus::Known(UploadRootState::Reserved)
        ])
    );
    uploads.root_requests.remove(&[1; 32]);
    assert_eq!(
        observe_roots(&registry, &uploads, caller(p(6)), scope(), &roots),
        Err(GatewayCallbackError::Uploads(
            UploadStoreError::InvalidRecord
        ))
    );
}
