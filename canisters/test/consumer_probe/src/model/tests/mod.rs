use super::*;
use ic_blob_storage_contracts::dto::reference::ReferenceChange;
use ic_blob_storage_contracts::dto::reference::ReferenceTransitionFailure;
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
fn fixture() -> (ConsumerRecord, Registration) {
    let service = Principal::from_slice(&[1, 1]);
    let tenant = Principal::from_slice(&[2, 1]);
    let record = ConsumerRecord::new(Principal::from_slice(&[3, 1]), service, tenant);
    let intent = Registration {
        asset: 1,
        payload: vec![7; 8],
        release_operation: 2,
        source: RegistrationSource::Existing(ReferenceCommand {
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
        }),
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
                request: retained(&intent),
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
                request: retained(&intent),
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
        let mut command = retained(&changed);
        command.reference = reference;
        command.operation = retain;
        changed.source = RegistrationSource::Existing(command);
        changed.release_operation = release;
        assert_eq!(record.prepare(&changed), Err(Failure::Conflict));
    }
    record.cancel(1).unwrap();
    assert_eq!(record.start(1, false), Ok(false));
    assert!(!record.view(1).unwrap().retain_started);
    record.validate().unwrap();
}

fn retained(intent: &Registration) -> ReferenceCommand {
    match intent.source {
        RegistrationSource::Existing(command) => command,
        RegistrationSource::Fresh(_) => panic!("existing-content test"),
    }
}

#[test]
fn first_reference_reservation_and_newer_completion_survive_conflicting_observations() {
    use ic_blob_storage_contracts::dto::upload::UploadState;
    use ic_blob_storage_contracts::dto::upload::UploadStatusResponse;
    let (mut record, mut intent) = fixture();
    let upload = retained(&intent).upload;
    let permission = ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest {
        upload,
        uploader: Principal::from_slice(&[4, 1]),
        expires_at_ns: 100,
    };
    intent.source = RegistrationSource::Fresh(permission);
    record.prepare(&intent).unwrap();
    let mut duplicate = intent.clone();
    duplicate.asset = 2;
    duplicate.release_operation = 3;
    assert_eq!(record.prepare(&duplicate), Err(Failure::Conflict));
    let (_, mut existing) = fixture();
    existing.asset = 2;
    assert_eq!(record.prepare(&existing), Err(Failure::Conflict));
    existing.release_operation = 3;
    record.prepare(&existing).unwrap();
    record.start_admission(1).unwrap();
    record
        .acknowledge_admission(
            1,
            Ok(
                ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse {
                    permission,
                    state: UploadState::Confirmed,
                    revoked: false,
                },
            ),
        )
        .unwrap();
    let confirmed = record.view(1).unwrap();
    for state in [
        UploadState::Reserved,
        UploadState::ExposurePossible,
        UploadState::Cancelled,
    ] {
        assert_eq!(
            record.observe_upload(
                1,
                UploadStatusResponse {
                    upload,
                    state,
                    revoked: false,
                }
            ),
            Err(Failure::Conflict)
        );
        assert_eq!(record.view(1), Ok(confirmed.clone()));
    }
    record.cancel(1).unwrap();
    assert_eq!(record.publish(1), Err(Failure::State));
    assert_eq!(
        record.command(1, true).unwrap().reference,
        upload.first_reference
    );
    assert_eq!(record.command(1, false), Err(Failure::State));
    record.validate().unwrap();
}
