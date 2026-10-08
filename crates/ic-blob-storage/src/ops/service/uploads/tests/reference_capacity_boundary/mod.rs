use super::*;
use crate::workflow::references::capacity::inspect;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityFailure as F;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityRequest;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityResponse;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceHeadroom;
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::reference::binding::ReferenceOperation;
use ic_blob_storage_contracts::reference::binding::ReferenceRequest;
use ic_blob_storage_contracts::reference::binding::ReferenceRequestId;
fn request(root: [u8; 32]) -> ReferenceCapacityRequest {
    ReferenceCapacityRequest {
        scope: TenantScope {
            service: p(1),
            tenant: p(4),
            namespace: 1,
        },
        root,
    }
}
#[test]
fn reference_capacity_boundary_checks_authority_before_absence_and_reports_restored_absence() {
    let memory = memory();
    let store = StableUploads::install(clone_memory(&memory), config()).unwrap();
    let request = request([9; 32]);
    let empty = ReferenceCapacityResponse {
        request,
        headroom: None,
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
                ReferenceCapacityRequest { scope, ..request }
            ),
            Err(F::Binding)
        );
    }
    assert_eq!(
        inspect(
            &store,
            context(4),
            ReferenceCapacityRequest {
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
                ReferenceCapacityRequest {
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
        Ok(ReferenceCapacityResponse {
            fenced: true,
            ..empty
        })
    );
}

#[test]
fn reference_capacity_boundary_retains_cleanup_reservations_and_retired_history() {
    let memory = memory();
    let mut store = StableUploads::install(clone_memory(&memory), config()).unwrap();
    let permission = lifecycle::exposed(&mut store);
    let request = request(*permission.request.object.root.as_bytes());
    assert_eq!(inspect(&store, context(4), request).unwrap().headroom, None);
    store.confirm_upload(permission.request).unwrap();
    let initial = inspect(&store, context(4), request).unwrap();
    assert_eq!(
        initial.headroom,
        Some(ReferenceHeadroom {
            reference_slots: 1,
            unreserved_receipts: 2,
            release_reserved_receipts: 1,
            fresh_retains: 1
        })
    );
    let foreign = ReferenceCapacityRequest {
        scope: TenantScope {
            tenant: p(6),
            ..request.scope
        },
        ..request
    };
    assert_eq!(inspect(&store, context(6), foreign).unwrap().headroom, None);
    let enrollment = store.tenant(context(4), p(4)).unwrap().unwrap();
    for (operation, reference, retain) in [(1, 2, true), (2, 1, false), (3, 2, false)] {
        let key = ReferenceKey::new(
            permission.request.object.first.object(),
            ReferenceId::new(NonZeroU128::new(reference).unwrap()),
        );
        store
            .apply_reference(
                context(4),
                permission.request,
                ReferenceRequest {
                    id: ReferenceRequestId::new(NonZeroU128::new(operation).unwrap()),
                    operation: if retain {
                        ReferenceOperation::Retain(key)
                    } else {
                        ReferenceOperation::Release(key)
                    },
                },
            )
            .unwrap();
        if retain {
            store
                .update_tenant(
                    context(2),
                    TenantUpdate {
                        tenant: p(4),
                        expected: Some(enrollment),
                        active: false,
                    },
                )
                .unwrap();
            let held = inspect(&store, context(4), request)
                .unwrap()
                .headroom
                .unwrap();
            assert_eq!(held.release_reserved_receipts, 2);
            assert_eq!(held.unreserved_receipts, 0);
            assert_eq!(held.fresh_retains, 0);
        }
    }
    let retired = inspect(&store, context(4), request).unwrap();
    assert_eq!(
        retired.headroom,
        Some(ReferenceHeadroom {
            reference_slots: 0,
            unreserved_receipts: 0,
            release_reserved_receipts: 0,
            fresh_retains: 0
        })
    );
    store.confirm_provider_deleted(permission.request).unwrap();
    store.confirm_billing_stopped(permission.request).unwrap();
    assert_eq!(inspect(&store, context(4), request), Ok(retired));
    drop(store);
    let restored = StableUploads::open(memory, config()).unwrap();
    let retained = ReferenceCapacityResponse {
        fenced: true,
        ..retired
    };
    assert_eq!(inspect(&restored, context(4), request), Ok(retained));
    let bytes = candid::encode_one(retained).unwrap();
    assert_eq!(
        candid::decode_one::<ReferenceCapacityResponse>(&bytes).unwrap(),
        retained
    );
}
