use super::*;
use std::num::NonZeroU128;

fn key(value: u128) -> ReferenceKey {
    use ic_blob_storage_contracts::binding::ObjectIdentity;
    use ic_platform::Principal;
    let one = NonZeroU128::new(1).expect("positive identity");
    let object = ObjectBinding::new(
        Principal::from_slice(&[1, 1]),
        Principal::from_slice(&[2, 1]),
        ObjectIdentity {
            namespace: one,
            object: one,
            incarnation: one,
        },
    )
    .expect("valid binding");
    ReferenceKey::new(
        object,
        ReferenceId::new(NonZeroU128::new(value).expect("positive ID")),
    )
}
fn object() -> BlobLifecycle {
    BlobLifecycle::from_confirmed_upload(100, key(1), NonZeroUsize::new(2).expect("positive bound"))
}

#[test]
fn release_deletion_and_billing_discharge_separate_obligations() {
    let mut blob = object();
    assert_eq!(blob.retain(key(2)), Ok(LifecycleChange::Changed));
    assert_eq!(blob.release(key(1)), Ok(LifecycleChange::Changed));
    assert_eq!(blob.active_references(), 1);
    assert_eq!(
        (
            blob.logical_bytes(),
            blob.physical_bytes(),
            blob.liability_bytes()
        ),
        (100, 100, 100)
    );
    assert_eq!(blob.release(key(2)), Ok(LifecycleChange::Changed));
    assert_eq!(blob.phase(), LifecyclePhase::DeletionPending);
    assert_eq!(blob.active_references(), 0);
    assert_eq!(
        (
            blob.logical_bytes(),
            blob.physical_bytes(),
            blob.liability_bytes()
        ),
        (0, 100, 100)
    );
    assert_eq!(
        blob.confirm_provider_deleted(blob.binding()),
        Ok(LifecycleChange::Changed)
    );
    assert_eq!(blob.phase(), LifecyclePhase::ProviderDeleted);
    assert_eq!(
        (
            blob.logical_bytes(),
            blob.physical_bytes(),
            blob.liability_bytes()
        ),
        (0, 0, 100)
    );
    assert_eq!(
        blob.confirm_billing_stopped(blob.binding()),
        Ok(LifecycleChange::Changed)
    );
    assert_eq!(blob.phase(), LifecyclePhase::Settled);
    assert_eq!(
        (
            blob.logical_bytes(),
            blob.physical_bytes(),
            blob.liability_bytes()
        ),
        (0, 0, 0)
    );
    assert_eq!(blob.reference_slots(), 2);
}

#[test]
fn replayed_and_unknown_releases_do_not_delete_another_reference() {
    let mut blob = object();
    blob.retain(key(2)).expect("second reference");
    blob.release(key(1)).expect("first release");
    let before = blob.clone();
    assert_eq!(blob.release(key(1)), Ok(LifecycleChange::Unchanged));
    assert_eq!(blob.release(key(3)), Err(LifecycleError::UnknownReference));
    assert_eq!(
        blob.confirm_provider_deleted(blob.binding()),
        Err(LifecycleError::LiveReferencesRemain)
    );
    assert_eq!(
        blob.confirm_billing_stopped(blob.binding()),
        Err(LifecycleError::DeletionNotConfirmed)
    );
    assert_eq!(blob, before);
}

#[test]
fn reference_slots_include_tombstones_and_rejections_preserve_state() {
    let mut blob = object();
    blob.retain(key(2)).expect("second reference");
    blob.release(key(1)).expect("retain release evidence");
    let before = blob.clone();
    assert_eq!(blob.retain(key(2)), Ok(LifecycleChange::Unchanged));
    assert_eq!(blob.retain(key(1)), Err(LifecycleError::ReferenceReleased));
    assert_eq!(
        blob.retain(key(3)),
        Err(LifecycleError::ReferenceLimitReached)
    );
    assert_eq!(blob, before);
}

#[test]
fn zero_bytes_and_maximum_length_both_require_explicit_settlement() {
    for bytes in [0, u64::MAX] {
        let mut blob = BlobLifecycle::from_confirmed_upload(
            bytes,
            key(1),
            NonZeroUsize::new(1).expect("positive bound"),
        );
        assert!(blob.has_unsettled_obligations());
        blob.release(key(1)).expect("last release");
        assert_eq!(blob.physical_bytes(), bytes);
        assert!(blob.has_unsettled_obligations());
        blob.confirm_provider_deleted(blob.binding())
            .expect("deletion evidence");
        assert_eq!(blob.physical_bytes(), 0);
        assert_eq!(blob.liability_bytes(), bytes);
        assert!(blob.has_unsettled_obligations());
        blob.confirm_billing_stopped(blob.binding())
            .expect("billing evidence");
        assert!(!blob.has_unsettled_obligations());
        assert_eq!(blob.liability_bytes(), 0);
    }
}

#[test]
fn queued_deletion_cannot_be_cancelled_by_retain_and_repeated_confirmation_is_harmless() {
    let mut blob = object();
    blob.release(key(1)).expect("last release");
    let pending = blob.clone();
    assert_eq!(
        blob.retain(key(2)),
        Err(LifecycleError::DeletionAlreadyQueued)
    );
    assert_eq!(
        blob.confirm_billing_stopped(blob.binding()),
        Err(LifecycleError::DeletionNotConfirmed)
    );
    assert_eq!(blob, pending);
    blob.confirm_provider_deleted(blob.binding())
        .expect("deletion evidence");
    for settled in [false, true] {
        if settled {
            blob.confirm_billing_stopped(blob.binding())
                .expect("billing evidence");
        }
        let before = blob.clone();
        assert_eq!(blob.release(key(1)), Ok(LifecycleChange::Unchanged));
        assert_eq!(
            blob.confirm_provider_deleted(blob.binding()),
            Ok(LifecycleChange::Unchanged)
        );
        assert_eq!(blob.retain(key(1)), Err(LifecycleError::ReferenceReleased));
        assert_eq!(
            blob.retain(key(2)),
            Err(LifecycleError::DeletionAlreadyQueued)
        );
        if settled {
            assert_eq!(
                blob.confirm_billing_stopped(blob.binding()),
                Ok(LifecycleChange::Unchanged)
            );
        }
        assert_eq!(blob, before);
    }
}
