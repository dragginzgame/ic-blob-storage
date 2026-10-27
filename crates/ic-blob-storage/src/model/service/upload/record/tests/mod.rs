use super::*;
fn permission() -> UploadPermissionRecord {
    UploadPermissionRecord::new(
        UploadPermission {
            request: UploadRequest {
                id: UploadRequestId::new(NonZeroU128::MAX),
                object: UploadObject {
                    root: ProviderRootHash::try_from([255; 32].as_slice()).unwrap(),
                    bytes: u64::MAX,
                    first: ReferenceKey::new(
                        ObjectBinding::new(
                            Principal::from_slice(&[1; 29]),
                            Principal::from_slice(&[2; 29]),
                            ObjectIdentity {
                                namespace: NonZeroU128::MAX,
                                object: NonZeroU128::MAX,
                                incarnation: NonZeroU128::MAX,
                            },
                        )
                        .unwrap(),
                        ReferenceId::new(NonZeroU128::MAX),
                    ),
                },
            },
            uploader: Principal::from_slice(&[3; 29]),
            expires_at_ns: u64::MAX,
        },
        u64::MAX - 1,
        NonZeroU64::MAX,
    )
}
#[test]
fn maximal_permission_and_usage_round_trip_without_narrowing() {
    let permission = permission();
    assert!(permission.view().is_some());
    let record = UploadStoreRecord::Permission(permission);
    assert_eq!(UploadStoreRecord::from_bytes(record.to_bytes()), record);
    let usage = UploadUsageRecord {
        version: 1,
        operations: u64::MAX,
        active: u64::MAX,
        bytes: u128::MAX,
        chunks: u64::MAX,
        logical: u128::MAX,
        physical: u128::MAX,
        liability: u128::MAX,
    };
    assert_eq!(UploadUsageRecord::from_bytes(usage.to_bytes()), usage);
}
#[test]
fn inconsistent_permission_phases_cannot_become_valid_views() {
    let original = permission();
    let mut record = original.clone();
    record.revoked = true;
    assert!(record.view().is_none());
    record = original.clone();
    record.phase = UploadPhaseRecord::Cancelled;
    assert!(record.view().is_none());
    record = original;
    record.phase = UploadPhaseRecord::Registered;
    assert!(record.view().is_none());
    record.manifest = true;
    assert!(record.view().is_some());
    record.revoke();
    assert!(record.view().unwrap().revoked);
}
#[test]
fn variable_page_layout_keeps_a_hard_record_decode_bound() {
    for bytes in [vec![0; MANIFEST_BYTES + 1], b"truncated".to_vec()] {
        assert!(
            std::panic::catch_unwind(|| UploadManifestPartRecord::from_bytes(Cow::Owned(bytes)))
                .is_err()
        );
    }
}

#[test]
fn admitted_large_leaf_and_header_envelopes_fit_the_wire_record_policy() {
    // Both profiles fit the unchanged public admission formula. Exercise many
    // fixed-array leaves and many small strings, whose traversal costs differ.
    for (chunks, headers) in [(1954, 1), (200, 4096)] {
        let headers = (0..headers)
            .map(|index| UploadHeaderRecord {
                name: format!("x-{index:04x}"),
                value: String::new(),
            })
            .collect::<Vec<_>>();
        let header_bytes: usize = headers.iter().map(|h| h.name.len() + h.value.len()).sum();
        assert!(1024 + 33 * chunks + header_bytes + 8 * headers.len() <= MANIFEST_BYTES);
        let record = PreparedUploadManifest {
            chunks: vec![[255; 32]; chunks],
            headers,
        };
        for part in record.split() {
            let bytes = part.to_bytes();
            assert!(bytes.len() <= MANIFEST_BYTES);
            assert_eq!(UploadManifestPartRecord::from_bytes(bytes), part);
        }
    }
}
