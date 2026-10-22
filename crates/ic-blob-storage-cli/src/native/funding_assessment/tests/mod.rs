use super::*;
use ic_blob_storage_contracts::dto::operator::LocalFundingStatus;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_platform::Principal;
fn request() -> Request {
    Request {
        scope: OperatorScope {
            service: Principal::self_authenticating([1]),
            namespace: u128::MAX,
            cashier: Principal::self_authenticating([2]),
            payment_account: Principal::self_authenticating([3]),
        },
        operation: u128::MAX,
        offered: u128::MAX,
        target_balance: Some(u128::MAX),
    }
}
fn response() -> Response {
    Response {
        request: request(),
        journal: LocalFundingStatus {
            cumulative_allocation: u128::MAX,
            renewal_ceiling: u128::MAX,
            available_allocation: u128::MAX,
            attachment_allowance: u128::MAX - 1,
            transport_accepted: 0,
            uncredited_accepted: 0,
            provider_acknowledged: 0,
            refunded: 0,
            not_enqueued: 0,
            reserved_or_uncertain: 0,
            retained_intents: 0,
            intent_capacity: u64::MAX,
            last_operation: None,
            fenced: false,
        },
        blockers: vec![
            B::ProviderUnqualified,
            B::RecoveryUnknown,
            B::FundingUnknown,
            B::SpendabilityUnknown,
            B::AllocationReserve {
                transferable_cycles: u128::MAX - 1,
            },
        ],
    }
}
fn bytes(value: &Response) -> Vec<u8> {
    ic_blob_storage_contracts::wire::encode_response(
        "blob_funding_preparation_assessment",
        Ok::<_, E>(value),
    )
    .unwrap()
}
fn arguments() -> Vec<String> {
    let r = request();
    [
        "funding-assessment",
        "--network",
        "ic",
        "--url",
        "https://icp-api.io",
        "--identity",
        "absent.pem",
        "--operator",
        &r.scope.service.to_text(),
        "--service",
        &r.scope.service.to_text(),
        "--namespace",
        &r.scope.namespace.to_string(),
        "--cashier",
        &r.scope.cashier.to_text(),
        "--payer",
        &r.scope.payment_account.to_text(),
        "--operation",
        &r.operation.to_string(),
        "--offered",
        &r.offered.to_string(),
        "--target-balance",
        &u128::MAX.to_string(),
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
#[test]
fn proposed_intent_retains_exact_width_and_absent_target_without_authority_claims() {
    let reply = response();
    assert_eq!(decode(request(), &bytes(&reply)), Ok(reply.clone()));
    let options = Options::parse(&arguments()).unwrap().1;
    let value = output(&options, &reply);
    assert_eq!(value["request"]["operation"], u128::MAX.to_string());
    assert_eq!(value["request"]["offered"], u128::MAX.to_string());
    assert_eq!(
        value["journal"]["attachment_allowance"],
        (u128::MAX - 1).to_string()
    );
    for field in [
        "preparation_authorized",
        "dispatch_authorized",
        "retry_authorized",
    ] {
        assert_eq!(value[field], false);
    }
    let mut args = arguments();
    args.pop();
    args.pop();
    assert!(matches!(
        Options::parse(&args).unwrap().0,
        super::super::arguments::Command::FundingAssessment(Request {
            target_balance: None,
            ..
        })
    ));
    args.extend(["--provider-qualified".into(), "true".into()]);
    assert!(matches!(Options::parse(&args), Err(Failure::Arguments)));
}
#[test]
fn decoder_preserves_contract_refusal_codes() {
    let mut changed = response();
    changed.request.target_balance = None;
    assert_eq!(decode(request(), &bytes(&changed)), Err(Failure::Binding));
    assert_eq!(decode(request(), &[0]), Err(Failure::InvalidReply));
    assert_eq!(
        decode(request(), &vec![0; 4097]),
        Err(Failure::ReplyTooLarge)
    );
    for error in [E::Invalid, E::Denied, E::Binding, E::Conflict, E::Internal] {
        assert_eq!(
            decode(
                request(),
                &ic_blob_storage_contracts::wire::encode_response(
                    "blob_funding_preparation_assessment",
                    Err::<Response, _>(error)
                )
                .unwrap()
            ),
            Err(Failure::FundingAssessmentRefused(error)),
        );
    }
}
