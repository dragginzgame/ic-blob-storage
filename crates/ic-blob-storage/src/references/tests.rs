use super::{ObjectReferences, ReferenceError, ReferenceId, ReferenceState, RetentionState};
use std::num::NonZeroUsize;

fn id(value: u128) -> ReferenceId {
    ReferenceId::new(value).unwrap()
}

fn owner(limit: usize) -> ObjectReferences {
    ObjectReferences::new(id(1), NonZeroUsize::new(limit).unwrap())
}

#[test]
fn overlapping_uses_close_only_after_the_final_release() {
    let mut refs = owner(2);
    assert_eq!(refs.reference_state(id(1)), Some(ReferenceState::Live));
    assert_eq!(refs.retain(id(2)), Ok(()));
    assert_eq!(refs.release(id(1)), Ok(()));
    assert_eq!(refs.retention_state(), RetentionState::Open);
    assert_eq!(refs.live_count(), 1);
    assert_eq!(refs.reference_state(id(2)), Some(ReferenceState::Live));

    assert_eq!(refs.release(id(2)), Ok(()));
    assert_eq!(refs.retention_state(), RetentionState::Closing);
    assert_eq!(refs.live_count(), 0);
    assert_eq!(refs.occupied_slots(), 2);
    assert_eq!(refs.retain(id(3)), Err(ReferenceError::Closing));
    assert_eq!(refs.reference_state(id(3)), None);
}

#[test]
fn successful_retain_replays_never_resurrect_released_uses() {
    let mut refs = owner(2);
    refs.retain(id(2)).unwrap();
    refs.release(id(2)).unwrap();

    // Replay while another reference still keeps the object Open.
    assert_eq!(refs.retain(id(2)), Ok(()));
    assert_eq!(refs.reference_state(id(2)), Some(ReferenceState::Released));
    assert_eq!(refs.live_count(), 1);

    refs.release(id(1)).unwrap();
    // The same historical success is available even after closure.
    assert_eq!(refs.retain(id(2)), Ok(()));
    assert_eq!(refs.retain(id(1)), Ok(()));
    assert_eq!(refs.reference_state(id(1)), Some(ReferenceState::Released));
    assert_eq!(refs.reference_state(id(2)), Some(ReferenceState::Released));
    assert_eq!(refs.live_count(), 0);
    assert_eq!(refs.retention_state(), RetentionState::Closing);
}

#[test]
fn exhausted_capacity_preserves_inspection_replay_and_all_admitted_cleanup() {
    let mut refs = owner(2);
    refs.retain(id(2)).unwrap();
    assert_eq!(refs.retain(id(3)), Err(ReferenceError::CapacityExhausted));
    assert_eq!(refs.reference_state(id(3)), None);
    assert_eq!(refs.live_count(), 2);
    assert_eq!(refs.retain(id(2)), Ok(()));

    refs.release(id(2)).unwrap();
    // Released identities still occupy slots; churn cannot recycle capacity.
    assert_eq!(refs.retain(id(3)), Err(ReferenceError::CapacityExhausted));
    assert_eq!(refs.reference_state(id(2)), Some(ReferenceState::Released));
    assert_eq!(refs.occupied_slots(), 2);
    assert_eq!(refs.release(id(1)), Ok(()));
    assert_eq!(refs.live_count(), 0);
}

#[test]
fn duplicate_release_cannot_close_a_siblings_live_reference() {
    let mut refs = owner(2);
    refs.retain(id(2)).unwrap();
    for _ in 0..3 {
        assert_eq!(refs.release(id(1)), Ok(()));
        assert_eq!(refs.live_count(), 1);
        assert_eq!(refs.reference_state(id(2)), Some(ReferenceState::Live));
        assert_eq!(refs.retention_state(), RetentionState::Open);
    }
    refs.release(id(2)).unwrap();
    assert_eq!(refs.release(id(2)), Ok(()));
    assert_eq!(refs.live_count(), 0);
}

#[test]
fn unknown_release_is_observational_and_does_not_reserve_an_identity() {
    let mut refs = owner(2);
    assert_eq!(refs.release(id(2)), Err(ReferenceError::UnknownReference));
    assert_eq!(refs.live_count(), 1);
    assert_eq!(refs.occupied_slots(), 1);
    assert_eq!(refs.reference_state(id(2)), None);
    refs.retain(id(2)).unwrap();
    assert_eq!(refs.reference_state(id(2)), Some(ReferenceState::Live));
    refs.release(id(1)).unwrap();
    refs.release(id(2)).unwrap();
    assert_eq!(refs.release(id(3)), Err(ReferenceError::UnknownReference));
    assert_eq!(refs.retention_state(), RetentionState::Closing);
    assert_eq!(refs.occupied_slots(), 2);
}

#[test]
fn a_single_slot_still_allows_final_release_and_replay() {
    let mut refs = owner(1);
    assert_eq!(refs.retain(id(1)), Ok(()));
    assert_eq!(refs.retain(id(2)), Err(ReferenceError::CapacityExhausted));
    refs.release(id(1)).unwrap();
    assert_eq!(refs.release(id(1)), Ok(()));
    assert_eq!(refs.retain(id(1)), Ok(()));
    assert_eq!(refs.retention_state(), RetentionState::Closing);
    assert_eq!(refs.occupied_slots(), 1);
}

#[test]
fn full_width_reference_ids_remain_distinct() {
    let mut refs = owner(3);
    refs.retain(id(u128::MAX)).unwrap();
    refs.retain(id(u128::from(u64::MAX) + 1)).unwrap();
    refs.release(id(u128::MAX)).unwrap();
    assert_eq!(
        refs.reference_state(id(u128::MAX)),
        Some(ReferenceState::Released)
    );
    assert_eq!(
        refs.reference_state(id(u128::from(u64::MAX) + 1)),
        Some(ReferenceState::Live)
    );
    assert_eq!(refs.live_count(), 2);
}
