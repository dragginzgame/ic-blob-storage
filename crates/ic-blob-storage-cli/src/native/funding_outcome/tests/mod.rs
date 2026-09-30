use super::super::arguments::Command;
use super::*;
use candid::Principal;
use ic_blob_storage::dto::{
    funding::{
        FundingPhase,
        outcome::{FundingOutcomeResponse, FundingReportedBalance},
    },
    operator::OperatorScope,
};

fn input() -> FundingOutcomeRequest {
    FundingOutcomeRequest {
        scope: OperatorScope {
            service: Principal::self_authenticating([1]),
            namespace: u128::MAX,
            cashier: Principal::self_authenticating([2]),
            payment_account: Principal::self_authenticating([3]),
        },
        operation: u128::MAX,
        offered: u128::MAX,
        target_balance: None,
    }
}
fn args() -> Vec<String> {
    let i = input();
    [
        "funding-outcome",
        "--network",
        "ic",
        "--url",
        "https://icp-api.io",
        "--identity",
        "unused.pem",
        "--operator",
        &i.scope.payment_account.to_text(),
        "--service",
        &i.scope.service.to_text(),
        "--namespace",
        &i.scope.namespace.to_string(),
        "--cashier",
        &i.scope.cashier.to_text(),
        "--payer",
        &i.scope.payment_account.to_text(),
        "--operation",
        &i.operation.to_string(),
        "--offered",
        &i.offered.to_string(),
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
fn render(view: Option<&FundingOutcomeResponse>) -> Value {
    output(
        input(),
        &candid::encode_one(Ok::<_, FundingOutcomeFailure>(view)).unwrap(),
        &Options::parse(&args()).unwrap(),
    )
    .unwrap()
}

#[test]
fn exact_original_amounts_and_optional_target_are_required_before_transport() {
    let Command::FundingOutcome(parsed) = Options::parse(&args()).unwrap().command else {
        panic!("wrong command")
    };
    assert_eq!(parsed, input());
    let mut targeted = args();
    targeted.extend(["--target-balance".into(), u128::MAX.to_string()]);
    let Command::FundingOutcome(parsed) = Options::parse(&targeted).unwrap().command else {
        panic!("wrong command")
    };
    assert_eq!(parsed.target_balance, Some(u128::MAX));
    for flag in ["--operation", "--offered", "--target-balance"] {
        for invalid in [
            "0",
            "01",
            "+1",
            "-1",
            "340282366920938463463374607431768211456",
        ] {
            let mut changed = targeted.clone();
            let index = changed.iter().position(|s| s == flag).unwrap();
            changed[index + 1] = invalid.into();
            assert_eq!(super::super::execute(&changed), Err(Failure::Arguments));
        }
    }
    let mut forbidden = args();
    forbidden.extend(["--retry".into(), "true".into()]);
    assert_eq!(super::super::execute(&forbidden), Err(Failure::Arguments));
}

#[test]
fn absence_and_transport_facts_never_establish_credit_or_retry_authority() {
    let absent = render(None);
    assert_eq!(absent["outcome"], "absent");
    assert_eq!(absent["record"], Value::Null);
    assert_eq!(absent["target_balance"], Value::Null);
    for phase in [FundingPhase::Prepared, FundingPhase::Uncertain] {
        let value = render(Some(&FundingOutcomeResponse {
            request: input(),
            phase,
            response: None,
            reconciliation: FundingReconciliation::TransferUnknown(u128::MAX),
            fenced: true,
        }));
        assert_eq!(value["record"]["response"], Value::Null);
        assert_eq!(
            value["record"]["reconciliation"]["offered"],
            u128::MAX.to_string()
        );
        assert_eq!(value["record"]["fenced"], true);
        assert_eq!(value["retry_authorized"], false);
        assert_eq!(value["provider_credit"], "not_established");
    }
    let reported = render(Some(&FundingOutcomeResponse {
        request: input(),
        phase: FundingPhase::Callback {
            refunded: u128::MAX - 1,
        },
        response: Some(FundingResponse::ReportedSuccess(FundingReportedBalance {
            total: u128::MAX,
            prepaid: u128::MAX - 1,
            promotional: 1,
            ledger: 0,
        })),
        reconciliation: FundingReconciliation::CreditRequired(1),
        fenced: false,
    }));
    assert_eq!(
        reported["record"]["phase"]["refunded"],
        (u128::MAX - 1).to_string()
    );
    assert_eq!(
        reported["record"]["response"]["balance"]["total"],
        u128::MAX.to_string()
    );
    assert_eq!(
        reported["record"]["reconciliation"],
        json!({"state":"credit_required","accepted":"1"})
    );
    assert_eq!(reported["provider_credit"], "not_established");
    let refunded = render(Some(&FundingOutcomeResponse {
        request: input(),
        phase: FundingPhase::Callback {
            refunded: u128::MAX,
        },
        response: Some(FundingResponse::InvalidBalance(FundingBalanceField::Ledger)),
        reconciliation: FundingReconciliation::NoTransfer,
        fenced: false,
    }));
    assert_eq!(
        refunded["record"]["response"],
        json!({"state":"invalid_balance","field":"ledger"})
    );
    assert_eq!(refunded["record"]["reconciliation"]["state"], "no_transfer");
    assert_eq!(refunded["retry_authorized"], false);
    assert_eq!(absent["retry_authorized"], false);
}

#[test]
fn exact_intent_conflict_binding_and_decode_failure_remain_observable() {
    let options = Options::parse(&args()).unwrap();
    let refused: Result<Option<FundingOutcomeResponse>, _> = Err(FundingOutcomeFailure::Conflict);
    assert_eq!(
        output(input(), &candid::encode_one(refused).unwrap(), &options),
        Err(Failure::FundingConflict)
    );
    let changed = FundingOutcomeResponse {
        request: FundingOutcomeRequest {
            target_balance: Some(1),
            ..input()
        },
        phase: FundingPhase::NotEnqueued,
        response: Some(FundingResponse::NotEnqueued),
        reconciliation: FundingReconciliation::NoTransfer,
        fenced: false,
    };
    assert_eq!(
        output(
            input(),
            &candid::encode_one(Ok::<_, FundingOutcomeFailure>(Some(changed))).unwrap(),
            &options
        ),
        Err(Failure::Binding)
    );
    assert_eq!(
        output(input(), b"not candid", &options),
        Err(Failure::InvalidReply)
    );
    assert_eq!(
        output(input(), &vec![0; 4097], &options),
        Err(Failure::ReplyLimit)
    );
}
