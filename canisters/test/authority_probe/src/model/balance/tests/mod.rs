use super::*;

fn scope() -> ScopeRecord {
    ScopeRecord::new(
        Principal::from_slice(&[1]),
        1,
        Principal::from_slice(&[2]),
        Principal::from_slice(&[3]),
    )
    .unwrap()
}

#[test]
fn exact_pending_identity_and_revision_prevent_rebinding_and_reuse() {
    let scope = scope();
    let mut journal = BalanceJournalRecord::new();
    journal.configure(scope).unwrap();
    let (id, _) = journal.begin(10).unwrap();
    assert_eq!(journal.begin(11), Err(Failure::Busy));
    let mut foreign = scope;
    foreign.account = Principal::from_slice(&[4]);
    assert_eq!(
        journal.complete(id, foreign, Ok([1; 4]), 12),
        Err(Failure::Stale)
    );
    assert!(journal.busy());
    journal.configure(scope).unwrap(); // Even reinstalling the same binding invalidates the reply.
    assert_eq!(journal.usability(false, 12), BalanceUsability::Invalidated);
    assert_eq!(journal.begin(12), Err(Failure::Busy));
    assert_eq!(
        journal.complete(id, scope, Ok([1; 4]), 13),
        Err(Failure::Stale)
    );
    assert!(!journal.busy());
    assert!(journal.valid(scope.service, 1));
    assert_eq!(
        journal.complete(id, scope, Ok([1; 4]), 14),
        Err(Failure::Stale)
    );
}

#[test]
fn local_age_uses_dispatch_time_and_never_clears_a_fence() {
    let scope = scope();
    let mut journal = BalanceJournalRecord::new();
    journal.configure(scope).unwrap();
    let (id, _) = journal.begin(10).unwrap();
    journal
        .complete(id, scope, Ok([u128::MAX, 1, 2, 3]), MAX_AGE_NS)
        .unwrap();
    assert_eq!(journal.total(false, MAX_AGE_NS + 10), Some(u128::MAX));
    assert_eq!(
        journal.usability(false, MAX_AGE_NS + 11),
        BalanceUsability::Expired
    );
    assert_eq!(journal.usability(false, 9), BalanceUsability::Expired);
    assert_eq!(journal.usability(false, 11), BalanceUsability::Expired);
    assert_eq!(journal.total(true, MAX_AGE_NS), None);
    assert_eq!(
        journal.usability(true, MAX_AGE_NS),
        BalanceUsability::Fenced
    );
    journal.attempts[0].outcome = Some(OutcomeRecord::Reported {
        amounts: [0; 4],
        received_at: 9,
    });
    assert!(!journal.valid(scope.service, 1));
}

#[test]
fn retained_history_is_bounded_and_invalid_records_reject() {
    let scope = scope();
    let mut journal = BalanceJournalRecord::new();
    journal.configure(scope).unwrap();
    for _ in 0..MAX_ATTEMPTS {
        let (id, _) = journal.begin(10).unwrap();
        assert_eq!(
            journal.complete(id, scope, Err(Failure::Transport), 11),
            Err(Failure::Transport)
        );
    }
    assert_eq!(journal.begin(12), Err(Failure::Limit));
    assert!(journal.valid(scope.service, 1));
    let mut invalid = journal.clone();
    invalid.attempts[0].outcome = None;
    assert!(!invalid.valid(scope.service, 1));
    invalid = journal.clone();
    invalid.attempts[0].scope.namespace = 2;
    assert!(!invalid.valid(scope.service, 1));
    invalid = journal.clone();
    invalid.attempts[0].outcome = Some(OutcomeRecord::Failed(Failure::Denied));
    assert!(!invalid.valid(scope.service, 1));
    // Past configuration revisions must still name one immutable scope.
    journal.configure(scope).unwrap();
    invalid = journal.clone();
    invalid.attempts[0].scope.account = Principal::from_slice(&[4]);
    assert!(!invalid.valid(scope.service, 1));
    journal.revision = u64::MAX;
    assert_eq!(journal.configure(scope), Err(Failure::Limit));
}

#[test]
fn limits_remain_bound_to_their_original_revision_and_reject_invalid_restore() {
    use ic_blob_storage::model::billing::FundingLimits;
    let scope = scope();
    let mut journal = BalanceJournalRecord::new();
    journal.configure(scope).unwrap();
    let limits = FundingLimits::new(100, 10, 50).unwrap();
    journal.configure_limits(scope, 1, limits).unwrap();
    assert_eq!(journal.current_limits(), Some(limits));
    journal.configure(scope).unwrap();
    assert_eq!(journal.current_limits(), None);
    assert!(journal.billing.configured.is_some());
    assert!(journal.valid(scope.service, 1));
    assert_eq!(
        journal.configure_limits(scope, 1, limits),
        Err(Failure::Binding)
    );
    journal.configure_limits(scope, 2, limits).unwrap();
    journal.billing.configured.as_mut().unwrap().target = 1;
    assert!(!journal.valid(scope.service, 1));
}
