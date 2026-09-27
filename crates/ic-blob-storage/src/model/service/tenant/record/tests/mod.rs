use super::*;

#[test]
fn maximal_records_fit_the_bound_and_round_trip_without_narrowing() {
    let metadata = TenantStoreRecord::Metadata(TenantStoreMetadataRecord {
        version: 1,
        service: Principal::from_slice(&[1; 29]),
        operator: Principal::from_slice(&[2; 29]),
        namespace: u128::MAX,
        max_tenants: u64::from(u32::MAX),
    });
    let enrollment =
        TenantStoreRecord::Enrollment(TenantEnrollmentRecord::new(TenantEnrollmentView {
            generation: NonZeroU64::MAX,
            active: true,
        }));
    for record in [metadata, enrollment] {
        let bytes = record.to_bytes();
        assert!(bytes.len() <= MAX_BYTES as usize);
        assert_eq!(TenantStoreRecord::from_bytes(bytes), record);
    }
}

#[test]
fn malformed_records_cannot_become_valid_enrollments() {
    assert!(
        TenantEnrollmentRecord {
            version: 1,
            generation: 0,
            active: true
        }
        .view()
        .is_none()
    );
    for bytes in [vec![0; MAX_BYTES as usize + 1], b"truncated".to_vec()] {
        assert!(
            std::panic::catch_unwind(|| TenantStoreRecord::from_bytes(Cow::Owned(bytes))).is_err()
        );
    }
}
