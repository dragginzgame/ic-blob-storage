//! Actual durable owner projection; provider effects remain separate.
use super::*;
use crate::model::gateway::registry::GatewayScope;
use crate::model::service::read::{
    ReadTarget,
    session::{ReadChunkTarget, ReadSessionIntent},
};
use crate::model::service::upload::manifest::UploadManifest;
use crate::policy::gateway::GatewayCallbackContext;
use crate::workflow::gateways::callbacks::{
    acknowledge_provider_deletions, provider_deletions, provider_liveness,
};
use ic_blob_storage_contracts::{
    binding::{ObjectBinding, ObjectIdentity, ReferenceId, ReferenceKey},
    identity::caffeine::{
        CaffeineHashLimits, CaffeineHeader, manifest::builder::CaffeineManifestBuilder,
    },
    reference::binding::{ReferenceOperation, ReferenceRequest, ReferenceRequestId},
    tenant::TenantUpdate,
    upload::binding::{
        UploadContext, UploadObject, UploadPermission, UploadRequest, UploadRequestId,
    },
};
use ic_platform::Principal;
use std::num::{NonZeroU64, NonZeroU128, NonZeroUsize};
fn p(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}
fn n(n: u128) -> NonZeroU128 {
    NonZeroU128::new(n).unwrap()
}
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One original root/read/reference/acknowledgment lifetime is carried through durable restoration"
)]
fn gateway_callbacks_preserve_reads_exact_replays_obligations_and_restored_fences() {
    let input = candidate();
    let config = validate_candidate(input.service, input).unwrap();
    let memory = std::array::from_fn(|_| FixtureMemory::default());
    let mut stores = ServiceStores::install(memories(&memory), config).unwrap();
    let scope =
        GatewayScope::new(input.service, n(input.namespace), input.billing.cashier).unwrap();
    let operator = UploadContext {
        service: input.service,
        actor: input.operator,
    };
    let tenant = UploadContext {
        service: input.service,
        actor: p(7),
    };
    let uploader = UploadContext {
        service: input.service,
        actor: p(8),
    };
    let gateway = GatewayCallbackContext {
        service: input.service,
        actor: p(9),
    };
    let public = GatewayCallbackContext {
        actor: Principal::anonymous(),
        ..gateway
    };
    stores.gateways.add(operator, scope, gateway.actor).unwrap();
    stores
        .uploads
        .update_tenant(
            operator,
            TenantUpdate {
                tenant: tenant.actor,
                expected: None,
                active: true,
            },
        )
        .unwrap();
    let headers = [
        CaffeineHeader {
            name: "Content-Length",
            value: "10",
        },
        CaffeineHeader {
            name: "Content-Type",
            value: "image/png",
        },
    ];
    let mut builder = CaffeineManifestBuilder::new(
        10,
        &headers,
        CaffeineHashLimits {
            max_content_bytes: NonZeroU64::new(10).unwrap(),
            max_append_bytes: NonZeroUsize::new(10).unwrap(),
            max_headers: NonZeroUsize::new(8).unwrap(),
            max_header_bytes: NonZeroUsize::new(1024).unwrap(),
        },
        NonZeroUsize::MIN,
    )
    .unwrap();
    builder.append(0, &[42; 10]).unwrap();
    let built = builder.finish().unwrap();
    let first = ReferenceKey::new(
        ObjectBinding::new(
            input.service,
            tenant.actor,
            ObjectIdentity {
                namespace: n(input.namespace),
                object: n(1),
                incarnation: n(1),
            },
        )
        .unwrap(),
        ReferenceId::new(n(1)),
    );
    let upload = UploadRequest {
        id: UploadRequestId::new(n(1)),
        object: UploadObject {
            root: built.hashes().provider_root,
            bytes: 10,
            first,
        },
    };
    let roots = vec![upload.object.root.as_bytes().to_vec()];
    stores
        .uploads
        .admit(
            tenant,
            UploadPermission {
                request: upload,
                uploader: uploader.actor,
                expires_at_ns: 100,
            },
            1,
        )
        .unwrap();
    assert_eq!(
        provider_liveness(&stores, public, scope, &roots).unwrap(),
        vec![true]
    );
    stores
        .uploads
        .prepare_manifest(
            uploader,
            upload,
            UploadManifest {
                chunks: built.manifest().chunks(),
                headers: &headers,
            },
            2,
        )
        .unwrap();
    stores.uploads.expose(uploader, upload, 3).unwrap();
    assert!(acknowledge_provider_deletions(&mut stores, gateway, scope, &roots).is_err());
    let ticket = stores
        .reads
        .reserve(&ReadSessionIntent {
            context: tenant,
            scope,
            chunk: ReadChunkTarget {
                target: ReadTarget {
                    root: upload.object.root,
                    reference: first,
                    gateway: gateway.actor,
                },
                index: 0,
            },
            gateway_generation: 1,
            tenant_generation: NonZeroU64::MIN,
        })
        .unwrap();
    stores
        .uploads
        .apply_reference(
            tenant,
            upload,
            ReferenceRequest {
                id: ReferenceRequestId::new(n(1)),
                operation: ReferenceOperation::Release(first),
            },
        )
        .unwrap();
    let obligations = stores.uploads.usage().unwrap();
    assert_eq!(
        provider_liveness(&stores, public, scope, &roots).unwrap(),
        vec![true]
    );
    assert_eq!(
        provider_deletions(&stores, gateway, scope).unwrap(),
        Vec::<Vec<u8>>::new()
    );
    assert!(acknowledge_provider_deletions(&mut stores, gateway, scope, &roots).is_err());
    stores.reads.complete(tenant, &ticket).unwrap();
    assert_eq!(
        provider_liveness(&stores, public, scope, &roots).unwrap(),
        vec![false]
    );
    assert_eq!(provider_deletions(&stores, gateway, scope).unwrap(), roots);
    let unknown = vec![88; 32];
    assert_eq!(
        provider_liveness(&stores, public, scope, std::slice::from_ref(&unknown)).unwrap(),
        vec![true]
    );
    // Entire batch validates before writes; one unknown must not acknowledge another root.
    assert!(
        acknowledge_provider_deletions(&mut stores, gateway, scope, &[roots[0].clone(), unknown])
            .is_err()
    );
    assert_eq!(provider_deletions(&stores, gateway, scope).unwrap(), roots);
    for bad in [
        public,
        GatewayCallbackContext {
            service: p(10),
            ..gateway
        },
    ] {
        assert!(provider_deletions(&stores, bad, scope).is_err());
        assert!(acknowledge_provider_deletions(&mut stores, bad, scope, &roots).is_err());
    }
    acknowledge_provider_deletions(
        &mut stores,
        gateway,
        scope,
        &[roots[0].clone(), roots[0].clone()],
    )
    .unwrap();
    acknowledge_provider_deletions(&mut stores, gateway, scope, &roots).unwrap();
    assert!(
        stores
            .uploads
            .confirmed(tenant, upload)
            .unwrap()
            .gateway_acknowledged
    );
    assert_eq!(stores.uploads.usage().unwrap(), obligations);
    assert_eq!(
        provider_deletions(&stores, gateway, scope).unwrap(),
        Vec::<Vec<u8>>::new()
    );
    stores
        .gateways
        .remove(operator, scope, gateway.actor)
        .unwrap();
    assert!(acknowledge_provider_deletions(&mut stores, gateway, scope, &roots).is_err());
    stores.gateways.add(operator, scope, gateway.actor).unwrap();
    drop(stores);
    let restored = ServiceStores::open(memories(&memory), config).unwrap();
    assert!(
        restored
            .uploads
            .confirmed(tenant, upload)
            .unwrap()
            .gateway_acknowledged
    );
    assert_eq!(restored.uploads.usage().unwrap(), obligations);
    assert!(provider_liveness(&restored, public, scope, &roots).is_err());
    assert!(provider_deletions(&restored, gateway, scope).is_err());
}
