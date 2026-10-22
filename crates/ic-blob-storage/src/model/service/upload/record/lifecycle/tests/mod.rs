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
            actor: Principal::from_slice(&[1; 29]).into(),
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
        gateway_acknowledged: false,
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

#[test]
fn gateway_acknowledgment_never_changes_physical_or_billing_phase_and_replays_late() {
    let live = ConfirmedLifecycleRecord::new();
    assert_eq!(
        {
            let mut record = live;
            record.acknowledge_gateway()
        },
        Err(LifecycleError::LiveReferencesRemain)
    );
    for phase in [
        PhaseRecord::DeletionPending,
        PhaseRecord::ProviderDeleted,
        PhaseRecord::Settled,
    ] {
        let mut record = ConfirmedLifecycleRecord {
            phase,
            active: 0,
            ..live
        };
        let before = record.counts();
        assert_eq!(record.acknowledge_gateway(), Ok(LifecycleChange::Changed));
        assert!(record.gateway_acknowledged());
        assert_eq!(record.phase, phase);
        assert_eq!(record.counts(), before);
        assert_eq!(record.acknowledge_gateway(), Ok(LifecycleChange::Unchanged));
    }
}
