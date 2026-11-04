use super::*;
use ic_blob_storage_contracts::binding::ObjectIdentity;
use ic_blob_storage_contracts::binding::ReferenceId;
use ic_blob_storage_contracts::binding::ReferenceKey;
use ic_blob_storage_contracts::upload::history::LifecyclePhase;
use std::num::NonZeroU128;

fn number(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).expect("positive ID")
}
fn bound(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("positive bound")
}
fn actor() -> Principal {
    Principal::from_slice(&[2, 1])
}
fn object() -> ObjectBinding {
    ObjectBinding::new(
        Principal::from_slice(&[1, 1]),
        actor(),
        ObjectIdentity {
            namespace: number(1),
            object: number(1),
            incarnation: number(1),
        },
    )
    .expect("valid binding")
}
fn key(value: u128) -> ReferenceKey {
    ReferenceKey::new(object(), ReferenceId::new(number(value)))
}
fn request(id: u128, operation: ReferenceOperation) -> ReferenceRequest {
    ReferenceRequest {
        id: ReferenceRequestId::new(number(id)),
        operation,
    }
}
fn journal(limit: usize) -> ReferenceRequests {
    ReferenceRequests::new(
        BlobLifecycle::from_confirmed_upload(100, key(1), bound(3)),
        bound(limit),
    )
    .expect("release capacity")
}

#[test]
fn capacity_accounts_for_two_receipts_per_fresh_reference_and_failed_requests() {
    let mut state = journal(4);
    let original = state.clone();
    assert_eq!(
        state.capacity(),
        ReferenceCapacityView {
            reference_slots: 2,
            unreserved_receipts: 3,
            release_reserved_receipts: 1,
            fresh_retains: 1,
        }
    );
    assert_eq!(state, original);
    // Admitted lifecycle failures retain a receipt but allocate no reference.
    let failed = request(1, ReferenceOperation::Release(key(9)));
    assert_eq!(
        state.apply(actor(), failed),
        Ok(ReferenceRequestOutcome::Recorded {
            result: Err(LifecycleError::UnknownReference),
        })
    );
    assert_eq!(state.capacity().unreserved_receipts, 2);
    assert_eq!(state.capacity().fresh_retains, 1);
    state
        .apply(actor(), request(2, ReferenceOperation::Retain(key(2))))
        .unwrap();
    assert_eq!(
        state.capacity(),
        ReferenceCapacityView {
            reference_slots: 1,
            unreserved_receipts: 0,
            release_reserved_receipts: 2,
            fresh_retains: 0,
        }
    );
    let full = state.clone();
    state.apply(actor(), failed).unwrap();
    assert_eq!(state, full);
    state
        .apply(actor(), request(3, ReferenceOperation::Release(key(1))))
        .unwrap();
    state
        .apply(actor(), request(4, ReferenceOperation::Release(key(2))))
        .unwrap();
    assert_eq!(
        state.capacity(),
        ReferenceCapacityView {
            reference_slots: 1,
            unreserved_receipts: 0,
            release_reserved_receipts: 0,
            fresh_retains: 0,
        }
    );
    let mut pending = journal(9);
    pending
        .apply(actor(), request(1, ReferenceOperation::Release(key(1))))
        .unwrap();
    assert_eq!(pending.capacity().unreserved_receipts, 8);
    assert_eq!(pending.capacity().reference_slots, 2);
    assert_eq!(pending.capacity().fresh_retains, 0);
}

#[test]
fn replay_returns_original_success_without_reactivating_released_reference() {
    let mut state = journal(6);
    let retain = request(1, ReferenceOperation::Retain(key(2)));
    assert_eq!(
        state.apply(actor(), retain),
        Ok(ReferenceRequestOutcome::Recorded {
            result: Ok(LifecycleChange::Changed)
        })
    );
    state
        .apply(actor(), request(2, ReferenceOperation::Release(key(2))))
        .expect("release");
    let before = state.clone();
    assert_eq!(
        state.apply(actor(), retain),
        Ok(ReferenceRequestOutcome::Replayed {
            result: Ok(LifecycleChange::Changed)
        })
    );
    assert_eq!(state, before);
    assert_eq!(state.lifecycle().active_references(), 1);
}

#[test]
fn changed_payload_or_actor_cannot_reuse_an_accepted_request_id() {
    let mut state = journal(6);
    let original = request(1, ReferenceOperation::Retain(key(2)));
    state.apply(actor(), original).expect("original request");
    let before = state.clone();
    for operation in [
        ReferenceOperation::Release(key(2)),
        ReferenceOperation::Retain(key(3)),
    ] {
        assert_eq!(
            state.receipt(actor(), request(1, operation)),
            Err(ReferenceRequestError::RequestConflict)
        );
        assert_eq!(
            state.apply(actor(), request(1, operation)),
            Err(ReferenceRequestError::RequestConflict)
        );
        assert_eq!(state, before);
    }
    assert_eq!(
        state.receipt(Principal::from_slice(&[3, 1]), original),
        Err(ReferenceRequestError::RequestConflict)
    );
    assert_eq!(
        state.apply(Principal::from_slice(&[3, 1]), original),
        Err(ReferenceRequestError::RequestConflict)
    );
    let other = ObjectBinding::new(
        object().service(),
        object().tenant(),
        ObjectIdentity {
            incarnation: number(2),
            ..object().identity()
        },
    )
    .expect("other incarnation");
    assert_eq!(
        state.apply(
            actor(),
            request(
                1,
                ReferenceOperation::Retain(ReferenceKey::new(other, key(2).reference()))
            )
        ),
        Err(ReferenceRequestError::BindingMismatch(
            ObjectBindingMismatch::Incarnation
        ))
    );
    assert_eq!(state, before);
}

#[test]
fn a_recorded_failure_is_not_reinterpreted_after_state_changes() {
    let mut state = journal(6);
    let release = request(1, ReferenceOperation::Release(key(2)));
    assert_eq!(
        state.apply(actor(), release),
        Ok(ReferenceRequestOutcome::Recorded {
            result: Err(LifecycleError::UnknownReference)
        })
    );
    state
        .apply(actor(), request(2, ReferenceOperation::Retain(key(2))))
        .expect("later retain");
    let before = state.clone();
    assert_eq!(
        state.receipt(actor(), release),
        Ok(Some(ReferenceReceiptView {
            result: Err(LifecycleError::UnknownReference)
        }))
    );
    assert_eq!(
        state.apply(actor(), release),
        Ok(ReferenceRequestOutcome::Replayed {
            result: Err(LifecycleError::UnknownReference)
        })
    );
    assert_eq!(state, before);
    assert_eq!(state.lifecycle().active_references(), 2);
}

#[test]
fn receipt_pressure_preserves_release_capacity_and_replays_at_full_capacity() {
    let mut state = journal(4);
    let retained = request(1, ReferenceOperation::Retain(key(2)));
    state
        .apply(actor(), retained)
        .expect("retain second reference and reserve its release");
    state
        .apply(actor(), request(2, ReferenceOperation::Retain(key(1))))
        .expect("one unreserved slot");
    let before = state.clone();
    for operation in [
        ReferenceOperation::Retain(key(3)),
        ReferenceOperation::Retain(key(1)),
        ReferenceOperation::Release(key(3)),
    ] {
        assert_eq!(
            state.apply(actor(), request(3, operation)),
            Err(ReferenceRequestError::ReceiptLimitReached)
        );
        assert_eq!(state, before);
    }
    for (id, reference) in [(3, 1), (4, 2)] {
        assert_eq!(
            state.apply(
                actor(),
                request(id, ReferenceOperation::Release(key(reference)))
            ),
            Ok(ReferenceRequestOutcome::Recorded {
                result: Ok(LifecycleChange::Changed)
            })
        );
    }
    assert_eq!(state.receipt_count(), 4);
    assert_eq!(state.lifecycle().phase(), LifecyclePhase::DeletionPending);
    state.confirm_provider_deleted(object()).expect("deletion");
    state.confirm_billing_stopped(object()).expect("billing");
    let settled = state.clone();
    assert_eq!(
        state.receipt(actor(), retained),
        Ok(Some(ReferenceReceiptView {
            result: Ok(LifecycleChange::Changed)
        }))
    );
    assert_eq!(
        state.receipt(actor(), request(5, ReferenceOperation::Release(key(1)))),
        Ok(None)
    );
    assert_eq!(
        state.apply(actor(), retained),
        Ok(ReferenceRequestOutcome::Replayed {
            result: Ok(LifecycleChange::Changed)
        })
    );
    assert_eq!(
        state.apply(actor(), request(5, ReferenceOperation::Release(key(1)))),
        Err(ReferenceRequestError::ReceiptLimitReached)
    );
    assert_eq!(state, settled);
}

#[test]
fn construction_requires_release_slots_for_all_existing_references() {
    let mut value = BlobLifecycle::from_confirmed_upload(100, key(1), bound(3));
    value.retain(key(2)).expect("second reference");
    assert_eq!(
        ReferenceRequests::new(value.clone(), bound(1)),
        Err(ReferenceRequestError::ReceiptLimitReached)
    );
    let mut state = ReferenceRequests::new(value, bound(2)).expect("exact release reservation");
    for reference in [1, 2] {
        assert_eq!(
            state.apply(
                actor(),
                request(reference, ReferenceOperation::Release(key(reference)))
            ),
            Ok(ReferenceRequestOutcome::Recorded {
                result: Ok(LifecycleChange::Changed)
            })
        );
    }
    assert_eq!(state.lifecycle().phase(), LifecyclePhase::DeletionPending);
}
