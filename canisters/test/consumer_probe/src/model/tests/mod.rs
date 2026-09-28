use super::*;
use ic_blob_storage::dto::reference::{
    ReferenceChange, ReferenceTransitionFailure, ReferenceUpload,
};
fn fixture() -> (ConsumerRecord, Registration) {
    let service = Principal::from_slice(&[1, 1]);
    let tenant = Principal::from_slice(&[2, 1]);
    let record = ConsumerRecord::new(Principal::from_slice(&[3, 1]), service, tenant);
    let intent = Registration {
        asset: 1,
        payload: vec![7; 8],
        release_operation: 2,
        retain: ReferenceCommand {
            upload: ReferenceUpload {
                service,
                tenant,
                namespace: 1,
                upload: 10,
                object: 20,
                incarnation: 30,
                first_reference: 1,
                root: [9; 32],
                bytes: 10,
            },
            operation: 1,
            reference: 2,
            action: ReferenceAction::Retain,
        },
    };
    (record, intent)
}
#[test]
fn inner_failures_remain_repairable_without_publication_or_successful_cleanup() {
    let (mut record, intent) = fixture();
    record.prepare(&intent).unwrap();
    record.start(1, false).unwrap();
    record
        .complete(
            1,
            false,
            ReferenceReceiptResponse {
                request: intent.retain,
                result: Err(ReferenceTransitionFailure::Limit),
            },
        )
        .unwrap();
    assert_eq!(record.publish(1), Err(Failure::State));
    record.cancel(1).unwrap();
    assert_eq!(record.start(1, true), Err(Failure::State));
    record.validate().unwrap();
    let (mut record, intent) = fixture();
    record.prepare(&intent).unwrap();
    record.start(1, false).unwrap();
    record
        .complete(
            1,
            false,
            ReferenceReceiptResponse {
                request: intent.retain,
                result: Ok(ReferenceChange::Changed),
            },
        )
        .unwrap();
    record.cancel(1).unwrap();
    record.start(1, true).unwrap();
    record
        .complete(
            1,
            true,
            ReferenceReceiptResponse {
                request: record.command(1, true).unwrap(),
                result: Err(ReferenceTransitionFailure::UnknownReference),
            },
        )
        .unwrap();
    assert_eq!(
        record.view(1).unwrap().release_result,
        Some(Err(ReferenceTransitionFailure::UnknownReference))
    );
    assert_eq!(record.start(1, true), Ok(false));
    assert_eq!(record.publish(1), Err(Failure::State));
    record.validate().unwrap();
}
#[test]
fn reserved_reference_and_both_operation_ids_cannot_be_reassigned_to_another_asset() {
    let (mut record, intent) = fixture();
    record.prepare(&intent).unwrap();
    for (reference, retain, release) in [(2, 3, 4), (3, 2, 4), (3, 3, 1)] {
        let mut changed = intent.clone();
        changed.asset = 2;
        changed.retain.reference = reference;
        changed.retain.operation = retain;
        changed.release_operation = release;
        assert_eq!(record.prepare(&changed), Err(Failure::Conflict));
    }
    record.cancel(1).unwrap();
    assert_eq!(record.start(1, false), Ok(false));
    assert!(!record.view(1).unwrap().retain_started);
    record.validate().unwrap();
}
