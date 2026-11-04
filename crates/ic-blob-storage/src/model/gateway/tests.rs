use super::*;
use std::num::NonZeroUsize;

fn p(id: u8) -> Principal {
    Principal::from_slice(&[id, 1])
}

fn limits(entries: usize, unique: usize) -> GatewayListLimits {
    GatewayListLimits {
        max_entries: NonZeroUsize::new(entries).expect("positive bound"),
        max_unique: NonZeroUsize::new(unique).expect("positive bound"),
    }
}

#[test]
fn deduplication_preserves_first_occurrence_order_at_both_bounds() {
    let input = [p(3), p(1), p(3), p(2), p(1)];
    let list = GatewayList::new(&input, limits(5, 3)).expect("bounded list");
    assert_eq!(list.principals(), &[p(3), p(1), p(2)]);
    for principal in input {
        assert!(list.contains(principal));
    }
    assert!(!list.contains(p(4)));
    assert_eq!(list.limits(), limits(5, 3));
}

#[test]
fn empty_and_invalid_principals_reject_even_after_valid_prefix() {
    assert_eq!(
        GatewayList::new(&[], limits(4, 4)),
        Err(GatewayListError::Empty)
    );
    for principal in [Principal::anonymous(), Principal::management_canister()] {
        for input in [vec![principal], vec![p(1), p(1), principal]] {
            assert_eq!(
                GatewayList::new(&input, limits(4, 4)),
                Err(GatewayListError::InvalidPrincipal {
                    index: input.len() - 1,
                    principal,
                })
            );
        }
    }
}

#[test]
fn duplicate_flood_and_unique_growth_have_independent_limits() {
    assert_eq!(
        GatewayList::new(&[p(1); 5], limits(4, 2)),
        Err(GatewayListError::TooManyEntries {
            actual: 5,
            maximum: 4,
        })
    );
    assert_eq!(
        GatewayList::new(&[p(1), p(1), p(2), p(3)], limits(4, 2)),
        Err(GatewayListError::TooManyUnique { maximum: 2 })
    );
    let repeated =
        GatewayList::new(&[p(1); 4], limits(4, 1)).expect("duplicates within input bound");
    assert_eq!(repeated.principals(), &[p(1)]);
    // The raw limit rejects before principal inspection.
    assert_eq!(
        GatewayList::new(&[Principal::anonymous(); 5], limits(4, 2)),
        Err(GatewayListError::TooManyEntries {
            actual: 5,
            maximum: 4,
        })
    );
}

#[test]
fn failed_replacement_preserves_membership_order_and_limits_then_recovers() {
    let mut list = GatewayList::new(&[p(2), p(1)], limits(4, 2)).expect("initial list");
    let original = list.clone();
    for (input, error) in [
        (vec![], GatewayListError::Empty),
        (
            vec![p(3); 5],
            GatewayListError::TooManyEntries {
                actual: 5,
                maximum: 4,
            },
        ),
        (
            vec![p(3), p(4), p(5)],
            GatewayListError::TooManyUnique { maximum: 2 },
        ),
        (
            vec![p(3), p(4), Principal::anonymous()],
            GatewayListError::InvalidPrincipal {
                index: 2,
                principal: Principal::anonymous(),
            },
        ),
    ] {
        assert_eq!(list.replace(&input), Err(error));
        assert_eq!(list, original);
    }
    list.replace(&[p(3), p(1), p(3)])
        .expect("valid replacement");
    assert_eq!(list.principals(), &[p(3), p(1)]);
    assert!(!list.contains(p(2)));
    assert_eq!(list.limits(), original.limits());
    let replaced = list.clone();
    list.replace(&[p(3), p(1), p(3)])
        .expect("repeat replacement");
    assert_eq!(list, replaced);
}
