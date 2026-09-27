use super::*;
use blob_test_protocol::funding::{FundingReconciliationView, FundingReplyMode};

type JournalMutation = fn(&mut FundingJournalRecord);

#[test]
fn exact_lookup_checks_authority_bindings_and_every_immutable_input() {
    use blob_test_protocol::funding::lookup::{
        FundingLookupFailure as Failure, FundingLookupRequest,
    };
    let mut journal = journal();
    let query = FundingLookupRequest {
        service: principal(1),
        peer: principal(2),
        attempt: request(1),
    };
    let original = journal.lookup(principal(3), query).unwrap().unwrap();
    assert_eq!(original.request, query.attempt);
    assert_eq!(journal.lookup(principal(2), query), Err(Failure::Denied));
    assert_eq!(
        journal.lookup(
            principal(3),
            FundingLookupRequest {
                service: principal(4),
                ..query
            }
        ),
        Err(Failure::Binding)
    );
    assert_eq!(
        journal.lookup(
            principal(3),
            FundingLookupRequest {
                peer: principal(4),
                ..query
            }
        ),
        Err(Failure::Binding)
    );
    for attempt in [
        FundingRequest {
            offered: 101,
            ..query.attempt
        },
        FundingRequest {
            accept: 39,
            ..query.attempt
        },
        FundingRequest {
            reply: FundingReplyMode::InternalError,
            ..query.attempt
        },
        FundingRequest {
            trap_callback: true,
            ..query.attempt
        },
    ] {
        assert_eq!(
            journal.lookup(principal(3), FundingLookupRequest { attempt, ..query }),
            Err(Failure::Conflict)
        );
    }
    assert_eq!(
        journal.lookup(
            principal(3),
            FundingLookupRequest {
                attempt: request(2),
                ..query
            }
        ),
        Ok(None)
    );
    assert_eq!(
        journal.lookup(
            principal(3),
            FundingLookupRequest {
                attempt: FundingRequest {
                    offered: 0,
                    ..query.attempt
                },
                ..query
            }
        ),
        Err(Failure::InvalidRequest)
    );
    journal.fence();
    assert_eq!(journal.lookup(principal(3), query), Ok(Some(original)));
}

fn principal(value: u8) -> Principal {
    Principal::from_slice(&[value, 1])
}

fn request(id: u64) -> FundingRequest {
    FundingRequest {
        id,
        offered: 100,
        accept: 40,
        reply: FundingReplyMode::Success,
        trap_callback: false,
    }
}

fn journal() -> FundingJournalRecord {
    let mut journal = FundingJournalRecord::new(
        principal(1),
        principal(2),
        principal(3),
        budget::FundingBudgetRecord::new(1000, 100, 1, 0).unwrap(),
    );
    journal.admit(principal(3), request(1)).unwrap();
    journal.complete(
        request(1),
        FundingObservation {
            refunded: Some(60),
            transport_accepted: Some(40),
            outcome: FundingOutcome::ReportedSuccess,
            reconciliation: FundingReconciliationView::CreditRequired(40),
        },
    );
    journal.record_acceptance(FundingReceiptRecord {
        id: 1,
        available: 100,
        accepted: 40,
    });
    journal
}

#[test]
fn journal_requires_bound_release_unique_identities_and_conserved_transfers() {
    assert_eq!(journal().validate(principal(1)), Ok(()));
    assert_eq!(
        journal().validate(principal(2)),
        Err(JournalFailure::Binding)
    );
    let cases: &[(JournalMutation, JournalFailure)] = &[
        (|j| j.release[0] ^= 1, JournalFailure::Release),
        (|j| j.peer = j.service, JournalFailure::Binding),
        (
            |j| j.driver = Principal::anonymous(),
            JournalFailure::Binding,
        ),
        (
            |j| j.attempts[0].request.offered = 0,
            JournalFailure::Request,
        ),
        (
            |j| j.attempts[0].request.accept = 101,
            JournalFailure::Request,
        ),
        (
            |j| j.attempts.push(j.attempts[0]),
            JournalFailure::Duplicate,
        ),
        (
            |j| j.receipts.push(j.receipts[0]),
            JournalFailure::Duplicate,
        ),
        (|j| j.receipts[0].accepted = 101, JournalFailure::Receipt),
        (|j| j.receipts[0].available = 0, JournalFailure::Receipt),
        (
            |j| j.receipts.resize(MAX_ATTEMPTS + 1, j.receipts[0]),
            JournalFailure::Capacity,
        ),
        (
            |j| j.attempts[0].observation.as_mut().unwrap().refunded = Some(101),
            JournalFailure::Transfer,
        ),
        (
            |j| {
                j.attempts[0]
                    .observation
                    .as_mut()
                    .unwrap()
                    .transport_accepted = Some(0);
            },
            JournalFailure::Transfer,
        ),
        (
            |j| j.attempts[0].observation.as_mut().unwrap().refunded = None,
            JournalFailure::Transfer,
        ),
        (
            |j| j.attempts[0].request.trap_callback = true,
            JournalFailure::Callback,
        ),
    ];
    for (alter, expected) in cases {
        let mut record = journal();
        alter(&mut record);
        assert_eq!(record.validate(principal(1)), Err(*expected));
    }
}

#[test]
fn unknown_attempt_must_be_last_and_fence_never_discards_it() {
    let mut journal = journal();
    journal.admit(principal(3), request(2)).unwrap();
    assert_eq!(journal.validate(principal(1)), Ok(()));
    let before = journal.attempts(principal(3));
    journal.fence();
    assert_eq!(journal.validate(principal(1)), Ok(()));
    assert_eq!(
        journal.admit(principal(3), request(1)),
        Err(FundingFailure::Fenced)
    );
    assert_eq!(
        journal.admit(principal(3), request(3)),
        Err(FundingFailure::Fenced)
    );
    assert_eq!(
        journal.admit(principal(2), request(3)),
        Err(FundingFailure::Denied)
    );
    assert_eq!(journal.attempts(principal(3)), before);
    journal.attempts.swap(0, 1);
    assert_eq!(journal.validate(principal(1)), Err(JournalFailure::Pending));
}

#[test]
fn callback_control_allows_only_unsent_terminal_records_to_restore() {
    for outcome in [
        FundingOutcome::NotEnqueued,
        FundingOutcome::LiquidityBlocked,
    ] {
        let mut journal = journal();
        let attempt = FundingRequest {
            trap_callback: true,
            ..request(2)
        };
        journal.admit(principal(3), attempt).unwrap();
        journal.complete(
            attempt,
            FundingObservation {
                refunded: None,
                transport_accepted: Some(0),
                outcome,
                reconciliation: FundingReconciliationView::NoTransfer,
            },
        );
        assert_eq!(journal.validate(principal(1)), Ok(()));
        let retained = journal.budget();
        journal.fence();
        assert_eq!(journal.validate(principal(1)), Ok(()));
        assert_eq!(journal.budget(), retained);
        assert_eq!(
            journal.admit(principal(3), attempt),
            Err(FundingFailure::Fenced)
        );
    }
}
