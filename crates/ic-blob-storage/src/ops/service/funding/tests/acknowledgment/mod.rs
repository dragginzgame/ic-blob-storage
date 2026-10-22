//! Provider acknowledgments are operational success, separate from account credit.
use super::credit::host;
use super::*;
use crate::model::billing::journal::record::response::FundingResponseRecord as R;
use crate::model::billing::journal::renewal::FundingBudgetRenewal;
use crate::policy::billing::admission::FundingActivity;
use crate::policy::billing::reconciliation::assess_uncredited_allocation;
use crate::workflow::funding::{FundingPreparationResult, prepare_new};

#[test]
fn two_acknowledged_payments_preserve_spending_exact_replay_renewal_and_restore() {
    let memory = memories();
    let allocation = allocation().with_renewal_ceiling(1200).unwrap();
    let mut store = StableFundingJournal::install(copy(&memory), config(), allocation).unwrap();
    for operation in 1..=2 {
        let intent = input(operation, 200);
        assert!(matches!(
            prepare_new(&mut store, execution(), intent, host(intent)).unwrap(),
            FundingPreparationResult::Prepared(_)
        ));
        store.mark_attempted(execution(), intent).unwrap();
        // A known callback alone is insufficient; only its matching successful response qualifies.
        store
            .record_transport(
                execution(),
                intent,
                source(),
                FundingTransportOutcome::Callback { refunded: 50 },
            )
            .unwrap();
        assert_eq!(store.allocation(execution()).unwrap().unresolved(), 150);
        assert!(
            store
                .complete_transport(
                    execution(),
                    intent,
                    source(),
                    FundingTransportOutcome::Callback { refunded: 50 },
                    Some(R::Balance {
                        total: 1,
                        prepaid: 1,
                        promotional: 0,
                        ledger: 0
                    })
                )
                .unwrap()
        );
        let totals = store.allocation(execution()).unwrap();
        assert_eq!(totals.accepted(), operation * 150);
        assert_eq!(totals.acknowledged(), operation * 150);
        assert_eq!(totals.credit_confirmed(), 0);
        assert_eq!(totals.uncredited(), operation * 150);
        assert_eq!(totals.unresolved(), 0);
        assert_eq!(totals.available(), 1000 - operation * 150);
        assert_eq!(assess_uncredited_allocation(totals), FundingActivity::Clear);
        assert!(
            !store
                .complete_transport(
                    execution(),
                    intent,
                    source(),
                    FundingTransportOutcome::Callback { refunded: 50 },
                    Some(R::Balance {
                        total: 1,
                        prepaid: 1,
                        promotional: 0,
                        ledger: 0
                    })
                )
                .unwrap()
        );
        assert_eq!(store.allocation(execution()).unwrap(), totals);
        assert_eq!(
            store.record_credit(execution(), credit::receipt(intent, 150, 1)),
            Err(
                crate::model::billing::journal::credit::FundingCreditError::ProviderAcknowledged
                    .into()
            )
        );
    }
    let grant = FundingBudgetRenewal {
        intent: input(2, 200),
        additional: n(150),
    };
    assert!(store.renew_budget(execution(), grant).unwrap());
    assert!(!store.renew_budget(execution(), grant).unwrap());
    let totals = store.allocation(execution()).unwrap();
    assert_eq!(totals.allocated(), 1150);
    assert_eq!(totals.accepted(), 300);
    assert_eq!(totals.available(), 850);
    drop(store);
    let reopened = StableFundingJournal::open(copy(&memory), config(), allocation).unwrap();
    assert_eq!(reopened.allocation(execution()).unwrap(), totals);
    assert_eq!(
        reopened
            .outcome(execution(), input(1, 200))
            .unwrap()
            .unwrap()
            .acknowledged,
        Some(n(150))
    );
    assert!(reopened.fenced);
}

#[test]
fn errors_missing_and_malformed_replies_do_not_acknowledge_or_enable_more_funding() {
    for response in [
        R::Missing,
        R::Rejected(4),
        R::InternalError,
        R::InvalidReply,
        R::ReplyTooLarge,
        R::AccountBalanceOverflow,
        R::TopUpWithoutCycles,
    ] {
        let mut store =
            StableFundingJournal::install(copy(&memories()), config(), allocation()).unwrap();
        let first = input(1, 200);
        store.prepare(execution(), first).unwrap();
        store.mark_attempted(execution(), first).unwrap();
        assert!(matches!(
            prepare_new(&mut store, execution(), input(2, 1), host(input(2, 1))).unwrap(),
            FundingPreparationResult::Blocked(_)
        ));
        store
            .complete_transport(
                execution(),
                first,
                source(),
                FundingTransportOutcome::Callback { refunded: 50 },
                Some(response),
            )
            .unwrap();
        let totals = store.allocation(execution()).unwrap();
        assert_eq!(
            (
                totals.accepted(),
                totals.acknowledged(),
                totals.unresolved()
            ),
            (150, 0, 150)
        );
        assert!(matches!(
            prepare_new(&mut store, execution(), input(2, 1), host(input(2, 1))).unwrap(),
            FundingPreparationResult::Blocked(_)
        ));
    }
}
