use super::*;
use crate::model::service::storage_key::ManifestKeyRecord;

#[test]
fn reopening_preserves_independent_activations_and_rejects_a_later_future_generation() {
    for split_tenant in [false, true] {
        let m = memory();
        let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
        enroll(&mut store);
        store.admit(context(4), permission(1), 1).unwrap();
        let tenant = if split_tenant { p(6) } else { p(4) };
        let prior = store.tenant(context(2), tenant).unwrap();
        let suspended = store
            .update_tenant(
                context(2),
                TenantUpdate {
                    tenant,
                    expected: prior,
                    active: false,
                },
            )
            .unwrap();
        let activated = store
            .update_tenant(
                context(2),
                TenantUpdate {
                    tenant,
                    expected: Some(suspended),
                    active: true,
                },
            )
            .unwrap();
        let mut second = permission(2);
        second.request.object.first = ReferenceKey::new(
            ObjectBinding::new(
                p(1),
                tenant,
                second.request.object.first.object().identity(),
            )
            .unwrap(),
            second.request.object.first.reference(),
        );
        store
            .admit(
                UploadContext {
                    actor: tenant,
                    ..context(4)
                },
                second,
                1,
            )
            .unwrap();
        drop(store);
        let restored = StableUploads::open(clone_memory(&m), config()).unwrap();
        assert_eq!(
            restored
                .lookup(context(5), permission(1).request)
                .unwrap()
                .tenant_generation,
            NonZeroU64::MIN
        );
        assert_eq!(
            restored
                .lookup(context(5), second.request)
                .unwrap()
                .tenant_generation,
            activated.generation
        );
        assert!(restored.is_fenced());
        drop(restored);

        let mut permissions =
            StableBTreeMap::load(m.permissions.lease()).expect("current stable collection");
        permissions
            .insert(
                key(second.request),
                UploadStoreRecord::Permission(UploadPermissionRecord::new(
                    second,
                    1,
                    activated.generation.checked_add(1).unwrap(),
                )),
            )
            .expect("stable collection operation");
        drop(permissions);
        let memories = [
            &m.tenants,
            &m.roots,
            &m.objects,
            &m.permissions,
            &m.usage,
            &m.manifests,
            &m.confirmed,
            &m.references,
            &m.receipts,
            &m.deletions,
        ];
        let before = memories.map(|memory| memory.borrow().clone());
        assert!(matches!(
            StableUploads::open(clone_memory(&m), config()),
            Err(UploadStoreError::InvalidRecord)
        ));
        assert_eq!(memories.map(|memory| memory.borrow().clone()), before);
    }
}

#[test]
fn reopening_binds_each_root_to_one_exact_tenant_and_request() {
    for foreign_tenant in [false, true] {
        let m = memory();
        let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
        enroll(&mut store);
        let first = permission(1);
        let mut second = permission(2);
        if foreign_tenant {
            store
                .update_tenant(
                    context(2),
                    TenantUpdate {
                        tenant: p(6),
                        expected: None,
                        active: true,
                    },
                )
                .unwrap();
            second.request.id = first.request.id;
            second.request.object.first = ReferenceKey::new(
                ObjectBinding::new(p(1), p(6), first.request.object.first.object().identity())
                    .unwrap(),
                first.request.object.first.reference(),
            );
        }
        let actor = second.request.object.first.object().tenant();
        let second_context = UploadContext {
            actor,
            ..context(4)
        };
        store.admit(context(4), first, 1).unwrap();
        store.admit(second_context, second, 1).unwrap();
        drop(store);

        // Distinct roots remain valid even when two tenants use the same ID.
        let restored = StableUploads::open(clone_memory(&m), config()).unwrap();
        assert_eq!(
            restored
                .lookup(context(5), second.request)
                .unwrap()
                .permission,
            second
        );
        assert!(restored.is_fenced());
        drop(restored);

        // Keep collection cardinalities unchanged: restoration must verify the
        // actual root-to-request and root-to-tenant bindings, not just counts.
        second.request.object.root = first.request.object.root;
        let mut permissions =
            StableBTreeMap::load(m.permissions.lease()).expect("current stable collection");
        permissions
            .insert(
                key(second.request),
                UploadStoreRecord::Permission(UploadPermissionRecord::new(
                    second,
                    1,
                    NonZeroU64::MIN,
                )),
            )
            .expect("stable collection operation");
        drop(permissions);
        let memories = [
            &m.tenants,
            &m.roots,
            &m.objects,
            &m.permissions,
            &m.usage,
            &m.manifests,
            &m.confirmed,
            &m.references,
            &m.receipts,
            &m.deletions,
        ];
        let before = memories.map(|memory| memory.borrow().clone());
        assert!(matches!(
            StableUploads::open(clone_memory(&m), config()),
            Err(UploadStoreError::InvalidRecord)
        ));
        assert_eq!(memories.map(|memory| memory.borrow().clone()), before);
    }
}

#[test]
fn pending_candidates_replay_exactly_and_reopen_refuses_missing_or_orphaned_entries() {
    use ic_blob_storage_contracts::reference::binding::{
        ReferenceOperation, ReferenceRequest, ReferenceRequestId,
    };
    for corrupt in [false, true] {
        let m = memory();
        let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
        let input = lifecycle::exposed(&mut store);
        let request = ReferenceRequest {
            id: ReferenceRequestId::new(NonZeroU128::MIN),
            operation: ReferenceOperation::Release(input.request.object.first),
        };
        store
            .apply_reference(context(4), input.request, request)
            .unwrap();
        let roots = vec![input.request.object.root.as_bytes().to_vec()];
        assert_eq!(store.gateway_deletions().unwrap(), roots);
        let obligations = store.usage().unwrap();
        if corrupt {
            // Preserve the candidate count while replacing its actual identity.
            store
                .deletions
                .remove(&key(input.request))
                .expect("stable collection operation");
            store
                .deletions
                .insert(UploadKeyRecord(p(4).into(), u128::MAX))
                .expect("stable collection operation");
        } else {
            store
                .acknowledge_gateway_roots(&[input.request.object.root].into())
                .unwrap();
            store
                .apply_reference(context(4), input.request, request)
                .unwrap();
            assert_eq!(store.gateway_deletions().unwrap(), Vec::<Vec<u8>>::new());
            assert_eq!(store.usage().unwrap(), obligations);
        }
        drop(store);
        let before = m.deletions.borrow().clone();
        let restored = StableUploads::open(clone_memory(&m), config());
        if corrupt {
            assert!(matches!(restored, Err(UploadStoreError::InvalidRecord)));
        } else {
            let restored = restored.unwrap();
            assert!(restored.is_fenced());
            assert!(restored.deletions.is_empty());
            assert_eq!(restored.usage().unwrap(), obligations);
        }
        assert_eq!(*m.deletions.borrow(), before);
    }
}

#[test]
fn reopening_refuses_missing_pending_candidate_without_repair() {
    use ic_blob_storage_contracts::reference::binding::{
        ReferenceOperation, ReferenceRequest, ReferenceRequestId,
    };
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    let input = lifecycle::exposed(&mut store);
    store
        .apply_reference(
            context(4),
            input.request,
            ReferenceRequest {
                id: ReferenceRequestId::new(NonZeroU128::MIN),
                operation: ReferenceOperation::Release(input.request.object.first),
            },
        )
        .unwrap();
    store
        .deletions
        .remove(&key(input.request))
        .expect("stable collection operation");
    drop(store);
    let before = m.deletions.borrow().clone();
    assert!(matches!(
        StableUploads::open(clone_memory(&m), config()),
        Err(UploadStoreError::InvalidRecord)
    ));
    assert_eq!(*m.deletions.borrow(), before);
}

#[test]
fn manifest_parts_refuse_missing_reversed_and_orphaned_records_on_reopen() {
    for damage in 0..3 {
        let m = memory();
        let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
        let input = lifecycle::exposed(&mut store);
        let owner = key(input.request);
        let header = store
            .manifests
            .get(&ManifestKeyRecord(owner, 1))
            .expect("stable collection operation")
            .unwrap();
        match damage {
            0 => {
                store
                    .manifests
                    .remove(&ManifestKeyRecord(owner, 0))
                    .expect("stable collection operation");
            }
            1 => {
                store
                    .manifests
                    .insert(ManifestKeyRecord(owner, 0), header)
                    .expect("stable collection operation");
            }
            _ => {
                store
                    .manifests
                    .insert(ManifestKeyRecord(owner, 2), header)
                    .expect("stable collection operation");
            }
        }
        drop(store);
        let before = m.manifests.borrow().clone();
        assert!(matches!(
            StableUploads::open(clone_memory(&m), config()),
            Err(UploadStoreError::InvalidRecord)
        ));
        assert_eq!(*m.manifests.borrow(), before);
    }
}
