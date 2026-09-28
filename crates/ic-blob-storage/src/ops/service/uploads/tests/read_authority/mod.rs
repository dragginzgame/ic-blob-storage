use super::*;
use crate::model::service::read::ReadTarget;
use crate::{
    model::{
        gateway::registry::GatewayScope,
        lifecycle::requests::{ReferenceOperation, ReferenceRequest, ReferenceRequestId},
    },
    ops::{
        caffeine::gateway::GatewayReplyLimits,
        service::gateways::{GatewayStoreError, StableGatewayRegistry},
    },
    workflow::{
        gateways,
        reads::{ReadAuthorityError, capture, recheck},
    },
};
fn scope() -> GatewayScope {
    GatewayScope::new(p(1), NonZeroU128::MIN, p(3)).unwrap()
}
fn setup() -> (
    StableGatewayRegistry<VectorMemory>,
    StableUploads<VectorMemory>,
    UploadPermission,
) {
    let mut registry = StableGatewayRegistry::install(VectorMemory::default(), config()).unwrap();
    registry.add(context(2), scope(), p(6)).unwrap();
    let mut uploads = StableUploads::install(memory(), config()).unwrap();
    let input = lifecycle::exposed(&mut uploads);
    uploads.confirm_upload(input.request).unwrap();
    (registry, uploads, input)
}
fn target(input: UploadPermission) -> ReadTarget {
    ReadTarget {
        root: input.request.object.root,
        reference: input.request.object.first,
        gateway: p(6),
    }
}
#[test]
fn read_authority_rejects_remove_readd_and_successful_same_member_sync() {
    let (mut registry, uploads, input) = setup();
    let stamp = capture(&registry, &uploads, context(4), scope(), target(input)).unwrap();
    let token = registry.begin_sync(context(2), scope()).unwrap();
    registry.cancel_sync(context(2), scope(), token).unwrap();
    assert_eq!(
        registry.add(context(2), scope(), Principal::anonymous()),
        Err(GatewayStoreError::List(
            crate::model::gateway::GatewayListError::InvalidPrincipal {
                index: 0,
                principal: Principal::anonymous()
            }
        ))
    );
    assert_eq!(recheck(&registry, &uploads, context(4), &stamp), Ok(()));
    registry.remove(context(2), scope(), p(6)).unwrap();
    registry.add(context(2), scope(), p(6)).unwrap();
    assert_eq!(
        recheck(&registry, &uploads, context(4), &stamp),
        Err(ReadAuthorityError::Stale)
    );
    let stamp = capture(&registry, &uploads, context(4), scope(), target(input)).unwrap();
    let attempt = gateways::begin_sync(&mut registry, context(2), scope()).unwrap();
    let limits = GatewayReplyLimits {
        max_bytes: 2048.try_into().unwrap(),
        decoding_quota: 100_000.try_into().unwrap(),
        skipping_quota: 1000.try_into().unwrap(),
        max_type_entries: 32.try_into().unwrap(),
    };
    assert_eq!(
        gateways::complete_sync(&mut registry, context(2), &attempt, scope(), &[], limits),
        Err(
            crate::ops::service::gateways::reply::GatewaySyncReplyError::Reply(
                crate::ops::caffeine::query::reply::BoundGatewayReplyError::Reply(
                    crate::ops::caffeine::gateway::GatewayReplyError::InvalidReply
                )
            )
        )
    );
    assert_eq!(recheck(&registry, &uploads, context(4), &stamp), Ok(()));
    gateways::complete_sync(
        &mut registry,
        context(2),
        &attempt,
        scope(),
        &candid::encode_one(vec![p(6)]).unwrap(),
        limits,
    )
    .unwrap();
    assert_eq!(
        recheck(&registry, &uploads, context(4), &stamp),
        Err(ReadAuthorityError::Stale)
    );
}
#[test]
fn read_authority_binds_tenant_generation_caller_and_exact_reference() {
    let (registry, mut uploads, input) = setup();
    let stamp = capture(&registry, &uploads, context(4), scope(), target(input)).unwrap();
    assert_eq!(
        recheck(&registry, &uploads, context(2), &stamp),
        Err(ReadAuthorityError::Stale)
    );
    let active = uploads.tenant(context(4), p(4)).unwrap().unwrap();
    let suspended = uploads
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: Some(active),
                active: false,
            },
        )
        .unwrap();
    assert_eq!(
        recheck(&registry, &uploads, context(4), &stamp),
        Err(ReadAuthorityError::Uploads(UploadStoreError::Admission(
            UploadAdmissionError::Tenant(crate::model::service::tenant::TenantError::Suspended)
        )))
    );
    uploads
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: Some(suspended),
                active: true,
            },
        )
        .unwrap();
    assert_eq!(
        recheck(&registry, &uploads, context(4), &stamp),
        Err(ReadAuthorityError::Stale)
    );
    let stamp = capture(&registry, &uploads, context(4), scope(), target(input)).unwrap();
    let other = ReferenceKey::new(
        input.request.object.first.object(),
        ReferenceId::new(NonZeroU128::new(2).unwrap()),
    );
    for (id, operation) in [
        (1, ReferenceOperation::Retain(other)),
        (2, ReferenceOperation::Release(input.request.object.first)),
    ] {
        uploads
            .apply_reference(
                context(4),
                input.request,
                ReferenceRequest {
                    id: ReferenceRequestId::new(NonZeroU128::new(id).unwrap()),
                    operation,
                },
            )
            .unwrap();
    }
    assert_eq!(
        recheck(&registry, &uploads, context(4), &stamp),
        Err(ReadAuthorityError::Unavailable)
    );
    assert!(
        capture(
            &registry,
            &uploads,
            context(4),
            scope(),
            ReadTarget {
                reference: other,
                ..target(input)
            }
        )
        .is_ok()
    );
    assert!(matches!(
        capture(&registry, &uploads, context(5), scope(), target(input)),
        Err(ReadAuthorityError::Uploads(UploadStoreError::Admission(
            UploadAdmissionError::NotProject
        )))
    ));
}
#[test]
fn read_authority_refuses_either_restored_owner_and_wrong_scope() {
    let gm = VectorMemory::default();
    let um = memory();
    let mut registry = StableGatewayRegistry::install(gm.clone(), config()).unwrap();
    registry.add(context(2), scope(), p(6)).unwrap();
    let mut uploads = StableUploads::install(clone_memory(&um), config()).unwrap();
    let input = lifecycle::exposed(&mut uploads);
    uploads.confirm_upload(input.request).unwrap();
    let stamp = capture(&registry, &uploads, context(4), scope(), target(input)).unwrap();
    let reopened_registry = StableGatewayRegistry::open(gm, config()).unwrap();
    let reopened_uploads = StableUploads::open(clone_memory(&um), config()).unwrap();
    assert_eq!(
        recheck(&reopened_registry, &uploads, context(4), &stamp),
        Err(ReadAuthorityError::Registry(GatewayStoreError::Fenced))
    );
    assert_eq!(
        recheck(&registry, &reopened_uploads, context(4), &stamp),
        Err(ReadAuthorityError::Uploads(UploadStoreError::Fenced))
    );
    assert!(matches!(
        capture(
            &registry,
            &uploads,
            context(4),
            GatewayScope::new(p(1), NonZeroU128::new(2).unwrap(), p(3)).unwrap(),
            target(input)
        ),
        Err(ReadAuthorityError::Registry(GatewayStoreError::Binding))
    ));
}
