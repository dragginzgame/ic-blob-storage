use super::*;
use blob_test_protocol::{
    funding::{FundingAttemptStatusView, FundingReconciliationView},
    status::FundingActivityView,
};
use candid::Principal;
use ic_testkit::Fake;

fn target() -> Target {
    Target {
        server: "127.0.0.1:1".parse().unwrap(),
        instance: 0,
        canister: Fake::principal(1),
        caller: Principal::anonymous(),
        binding: Binding::Funding(Fake::principal(2)),
    }
}

fn status() -> FundingOperatorStatusView {
    FundingOperatorStatusView {
        service: Fake::principal(1),
        peer: Fake::principal(2),
        fenced: true,
        provider_qualified: false,
        billing_configured: false,
        provider_balance: Some(u128::MAX),
        available_funding_cycles: None,
        budget: blob_test_protocol::funding::budget::FundingBudgetView {
            operating_reserve: 1_000_000_000,
            other_liabilities: 0,
            allocated: u128::MAX,
            reserve: 1,
            revision: 1,
            available: 1,
            accepted: 0,
            refunded: 0,
            not_enqueued: 0,
            reserved_or_uncertain: u128::MAX - 1,
        },
        funding_activity: FundingActivityView::Uncertain,
        attempts: vec![FundingAttemptStatusView {
            id: u64::MAX,
            offered: u128::MAX,
            refunded: None,
            transport_accepted: None,
            outcome: None,
            provider_credit: None,
            reconciliation: FundingReconciliationView::TransferUnknown(u128::MAX),
        }],
        receipts: vec![],
        blockers: vec![],
        warnings: vec![],
    }
}

#[test]
fn bounded_decode_preserves_exact_amounts_and_unknowns() {
    let bytes = candid::encode_one(Some(status())).unwrap();
    let value = report(&target(), &bytes).unwrap().value;
    assert_eq!(value["provider_balance"], u128::MAX.to_string());
    assert_eq!(value["attempts"][0]["id"], u64::MAX.to_string());
    assert_eq!(
        value["attempts"][0]["reconciliation"]["cycles"],
        u128::MAX.to_string()
    );
    assert!(value["attempts"][0]["transport_accepted"].is_null());
    assert!(value["attempts"][0]["provider_credit"].is_null());
    assert!(value["available_funding_cycles"].is_null());
    assert_eq!(
        report(&target(), &bytes[..bytes.len() / 2]),
        Err(Failure::InvalidReply)
    );
    assert_eq!(
        report(&target(), &vec![0; 65_537]),
        Err(Failure::ReplyTooLarge)
    );
    assert_eq!(
        report(
            &target(),
            &candid::encode_one(None::<FundingOperatorStatusView>).unwrap()
        ),
        Err(Failure::Denied)
    );
    let mut changed = status();
    changed.peer = Fake::principal(3);
    assert_eq!(
        report(&target(), &candid::encode_one(Some(changed)).unwrap()),
        Err(Failure::Binding)
    );
    let mut changed = status();
    changed.service = Fake::principal(3);
    assert_eq!(
        report(&target(), &candid::encode_one(Some(changed)).unwrap()),
        Err(Failure::Binding)
    );
}
