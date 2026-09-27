use super::*;
use crate::operator::model::{Binding, Target};
use blob_test_protocol::funding::{
    FundingObservation, FundingOutcome, FundingReconciliationView, FundingReplyMode,
    FundingRequest, lookup::FundingLookupRequest,
};
use ic_testkit::Fake;

fn selection() -> Selection {
    let service = Fake::principal(1);
    let peer = Fake::principal(2);
    Selection {
        target: Target {
            server: "127.0.0.1:1".parse().unwrap(),
            instance: 0,
            canister: service,
            caller: Fake::principal(3),
            binding: Binding::Funding(peer),
        },
        request: FundingLookupRequest {
            service,
            peer,
            attempt: FundingRequest {
                id: u64::MAX,
                offered: u128::MAX,
                accept: u128::MAX,
                reply: FundingReplyMode::Success,
                trap_callback: false,
            },
        },
    }
}
fn encode(view: FundingLookupView) -> Vec<u8> {
    candid::encode_one(Ok::<_, FundingLookupFailure>(view)).unwrap()
}

#[test]
fn lookup_preserves_exact_width_unknowns_and_all_request_bindings() {
    let selection = selection();
    let view = FundingLookupView {
        request: selection.request,
        fenced: true,
        state: FundingLookupState::Observed(FundingObservation {
            refunded: Some(0),
            transport_accepted: Some(u128::MAX),
            outcome: FundingOutcome::ReportedSuccess,
            reconciliation: FundingReconciliationView::CreditRequired(u128::MAX),
        }),
    };
    let result = report(&selection, &encode(view)).unwrap();
    assert!(!result.blocked);
    assert_eq!(result.value["request"]["id"], u64::MAX.to_string());
    assert_eq!(
        result.value["observation"]["transport_accepted"],
        u128::MAX.to_string()
    );
    assert_eq!(
        result.value["observation"]["reconciliation"]["cycles"],
        u128::MAX.to_string()
    );
    assert_eq!(result.value["fenced"], true);
    for state in [FundingLookupState::Absent, FundingLookupState::Pending] {
        let result = report(&selection, &encode(FundingLookupView { state, ..view })).unwrap();
        assert!(result.blocked);
        assert!(result.value["observation"].is_null());
    }
    let q = selection.request;
    let r = q.attempt;
    for request in [
        FundingLookupRequest {
            service: q.peer,
            ..q
        },
        FundingLookupRequest {
            peer: q.service,
            ..q
        },
        FundingLookupRequest {
            attempt: FundingRequest { id: 0, ..r },
            ..q
        },
        FundingLookupRequest {
            attempt: FundingRequest { offered: 1, ..r },
            ..q
        },
        FundingLookupRequest {
            attempt: FundingRequest { accept: 1, ..r },
            ..q
        },
        FundingLookupRequest {
            attempt: FundingRequest {
                reply: FundingReplyMode::Trap,
                ..r
            },
            ..q
        },
        FundingLookupRequest {
            attempt: FundingRequest {
                trap_callback: true,
                ..r
            },
            ..q
        },
    ] {
        assert_eq!(
            report(&selection, &encode(FundingLookupView { request, ..view })),
            Err(Failure::Binding)
        );
    }
}

#[test]
fn failed_or_invalid_lookup_is_never_an_absent_operation() {
    let selection = selection();
    for (failure, expected) in [
        (FundingLookupFailure::Denied, Failure::Denied),
        (FundingLookupFailure::Binding, Failure::Binding),
        (FundingLookupFailure::Conflict, Failure::Conflict),
        (
            FundingLookupFailure::InvalidRequest,
            Failure::InvalidRequest,
        ),
    ] {
        let bytes = candid::encode_one(Err::<FundingLookupView, _>(failure)).unwrap();
        assert_eq!(report(&selection, &bytes), Err(expected));
    }
    assert_eq!(
        report(&selection, b"not candid"),
        Err(Failure::InvalidReply)
    );
    assert_eq!(report(&selection, &[0; 4097]), Err(Failure::ReplyTooLarge));
}
