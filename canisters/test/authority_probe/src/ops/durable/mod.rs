//! Disposable governed backing for caller-policy experiments using the production owner.
//! The separate archive retains inspection evidence; no disposable owner restores authority.
use super::{bound, number};
use ic_blob_storage::{
    ic_storage::{
        AllocationGrants, GenericAllocationPolicy, MemoryAuthority, MemoryRuntime,
        SealedDeclarationSnapshot, StorageConfig, VectorMemory,
    },
    model::billing::allocation::FundingAllocation,
    ops::service::{
        stores::{ServiceStoreConfiguration, grants},
        uploads::StableUploads,
    },
};
use ic_blob_storage_contracts::{
    configuration::{
        billing::BillingConfiguration,
        funding::FundingLimits,
        limits::{CatalogLimits, UploadLimits},
        service::{ServiceBindings, ServiceConfiguration, ServiceLimits, ServiceManifestLimits},
    },
    identity::caffeine::{
        CaffeineHashLimits, CaffeineHeader,
        manifest::builder::{CaffeineBuiltManifest, CaffeineManifestBuilder},
    },
    tenant::TenantUpdate,
    upload::binding::{UploadContext, UploadPermission, UploadRequest},
};
use ic_platform::Principal;
use std::num::NonZeroU64;

pub(super) type Owner = StableUploads<VectorMemory>;

pub(crate) fn context(service: Principal, actor: Principal) -> UploadContext {
    UploadContext { service, actor }
}

pub(super) fn install(
    service: Principal,
    operator: Principal,
    tenants: [Principal; 2],
    catalog: CatalogLimits,
    uploads: UploadLimits,
) -> (Owner, MemoryRuntime<VectorMemory>) {
    let config = ServiceConfiguration::new(
        ServiceBindings {
            service,
            operator,
            payment_account: service,
            namespace: number(1),
        },
        ServiceLimits {
            max_tenants: bound(2),
            max_object_bytes: NonZeroU64::new(
                u64::try_from(catalog.max_tenant_logical_bytes.get().min(10 * 1024 * 1024))
                    .unwrap(),
            )
            .unwrap(),
            max_headers: bound(8),
            max_header_bytes: bound(4096),
            manifests: ServiceManifestLimits {
                max_chunks: bound(1024),
                max_tenant_chunks: bound(1024),
            },
            catalog,
            uploads,
        },
        BillingConfiguration::new(operator, FundingLimits::new(1, 10, 100).unwrap(), 1, 1).unwrap(),
    )
    .unwrap();
    let source = SealedDeclarationSnapshot::new(grants::requests("fixture").unwrap()).unwrap();
    let grants =
        AllocationGrants::new(vec![MemoryAuthority::new("fixture", "blob.").unwrap()]).unwrap();
    let mut runtime =
        MemoryRuntime::new_with_config(VectorMemory::default(), StorageConfig::default()).unwrap();
    runtime
        .bootstrap(&source, &grants, &GenericAllocationPolicy)
        .unwrap();
    let stores = ic_blob_storage::test_support::stores::install(
        ic_blob_storage::ops::service::stores::grants::open(|key| runtime.open_memory(key))
            .unwrap(),
        ServiceStoreConfiguration::new(
            config,
            FundingAllocation::new(100, number(1), bound(4)).unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    let mut owner = stores.uploads;
    for tenant in tenants {
        owner
            .update_tenant(
                context(service, operator),
                TenantUpdate {
                    tenant,
                    expected: None,
                    active: true,
                },
            )
            .unwrap();
    }
    (owner, runtime)
}

pub(super) fn permission(request: UploadRequest) -> UploadPermission {
    UploadPermission {
        request,
        uploader: request.object.first.object().tenant(),
        expires_at_ns: u64::MAX,
    }
}

pub(super) fn manifest(id: u8) -> CaffeineBuiltManifest {
    let bytes = if id == 2 { 200 } else { 100 };
    let length = bytes.to_string();
    let headers = [CaffeineHeader {
        name: "Content-Length",
        value: &length,
    }];
    let mut builder = CaffeineManifestBuilder::new(
        bytes,
        &headers,
        CaffeineHashLimits {
            max_content_bytes: NonZeroU64::new(bytes).unwrap(),
            max_append_bytes: bound(200),
            max_headers: bound(1),
            max_header_bytes: bound(128),
        },
        bound(1),
    )
    .unwrap();
    builder
        .append(0, &vec![id; usize::try_from(bytes).unwrap()])
        .unwrap();
    builder.finish().unwrap()
}

pub(super) fn prepare_sample(owner: &mut Owner, request: UploadRequest, id: u8) {
    let built = manifest(id);
    let length = request.object.bytes.to_string();
    let headers = [CaffeineHeader {
        name: "Content-Length",
        value: &length,
    }];
    owner
        .prepare_manifest(
            context(
                request.object.first.object().service(),
                request.object.first.object().tenant(),
            ),
            request,
            ic_blob_storage::model::service::upload::manifest::UploadManifest {
                chunks: built.manifest().chunks(),
                headers: &headers,
            },
            0,
        )
        .unwrap();
}
