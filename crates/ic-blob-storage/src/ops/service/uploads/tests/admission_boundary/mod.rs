use super::*;
use crate::{
    dto::{
        reference::ReferenceUpload,
        upload::{
            UploadState,
            admission::{UploadAdmissionFailure as F, UploadAdmissionRequest},
        },
    },
    workflow::uploads::admission::{admit, inspect},
};
#[test]
fn exact_admission_recovers_permission_without_renewal_through_expiry_suspension_and_restore() {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    let input = UploadAdmissionRequest {
        upload: ReferenceUpload {
            service: p(1),
            tenant: p(4),
            namespace: 1,
            upload: u128::MAX,
            object: u128::MAX - 1,
            incarnation: 7,
            first_reference: 9,
            root: [1; 32],
            bytes: 10,
        },
        uploader: p(5),
        expires_at_ns: 100,
    };
    assert_eq!(inspect(&store, context(4), input), Err(F::Unknown));
    assert_eq!(admit(&mut store, context(4), input, 1), Err(F::Inactive));
    let enrolled = enroll(&mut store);
    for actor in [2, 5, 6] {
        assert_eq!(admit(&mut store, context(actor), input, 1), Err(F::Denied));
        assert_eq!(inspect(&store, context(actor), input), Err(F::Denied));
    }
    assert_eq!(admit(&mut store, context(4), input, 100), Err(F::Expired));
    assert_eq!(store.usage().unwrap().operations, 0);
    let admitted = admit(&mut store, context(4), input, 1).unwrap();
    assert!(!admitted.replayed);
    assert_eq!(admitted.admission.state, UploadState::Reserved);
    let usage = store.usage().unwrap();
    for changed in [
        UploadAdmissionRequest {
            uploader: p(6),
            ..input
        },
        UploadAdmissionRequest {
            expires_at_ns: 101,
            ..input
        },
    ] {
        assert_eq!(admit(&mut store, context(4), changed, 2), Err(F::Conflict));
        assert_eq!(inspect(&store, context(4), changed), Err(F::Conflict));
    }
    store
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: Some(enrolled),
                active: false,
            },
        )
        .unwrap();
    let replay = admit(&mut store, context(4), input, 200).unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.admission, admitted.admission);
    assert_eq!(store.usage().unwrap(), usage);
    let permission = super::super::admission::parse(context(4), input).unwrap();
    store.revoke(context(4), permission.request).unwrap();
    let cancelled = inspect(&store, context(4), input).unwrap();
    assert_eq!(cancelled.state, UploadState::Cancelled);
    assert!(cancelled.revoked);
    assert_eq!(
        admit(&mut store, context(4), input, 200).unwrap().admission,
        cancelled
    );
    drop(store);
    let mut restored = StableUploads::open(m, config()).unwrap();
    assert_eq!(inspect(&restored, context(4), input), Ok(cancelled));
    assert_eq!(admit(&mut restored, context(4), input, 200), Err(F::Fenced));
}
