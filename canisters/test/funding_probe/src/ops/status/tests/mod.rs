use super::*;
use blob_test_protocol::funding::{
    FundingObservation, FundingOutcome, FundingReplyMode, FundingRequest,
};

#[test]
fn inconsistent_transport_facts_stay_unknown() {
    let mut entry = FundingAttemptRecord {
        request: FundingRequest {
            id: 1,
            offered: 100,
            accept: 50,
            reply: FundingReplyMode::Success,
            trap_callback: false,
        },
        observation: None,
    };
    let unknown = FundingTransfer::unknown(NonZeroU128::new(100).unwrap());
    assert_eq!(transfer(&entry), unknown);
    for (refunded, accepted, outcome) in [
        (Some(101), Some(0), FundingOutcome::ReportedSuccess),
        (Some(50), Some(0), FundingOutcome::ReportedSuccess),
        (None, Some(0), FundingOutcome::Rejected(5)),
        (None, Some(50), FundingOutcome::NotEnqueued),
        (Some(100), Some(0), FundingOutcome::NotEnqueued),
    ] {
        entry.observation = Some(FundingObservation {
            refunded,
            transport_accepted: accepted,
            outcome,
            // A persisted derived label cannot override contradictory facts.
            reconciliation: FundingReconciliationView::NoTransfer,
        });
        assert_eq!(transfer(&entry), unknown);
    }
    // Also ignore a contradictory derived label in favor of exact raw facts.
    entry.observation = Some(FundingObservation {
        refunded: Some(50),
        transport_accepted: Some(50),
        outcome: FundingOutcome::ReportedSuccess,
        reconciliation: FundingReconciliationView::NoTransfer,
    });
    assert_eq!(transfer(&entry).accepted(), Some(50));
}
