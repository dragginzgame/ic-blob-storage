use super::*;
use crate::operator::model::{Binding, Target};
use blob_test_protocol::funding::preview::FundingPreviewRequest;
use ic_testkit::Fake;

#[test]
fn preview_decode_binds_every_request_field_and_preserves_full_width_reserve_values() {
    let request = FundingPreviewRequest {
        service: Fake::principal(1),
        peer: Fake::principal(2),
        id: u64::MAX,
        requested_cycles: u128::MAX,
        revision: 0,
    };
    let selection = Selection {
        request,
        target: Target {
            server: "127.0.0.1:1".parse().unwrap(),
            instance: 0,
            canister: request.service,
            caller: Fake::principal(3),
            binding: Binding::Funding(request.peer),
        },
    };
    let view = sample_view(request);
    let encode = |view| candid::encode_one(Ok::<_, FundingPreviewFailure>(view)).unwrap();
    let decoded = report(&selection, &encode(view.clone())).unwrap();
    assert!(decoded.blocked);
    assert_eq!(
        decoded.value["blockers"][3]["AmountLimitExceeded"]["maximum_cycles"],
        u128::MAX.to_string()
    );
    assert_eq!(decoded.value["requested_cycles"], u128::MAX.to_string());
    assert_eq!(decoded.value["available_cycles"], u128::MAX.to_string());
    assert_eq!(decoded.value["budget"]["allocated"], u128::MAX.to_string());
    assert_eq!(decoded.value["budget"]["available"], u128::MAX.to_string());
    assert_eq!(decoded.value["budget"]["scope"], "local_attachment_budget");
    assert_eq!(
        decoded.value["liquidity"]["liquid_cycles"],
        u128::MAX.to_string()
    );
    assert_eq!(
        decoded.value["liquidity"]["call_cost"],
        (u128::MAX - 1).to_string()
    );
    assert_eq!(
        decoded.value["blockers"][2]["LiquidityWouldBeViolated"]["transferable_cycles"],
        (u128::MAX - 1).to_string()
    );
    assert_eq!(
        decoded.value["blockers"][1]["BudgetReserveWouldBeViolated"]["transferable_cycles"],
        (u128::MAX - 1).to_string()
    );
    assert_eq!(
        decoded.value["blockers"][0]["ReserveWouldBeViolated"]["transferable_cycles"],
        (u128::MAX - 1).to_string()
    );
    for wrong in [
        FundingPreviewRequest {
            service: request.peer,
            ..request
        },
        FundingPreviewRequest {
            peer: request.service,
            ..request
        },
        FundingPreviewRequest { id: 1, ..request },
        FundingPreviewRequest {
            revision: 1,
            ..request
        },
        FundingPreviewRequest {
            requested_cycles: 1,
            ..request
        },
    ] {
        assert_eq!(
            report(
                &selection,
                &encode(FundingPreviewView {
                    request: wrong,
                    ..view.clone()
                })
            ),
            Err(Failure::Binding)
        );
    }
    assert_eq!(report(&selection, &[0; 4097]), Err(Failure::ReplyTooLarge));
    assert_eq!(report(&selection, b"malformed"), Err(Failure::InvalidReply));
}

fn sample_view(request: FundingPreviewRequest) -> FundingPreviewView {
    FundingPreviewView {
        liquidity: blob_test_protocol::funding::preview::FundingLiquidityView {
            liquid_cycles: u128::MAX,
            call_cost: u128::MAX - 1,
        },
        budget: blob_test_protocol::funding::budget::FundingBudgetView {
            operating_reserve: 1_000_000_000,
            other_liabilities: 0,
            allocated: u128::MAX,
            reserve: 1,
            revision: 0,
            available: u128::MAX,
            accepted: 0,
            refunded: 0,
            not_enqueued: 0,
            reserved_or_uncertain: 0,
        },
        request,
        available_cycles: Some(u128::MAX),
        blockers: vec![
            FundingPreviewBlocker::ReserveWouldBeViolated {
                requested_cycles: u128::MAX,
                transferable_cycles: u128::MAX - 1,
            },
            FundingPreviewBlocker::BudgetReserveWouldBeViolated {
                transferable_cycles: u128::MAX - 1,
            },
            FundingPreviewBlocker::LiquidityWouldBeViolated {
                transferable_cycles: u128::MAX - 1,
            },
            FundingPreviewBlocker::AmountLimitExceeded {
                maximum_cycles: u128::MAX,
            },
        ],
    }
}
