use super::ReadSlot;

#[test]
fn invalidation_retains_capacity_and_old_callbacks_cannot_free_a_new_read() {
    let mut slot = ReadSlot::new();
    let old = slot.begin().expect("first read");
    slot.invalidate();
    assert!(slot.busy());
    assert_eq!(slot.begin(), None);
    assert!(!slot.finish(old));
    let new = slot.begin().expect("old callback freed its slot");
    assert_ne!(old, new);
    assert!(!slot.finish(old));
    assert!(slot.busy());
    assert!(slot.finish(new));
    assert!(!slot.busy());
}
