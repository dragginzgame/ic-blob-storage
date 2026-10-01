use super::*;
use candid::{Int, Nat, Principal};
use ic_blob_storage::dto::{
    account::AccountRelationshipView, funding::outcome::FundingReportedBalance,
    operator::OperatorScope,
};

fn request(kind: Kind) -> AccountInspectionRequest {
    AccountInspectionRequest {
        scope: OperatorScope {
            service: Principal::from_slice(&[1, 1]),
            namespace: u128::MAX,
            cashier: Principal::from_slice(&[2, 1]),
            payment_account: Principal::from_slice(&[3, 1]),
        },
        kind,
    }
}
fn arguments(request: AccountInspectionRequest) -> Vec<String> {
    [
        "inspect-account",
        "--network",
        "ic",
        "--url",
        "https://icp-api.io",
        "--identity",
        "unused.pem",
        "--operator",
        &request.scope.service.to_text(),
        "--service",
        &request.scope.service.to_text(),
        "--namespace",
        &request.scope.namespace.to_string(),
        "--cashier",
        &request.scope.cashier.to_text(),
        "--payer",
        &request.scope.payment_account.to_text(),
        "--kind",
        match request.kind {
            Kind::Balance => "balance",
            Kind::PaymentRelationship => "relationship",
        },
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
fn encoded(response: &AccountInspectionResponse) -> Vec<u8> {
    candid::encode_one(Ok::<_, E>(response)).unwrap()
}
fn relationship(request: AccountInspectionRequest) -> AccountRelationshipView {
    AccountRelationshipView {
        paid_canister: request.scope.service,
        payment_account: request.scope.payment_account,
        spending_limit_per_day: Int::parse(b"-340282366920938463463374607431768211456").unwrap(),
        current_period_spent: Int::parse(b"340282366920938463463374607431768211456").unwrap(),
        current_period_start: u64::MAX,
        added_timestamp: 1,
        expiration_timestamp: None,
        bandwidth_baseline_uploaded: Nat::parse(b"340282366920938463463374607431768211456")
            .unwrap(),
        bandwidth_baseline_downloaded: 2u64.into(),
        bandwidth_baseline_ts_ns: 3,
    }
}

#[test]
fn explicit_kind_and_complete_installed_account_scope_are_required() {
    for kind in [Kind::Balance, Kind::PaymentRelationship] {
        let request = request(kind);
        assert!(
            matches!(Options::parse(&arguments(request)).unwrap().command,
            super::super::arguments::Command::InspectAccount(value) if value == request)
        );
    }
    let mut args = arguments(request(Kind::Balance));
    args.pop();
    args.pop();
    assert!(matches!(Options::parse(&args), Err(Failure::Arguments)));
    args.extend(["--kind".into(), "both".into()]);
    assert!(matches!(Options::parse(&args), Err(Failure::Arguments)));
}

#[test]
fn reported_balance_keeps_independent_full_width_amounts_without_credit() {
    let request = request(Kind::Balance);
    let response = AccountInspectionResponse {
        request,
        observation: Observation::Balance(FundingReportedBalance {
            total: u128::MAX,
            prepaid: 2,
            promotional: 3,
            ledger: 4,
        }),
    };
    assert_eq!(decode(request, &encoded(&response)), Ok(response.clone()));
    let output = output(&response, &Options::parse(&arguments(request)).unwrap());
    assert_eq!(output["provider_report"]["total"], u128::MAX.to_string());
    assert_eq!(output["scope"]["namespace"], u128::MAX.to_string());
    assert_eq!(output["provider_credit"], "not_established");
    assert_eq!(output["spendability"], "not_established");
    assert_eq!(output["retry_authorized"], false);
}

#[test]
fn relationship_json_preserves_signed_and_arbitrary_width_decimal_values() {
    let request = request(Kind::PaymentRelationship);
    let response = AccountInspectionResponse {
        request,
        observation: Observation::Relationship(Box::new(relationship(request))),
    };
    assert_eq!(decode(request, &encoded(&response)), Ok(response.clone()));
    let output = output(&response, &Options::parse(&arguments(request)).unwrap());
    let report = &output["provider_report"];
    assert_eq!(
        report["spending_limit_per_day"],
        "-340282366920938463463374607431768211456"
    );
    for key in ["current_period_spent", "bandwidth_baseline_uploaded"] {
        assert_eq!(report[key], "340282366920938463463374607431768211456");
    }
    assert_eq!(report["current_period_start"], u64::MAX.to_string());
    assert_eq!(report["expiration_timestamp"], Value::Null);
}

#[test]
fn bounded_reply_rejects_scope_kind_and_relationship_substitution() {
    let request = request(Kind::PaymentRelationship);
    let original = AccountInspectionResponse {
        request,
        observation: Observation::Relationship(Box::new(relationship(request))),
    };
    let mut changed = original.clone();
    changed.request.scope.namespace = 1;
    assert_eq!(decode(request, &encoded(&changed)), Err(Failure::Binding));
    changed = original.clone();
    changed.request.kind = Kind::Balance;
    assert_eq!(decode(request, &encoded(&changed)), Err(Failure::Binding));
    for payer in [true, false] {
        let mut value = relationship(request);
        if payer {
            value.payment_account = request.scope.service;
        } else {
            value.paid_canister = request.scope.cashier;
        }
        changed.observation = Observation::Relationship(Box::new(value));
        changed.request = request;
        assert_eq!(decode(request, &encoded(&changed)), Err(Failure::Binding));
    }
    changed.observation = Observation::AccountNotFound;
    assert_eq!(
        decode(request, &encoded(&changed)),
        Err(Failure::InvalidReply)
    );
    assert_eq!(decode(request, b"DIDL"), Err(Failure::InvalidReply));
    assert_eq!(decode(request, &vec![0; 4097]), Err(Failure::ReplyTooLarge));
}

#[test]
fn provider_absence_and_errors_remain_reports_while_service_refusals_remain_failures() {
    let principal = Principal::anonymous();
    for (kind, observation) in [
        (Kind::Balance, Observation::AccountNotFound),
        (Kind::Balance, Observation::ProviderInternalError),
        (
            Kind::PaymentRelationship,
            Observation::NoRelationshipReported,
        ),
        (
            Kind::PaymentRelationship,
            Observation::RelationshipNotFound(principal),
        ),
        (
            Kind::PaymentRelationship,
            Observation::NotAuthorized(principal),
        ),
        (Kind::PaymentRelationship, Observation::InvalidRequest),
        (
            Kind::PaymentRelationship,
            Observation::ProviderInternalError,
        ),
    ] {
        let request = request(kind);
        let response = AccountInspectionResponse {
            request,
            observation,
        };
        assert_eq!(decode(request, &encoded(&response)), Ok(response));
    }
    for error in [
        E::Invalid,
        E::Denied,
        E::Binding,
        E::Fenced,
        E::Internal,
        E::NotEnqueued,
        E::Rejected(4),
        E::ReplyTooLarge,
        E::InvalidReply,
    ] {
        assert_eq!(
            decode(
                request(Kind::Balance),
                &candid::encode_one(Err::<AccountInspectionResponse, _>(error)).unwrap()
            ),
            Err(Failure::AccountRefused(error))
        );
    }
}
