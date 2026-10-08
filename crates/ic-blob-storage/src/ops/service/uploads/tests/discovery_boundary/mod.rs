use super::*;
use crate::workflow::uploads::discovery::inspect;
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryFailure as F;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryRequest;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryResponse;
fn request(root: [u8; 32]) -> UploadDiscoveryRequest {
    UploadDiscoveryRequest {
        scope: TenantScope {
            service: p(1),
            tenant: p(4),
            namespace: 1,
        },
        root,
    }
}
#[test]
fn discovery_boundary_checks_authority_before_absence_and_reports_restored_absence() {
    let memory = memory();
    let store = StableUploads::install(clone_memory(&memory), config()).unwrap();
    let request = request([9; 32]);
    let empty = UploadDiscoveryResponse {
        request,
        content: None,
        fenced: false,
    };
    assert_eq!(inspect(&store, context(4), request), Ok(empty));
    let heap = UploadAdmissions::new(config());
    assert_eq!(inspect(&heap, context(4), request), Ok(empty));
    for actor in [2, 5, 6] {
        assert_eq!(inspect(&store, context(actor), request), Err(F::Denied));
    }
    for scope in [
        TenantScope {
            service: p(9),
            ..request.scope
        },
        TenantScope {
            namespace: 2,
            ..request.scope
        },
    ] {
        assert_eq!(
            inspect(
                &store,
                context(4),
                UploadDiscoveryRequest { scope, ..request }
            ),
            Err(F::Binding)
        );
    }
    assert_eq!(
        inspect(
            &store,
            context(4),
            UploadDiscoveryRequest {
                scope: TenantScope {
                    namespace: 0,
                    ..request.scope
                },
                ..request
            }
        ),
        Err(F::Invalid)
    );
    for tenant in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            inspect(
                &store,
                UploadContext {
                    actor: tenant,
                    ..context(4)
                },
                UploadDiscoveryRequest {
                    scope: TenantScope {
                        tenant,
                        ..request.scope
                    },
                    ..request
                }
            ),
            Err(F::Invalid)
        );
    }
    drop(store);
    let restored = StableUploads::open(memory, config()).unwrap();
    assert_eq!(
        inspect(&restored, context(4), request),
        Ok(UploadDiscoveryResponse {
            fenced: true,
            ..empty
        })
    );
}

#[test]
fn discovery_boundary_preserves_independent_full_width_identities_and_cancelled_history() {
    use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
    use ic_blob_storage_contracts::dto::upload::history::UploadContentState;
    use ic_blob_storage_contracts::dto::upload::history::UploadHistoryEntry;
    let memory = memory();
    let mut store = StableUploads::install(clone_memory(&memory), config()).unwrap();
    let mut heap = UploadAdmissions::new(config());
    let command = TenantUpdate {
        tenant: p(4),
        expected: None,
        active: true,
    };
    store.update_tenant(context(2), command).unwrap();
    heap.update_tenant(context(2), command).unwrap();
    let mut input = permission(1);
    input.request.id = UploadRequestId::new(NonZeroU128::new(u128::MAX).unwrap());
    input.request.object.first = ReferenceKey::new(
        ObjectBinding::new(
            p(1),
            p(4),
            ObjectIdentity {
                namespace: NonZeroU128::MIN,
                object: NonZeroU128::new(u128::MAX - 1).unwrap(),
                incarnation: NonZeroU128::new(u128::MAX - 2).unwrap(),
            },
        )
        .unwrap(),
        ReferenceId::new(NonZeroU128::new(u128::MAX - 3).unwrap()),
    );
    store.admit(context(4), input, 1).unwrap();
    heap.admit(context(4), input, 1).unwrap();
    let request = request(*input.request.object.root.as_bytes());
    let mut expected = UploadDiscoveryResponse {
        request,
        fenced: false,
        content: Some(UploadHistoryEntry {
            request: ReferenceUpload {
                service: p(1),
                tenant: p(4),
                namespace: 1,
                upload: u128::MAX,
                object: u128::MAX - 1,
                incarnation: u128::MAX - 2,
                first_reference: u128::MAX - 3,
                root: request.root,
                bytes: input.request.object.bytes,
            },
            state: UploadContentState::Reserved,
        }),
    };
    assert_eq!(inspect(&store, context(4), request), Ok(expected));
    assert_eq!(inspect(&heap, context(4), request), Ok(expected));
    let foreign = UploadDiscoveryRequest {
        scope: TenantScope {
            tenant: p(6),
            ..request.scope
        },
        ..request
    };
    assert_eq!(inspect(&store, context(6), foreign).unwrap().content, None);
    assert_eq!(inspect(&heap, context(6), foreign).unwrap().content, None);
    store.revoke(context(4), input.request).unwrap();
    heap.revoke(context(4), input.request).unwrap();
    expected.content.as_mut().unwrap().state = UploadContentState::Cancelled;
    assert_eq!(inspect(&store, context(4), request), Ok(expected));
    assert_eq!(inspect(&heap, context(4), request), Ok(expected));
    drop(store);
    let restored = StableUploads::open(memory, config()).unwrap();
    expected.fenced = true;
    assert_eq!(inspect(&restored, context(4), request), Ok(expected));
    assert_eq!(
        candid::decode_one::<UploadDiscoveryResponse>(&candid::encode_one(expected).unwrap())
            .unwrap(),
        expected
    );
}
