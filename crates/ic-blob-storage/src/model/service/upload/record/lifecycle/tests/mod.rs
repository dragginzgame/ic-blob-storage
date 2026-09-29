use super::*;
#[test]
fn bounded_codecs_preserve_maximum_receipt_identity_and_typed_outcomes() {
    for result in [
        ResultRecord::Changed,
        ResultRecord::Unchanged,
        ResultRecord::UnknownReference,
        ResultRecord::ReferenceReleased,
        ResultRecord::ReferenceLimitReached,
        ResultRecord::DeletionAlreadyQueued,
    ] {
        let record = ReferenceReceiptRecord {
            version: 1,
            actor: Principal::from_slice(&[1; 29]),
            reference: u128::MAX,
            retain: true,
            result,
        };
        assert_eq!(
            ReferenceReceiptRecord::from_bytes(record.to_bytes()),
            record
        );
    }
    let record = ConfirmedLifecycleRecord {
        version: 1,
        phase: PhaseRecord::Live,
        references: u64::MAX,
        active: u64::MAX,
        receipts: u64::MAX,
        completion: CompletionRecord::Attested(AttestationRecord {
            verifier: Principal::from_slice(&[255; 29]),
            content_digest: [255; 32],
            observed_at_ns: u64::MAX,
            accepted_at_ns: u64::MAX,
        }),
    };
    assert_eq!(
        ConfirmedLifecycleRecord::from_bytes(record.to_bytes()),
        record
    );
    for state in [ReferenceState::Active, ReferenceState::Released] {
        let record = ReferenceRecord::new(state);
        assert_eq!(ReferenceRecord::from_bytes(record.to_bytes()), record);
    }
}
#[test]
fn oversized_or_malformed_receipts_trap_instead_of_losing_history() {
    for bytes in [vec![0; 257], b"truncated".to_vec()] {
        assert!(
            std::panic::catch_unwind(|| ReferenceReceiptRecord::from_bytes(Cow::Owned(bytes)))
                .is_err()
        );
    }
}
