use super::*;
use blob_test_protocol::funding::{FundingReconciliationView, FundingReplyMode};

type JournalMutation = fn(&mut FundingJournalRecord);

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
    let mut journal = FundingJournalRecord::new(principal(1), principal(2), principal(3));
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
