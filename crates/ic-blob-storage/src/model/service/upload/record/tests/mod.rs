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
    record.phase = UploadPhaseRecord::ExposurePossible;
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
            std::panic::catch_unwind(|| UploadManifestRecord::from_bytes(Cow::Owned(bytes)))
                .is_err()
        );
    }
}

#[test]
fn admitted_leaf_selection_preserves_position_and_exact_final_length() {
    use crate::model::identity::caffeine::{CAFFEINE_CHUNK_BYTES, manifest::CaffeineChunkRange};
    let record = UploadManifestRecord {
        version: 1,
        chunks: vec![[1; 32], [2; 32]],
        headers: vec![],
    };
    let bytes = CAFFEINE_CHUNK_BYTES as u64 + 7;
    for (index, offset, length, hash) in [
        (0, 0, CAFFEINE_CHUNK_BYTES, [1; 32]),
        (1, CAFFEINE_CHUNK_BYTES as u64, 7, [2; 32]),
    ] {
        assert_eq!(
            record.read_leaf(bytes, index),
            Some((
                CaffeineChunkRange {
                    index,
                    offset,
                    bytes: length
                },
                CaffeineChunkHash::try_from(hash.as_slice()).unwrap()
            ))
        );
    }
    assert_eq!(record.read_leaf(bytes, 2), None);
    assert_eq!(record.read_leaf(bytes, u64::MAX), None);
    assert_eq!(record.read_leaf(CAFFEINE_CHUNK_BYTES as u64, 0), None);
}
