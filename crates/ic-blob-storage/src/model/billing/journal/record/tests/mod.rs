//! Stored progress keeps terminal evidence correlated without losing uncertain effects.
use super::*;
use ic_platform::Principal;
use ic_storage::adapter::Storable;
use std::borrow::Cow;

fn intent() -> FundingIntent {
    FundingIntent {
        service: Principal::from_slice(&[1]),
        cashier: Principal::from_slice(&[2]),
        account: Principal::from_slice(&[3]),
        namespace: NonZeroU128::MIN,
        operation: NonZeroU128::MIN,
        offered: NonZeroU128::new(900).unwrap(),
        target_balance: None,
    }
}
#[test]
fn retired_separate_phase_response_frames_refuse_without_a_fallback_reader() {
    // Captured from the prior BLFUNIN1 writer: prepared and callback/refund 500,
    // both with missing diagnostics, using the exact intent above. No re-encoding.
    for bytes in [
        &[
            73, 87, 66, 76, 70, 85, 78, 73, 78, 49, 0, 1, 1, 1, 1, 2, 1, 3, 1, 1, 132, 7, 0, 0, 0,
            0, 0, 0,
        ][..],
        &[
            73, 87, 66, 76, 70, 85, 78, 73, 78, 49, 0, 1, 1, 1, 1, 2, 1, 3, 1, 1, 132, 7, 0, 0, 3,
            244, 3, 0, 0, 0,
        ][..],
    ] {
        assert!(FundingIntentRecord::from_bytes(Cow::Borrowed(bytes)).is_err());
    }
    let current = FundingIntentRecord::new(intent())
        .attempted()
        .unwrap()
        .complete(FundingProgressRecord::Callback {
            refunded: 500,
            response: None,
        })
        .unwrap();
    assert_eq!(
        FundingIntentRecord::from_bytes(current.to_bytes().unwrap()).unwrap(),
        current
    );
}
#[test]
fn terminal_replay_can_enrich_evidence_but_never_erase_or_rewrite_it() {
    let observed = FundingProgressRecord::Callback {
        refunded: 500,
        response: Some(FundingResponseRecord::Rejected(4)),
    };
    let unknown = FundingProgressRecord::Callback {
        refunded: 500,
        response: None,
    };
    let row = FundingIntentRecord::new(intent());
    assert_eq!(row.complete(unknown), Err(FundingIntentError::NotAttempted));
    let row = row.attempted().unwrap().complete(unknown).unwrap();
    let observed_row = row.complete(observed).unwrap();
    assert_eq!(observed_row.complete(unknown), Ok(observed_row));
    assert_eq!(observed_row.complete(observed), Ok(observed_row));
    for conflict in [
        FundingProgressRecord::Callback {
            refunded: 499,
            response: None,
        },
        FundingProgressRecord::Callback {
            refunded: 500,
            response: Some(FundingResponseRecord::InternalError),
        },
        FundingProgressRecord::NotEnqueued(None),
    ] {
        assert_eq!(
            observed_row.complete(conflict),
            Err(FundingIntentError::OutcomeConflict)
        );
    }
    let unsent =
        FundingProgressRecord::NotEnqueued(Some(progress::UnsentResponseRecord::NotDispatched));
    let row = FundingIntentRecord::new(intent())
        .attempted()
        .unwrap()
        .complete(unsent)
        .unwrap();
    assert_eq!(
        row.complete(FundingProgressRecord::NotEnqueued(None)),
        Ok(row)
    );
    assert_eq!(
        row.complete(FundingProgressRecord::NotEnqueued(Some(
            progress::UnsentResponseRecord::NotEnqueued
        ))),
        Err(FundingIntentError::OutcomeConflict)
    );
    assert_eq!(
        row.complete(unknown),
        Err(FundingIntentError::OutcomeConflict)
    );
}
#[test]
fn restoration_still_refuses_over_refunds_and_credit_without_a_callback() {
    let mut row = FundingIntentRecord::new(intent());
    row.progress = FundingProgressRecord::Callback {
        refunded: 901,
        response: Some(FundingResponseRecord::InternalError),
    };
    let restored = FundingIntentRecord::from_bytes(row.to_bytes().unwrap()).unwrap();
    assert!(restored.view().is_none());
    row.progress = FundingProgressRecord::Uncertain;
    row.credit = FundingCreditRecord::Confirmed {
        accepted: 400,
        receipt_digest: [1; 32],
    };
    let restored = FundingIntentRecord::from_bytes(row.to_bytes().unwrap()).unwrap();
    assert!(restored.view().is_none());
}
