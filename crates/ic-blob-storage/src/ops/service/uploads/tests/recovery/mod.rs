use super::*;

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

        let mut permissions = BTreeMap::load(m.permissions.clone());
        permissions.insert(
            key(second.request),
            UploadStoreRecord::Permission(UploadPermissionRecord::new(
                second,
                1,
                activated.generation.checked_add(1).unwrap(),
            )),
        );
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
            &m.root_requests,
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
        let mut permissions = BTreeMap::load(m.permissions.clone());
        permissions.insert(
            key(second.request),
            UploadStoreRecord::Permission(UploadPermissionRecord::new(second, 1, NonZeroU64::MIN)),
        );
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
            &m.root_requests,
        ];
        let before = memories.map(|memory| memory.borrow().clone());
        assert!(matches!(
            StableUploads::open(clone_memory(&m), config()),
            Err(UploadStoreError::InvalidRecord)
        ));
        assert_eq!(memories.map(|memory| memory.borrow().clone()), before);
    }
}
