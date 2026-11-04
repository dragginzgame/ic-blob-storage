use std::num::NonZeroUsize;

use super::*;

fn p(id: u8) -> Principal {
    Principal::from_slice(&[id, 1])
}

fn membership() -> GatewayMembership {
    GatewayMembership::new(GatewayListLimits {
        max_entries: NonZeroUsize::new(4).expect("positive limit"),
        max_unique: NonZeroUsize::new(2).expect("positive limit"),
    })
}

#[test]
fn repeated_add_at_capacity_is_idempotent_and_rejection_preserves_state() {
    let mut members = membership();
    assert_eq!(members.principals(), []);
    assert_eq!(members.add(p(2)), Ok(GatewayAddOutcome::Added));
    assert_eq!(members.add(p(1)), Ok(GatewayAddOutcome::Added));
    let full = members.clone();
    assert_eq!(members.add(p(2)), Ok(GatewayAddOutcome::AlreadyPresent));
    assert_eq!(members, full);
    assert_eq!(
        members.add(p(3)),
        Err(GatewayListError::TooManyUnique { maximum: 2 })
    );
    assert_eq!(members, full);
    for principal in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            members.add(principal),
            Err(GatewayListError::InvalidPrincipal {
                index: 0,
                principal
            })
        );
        assert_eq!(members, full);
    }
}

#[test]
fn removals_release_capacity_and_can_remove_the_final_member() {
    let mut members = membership();
    members.add(p(1)).expect("first member");
    members.add(p(2)).expect("second member");
    assert!(!members.remove(p(3)));
    assert!(members.remove(p(1)));
    assert!(!members.remove(p(1)));
    assert!(!members.contains(p(1)));
    assert_eq!(members.add(p(3)), Ok(GatewayAddOutcome::Added));
    assert_eq!(members.principals(), &[p(2), p(3)]);
    assert!(members.remove(p(2)));
    assert!(members.remove(p(3)));
    assert_eq!(members.principals(), []);
    assert_eq!(members.replace_from_sync(&[]), Err(GatewayListError::Empty));
    members.add(p(1)).expect("explicit re-add after removal");
    assert_eq!(members.principals(), &[p(1)]);
}

#[test]
fn complete_sync_replaces_members_and_failed_sync_leaves_them_intact() {
    let mut members = membership();
    members.add(p(1)).expect("first member");
    members.add(p(2)).expect("second member");
    members
        .replace_from_sync(&[p(3), p(2), p(3)])
        .expect("valid sync input");
    assert_eq!(members.principals(), &[p(3), p(2)]);
    assert!(!members.contains(p(1)));
    let synced = members.clone();
    for (input, error) in [
        (vec![], GatewayListError::Empty),
        (
            vec![p(1); 5],
            GatewayListError::TooManyEntries {
                actual: 5,
                maximum: 4,
            },
        ),
        (
            vec![p(1), p(2), p(3)],
            GatewayListError::TooManyUnique { maximum: 2 },
        ),
        (
            vec![p(1), Principal::anonymous()],
            GatewayListError::InvalidPrincipal {
                index: 1,
                principal: Principal::anonymous(),
            },
        ),
    ] {
        assert_eq!(members.replace_from_sync(&input), Err(error));
        assert_eq!(members, synced);
    }
    members
        .replace_from_sync(&[p(3), p(2), p(3)])
        .expect("repeat sync input");
    assert_eq!(members, synced);
    members
        .replace_from_sync(&[p(1)])
        .expect("successful subsequent sync input");
    assert_eq!(members.principals(), &[p(1)]);
    assert_eq!(members.limits(), synced.limits());
}
